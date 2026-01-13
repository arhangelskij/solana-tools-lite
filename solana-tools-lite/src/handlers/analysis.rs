use crate::codec::serialize_transaction;
use crate::constants::{compute_budget, programs};
use crate::extensions::light_protocol::LightProtocol;
use crate::extensions::traits::ProtocolAnalyzer;
use crate::models::analysis::{
    AnalysisWarning, PrivacyLevel, SigningSummary, TokenProgramKind, TransferView, TxAnalysis,
};
use crate::models::extensions::ExtensionAction;
use crate::models::message::{Message, MessageAddressTableLookup};
use crate::models::pubkey_base58::PubkeyBase58;
use crate::models::transaction::Transaction;
use crate::ToolError;
use std::collections::{HashMap, HashSet};

/// Estimated base fee per signature in lamports (not fetched from cluster).
const ESTIMATED_BASE_FEE_PER_SIGNATURE: u64 = 5000;
/// System Program instruction discriminator for Transfer.
const SYSTEM_TRANSFER_TAG: u32 = 2;
const SYSTEM_TRANSFER_DATA_LEN: usize = 12; // tag (u32) + lamports (u64)
/// ComputeBudget instruction discriminator for SetComputeUnitLimit.
const COMPUTE_BUDGET_SET_UNIT_LIMIT: u8 = 2;
/// ComputeBudget instruction discriminator for SetComputeUnitPrice.
const COMPUTE_BUDGET_SET_UNIT_PRICE: u8 = 3;
const COMPUTE_BUDGET_TAG_LEN: usize = 1;
const COMPUTE_UNIT_LIMIT_LEN: usize = 4;
const COMPUTE_UNIT_PRICE_LEN: usize = 8;
const MICRO_LAMPORTS_PER_LAMPORT: u128 = 1_000_000;

/// Internal state used to collect metrics and flags during transaction analysis.
#[derive(Default)]
struct AnalysisState {
    transfers: Vec<TransferView>,
    total_send_by_signer: u128,
    saw_token_spl: bool,
    saw_token_2022: bool,
    cu_price_micro: Option<u64>,
    cu_limit: Option<u32>,
    extension_actions: Vec<ExtensionAction>,
}

/// Analyze a message to produce fee estimates, transfers, and warnings.
///
/// For v0 messages, resolves loaded accounts using lookup tables when provided.
/// When tables are missing, analysis falls back to static accounts and emits warnings.
pub fn analyze_transaction(
    message: &Message,
    signer: &PubkeyBase58,
    tables: Option<&HashMap<PubkeyBase58, Vec<PubkeyBase58>>>,
) -> TxAnalysis {
    let mut state = AnalysisState::default();
    let mut warnings = Vec::new();
    let mut unknown_programs: HashSet<PubkeyBase58> = HashSet::new();
    
    // Extensions
    let analyzers: Vec<Box<dyn ProtocolAnalyzer>> = vec![Box::new(LightProtocol)];

    let (account_list, instructions, message_version, address_lookups) = match message {
        Message::Legacy(m) => (m.account_keys.clone(), m.instructions.clone(), "legacy", None),
        Message::V0(v0) => (
            resolve_v0_accounts(
                v0.account_keys.clone(),
                &v0.address_table_lookups,
                tables,
                &mut warnings,
            ),
            v0.instructions.clone(),
            "v0",
            Some(&v0.address_table_lookups),
        ),
    };

    if let Some(lookups) = address_lookups {
        if !lookups.is_empty() && tables.is_none() {
            warnings.push(AnalysisWarning::LookupTablesNotProvided);
        }
    }

    // Detect compute budget settings

    for instr in &instructions {
        let program_id = account_list.get(instr.program_id_index as usize);
        let program_id = match program_id {
            Some(pk) => pk,
            None => continue,
        };

        // Run protocol extensions
        for analyzer in &analyzers {
            if let Some(action) = analyzer.analyze(program_id, &instr.data) {
                state.extension_actions.push(action);
            }
        }

        if program_id == programs::system_program() && instr.data.len() >= SYSTEM_TRANSFER_DATA_LEN {
            let kind = u32::from_le_bytes(instr.data[0..4].try_into().unwrap());
            if kind == SYSTEM_TRANSFER_TAG && instr.accounts.len() >= 2 {
                let lamports = u64::from_le_bytes(instr.data[4..12].try_into().unwrap());
                let from = account_to_string(&account_list, instr.accounts[0]);
                let to = account_to_string(&account_list, instr.accounts[1]);
                let from_is_signer = account_list
                    .get(instr.accounts[0] as usize)
                    .map(|pk| pk == signer)
                    .unwrap_or(false);
                if from_is_signer {
                    state.total_send_by_signer += lamports as u128;
                }
                state.transfers.push(TransferView {
                    from,
                    to,
                    lamports,
                    from_is_signer,
                });
            }
        } else if program_id == programs::compute_budget_program() && !instr.data.is_empty() {
            match instr.data[0] {
                COMPUTE_BUDGET_SET_UNIT_LIMIT => {
                    // SetComputeUnitLimit
                    if instr.data.len() >= COMPUTE_BUDGET_TAG_LEN + COMPUTE_UNIT_LIMIT_LEN {
                        state.cu_limit = Some(u32::from_le_bytes(instr.data[1..5].try_into().unwrap()));
                    }
                }
                COMPUTE_BUDGET_SET_UNIT_PRICE => {
                    // SetComputeUnitPrice
                    if instr.data.len() >= COMPUTE_BUDGET_TAG_LEN + COMPUTE_UNIT_PRICE_LEN {
                        state.cu_price_micro =
                            Some(u64::from_le_bytes(instr.data[1..9].try_into().unwrap()));
                    }
                }
                _ => {}
            }
        } else if program_id == programs::token_program() {
            state.saw_token_spl = true;
        } else if program_id == programs::token_2022_program() {
            state.saw_token_2022 = true;
        } else {
            unknown_programs.insert(program_id.clone());
        }
    }

    if state.saw_token_spl {
        warnings.push(AnalysisWarning::TokenTransferDetected(
            TokenProgramKind::SplToken,
        ));
    }
    if state.saw_token_2022 {
        warnings.push(AnalysisWarning::TokenTransferDetected(
            TokenProgramKind::Token2022,
        ));
    }
    for program_id in unknown_programs {
        warnings.push(AnalysisWarning::UnknownProgram { program_id });
    }

    let base_fee_lamports = ESTIMATED_BASE_FEE_PER_SIGNATURE as u128
        * signature_count(message) as u128;
    let priority_fee_lamports = state.cu_price_micro.map(|price_micro| {
        let limit = state.cu_limit.unwrap_or(compute_budget::DEFAULT_COMPUTE_UNIT_LIMIT);
        let fee = (price_micro as u128 * limit as u128) / MICRO_LAMPORTS_PER_LAMPORT;
        let estimated = state.cu_limit.is_none();
        (fee, estimated)
    });

    let total_fee_lamports =
        base_fee_lamports + priority_fee_lamports.map(|(f, _)| f).unwrap_or(0);

    TxAnalysis {
        transfers: state.transfers,
        base_fee_lamports,
        priority_fee_lamports,
        total_fee_lamports,
        total_send_by_signer: state.total_send_by_signer,
        compute_unit_limit: state.cu_limit,
        compute_unit_price_micro: state.cu_price_micro,
        warnings,
        message_version,
        privacy_level: PrivacyLevel::Public, // Default for now
        extension_actions: state.extension_actions,
    }
}

/// Build a signing summary from a signed transaction and its analysis.
///
/// Returns a struct for serialization; does not emit JSON.
pub fn build_signing_summary(
    tx: &Transaction,
    analysis: &TxAnalysis,
) -> Result<SigningSummary, ToolError> {
    let to_u64 = |v: u128| -> Result<u64, ToolError> {
        u64::try_from(v).map_err(|_| ToolError::InvalidInput("lamports overflowed u64".into()))
    };

    let raw = serialize_transaction(tx);
    let signed_tx_base64 = data_encoding::BASE64.encode(&raw);
    let signatures: Vec<String> = tx
        .signatures
        .iter()
        .map(|s| bs58::encode(s.to_bytes()).into_string())
        .collect();

    let (priority_fee_lamports, priority_fee_estimated) =
        if let Some((fee, est)) = analysis.priority_fee_lamports {
            (to_u64(fee)?, est)
        } else {
            (0, false)
        };

    Ok(SigningSummary {
        message_version: analysis.message_version.to_string(),
        signatures,
        signed_tx_base64,
        base_fee_lamports: to_u64(analysis.base_fee_lamports)?,
        priority_fee_lamports,
        priority_fee_estimated,
        fee_is_estimate: priority_fee_estimated,
        compute_unit_price_micro: analysis.compute_unit_price_micro,
        compute_unit_limit: analysis.compute_unit_limit,
        total_fee_lamports: to_u64(analysis.total_fee_lamports)?,
        total_send_by_signer: to_u64(analysis.total_send_by_signer)?,
        max_total_cost_lamports: to_u64(
            analysis.total_fee_lamports + analysis.total_send_by_signer,
        )?,
        warnings: analysis.warnings.clone(),
        extension_actions: analysis.extension_actions.clone(),
    })
}

/// Parse lookup tables JSON in the form:
/// { "<table_b58>": ["addr1", "addr2", ...], ... }
///
/// Returns a typed map keyed by lookup table account pubkeys.
pub fn parse_lookup_tables(
    json: &str,
) -> Result<HashMap<PubkeyBase58, Vec<PubkeyBase58>>, ToolError> {
    let raw: HashMap<String, Vec<String>> = serde_json::from_str(json)
        .map_err(|e| ToolError::InvalidInput(format!("invalid lookup tables JSON: {e}")))?;

    let mut out = HashMap::new();
    for (table_key, addresses) in raw {
        let table_pk = PubkeyBase58::try_from(table_key.as_str()).map_err(|e| {
            ToolError::InvalidInput(format!("invalid lookup table key {table_key}: {e}"))
        })?;

        let mut parsed = Vec::with_capacity(addresses.len());
        for addr in addresses {
            let pk = PubkeyBase58::try_from(addr.as_str()).map_err(|e| {
                ToolError::InvalidInput(format!(
                    "invalid lookup address {addr} for table {table_key}: {e}"
                ))
            })?;
            parsed.push(pk);
        }
        out.insert(table_pk, parsed);
    }

    Ok(out)
}

/// Resolve V0 message accounts by merging static keys with lookup table entries.
/// Emits warnings for missing or incomplete lookup tables.
fn resolve_v0_accounts(
    static_keys: Vec<PubkeyBase58>,
    lookups: &[MessageAddressTableLookup],
    tables: Option<&HashMap<PubkeyBase58, Vec<PubkeyBase58>>>,
    warnings: &mut Vec<AnalysisWarning>,
) -> Vec<PubkeyBase58> {
    let mut combined = static_keys;
    let mut missing_tables: HashSet<PubkeyBase58> = HashSet::new();

    if let Some(map) = tables {
        for lut in lookups {
            let Some(entries) = map.get(&lut.account_key) else {
                missing_tables.insert(lut.account_key.clone());
                continue;
            };

            let mut push_index = |idx: u8| {
                let i = idx as usize;
                if let Some(pk) = entries.get(i) {
                    combined.push(pk.clone());
                } else {
                    missing_tables.insert(lut.account_key.clone());
                }
            };

            for idx in &lut.writable_indexes {
                push_index(*idx);
            }
            for idx in &lut.readonly_indexes {
                push_index(*idx);
            }
        }
    } else if !lookups.is_empty() {
        // Tables not provided; only static keys available.
        return combined;
    }

    for key in missing_tables {
        warnings.push(AnalysisWarning::LookupTableMissing(key));
    }

    combined
}

/// Return the number of required signatures for this message.
fn signature_count(message: &Message) -> usize {
    message.header().num_required_signatures as usize
}

/// Convert account index to Base58 string, or "index#N" if out of bounds.
fn account_to_string(accounts: &[PubkeyBase58], index: u8) -> String {
    accounts
        .get(index as usize)
        .map(|pk| pk.to_string())
        .unwrap_or_else(|| format!("index#{}", index))
}

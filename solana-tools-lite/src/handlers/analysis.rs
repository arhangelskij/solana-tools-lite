use crate::codec::serialize_transaction;
use crate::constants::{compute_budget, programs};
use crate::extensions::registry;
use crate::models::analysis::{
    AnalysisWarning, PrivacyLevel, SigningSummary, TokenProgramKind, TransferView, TxAnalysis,
};
use crate::models::extensions::{AnalysisExtensionAction, PrivacyImpact};
use crate::models::instruction::Instruction;
use crate::models::message::{Message, MessageAddressTableLookup};
use crate::models::pubkey_base58::PubkeyBase58;
use crate::models::transaction::Transaction;
use crate::ToolError;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

// --- Constants ---
const ESTIMATED_BASE_FEE_PER_SIGNATURE: u64 = 5000;
const MICRO_LAMPORTS_PER_LAMPORT: u128 = 1_000_000;

// System Program
const SYSTEM_TRANSFER_TAG: u32 = 2;
const SYSTEM_TRANSFER_DATA_LEN: usize = 12; // tag (4) + lamports (8)

// Compute Budget
const COMPUTE_BUDGET_SET_UNIT_LIMIT: u8 = 2;
const COMPUTE_BUDGET_SET_UNIT_PRICE: u8 = 3;
const COMPUTE_BUDGET_TAG_LEN: usize = 1;
const COMPUTE_UNIT_LIMIT_LEN: usize = 4;
const COMPUTE_UNIT_PRICE_LEN: usize = 8;

// Limits for Anti-DoS
const MAX_TRANSFERS_TO_DISPLAY: usize = 50;

/// Internal state used to collect metrics and flags during transaction analysis.
#[derive(Default)]
struct AnalysisState {
    transfers: Vec<TransferView>,
    total_sol_send_by_signer: u128,
    saw_token_spl: bool,
    saw_token_2022: bool,
    saw_system_transfer: bool,
    unknown_programs: HashSet<PubkeyBase58>,
    cu_price_micro: Option<u64>,
    cu_limit: Option<u32>,
    extension_actions: Vec<AnalysisExtensionAction>,
    // Counts for aggregated display
    confidential_ops_count: usize,
    storage_ops_count: usize,
    is_fee_payer: bool,
}

enum ComputeBudgetAction {
    SetLimit(u32),
    SetPrice(u64),
    None,
}

/// Analyze a message to produce fee estimates, transfers, and warnings.
pub fn analyze_transaction(
    message: &Message,
    signer: &PubkeyBase58,
    tables: Option<&HashMap<PubkeyBase58, Vec<PubkeyBase58>>>,
) -> TxAnalysis {
    let mut warnings = Vec::new();

    // 1. Resolve message components
    let (account_list, instructions, message_version, address_lookups) =
        resolve_message_components(message, tables, &mut warnings);

    // [Security Check] Verify if the provided signer is actually required to sign
    verify_signer_requirement(message, &account_list, signer, &mut warnings);

    // [Fee Payer Check] The first account in any Solana message is the fee payer.
    let is_fee_payer = account_list.first().map(|pk| pk == signer).unwrap_or(false);

    if let Some(lookups) = address_lookups {
        if !lookups.is_empty() && tables.is_none() {
            warnings.push(AnalysisWarning::LookupTablesNotProvided);
        }
    }

    // 2. Process instructions
    let mut state = AnalysisState {
        is_fee_payer,
        ..Default::default()
    };

    for instr in instructions {
        let program_id = match account_list.get(instr.program_id_index as usize) {
            Some(pk) => pk,
            None => continue,
        };

        let mut handled = false;

//TODO: 🟡 mb refactoring
        if program_id == programs::system_program() {
            handled = true;
            if let Some(lamports) = parse_system_transfer_amount(&instr.data) {
                // Ensure we have at least 2 accounts (from, to)
                if instr.accounts.len() >= 2 {
                    state.saw_system_transfer = true;
                    process_transfer(&mut state, &account_list, instr, lamports, signer);
                }
            }
        } else if program_id == programs::compute_budget_program() {
            handled = true;
            match parse_compute_budget(&instr.data) {
                ComputeBudgetAction::SetLimit(l) => state.cu_limit = Some(l),
                ComputeBudgetAction::SetPrice(p) => state.cu_price_micro = Some(p),
                ComputeBudgetAction::None => {}
            }
        } else if program_id == programs::token_program() {
            handled = true;
            state.saw_token_spl = true;
        } else if program_id == programs::token_2022_program() {
            handled = true;
            state.saw_token_2022 = true;
        }

        if !handled {
            state.unknown_programs.insert(program_id.clone());
        }
    }

    // 3. Finalize results
    let mut analysis = finalize_analysis(message, state, warnings, message_version);

    // 4. Run protocol extensions (Plugins)
    let plugins = registry::get_all_analyzers();
    
    for plugin in plugins {
        if plugin.detect(message) {
            plugin.analyze(message, &account_list, signer, &mut analysis);
            plugin.enrich_notice(&mut analysis);
            
            //TODO: 🔴 refactoring
            // Remove this plugin's programs from unknown warnings
            if let Ok(supported) = plugin.supported_programs() {
                analysis.warnings.retain(|w| {
                    if let AnalysisWarning::UnknownProgram { program_id } = w {
                        !supported.contains(program_id)
                    } else {
                        true
                    }
                });
            }
        }
    }

    // Refresh privacy level after plugins
    analysis.recalculate_privacy_level();

    analysis
}

/// Verify that the current user (signer) is actually listed as a required signer in the message header.
fn verify_signer_requirement(
    message: &Message,
    accounts: &[PubkeyBase58],
    signer: &PubkeyBase58,
    warnings: &mut Vec<AnalysisWarning>,
) {
    let num_required_signatures = message.header().num_required_signatures as usize;
    
    // The first `num_required_signatures` accounts in the list are the signers.
    let signing_accounts = if accounts.len() >= num_required_signatures {
        &accounts[0..num_required_signatures]
    } else {
        &accounts[..] // Should not happen for valid messages, but safety first
    };

    let is_required = signing_accounts.iter().any(|pk| pk == signer);

    if !is_required {
        warnings.push(AnalysisWarning::SignerNotRequired); 
    }
}

fn resolve_message_components<'a>(
    message: &'a Message,
    tables: Option<&HashMap<PubkeyBase58, Vec<PubkeyBase58>>>,
    warnings: &mut Vec<AnalysisWarning>,
) -> (
    Cow<'a, [PubkeyBase58]>,
    &'a [Instruction],
    &'static str,
    Option<&'a [MessageAddressTableLookup]>,
) {
    match message {
        Message::Legacy(m) => (
            Cow::Borrowed(&m.account_keys),
            &m.instructions,
            "legacy",
            None,
        ),
        Message::V0(v0) => (
            Cow::Owned(resolve_v0_accounts(
                &v0.account_keys,
                &v0.address_table_lookups,
                tables,
                warnings,
            )),
            &v0.instructions,
            "v0",
            Some(&v0.address_table_lookups),
        ),
    }
}

fn resolve_v0_accounts(
    static_keys: &[PubkeyBase58],
    lookups: &[MessageAddressTableLookup],
    tables: Option<&HashMap<PubkeyBase58, Vec<PubkeyBase58>>>,
    warnings: &mut Vec<AnalysisWarning>,
) -> Vec<PubkeyBase58> {
    let mut combined = static_keys.to_vec();
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
        return combined;
    }

    for key in missing_tables {
        warnings.push(AnalysisWarning::LookupTableMissing(key));
    }

    combined
}

fn parse_system_transfer_amount(data: &[u8]) -> Option<u64> {
    if data.len() < SYSTEM_TRANSFER_DATA_LEN {
        return None;
    }
    // Safe slice access checked by len check above
    let kind = u32::from_le_bytes(data[0..4].try_into().ok()?);
    if kind == SYSTEM_TRANSFER_TAG {
        return Some(u64::from_le_bytes(data[4..12].try_into().ok()?));
    }
    None
}

fn parse_compute_budget(data: &[u8]) -> ComputeBudgetAction {
    if data.is_empty() {
        return ComputeBudgetAction::None;
    }
    match data[0] {
        COMPUTE_BUDGET_SET_UNIT_LIMIT => {
            if data.len() >= COMPUTE_BUDGET_TAG_LEN + COMPUTE_UNIT_LIMIT_LEN {
                // Strict parsing: if try_into fails (shouldn't due to len check), return None
                if let Ok(bytes) = data[1..5].try_into() {
                    return ComputeBudgetAction::SetLimit(u32::from_le_bytes(bytes));
                }
            }
            ComputeBudgetAction::None
        }
        COMPUTE_BUDGET_SET_UNIT_PRICE => {
            if data.len() >= COMPUTE_BUDGET_TAG_LEN + COMPUTE_UNIT_PRICE_LEN {
                if let Ok(bytes) = data[1..9].try_into() {
                    return ComputeBudgetAction::SetPrice(u64::from_le_bytes(bytes));
                }
            }
            ComputeBudgetAction::None
        }
        _ => ComputeBudgetAction::None,
    }
}

fn process_transfer(
    state: &mut AnalysisState,
    accounts: &[PubkeyBase58],
    instr: &Instruction,
    lamports: u64,
    signer: &PubkeyBase58,
) {
    // Anti-DoS: Don't collect thousands of transfers
    if state.transfers.len() >= MAX_TRANSFERS_TO_DISPLAY {
        return;
    }

    let Some(&from_idx) = instr.accounts.first() else { return };
    let Some(&to_idx) = instr.accounts.get(1) else { return };

    let from = account_to_string(accounts, from_idx);
    let to = account_to_string(accounts, to_idx);

    let from_is_signer = accounts
        .get(from_idx as usize)
        .map(|pk| pk == signer)
        .unwrap_or(false);

    if from_is_signer {
        // Safe saturating add to prevent overflow in accumulation
        state.total_sol_send_by_signer = state.total_sol_send_by_signer.saturating_add(lamports as u128);
    }

    state.transfers.push(TransferView {
        from,
        to,
        lamports,
        from_is_signer,
    });
}

fn finalize_analysis(
    message: &Message,
    state: AnalysisState,
    mut warnings: Vec<AnalysisWarning>,
    message_version: &'static str,
) -> TxAnalysis {
    /* TODO: 🟡 Refine CPI warning logic to avoid spam. 
       Currently suppressed to avoid noise in transactions with common unknown programs (Jito, etc.).
    if !state.unknown_programs.is_empty() {
        warnings.push(AnalysisWarning::CpiLimit);
    }
    */

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
    for program_id in state.unknown_programs {
        warnings.push(AnalysisWarning::UnknownProgram { program_id });
    }
    // Privacy Level Calculation
    let has_confidential = state.confidential_ops_count > 0 
        || warnings.iter().any(|w| matches!(w, AnalysisWarning::ConfidentialTransferDetected));
    
    let mut has_hybrid_action = false;
    let has_storage = state.storage_ops_count > 0;

    for action in &state.extension_actions {
        if action.privacy_impact() == PrivacyImpact::Hybrid {
            has_hybrid_action = true;
        }
    }

    let has_public_mixing = state.saw_system_transfer || state.saw_token_spl || !state.transfers.is_empty();

    //TODO: switch to match
    let privacy_level = if has_hybrid_action {
        // Any explicit Hybrid action (like Decompress/bridge exit) forces Hybrid.
        PrivacyLevel::Hybrid
    } else if has_confidential {
        // Confidential operations exist.
        if has_public_mixing {
            PrivacyLevel::Hybrid
        } else {
            PrivacyLevel::Confidential
        }
    } else if has_storage {
        // Only storage compression (entrance/internal) exists.
        if has_public_mixing {
            PrivacyLevel::Hybrid
        } else {
            PrivacyLevel::Compressed
        }
    } else {
        PrivacyLevel::Public
    };

    // Fee Calculation with Overflow Protection
    let sig_count = signature_count(message) as u128;
    let base_fee_lamports = (ESTIMATED_BASE_FEE_PER_SIGNATURE as u128)
        .checked_mul(sig_count)
        .unwrap_or(u128::MAX); // Cap at MAX if crazy overflow

    let priority_fee_lamports = state.cu_price_micro.map(|price_micro| {
        let limit = state
            .cu_limit
            .unwrap_or(compute_budget::DEFAULT_COMPUTE_UNIT_LIMIT);
        
        // fee = (price * limit) / 1_000_000
        let fee = (price_micro as u128)
            .checked_mul(limit as u128)
            .and_then(|prod| prod.checked_div(MICRO_LAMPORTS_PER_LAMPORT))
            .unwrap_or(0);
            
        let estimated = state.cu_limit.is_none();
        (fee, estimated)
    });

    let total_fee_lamports = base_fee_lamports
        .checked_add(priority_fee_lamports.map(|(f, _)| f).unwrap_or(0))
        .unwrap_or(base_fee_lamports);

    TxAnalysis {
        transfers: state.transfers,
        base_fee_lamports,
        priority_fee_lamports,
        total_fee_lamports,
        total_sol_send_by_signer: state.total_sol_send_by_signer,
        compute_unit_limit: state.cu_limit,
        compute_unit_price_micro: state.cu_price_micro,
        warnings,
        message_version,
        privacy_level,
        extension_actions: state.extension_actions,
        extension_notices: Vec::new(),
        confidential_ops_count: state.confidential_ops_count,
        storage_ops_count: state.storage_ops_count,
        is_fee_payer: state.is_fee_payer,
        has_non_sol_assets: state.saw_token_spl || state.saw_token_2022,
    }
}

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
    
    // Safe addition for max cost
    let max_cost = analysis.total_fee_lamports
        .checked_add(analysis.total_sol_send_by_signer)
        .ok_or_else(|| ToolError::InvalidInput("Total cost overflowed u128".into()))?;

    let is_fee_payer = analysis.is_fee_payer;

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
        total_sol_send_by_signer: to_u64(analysis.total_sol_send_by_signer)?,
        max_total_cost_lamports: to_u64(max_cost)?,
        is_fee_payer,
        has_non_sol_assets: analysis.has_non_sol_assets,
        warnings: analysis.warnings.clone(),
        extension_actions: analysis.extension_actions.iter().map(|a| a.description()).collect(),
        extension_notices: analysis.extension_notices.clone(),
        confidential_ops_count: analysis.confidential_ops_count,
        storage_ops_count: analysis.storage_ops_count,
    })
}

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

fn signature_count(message: &Message) -> usize {
    message.header().num_required_signatures as usize
}

fn account_to_string(accounts: &[PubkeyBase58], index: u8) -> String {
    accounts
        .get(index as usize)
        .map(|pk| pk.to_string())
        .unwrap_or_else(|| format!("<unresolved:#{}>", index))
}

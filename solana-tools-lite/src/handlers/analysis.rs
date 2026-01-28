use crate::codec::{serialize_transaction, decode_system_transfer_amount, decode_compute_budget, ComputeBudgetAction};
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
use crate::models::input_transaction::InputTransaction;
use crate::serde::{LookupTableEntry};
use crate::ToolError;
use std::borrow::Cow;
use std::collections::HashSet;
use crate::Result;


// --- Constants ---
const ESTIMATED_BASE_FEE_PER_SIGNATURE: u64 = 5000;
const MICRO_LAMPORTS_PER_LAMPORT: u128 = 1_000_000;

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



/// Analyze an input transaction (raw, unsigned) to produce fee estimates, transfers, and warnings.
/// This function handles the conversion from InputTransaction to Message internally.
pub fn analyze_input_transaction(
    input_tx: &InputTransaction,
    signer: &PubkeyBase58,
    tables: Option<&LookupTableEntry>,
) -> Result<TxAnalysis> {
    let tx: Transaction = Transaction::try_from(input_tx)?;
    tx.message.sanitize()?;

    Ok(analyze_transaction(&tx.message, signer, tables))
}

/// Analyze a message to produce fee estimates, transfers, and warnings.
pub fn analyze_transaction(
    message: &Message,
    signer: &PubkeyBase58,
    tables: Option<&LookupTableEntry>,
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
            warnings.push(AnalysisWarning::LookupTablesNotProvided); //TODO: 🟡 double 
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

        let program_id_str = program_id.to_string();
        
        let handled = match program_id_str.as_str() {
            programs::SYSTEM_PROGRAM_ID => {
                if let Some(lamports) = decode_system_transfer_amount(&instr.data) {
                    // Ensure we have at least 2 accounts (from, to)
                    if instr.accounts.len() >= 2 {
                        state.saw_system_transfer = true;
                        process_transfer(&mut state, &account_list, instr, lamports, signer);
                    }
                }
                true
            }
            programs::COMPUTE_BUDGET_ID => {
                match decode_compute_budget(&instr.data) {
                    ComputeBudgetAction::SetLimit(l) => state.cu_limit = Some(l),
                    ComputeBudgetAction::SetPrice(p) => state.cu_price_micro = Some(p),
                    ComputeBudgetAction::None => {}
                }
                true
            }
            programs::TOKEN_PROGRAM_ID => {
                state.saw_token_spl = true;
                true
            }
            programs::TOKEN_2022_PROGRAM_ID => {
                state.saw_token_2022 = true;
                true
            }
            _ => false,
        };

        if !handled {
            state.unknown_programs.insert(program_id.clone());
        }
    }

    // 3. Finalize results
    let mut analysis = finalize_analysis(message, state, warnings, message_version);

    // 4. Run protocol extensions (Plugins)
    let plugins = registry::get_all_analyzers();

    for plugin in plugins {
        // Check if protocol is involved either via direct instructions or account presence
        let has_instructions = plugin.detect(message);
        
        // Also check in resolved account_list (includes lookup tables)
        let supported = match plugin.supported_programs() {
            Ok(programs) => programs,
            Err(_) => &[],
        };
        
        let in_resolved_accounts = account_list.iter().any(|pk| supported.contains(pk));
        
        if has_instructions {
            // Full analysis when protocol is directly invoked
            plugin.analyze(message, &account_list, signer, &mut analysis);
            plugin.enrich_notice(&mut analysis);

            if let Ok(supported) = plugin.supported_programs() {
                analysis.resolve_unknown_programs(supported);
            }
        } else if in_resolved_accounts {
            // Protocol present in accounts but not directly invoked (potential CPI)
            // Find and list which specific programs are present
            let supported = match plugin.supported_programs() {
                Ok(programs) => programs,
                Err(_) => continue,
            };

            // Gather found programs for notice
            let found_programs: Vec<String> = account_list
                .iter()
                .filter(|pk| supported.contains(pk))
                .map(|pk| {
                    let addr = pk.to_string();
                    let program_desc = plugin
                        .program_description(pk)
                        .unwrap_or("Unknown Program");
                    format!("  {} ({})", addr, program_desc)
                })
                .collect();
            
            // Only add notice if we actually found matching programs
            if !found_programs.is_empty() {
                let programs_list = found_programs.join("\n");
                let protocol_name = plugin.name();
                
                let notice = format!(
                    "PROTOCOL INTERACTION DETECTED:\n\
                    {} programs found in transaction accounts:\n\
                    {}\n\
                    \nThis may indicate Cross-Program Invocation (CPI) usage.",
                    protocol_name, programs_list
                );
                analysis.extension_notices.push(notice);
                
                analysis.resolve_unknown_programs(supported);
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
    let is_required = accounts
        .iter()
        .take(num_required_signatures)
        .any(|pk| pk == signer);

    if !is_required {
        warnings.push(AnalysisWarning::SignerNotRequired);
    }
}

fn resolve_message_components<'a>(
    message: &'a Message,
    tables: Option<&LookupTableEntry>,
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
    table: Option<&LookupTableEntry>,
    warnings: &mut Vec<AnalysisWarning>,
) -> Vec<PubkeyBase58> {
    let extra_capacity = table.map_or(0, |t| t.writable.len() + t.readonly.len());
    let mut combined = Vec::with_capacity(static_keys.len() + extra_capacity);

    combined.extend_from_slice(static_keys);

    if let Some(lut_entry) = table {
        // Add all writable accounts from the lookup table
        combined.extend_from_slice(&lut_entry.writable);
        // Add all readonly accounts from the lookup table
        combined.extend_from_slice(&lut_entry.readonly);
    } else if !lookups.is_empty() {
        warnings.push(AnalysisWarning::LookupTableNotProvided);
    }

    combined
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

    let Some(&from_idx) = instr.accounts.first() else {
        return;
    };
    let Some(&to_idx) = instr.accounts.get(1) else {
        return;
    };

    let from = account_to_string(accounts, from_idx);
    let to = account_to_string(accounts, to_idx);

    let from_is_signer = accounts
        .get(from_idx as usize)
        .map(|pk| pk == signer)
        .unwrap_or(false);

    if from_is_signer {
        // Safe saturating add to prevent overflow in accumulation
        state.total_sol_send_by_signer = state
            .total_sol_send_by_signer
            .saturating_add(lamports as u128);
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
        || warnings
            .iter()
            .any(|w| matches!(w, AnalysisWarning::ConfidentialTransferDetected));

    let mut has_hybrid_action = false;
    let has_storage = state.storage_ops_count > 0;

    for action in &state.extension_actions {
        if action.privacy_impact() == PrivacyImpact::Hybrid {
            has_hybrid_action = true;
        }
    }

    let has_public_mixing =
        state.saw_system_transfer || state.saw_token_spl || !state.transfers.is_empty();

    let privacy_level = match (has_hybrid_action, has_confidential, has_storage, has_public_mixing) {
        (true, _, _, _) => PrivacyLevel::Hybrid,
        (_, true, _, true)  => PrivacyLevel::Hybrid,
        (_, true, _, false) => PrivacyLevel::Confidential,
        (_, _, true, true)  => PrivacyLevel::Hybrid,
        (_, _, true, false) => PrivacyLevel::Compressed,
        _ => PrivacyLevel::Public,
    };

    // Fee Calculation with Overflow Protection
    let sig_count = message.header().num_required_signatures as u128;
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
    let max_cost = analysis
        .total_fee_lamports
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
        extension_actions: analysis
            .extension_actions
            .iter()
            .map(|a| a.description())
            .collect(),
        extension_notices: analysis.extension_notices.clone(),
        confidential_ops_count: analysis.confidential_ops_count,
        storage_ops_count: analysis.storage_ops_count,
    })
}

fn account_to_string(accounts: &[PubkeyBase58], index: u8) -> String {
    accounts
        .get(index as usize)
        .map(|pk| pk.to_string())
        .unwrap_or_else(|| format!("<unresolved: #{}>", index))
}

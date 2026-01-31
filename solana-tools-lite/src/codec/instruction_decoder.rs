// System Program
const SYSTEM_TRANSFER_TAG: u32 = 2;
const SYSTEM_TRANSFER_DATA_LEN: usize = 12; // tag (4) + lamports (8)

// Compute Budget
const COMPUTE_BUDGET_SET_UNIT_LIMIT: u8 = 2;
const COMPUTE_BUDGET_SET_UNIT_PRICE: u8 = 3;
const COMPUTE_BUDGET_TAG_LEN: usize = 1;
const COMPUTE_UNIT_LIMIT_LEN: usize = 4;
const COMPUTE_UNIT_PRICE_LEN: usize = 8;

pub enum ComputeBudgetAction {
    SetLimit(u32),
    SetPrice(u64),
    None,
}

pub fn decode_system_transfer_amount(data: &[u8]) -> Option<u64> {
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

pub fn decode_compute_budget(data: &[u8]) -> ComputeBudgetAction {
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

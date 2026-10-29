pub mod approval;
pub mod council;
pub mod execute;
pub mod timelock;

pub use approval::*;
pub use council::*;
pub use execute::*;
pub use timelock::*;

use anchor_lang::prelude::*;

use crate::errors::CouncilError;
use crate::state::{Action, Council};

/// Approvals can change only while the action is neither queued nor closed.
pub(crate) fn require_pending(action: &Action) -> Result<()> {
    require!(!action.cancelled, CouncilError::ActionCancelled);
    require!(!action.executed, CouncilError::AlreadyExecuted);
    require!(action.queued_at == 0, CouncilError::AlreadyQueued);
    Ok(())
}

pub(crate) fn require_member(council: &Council, key: &Pubkey) -> Result<usize> {
    council.member_index(key).ok_or(error!(CouncilError::NotMember))
}

mod global;
mod amp;
mod pitch;
mod fm;
mod disc;
mod feedback;
mod filter;

pub use amp::{handle_ad, handle_aenv_atk, handle_aenv_crv};
pub use pitch::{handle_pd, handle_pa, handle_penv_atk, handle_penv_crv};
pub use fm::{handle_fd, handle_fa, handle_fmev_atk, handle_fmev_crv};
pub use disc::{handle_da, handle_denv_atk, handle_denv_crv};
pub use feedback::{handle_fbev_atk, handle_fbev_crv};
pub use filter::{handle_flev_atk, handle_flev_crv};

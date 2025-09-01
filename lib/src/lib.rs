use serde::{Deserialize, Serialize};
use uint::construct_uint;
construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
pub mod crypto;
pub mod error;
pub mod sha256;
pub mod types;
pub mod util;

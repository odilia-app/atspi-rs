#[allow(unused_imports)]
mod auto;
pub use auto::functions::*;
pub use auto::*;
mod init;
pub use init::*;

pub mod prelude {
    pub use crate::traits::*;
}

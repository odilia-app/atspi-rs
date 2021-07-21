#[allow(unused_imports)]
mod auto;
pub use auto::functions::*;
pub use auto::*;
mod init;
pub use init::*;
mod desktop;
pub use desktop::*;

pub mod prelude {
    pub use crate::traits::*;
}

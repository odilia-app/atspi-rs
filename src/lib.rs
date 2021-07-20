#[allow(unused_imports)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod prelude {
    pub use crate::traits::*;
}

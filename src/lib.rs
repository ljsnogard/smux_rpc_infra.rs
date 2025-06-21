#![feature(try_trait_v2)]

pub mod client;
pub mod message;
pub mod session;
pub mod stream;
pub mod uri;

pub mod x_deps {
    pub use abs_buff::x_deps::abs_iter;
    pub use abs_smux;
    pub use abs_smux::x_deps::{abs_buff, abs_sync};
}
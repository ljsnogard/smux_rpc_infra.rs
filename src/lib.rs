#![feature(try_trait_v2)]

pub mod client;
pub mod message;
pub mod session;
pub mod stream;

pub mod x_deps {
    pub use abs_smux;

    pub use abs_smux::x_deps::{abs_buff, abs_sync};
}
use core::{
    error::Error,
    mem::MaybeUninit,
};

use abs_smux::x_deps::abs_sync;
use abs_sync::may_cancel::TrMayCancel;

use crate::message::TrRpcError;

/// An agent that a client consumes stream of items from server, and the agent 
/// that a service handler consumes stream of items produced by the client.
pub trait TrPullAgent {
    type Item;
    type Err: TrRpcError;

    fn pull_async(
        &mut self,
    ) -> impl TrMayCancel<'_, MayCancelOutput = Result<Self::Item, Self::Err>>;
}

/// An agent that a client produces stream of items and sends to the server, and 
/// an agent that a service handler produces stream of items to its subscribers.
pub trait TrPushAgent {
    type Item;
    type Err: TrRpcError;

    fn push_async<'f, F>(
        &'f mut self,
        emplace: F,
    ) -> impl TrMayCancel<'f, MayCancelOutput = Result<&'f Self::Item, Self::Err>>
    where
        F: FnOnce(&mut MaybeUninit<Self::Item>) -> Self::Item;
}

use core::{
    error::Error,
    mem::MaybeUninit,
};

use abs_smux::x_deps::abs_sync;
use abs_sync::may_cancel::TrMayCancel;

use crate::message::TrRpcError;

pub trait TrPullAgent {
    type Item;
    type Err: TrRpcError;

    fn pull_async(
        &mut self,
    ) -> impl TrMayCancel<'_, MayCancelOutput = Result<Self::Item, Self::Err>>;
}

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

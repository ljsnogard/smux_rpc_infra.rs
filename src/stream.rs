use core::{
    error::Error,
    mem::MaybeUninit,
};

use abs_smux::x_deps::abs_sync;
use abs_sync::may_cancel::TrMayCancel;

/// Represent the application level error
pub enum StreamAppError {
    /// The remote end of the item stream is closed
    Closed,

    /// The operation to push or pull is cancelled.
    Cancelled,
}

pub trait TrStreamError
where
    Self: Sized + Error,
{
    fn try_into_rpc_err(self) -> Result<StreamAppError, Self>;
}

pub trait TrPullAgent {
    type Item;
    type Err: TrStreamError;

    fn pull_async(
        &mut self,
    ) -> impl TrMayCancel<'_, MayCancelOutput = Result<Self::Item, Self::Err>>;
}

pub trait TrPushAgent {
    type Item;
    type Err: TrStreamError;

    fn push_async<'f, F>(
        &'f mut self,
        emplace: F,
    ) -> impl TrMayCancel<'f, MayCancelOutput = Result<&'f Self::Item, Self::Err>>
    where
        F: FnOnce(&mut MaybeUninit<Self::Item>) -> Self::Item;
}

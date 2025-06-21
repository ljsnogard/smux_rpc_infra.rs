use core::ops::Try;

use abs_smux::x_deps::abs_sync;
use abs_sync::may_cancel::TrMayCancel;

use crate::{
    message::TrAccess,
    session::*,
};

pub trait TrAccessBuilder<'a>
where
    Self: 'a,
{
    /// Data type or representation of the location
    type Uri;

    /// The header key data type
    type Key;

    /// The session type that defines context of the request and response
    type Sess: TrSession;

    /// The data type of access that will be created by this builder
    type Access: TrAccess<'a,
        Uri = Self::Uri,
        Key = Self::Key,
        Sess = Self::Sess,
    >;

    fn try_build_async<'f>(
        &'f mut self,
    ) -> impl TrMayCancel<'f, MayCancelOutput: Try<Output = Self::Access>>;
}

pub trait TrRpcClient 
{
    type Uri: AsRef<str>;

    fn head<'a, S: TrHeadSession>(
        &'a self,
        location: &'a Self::Uri,
    ) -> impl TrAccessBuilder<'a, Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn call<'a, S: TrCallSession>(
        &'a self,
        location: &'a Self::Uri,
        arguments: &'a S::Args,
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn view<'a, S: TrViewSession>(
        &'a self,
        location: &'a Self::Uri,
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn delete<'a, S: TrDeleteSession>(
        &'a self,
        location: &'a Self::Uri
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn post<'a, S: TrPostSession>(
        &'a self,
        location: &'a Self::Uri,
        content: &'a S::Body,
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn push<'a, S: TrPushSession>(
        &'a self,
        location: &'a Self::Uri,
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;

    fn pull<'a, S: TrPullSession>(
        &'a self,
        location: &'a Self::Uri,
    ) -> impl TrAccessBuilder<Access: TrAccess<
        'a,
        Uri = Self::Uri,
        Sess = S,
    >>;
}

#[cfg(test)]
mod demo_ {
    use core::ops::ControlFlow;
    use abs_smux::x_deps::abs_sync::{cancellation::NonCancellableToken, preludes::TrMayCancel};

    use crate::client::{TrAccessBuilder, TrRpcClient};
    use super::*;

    #[allow(unused)]
    struct SampleHeadSession;
    struct SampleHeadResp;
    impl TrSession for SampleHeadSession {
        type Resp = SampleHeadResp;
    }
    impl TrHeadSession for SampleHeadSession
    {}

    #[allow(unused)]
    async fn sample_<TClient>(client: TClient)
    where
        TClient: TrRpcClient<Uri = str>
    {
        let ControlFlow::Continue(mut a) = client.head::<SampleHeadSession>("path")
            .try_build_async()
            .may_cancel_with(NonCancellableToken::shared_mut())
            .await
            .branch()
        else {
            panic!()
        };
        let ControlFlow::Continue(resp) = a
            .send_async()
            .may_cancel_with(NonCancellableToken::shared_mut())
            .await
            .branch()
        else {
            panic!()
        };
    }
}

use core::{
    error::Error,
    iter::IntoIterator,
};

use abs_buff::TrBuffRead;
use abs_smux::x_deps::{abs_buff, abs_sync};
use abs_sync::may_cancel::TrMayCancel;

use crate::{
    session::*,
};
/// The access method or the operation that will exert to a resource via RPC.
#[derive(Clone, Copy, Debug)]
pub enum AccessMethod {
    /// Retrieve the meta information of a resource
    Head,

    /// Calling an API with guarantees.
    Call,

    /// View the resource in a specific manner.
    View,

    /// Post a new resource on to the server
    Post,

    /// Remove or disable a resource on the server
    Drop,

    /// Subscribe and receive live update event of a resource
    Pull,

    /// broadcast live update of a resource via the server
    Push,
}

pub trait TrRpcError
where
    Self: Sized + Error
{
    fn err_code(&self) -> ReactCode;
}

pub trait TrAccess<'a>
where
    Self: 'a,
{
    type Err: TrRpcError;
    type Path;
    type Key;
    type Sess: TrSession;

    fn method(&self) -> AccessMethod;

    fn location(&self) -> &Self::Path;

    fn headers<'f>(
        &'f self,
    ) -> impl IntoIterator<Item = (&'a Self::Key, &'a [u8])>;

    fn body<'f>(&'f self) -> impl TrBuffRead<u8>;

    fn send_async<'f>(
        &'f mut self,
    ) -> impl TrMayCancel<'f, MayCancelOutput =
        Result<<Self::Sess as TrSession>::Resp, Self::Err>>;
}

type CodeTy = u16;

/// The code identifying the reaction status
pub struct ReactCode(CodeTy);

impl ReactCode {
    pub const fn repr(&self) -> CodeTy {
        self.0
    }
}

impl ReactCode {
    /// The server cannot or will not process the request due to something that is perceived to be 
    /// a client error (e.g., malformed request syntax, invalid request message framing, or 
    /// deceptive request routing).
    pub const C_BAD_ACCESS        : ReactCode = ReactCode(400);

    /// The client must authenticate itself to get the requested response.
    pub const C_UNAUTHENTICATED   : ReactCode = ReactCode(401);

    /// The identity is known by the server but the permission check failed.
    pub const C_PERMISSION_DENIED : ReactCode = ReactCode(402);

    /// The client does not have access rights to the content; that is, it is unauthorized, so the 
    /// server is refusing to give the requested resource.
    /// Unlike 401 Unauthorized, the client's identity is known to the server.
    pub const C_ACCESS_FORBIDDEN  : ReactCode = ReactCode(403);

    /// The server cannot find the requested resource. This means the location is valid but not 
    /// recognized. In an API, this can also mean that the endpoint is valid but the resource itself 
    /// does not exist. Servers may also send this response instead of 403 Forbidden to hide the 
    /// existence of a resource from an unauthorized client. This response code is probably the most 
    /// well known due to its frequent occurrence on the web.
    pub const C_RESOURCE_NOT_FOUND: ReactCode = ReactCode(404);

    /// The request method is known by the server but is not supported by the target resource. For 
    /// example, an API may not allow DROP on a resource, or the TRACE method entirely.
    pub const C_METHOD_NOT_ALLOWED: ReactCode = ReactCode(405);
}

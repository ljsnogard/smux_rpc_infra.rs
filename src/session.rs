use crate::stream::{TrPullAgent, TrPushAgent};

/// A session describes the data type besides of the location uri, that the 
/// client side will send, and that the server side is expected to reply,
/// in the roundtrip of request response.
pub trait TrSession {
    /// The result data or instance that will be returned by the access instance
    type Resp;
}

pub trait TrHeadSession: TrSession 
{}

pub trait TrCallSession: TrSession {
    /// The arguement data needed to make the RPC call.
    type Args;
}

pub trait TrDeleteSession: TrSession
{}

pub trait TrViewSession: TrSession
{}

/// The post session will need to define the data type of the content body.
pub trait TrPostSession: TrSession {
    type Body;
}

/// The push session specializes the `Resp` type to `TrPushAgent`
pub trait TrPushSession: TrSession
where
    Self::Resp: TrPushAgent<Item = Self::Item>,
{
    type Item;
}

/// The pull session specializes the `Resp` type to `TrPullAgent`
pub trait TrPullSession: TrSession
where
    Self::Resp: TrPullAgent<Item = Self::Item>,
{
    type Item;
}

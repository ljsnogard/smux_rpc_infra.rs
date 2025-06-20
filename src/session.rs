pub trait TrSession {
    type Resp;
}

pub trait TrHeadSession: TrSession 
{}

pub trait TrCallSession: TrSession {
    type Args;
}

pub trait TrPostSession: TrSession {
    type Body;
}

pub trait TrPushSession: TrSession
{}

pub trait TrPullSession: TrSession
{}

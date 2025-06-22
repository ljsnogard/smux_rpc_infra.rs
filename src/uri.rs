use abs_buff::x_deps::abs_iter;
use abs_iter::TrAsSlice;
use abs_smux::x_deps::abs_buff;

/// Abstraction over all kinds of implementations of Uri.
pub trait TrUri {
    fn scheme(&self) -> impl AsRef<str>;

    fn authority(&self) -> impl TrAuthority;

    fn path(&self) -> impl TrPath;
}

pub trait TrAuthority {
    fn user_info(&self) -> impl AsRef<str>;

    fn host(&self) -> impl TrHostName;
}

/// The dot separated domain name that identifies a host in a network.
pub trait TrHostName {
    fn parts<'f>(&'f self) -> impl 'f + TrAsSlice<Elem: AsRef<str>>;
}

pub trait TrPath {
    fn segments<'f>(&'f self) -> impl 'f + TrAsSlice<Elem: AsRef<str>>;
}

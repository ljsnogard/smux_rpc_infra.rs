use abs_buff::x_deps::abs_iter;
use abs_iter::TrAsSlice;
use abs_smux::x_deps::abs_buff;

pub trait TrUri {
    fn scheme(&self) -> impl AsRef<str>;

    fn host(&self) -> impl TrHostName;

    fn path(&self) -> impl TrPath;
}

pub trait TrHostName {
    fn parts<'f>(&'f self) -> impl 'f + TrAsSlice<Elem: AsRef<str>>;
}

pub trait TrPath {
    fn segments<'f>(&'f self) -> impl 'f + TrAsSlice<Elem: AsRef<str>>;
}

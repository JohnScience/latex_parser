use core::result::Result;

// Proper declaration and implementation of MapRight requires GATs (https://github.com/rust-lang/rust/issues/44265)
pub(crate) trait MapRightTupleEntry<T1,T2>: Sized {
    fn map_right<U2,F: FnOnce(T2) -> U2>(self, f: F) -> (T1,U2);
}

impl<T1,T2> MapRightTupleEntry<T1,T2> for (T1,T2) {
    fn map_right<U2,F: FnOnce(T2) -> U2>(self, f: F) -> (T1,U2) {
        (self.0,f(self.1))
    }
}

pub(crate) trait MapRightResult<T1,T2,E>: Sized {
    fn map_right<U2,F: FnOnce(T2) -> U2>(self, f: F) -> Result<(T1,U2),E>;
}

impl<T1,T2,E> MapRightResult<T1,T2,E> for Result<(T1,T2),E> {
    fn map_right<U2,F: FnOnce(T2) -> U2>(self, f: F) -> Result<(T1,U2),E> {
        self.map(|(l,r)| (l,f(r)))
    }
}

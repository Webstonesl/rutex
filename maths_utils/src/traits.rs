use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

use crate::error::Error;
pub trait ErrorCheckedAdd: CheckedAdd {
    fn e_checked_add(&self, v: &Self) -> Result<Self, Error> {
        self.checked_add(v)
            .ok_or_else(|| new_error!(OverflowError, "Add overflowed"))
    }
}
pub trait ErrorCheckedSub: CheckedSub {
    fn e_checked_sub(&self, v: &Self) -> Result<Self, Error> {
        self.checked_sub(v)
            .ok_or_else(|| new_error!(OverflowError, "Sub overflowed"))
    }
}
pub trait ErrorCheckedMul: CheckedMul {
    fn e_checked_mul(&self, v: &Self) -> Result<Self, Error> {
        self.checked_mul(v)
            .ok_or_else(|| new_error!(OverflowError, "Mul overflowed"))
    }
}
pub trait ErrorCheckedDiv: CheckedDiv {
    fn e_checked_add(&self, v: &Self) -> Result<Self, Error> {
        self.checked_div(v)
            .ok_or_else(|| new_error!(OverflowError, "Div overflowed"))
    }
}
impl<T: CheckedAdd> ErrorCheckedAdd for T {}
impl<T: CheckedSub> ErrorCheckedSub for T {}
impl<T: CheckedMul> ErrorCheckedMul for T {}
impl<T: CheckedDiv> ErrorCheckedDiv for T {}

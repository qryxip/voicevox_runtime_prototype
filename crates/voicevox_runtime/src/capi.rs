use std::ptr::NonNull;

use crate::ModelRuntime;

pub struct VvrtModelRuntime(ModelRuntime);

#[no_mangle]
pub unsafe extern "C" fn VvrtModelRuntime_new(out: NonNull<Box<VvrtModelRuntime>>) -> i32 {
    let Ok(rt) = ModelRuntime::new() else {
        return 1;
    };
    out.as_ptr().write_unaligned(VvrtModelRuntime(rt).into());
    0
}

#[no_mangle]
pub extern "C" fn VvrtModelRuntime_delete(rt: Box<VvrtModelRuntime>) {
    drop(rt);
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort()
}

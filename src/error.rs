use core::panic::PanicInfo;

#[panic_handler]
fn error_handler(_info: &PanicInfo) -> ! {
    loop {}
}

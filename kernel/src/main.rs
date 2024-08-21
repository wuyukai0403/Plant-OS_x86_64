#![no_std]
#![no_main]

use limine::BaseRevision;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    framework::init_framework();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}




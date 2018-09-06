// Beginning with the MVP
// Toward a user interface for the BBC micro:bit or similar device
#![feature(panic_handler)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

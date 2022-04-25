// main.rs 

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points.
//#[lang = "copy"] // attribute that defines it as a language item.

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function.
pub extern "C" fn _start(0 -> ! {
    // thus function is the entry point, since the linker looks for a function.
    // named `_start` by default.
    loop{}
})

// this function is call on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}



//fn main() {}

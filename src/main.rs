// main.rs 

use core::panic::PanicInfo;

#![no_std]

// this function is call on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

//fn main() {}

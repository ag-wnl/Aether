

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/**
 * @description: our compiled executable target is for: thumbv7em-none-eabihf target triple,
 * as it has no underlying OS
 */


// invoked whenever there's a panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop{}
} 

/**
 * we are using extern "C" to use C programming caller convention
 * as OS/system linker expect such syntax as they are written in C
 * 
 * divergent function as its just directly invoked by bootloader/OS
 * and no other func ever calls it
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop{}
}
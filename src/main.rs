

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
mod vga_buffer;
mod interrupts;

/**
 * @description: our compiled executable target is for: thumbv7em-none-eabihf target triple,
 * as it has no underlying OS
 */


// invoked whenever there's a panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop{}
} 



static HELLO: &[u8] = b"welcome to aether v0.1.0";

/**
 * we are using extern "C" to use C programming caller convention
 * as OS/system linker expect such syntax as they are written in C
 * 
 * divergent function as its just directly invoked by bootloader/OS
 * and no other func ever calls it
 */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    // 0xb8000 - vga buffer address
    // let vga_buffer = 0xb8000 as *mut u8; // raw pointer 
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte; // writing string byte
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // adding a cyan color byte
    //     }
    // }

    // let mut writer = vga_buffer::Writer {
    //     col_pos: 0,
    //     color_code: vga_buffer::ColorCode::new(vga_buffer::Color::Cyan, vga_buffer::Color::Black),
    //     buffer: unsafe { &mut *(0xb8000 as *mut vga_buffer::Buffer) },
    // };


    // writer.write_string("hey, i'm @agwnl\ncout << hello world;");   


    // write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 42, 1.337).unwrap();

    println!("welcome to {}{}", "aether OS", "!");


    interrupts::initialize_idt();
    x86_64::instructions::interrupts::int3();

    print!("did not crash");

    loop {}
}
#![no_std] // Do not link the Rust stdlib
#![no_main] // Disable all Rust-level entry points
// Enable custom_test_frameworks
#![feature(custom_test_frameworks)] 
// Set the test_runner function
#![test_runner(os::test_runner)] 
// Change the name of the generated function (for running tests)
// to something different than main
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
/// This function is called on panic.
fn panic(info: &PanicInfo) -> ! {
    // Print the panic info
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info);
}

// Disable name mangling to ensure that Rust really
// outputs a function with the name _start, without
// the attribute the compiler would generate some 
// random name.
#[no_mangle]
// Mark as `extern "C"` to tell the compiler that 
// it should use the C calling convention for this
// function
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the
    // linker looks for a function named `_start` 
    // by default.

    println!("Hello from our modified println macro!");

    #[cfg(test)]
    // (This function is generated by the test framework (it 
    // calls the test_runner), normally the test framework 
    // generates a function called main)
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
// This needs to be no_std, you probably don't want to do heap allocations in this context anyway
// but if you do implement the allocator in the runner and expose its API via the function
// prototypes.
#![no_std]
#![no_main]
#![feature(start)]
use core::panic::PanicInfo;

// Include the function_prototypes struct here so that the types can be known at compile time. This
// is analogous to including the beacon.h file in cobalt strike.
include!("../runner/header.rs");

// Exports this function to the linker script so that it is called first.
#[no_mangle]
#[link_section = ".text.prologue"]
pub unsafe extern "C" fn _start(prototypes: FunctionPrototypes){

    // Call the first debug function that just prints arbitrary text.
    (prototypes.debug_test)();
    // Call the second debug function that takes an integer as an argument.
    (prototypes.debug_test2)(15);

    return;

}

// Panics are set to immediately abort but this is required to satisfy some requirement.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

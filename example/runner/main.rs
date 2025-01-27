use std::mem::transmute;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::env;
use header::FunctionPrototypes;
use libc::mmap;
mod header;

// The function prototypes declared inside of the header have their functionality defined here (or
// in any other file)
#[unsafe(no_mangle)]
pub extern "C" fn debug_test(){
    println!("WTF!");
} 

#[unsafe(no_mangle)]
pub extern "C" fn debug_test2(input:isize){
    println!("{}", input);
}

fn main() {
    unsafe {
    
    // Maps the exposed beacon API to the FunctionPrototypes struct so that the function addresses
    // can be passed to the BOF.
    let prototypes = FunctionPrototypes {
            debug_test,
            debug_test2,
    };


    // Reads stdin to get the binary to run.
    let args: Vec<String> = env::args().collect();

    // Reads the BOF to a buffer. This could just as easily be done via a reflective loader or some
    // other method.
    let mut bofbytes = File::open(args[1].clone()).unwrap();
    let mut bofbuffer = Vec::new();
    bofbytes.read_to_end(&mut bofbuffer).unwrap();

    // Allocates a section of RWX memory to contain the BOF.
    let shellcode =
        mmap(
            core::ptr::null_mut(),
            bofbuffer.len(), 
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, 
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, 
            -1, 
            0
        );

    // Copies the BOF to the RWX memory
    ptr::copy_nonoverlapping(bofbuffer.as_ptr(), shellcode as *mut u8, bofbuffer.len());
    
    // Casts the RWX memory containing the BOF to a function.
    let exec_shellcode: extern "C" fn(FunctionPrototypes) = transmute(shellcode);
    
    // Executes the loaded bof as if it were a function.
    let _bof = (exec_shellcode)(prototypes);
    
    };
}

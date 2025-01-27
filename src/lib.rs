#![feature(rustc_private)]

// This will always say that it's unresolved but it will still compile.
extern crate rustc_driver;

// Used to write the default linker file to the temp directory in the event one is not provided.
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::error::Error;
// Used to run strip and objcopy.
use std::process::Command;

pub fn compile(input_path:String, output_path:String, target:String, linker_file_path:Option<String>)->Result<String, Box<dyn Error>>{
    // Includes the bytes for the default linker file to use if it isn't overridden.
    // Attempting to enforce name casing should be punishable by death
    #[allow(non_upper_case_globals)]
    const default_linker: &[u8] = include_bytes!("./speed.ld");

    let linker = match linker_file_path {
        Some(x) => x,
        None => {
            let temp_dir = env::temp_dir();
            let linker_path = temp_dir.join("speed.ld");
            let mut file = File::create(linker_path.clone()).expect("Failed to write to {linker_path}, do you have your temp directory set up?");
            file.write_all(default_linker).expect("Failed data to {linker_path}, do you have write access to your temp directory?");
            // Why there's no path -> String method I don't know but it's retarded ( just like
            // everything else string related in this frequently turboshit language )
            linker_path.to_str().unwrap().to_string()
        }
    };

    // We aren't doing any analysis based on callbacks so this code is irrelevant to us, but has to
    // exist to satisfy a trait expectation in RunCompiler.
    struct MyCallbacks;
    impl rustc_driver::Callbacks for MyCallbacks{}
    
    // Compiles the bof rust file into an object file and links it using the provided linker file.
    let _result = rustc_driver::RunCompiler::new(
        
        &["".to_string(), // We need null in arg[0]
            "-C".to_string(), "panic=abort".to_string(), // Remove the need for panic unwinding 
            "-C".to_string(), format!("link-arg=-Wl,-T{},--build-id=none",linker), // Links using
            // the provided linker file (or the default one, speed.ld in the src directory)
            "-C".to_string(), "link-arg=-nostdlib".to_string(), // Doesn't link stdlib
            "-C".to_string(), "link-arg=-static".to_string(),  // Performs static linking
            "-C".to_string(), "link-arg=-nodefaultlibs".to_string(), // Doesn't link any default
            // libs (for size and stuff)
            "-C".to_string(), "opt-level=z".to_string(), // Optimizes for size
            "--emit=obj".to_string(), input_path, // Emits an object file 
            "--target".to_string(), target, // The compilation target you are compiling for
            "-o".to_string(), output_path.clone()], // The output directory of the object file
        
        &mut MyCallbacks)
        .run();

  
    match fs::metadata(output_path.clone()) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(format!("The compiled file does not appear to exist, maybe check your input/output paths or your target?. Error: {}", e).into())

    }

}

pub fn strip(input_path:String, output_path:String)->Result<String,Box<dyn Error>>{
    let _strip = Command::new("strip")
        .arg(input_path.clone())
        .spawn()?;

    let _objcopy = Command::new("objcopy")
        .arg("-O")
        .arg("binary")
        .arg("-j")
        .arg(".text.prologue")
        .arg("-j")
        .arg(".text")
        .arg("-j")
        .arg(".data")
        .arg(output_path.clone())
        .arg(format!("{}.{}", output_path.clone(), "bin"))
        .spawn()?;

    Ok(format!("{}.{}", output_path, "bin"))

}

#![feature(rustc_private)]

use speedloader;
use std::env;

fn main() {
    
    let help = "Usage: ./speedlib <path/to/input.rs> <path/to/output> <compiler-target> [linker-script]
<> == Required
[] == Optional
This is just for example purposes, for more information go read the github.";

    // Grab args so we can give this an input and an output. This should be changed to clap
    // obviously.
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{}", help);
        return;
    }
    if args[1].clone().is_empty() || args[1].clone() == "-h".to_string() || args[1].clone() == "help".to_string() {
        println!("{}", help);
        return;
    }
    let result = speedloader::compile(args[1].clone(), args[2].clone(), args[3].clone(), None).unwrap();
    let result = speedloader::strip(result.clone(), result.clone()).unwrap();
    println!("Your binary is at {}. Bye!", result);
    
}

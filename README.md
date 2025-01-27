# SpeedLoader
This is a skeleton project that you can steal to implement your own COFF/BOF loader format in your own projects. It contains functions that drive the compiler to generate shellcode from a no_std single rust file that can then be loaded into another application (like a beacon) and can call the beacon/host application's functions. This is somewhere between how Cobalt Strike BOFs work and just executing shellcode with arguments.

## Example Usage
This project doesn't really do much on its own, but here's the basic installation and usage instructions. Details on how exactly this works will be outlined below, but if you just want to see the text print out on screen, follow these instructions. NOTE: This only will run on Linux (but the payloads are cross platform).
### Installation
1. First, install Rust if you haven't already at https://www.rust-lang.org/tools/install
2. `git clone https://github.com/NoahKirchner/speedloader`
3. Then inside of the speedloader directory, `cargo build --all`
4. Make sure that you have `objcopy` and `strip` installed on your operating system (Though they should be installed by default)
5. If you run Arch, check troubleshooting.

### Executing Example Payload 
From the root directory, you can compile the payload file using `speedldr` like this:
`cargo run -p speedldr -- ./example/payload/example.rs ./example/payload/example x86_64-unknown-linux-gnu`
And then point the example runner at the example payload like this:
`cargo run -p runner -- ./example/payload/example.bin`

You should then see the text "WTF!" and the number "15" print out on the screen. Not very impressive on its own, but if you investigate the example code, you'll see that the shellcode in `example.bin` does not actually contain the code necessary to do that, and instead after it is loaded into the `runner` application, the payload is capable of calling functions that are only implemented inside of the `runner` application.

### Troubleshooting
* Depending on your operating system, you may have to run `export LD_LIBRARY_PATH=$HOME/.rustup/toolchains/nightly-2025-01-06-x86_64-unknown-linux-gnu/lib/` or add it into your `.bashrc` in order to drive the compiler. I have to do it on Arch, but you do not need to do it on Debian based distributions (I think).
* If you are attempting to compile for a different target than whichever one you already have installed, install it using `rustup target add <target-name>`, so for example `rustup target add x86_64-pc-windows-gnu`.
* If you're absolutely certain the syntax of everything is correct, throw up an issue with the commands you're running and the output and I'll look at it.

## Explanation
### Compilation/Linking
This project is built around a feature of Rust that allows you to "drive" the rustc compiler manually from inside of a rust project. Whenever you built the project at the beginning, you were installing the January 06th, 2025 version of the nightly rust compiler. From there, the `speedloader` library used the compiler's exported functions to compile the single `example.rs` file into an object file in accordance with the included `speed.ld` linker script. Then the `strip` and `objcopy` binaries extracted the `.text` and `.data` section from the object file and placed them into the final payload file, `example.bin`.
### Header File
Looking around the file structure, you may see the `header.rs` file included in the `example/runner/` directory. This file is analogous to the `beacon.h` CobaltStrike file used for developing BOFs. Inside of it is a struct that contains function prototypes which do not have their functionality defined. Both the runner and the payload import this file, but perform different actions with the information as outlined next.
![image](https://github.com/user-attachments/assets/f4494509-9395-4a45-a816-57e7be8558d4)

### Runner 
The runner file imports the struct as a module. At the top of the runner file are the definitions for the corresponding prototypes outlined in the header file (1), and inside of the main loop of the runner the `FunctionPrototypes` struct is constructed with the memory addresses of the pointers taking the place of the struct members (2).
![image](https://github.com/user-attachments/assets/563486ee-dd09-4a9e-b883-0f2b835d0de4)


The runner then allocates the payload to RWX memory (1). It casts the address of this RWX memory to a function pointer and calls it (2), passing the struct it just populated as the first argument. This struct contains the memory addresses for the functions defined inside of the runner.
![image](https://github.com/user-attachments/assets/357efb1b-56d3-40de-8cde-c698fb8bcd53)


### Payload 
The payload does not import the struct as a module, and rather includes it directly into the text of the .rs file similar to how the C includes would work (1). The first argument that the payload takes is the header struct FunctionPrototypes (2), and during its execution it dereferences the function pointers from the struct and calls them as if they were any other function (3). It has been provided with these function addresses dynamically at runtime, and thus does not need to include their functionality in its source code.
![image](https://github.com/user-attachments/assets/5d9e64de-09e9-4a29-a6f4-3399ff0b5057)


### Execution 
With these two previous sections put together, we now see how the runner's functions are executed by the BOF. When the runner calls the payload, it passes along its function addresses and allows the payload to call them. While the examples used here are extremely simplistic, the process of exposing the runner functions to the payload is fairly painless, and likewise the process of calling the functions from the payload's perspective is not much more difficult than calling any other function.

## References
Some great information on writing shellcode in Rust

https://os.phil-opp.com/freestanding-rust-binary/

https://jade.fyi/blog/writeonly-in-rust/

Driving the Rust compiler is extremely poorly documented, but this had enough information to struggle through it.

https://rustc-dev-guide.rust-lang.org/

Gotta pay it back to the OG of course.

https://github.com/trustedsec/COFFLoader


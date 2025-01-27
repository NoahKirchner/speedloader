// This is analogous to the beacon.h file for cobalt strike BOFs. Function prototypes are declared
// here inside of this struct and then they can be passed to the BOF at execution so that it can
// call the hosting process's functions. 
//
// This is included into the bof using the include!() macro and then imported as a module into
// whatever file declares the functions.

#[repr(C)]
pub struct FunctionPrototypes {
    pub debug_test: unsafe extern "C" fn()->(),
    pub debug_test2: unsafe extern "C" fn(isize)->(),
}

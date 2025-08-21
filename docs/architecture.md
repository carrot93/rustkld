# RustKLD Kernel
## Generated Rust bindings
To obtain the C code we want to work with, we use [Bindgen](https://rust-lang.github.io/rust-bindgen/) to generate Rust FFI bindings.
### Includes
Our wrapper.h file is located under kernel/headers. It lists all the .h files we would like to obtain FFI bindings from. 
```c 
/* wrapper.h */
#include <sys/param.h>
#include <sys/module.h>
#include <sys/systm.h>
#include <sys/conf.h>
#include <sys/uio.h>
#include <sys/malloc.h> 
```

### build.rs
Our build.rs file is responsible for the generation of our Rust FFI code. We use .allowlist calls to only generate what we use from the wrapper.h imports in our rust code to keep compile time low.
```rust
        // Allowed Functions
        .allowlist_function("uiomove*")

        .allowlist_function("make_dev_p*")
        .allowlist_function("destroy_dev")

        .allowlist_function("malloc")
        .allowlist_function("free")

        .allowlist_function("uprintf")

        .allowlist_function("dev_ref")
        .allowlist_function("dev_rel")
    
        // Allowed Types
        .allowlist_type("uio")

        .allowlist_type("cdev")
        .allowlist_type("cdevsw")
        
        .allowlist_type("moduledata_t")

        // Allowed Variables
        .allowlist_var("M_DEVBUF")
        .allowlist_var("M_WAITOK")

        .allowlist_var("MAKEDEV_WAITOK")
        .allowlist_var("MAKEDEV_CHECKNAME")

        .allowlist_var("UID_ROOT")
        .allowlist_var("GID_WHEEL")

        .allowlist_var("D_VERSION")

        // End of allowlist
```

## Safety
### Wrapper Structs
We use simple wrapper structs such as uio_wrap.rs and cdev_wrap.rs to provide a safe access to the C struct's pointer variables.
To showcase, this section in uio_wrap.rs:
```rust
pub struct Uio<'a>(&'a mut uio);

impl<'a> Uio<'a> {
    pub fn new(uio: &'a mut uio) -> Self {
        Self(uio)
    }
    fn get_resid(&self) -> usize{
        self.0.uio_resid as usize 
    }
    fn get_offset(&self) -> usize {
        self.0.uio_offset as usize
    }
...
```
We can safely obtain the uio_offset and uio_resid values from the C uio struct.

### Type safety
We also incorporate the use of [Bitflags](https://docs.rs/bitflags/latest/bitflags/) in flags.rs to ensure type safety when working with ioflag and oflags.
```rust
bitflags! {
    pub struct Oflags: u32 {
        const O_RDONLY = 0x0000;
        const O_WRONLY = 0x0001;
        const O_RDWR = 0x0002;
        const O_ACCMODE = 0x0003;
    }
...
```
Of course we implement a function to convert the given c_int to its respective bitflag:
```rust
impl Oflags {
    pub fn convert(c_oflags: c_int) -> Self {
        Self::from_bits_truncate(c_oflags as u32)
    }   
}
```

### Read/Write Traits
The Read and Write traits in lib.rs are meant to mimic those in io::Read and io::Write. 
```rust
pub trait Read {
    fn read(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize, libc::c_int>;   
    fn read_buf(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize, libc::c_int>;   
}

pub trait Write {
    fn write(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize, libc::c_int>;
    fn write_all(&mut self, buf: &mut alloc::vec::Vec<u8>) -> Result<usize, libc::c_int>;
}
```
The logic itself is defined in kernel/src/uio_wrap.rs

### Cdevsw Trait
The Cdevsw trait in lib.rs holds the safe methods that are called from char_ffi.rs
```rust
pub trait Cdevsw {    
    fn quiesce(&mut self) -> Result<(), libc::c_int>;
    fn open(&mut self, dev: Cdev, oflags: Oflags, devtype: libc::c_int, td: *mut thread) -> Result<(), libc::c_int>;
    fn close(&mut self, dev: Cdev, oflags: Oflags, devtype: libc::c_int, td: *mut thread) -> Result<(), libc::c_int>;
    fn write(&mut self, dev: Cdev, uio_ptr: Uio, ioflag: Ioflag) -> Result<libc::c_int, libc::c_int>;
    fn read(&mut self, dev: Cdev, uio_ptr: Uio, ioflag: Ioflag) -> Result<libc::c_int, libc::c_int>;
...
```
Instead of taking in the raw C data structures, char_ffi.rs makes the necessary initializations so that we work with safe Rust variants. The quiesce event is included to be used in the module_events.rs file.

### char_ffi.rs
char_ffi.rs houses our FFI for the driver logic used in a character device switch. It unpacks a double boxed Cdevsw stored in a given cdev's si_drv1, converts the incoming C arguments to our Rust equivalents, then calls its respective method. For example:
```rust
/// # Safety
///
/// This function extracts a dyn Cdevsw trait object and executes its read() method
pub unsafe extern "C" fn ffi_read(
    dev: *mut cdev,
    uio_ptr: *mut uio,
    c_ioflag: c_int
) -> c_int {
    let charDev = unsafe { 
        match get_Cdevsw(dev) {
            Ok(obj) => obj, // get our dyn trait object
            Err(error) => return error, 
        }
    };
    if uio_ptr.is_null() {
        println!("[char_ffi.rs] uio_ptr is NULL");
        return EFAULT;
    }
    let cdevr = unsafe {&mut *dev};
    let safe_dev = Cdev::new(cdevr); // Get our Cdev wrapper object
    
    let uior = unsafe {&mut *uio_ptr};
    let safe_uio = Uio::new(uior); // Get our Uio wrapper object
    
    let ioflag = Ioflag::convert(c_ioflag); // Get our bitmapped Ioflag
    
    match charDev.read(safe_dev, safe_uio, ioflag) { // call our Cdev read method
        Ok(n) => n,
        Err(error) => error,
    }
}
```
It is kept in the kernel/src/ as it works with any given double boxed Cdevsw trait object. This makes it so that any developer wanting to make their own character driver in rust does not need to directly mess with C objects or extern "C" functions as much as they normally would.

**Note:** A better way to go about this could be splitting the Cdevsw fat pointer and storing them in both si_drv1 and si_drv2. It a bit more complex but it something to keep in mind.

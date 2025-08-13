// Create a null-terminated constant string at compile time
#[macro_export]
macro_rules! cstr {
    ($arg:expr) => {
        concat!($arg, '\x00')
    };
}

#[macro_export]
macro_rules! print {
	// Static (zero-allocation) implementation that uses compile-time `concat!()` only
	($fmt:expr) => ({
		let msg = $crate::cstr!($fmt);
		let ptr = msg.as_ptr() as *const $crate::libc::c_char;
        unsafe {
	        uprintf(ptr);
        };
	});

	// Dynamic implementation that processes format arguments
	($fmt:expr, $($arg:tt)*) => ({
		use ::core::fmt::Write;
		use $crate::io::KernelDebugWriter;
		let mut writer = KernelDebugWriter {};
        writer.write_fmt(format_args!($fmt, $($arg)*)).unwrap();
	});
}

// Print kernel debug messages with a trailing newline
#[macro_export]
macro_rules! println {
	($fmt:expr)              => ($crate::print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)+) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

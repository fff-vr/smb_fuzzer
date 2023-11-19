extern crate libc;
use libc::{ioctl, syscall, SYS_gettid};
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

// Manually define the constants
const KCOV_INIT_TRACE: u64 = 1;
const KCOV_ENABLE: u64 = 100;
const KCOV_DISABLE: u64 = 101;

fn main() -> io::Result<()> {
    // Open the kcov device
    let mut file = File::create("/sys/kernel/debug/kcov")?;

    // Initialize kcov
    let buffer_size: u64 = 64; // Adjust the buffer size as needed
    unsafe {
        ioctl(file.as_raw_fd(), KCOV_INIT_TRACE, &buffer_size);
    }

    // Enable kcov for the current thread
    let tid = unsafe { syscall(SYS_gettid) };
    unsafe {
        ioctl(file.as_raw_fd(), KCOV_ENABLE , tid as u64);
    }

    // Your code here...

    // Disable kcov
    unsafe {
        ioctl(file.as_raw_fd(), KCOV_DISABLE, 0);
    }

    // Read coverage data
    let mut coverage_data = Vec::new();
    file.read_to_end(&mut coverage_data)?;

    println!("Coverage data: {:?}", coverage_data);

    Ok(())
}


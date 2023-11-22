use debug_print::debug_println;
use debug_print::debug_print;
pub fn hexdump(label: &str, data: &[u8]) {
    debug_println!("{}", label);
    for (i, byte) in data.iter().enumerate() {
        debug_print!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            debug_println!();
        }
    }
    debug_println!("\n");
}



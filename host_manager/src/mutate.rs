use rand::Rng;
use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};
/// Mutates a part of the given Vec<u8>.
/// 
/// # Arguments
///
/// * `data` - A mutable reference to a Vec<u8> to mutate.
/// * `mutation_rate` - A percentage (0-100) indicating how much of the Vec should be mutated.
fn hexdump(label: &str, data: &[u8]) {
    debug_println!("{}", label);
    for (i, byte) in data.iter().enumerate() {
        debug_print!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            debug_println!();
        }
    }
    debug_println!("\n");
}


pub fn mutate(data: &mut Vec<u8>, mutation_rate: f32) {
    let length = data.len();
    if length <4{
        return;
    }
    hexdump("before mutate",&data[0..]);
    let num_mutations = (length as f32 * mutation_rate / 100.0).round() as usize;

    let mut rng = rand::thread_rng();
    for _ in 0..num_mutations {
        let index = rng.gen_range(4..length);
        // Mutate the byte at the chosen index. This example simply toggles the byte,
        // but other mutations like random byte replacement can also be used.
        data[index] = rng.gen();
    }

    hexdump("after mutate",&data[0..]);
}


use rand::Rng;
use crate::protocol::smb1;
/// Mutates a part of the given Vec<u8>.
/// 
/// # Arguments
///
/// * `data` - A mutable reference to a Vec<u8> to mutate.
/// * `mutation_rate` - A percentage (0-100) indicating how much of the Vec should be mutated.
pub fn smb1_mutate(data: &mut Vec<u8>, mutation_rate: f32) {
    let length = data.len();
    if length <4{
        return;
    }
    //tools::hexdump("before mutate",&data[0..]);
    let num_mutations = (length as f32 * mutation_rate / 100.0).round() as usize;

    let mut rng = rand::thread_rng();
    for _ in 0..num_mutations {
        let index = rng.gen_range(4..length);
        // Mutate the byte at the chosen index. This example simply toggles the byte,
        // but other mutations like random byte replacement can also be used.
        data[index] = rng.gen();
    }

    //tools::hexdump("after mutate",&data[0..]);
}


//fix dynamic value : mabye mid,uid,pid ...
pub fn fix_dynamic_value()->Vec<u8>{
    vec![]
}

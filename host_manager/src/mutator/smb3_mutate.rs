use crate::mutator::input_queue;
use crate::tools;
use debug_print::debug_println;
use rand::Rng;

pub fn smb3_mutate_dumb(data: &mut Vec<u8>, mutation_rate: f32) -> input_queue::Fragments {
    let length = data.len();
    let mut fragments = input_queue::Fragments::new();
    if length < 4 {
        return fragments;
    }

    let num_mutations = (length as f32 * mutation_rate / 100.0).round() as usize;
    let mut rng = rand::thread_rng();
    for _ in 0..num_mutations {
        let index = rng.gen_range(4..length);
        let value: u8 = rng.gen();
        // Mutate the byte at the chosen index. This example simply toggles the byte,
        // but other mutations like random byte replacement can also be used.
        data[index] = value;
        let corpus = input_queue::Fragment {
            offset: index,
            value: value,
        };
        fragments.push(corpus)
    }
    fragments.clone()
}
fn apply_fragments(
    data: &mut Vec<u8>,
    fragments: &input_queue::Fragments,
    packet_count: u32,
) -> bool {
    for fragment in fragments.iter() {
        if fragment.offset >= data.len() {
            return false;
        }
        data[fragment.offset] = fragment.value;
    }
    true
}
pub fn smb3_mutate_coverage(
    data: &mut Vec<u8>,
    mutation_rate: f32,
    mut fragments: input_queue::Fragments,
    packet_count: u32,
) -> input_queue::Fragments {
    apply_fragments(data, &fragments, packet_count);
    let new_fragments = smb3_mutate_dumb(data, mutation_rate);
    fragments.merge(new_fragments);
    fragments
}
//fix dynamic value : mabye mid,uid,pid ...

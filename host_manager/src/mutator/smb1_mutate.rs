use crate::protocol::smb3::{self, Smb3NegotiateResponse};
use crate::tools;
use debug_print::{debug_print, debug_println};
use rand::Rng;
fn set_command(packet: &mut Vec<u8>, command: u16) {
    packet[16] = ((command >> 8) & 0xFF) as u8; // High byte of the command
    packet[17] = (command & 0xFF) as u8; // Low byte of the command
}
/// Mutates a part of the given Vec<u8>.
///
/// # Arguments
///
/// * `data` - A mutable reference to a Vec<u8> to mutate.
/// * `mutation_rate` - A percentage (0-100) indicating how much of the Vec should be mutated.
fn save_negotiate(packet: &Vec<u8>) -> smb3::Smb3NegotiateResponse {
    let re = smb3::parse_smb3_negotiate_response(packet);
    debug_println!("save timestamp = {:#x}", re.timestamp);
    re
}

fn set_negotiate(packet: &mut Vec<u8>, saved_negotiate: smb3::Smb3NegotiateResponse) {
    debug_println!("set timestamp = {:#x}", saved_negotiate.timestamp);
    smb3::set_smb3_negotiate_response(packet, saved_negotiate);
}
pub fn smb1_mutate(data: &mut Vec<u8>, mutation_rate: f32) {
    let length = data.len();
    if length < 4 {
        return;
    }

    let command = smb3::parse_command_from_packet(&data[..]);
    let mut saved_negotiate = smb3::Smb3NegotiateResponse::new();

    match command {
        smb3::Smb2Command::Negotiate => {
            tools::hexdump("timestamp", &data);
            saved_negotiate = save_negotiate(&data);
        }
        _ => panic!("TODO"),
    }
    let num_mutations = (length as f32 * mutation_rate / 100.0).round() as usize;
    let mut rng = rand::thread_rng();
    for _ in 0..num_mutations {
        let index = rng.gen_range(4..length);
        // Mutate the byte at the chosen index. This example simply toggles the byte,
        // but other mutations like random byte replacement can also be used.
        data[index] = rng.gen();
    }
    match command {
        smb3::Smb2Command::Negotiate => {
            set_negotiate(data, saved_negotiate);
        }
        _ => panic!("TODO"),
    }

    //restore opcode. TODO mutate this?
    set_command(data, command.to_u16());

    //tools::hexdump("after mutate",&data[0..]);
}

//fix dynamic value : mabye mid,uid,pid ...
pub fn fix_dynamic_value() -> Vec<u8> {
    vec![]
}

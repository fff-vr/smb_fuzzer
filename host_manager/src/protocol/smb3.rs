use crate::tools;
use core::time;
use debug_print::{debug_eprintln, debug_println};
#[derive(Debug)]

pub enum Smb2Command {
    Negotiate,      // 0x0000
    SessionSetup,   // 0x0001
    TreeConnect,    // 0x0003
    Create,         // 0x0005
    Close,          // 0x0006
    Read,           // 0x0008
    Write,          // 0x0009
    Ioctl,          // 0x0011
    ChangeNotify,   // 0x0012
    QueryDirectory, // 0x0014
    QueryInfo,      // 0x0014
    SetInfo,        // 0x0015
    OplockBreak,    // 0x0012
    // Add other SMB2 commands here as needed
    Unknown(u16),
}
impl Smb2Command {
    pub fn to_u16(&self) -> u16 {
        match *self {
            Smb2Command::Negotiate => 0x0000,
            Smb2Command::SessionSetup => 0x0001,
            Smb2Command::TreeConnect => 0x0003,
            Smb2Command::Create => 0x0005,
            Smb2Command::Close => 0x0006,
            Smb2Command::Read => 0x0008,
            Smb2Command::Write => 0x0009,
            Smb2Command::Ioctl => 0x0011,
            Smb2Command::ChangeNotify => 0x0012,
            Smb2Command::QueryDirectory => 0x0014,
            Smb2Command::QueryInfo => 0x0014,
            Smb2Command::SetInfo => 0x0015,
            Smb2Command::OplockBreak => 0x0012,
            Smb2Command::Unknown(value) => value,
        }
    }
}

#[derive(Debug)]
pub struct Smb2Header {
    protocol_id: [u8; 4],
    structure_size: u16,
    credit_charge: u16,
    channel_sequence_reserved: u16,
    channel_id_reserved: u16,
    command: Smb2Command,
    credits: u16,
    flags: u32,
    chain_offset: u32,
    message_id: u64, //MID
    reserved: u32,
    tree_id: u32,
    session_id: u64,
    signature: [u8; 12],
}

#[derive(Debug, Default)]
//only need to fix
pub struct Smb3NegotiateResponse {
    pub timestamp: u64, // Buffer: Security buffer and negotiate contexts (Variable size, not included here)
}
impl Smb3NegotiateResponse {
    pub fn new() -> Self {
        Self { timestamp: 0 }
    }
}

pub fn parse_smb2_header(packet: &[u8]) -> Smb2Header {
    if packet.len() < 64 {
        panic!("Packet too short to be a valid SMB2 header");
    }

    let protocol_id = [packet[4], packet[5], packet[6], packet[7]];
    let structure_size = u16::from_be_bytes([packet[8], packet[9]]);
    let credit_charge = u16::from_be_bytes([packet[10], packet[11]]);
    let channel_sequence_reserved = u16::from_be_bytes([packet[12], packet[13]]);
    let channel_id_reserved = u16::from_be_bytes([packet[14], packet[15]]);
    let command_value = u16::from_be_bytes([packet[16], packet[17]]);
    let command = match command_value {
        0x0000 => Smb2Command::Negotiate,
        0x0001 => Smb2Command::SessionSetup,
        0x0003 => Smb2Command::TreeConnect,
        0x0005 => Smb2Command::Create,
        0x0006 => Smb2Command::Close,
        0x0008 => Smb2Command::Read,
        0x0009 => Smb2Command::Write,
        0x0011 => Smb2Command::Ioctl,
        0x0012 => Smb2Command::ChangeNotify,
        0x0014 => Smb2Command::QueryDirectory,
        0x0015 => Smb2Command::SetInfo,
        // Add cases for other commands as needed
        _ => {
            println!("unknown command = {:#x}", command_value);
            Smb2Command::Unknown(command_value)
        }
    };
    let credits = u16::from_be_bytes([packet[18], packet[19]]);
    let flags = u32::from_be_bytes([packet[20], packet[21], packet[22], packet[23]]);
    let chain_offset = u32::from_be_bytes([packet[24], packet[25], packet[26], packet[27]]);
    let message_id = u64::from_be_bytes([
        packet[28], packet[29], packet[30], packet[31], packet[32], packet[33], packet[34],
        packet[35],
    ]);
    let reserved = u32::from_be_bytes([packet[36], packet[37], packet[38], packet[39]]);
    let tree_id = u32::from_be_bytes([packet[40], packet[41], packet[42], packet[43]]);
    let session_id = u64::from_be_bytes([
        packet[44], packet[45], packet[46], packet[47], packet[48], packet[49], packet[50],
        packet[51],
    ]);
    let signature = [
        packet[52], packet[53], packet[54], packet[55], packet[56], packet[57], packet[58],
        packet[59], packet[60], packet[61], packet[62], packet[63],
    ];

    Smb2Header {
        protocol_id,
        structure_size,
        credit_charge,
        channel_sequence_reserved,
        channel_id_reserved,
        command,
        credits,
        flags,
        chain_offset,
        message_id,
        reserved,
        tree_id,
        session_id,
        signature,
    }
}

pub fn parse_smb3_negotiate_response(packet: &[u8]) -> Smb3NegotiateResponse {
    tools::hexdump("[parse_smb3_negotiate_response]", packet);
    let base = 0x68 - 1 + 4;
    let timestamp = u64::from_le_bytes([
        packet[base + 1],
        packet[base + 2],
        packet[base + 3],
        packet[base + 4],
        packet[base + 5],
        packet[base + 6],
        packet[base + 7],
        packet[base + 8],
    ]);
    Smb3NegotiateResponse {
        timestamp: timestamp,
    }
}
pub fn set_smb3_negotiate_response(packet: &mut Vec<u8>, data: Smb3NegotiateResponse) {
    let bytes = data.timestamp.to_le_bytes();

    let base = 0x68 - 1 + 4;
    for i in 0..8 {
        packet[base + i] = bytes[i];
    }
}

pub fn parse_smb3_packet(packet: &[u8]) {
    let header = parse_smb2_header(&packet[..0x40]);
    let body = parse_smb3_negotiate_response(&packet[0x40..]);
    debug_println!("header = {:?}", header);
    debug_println!("body = {:?}", body);
}
pub fn parse_command_from_packet(packet: &[u8]) -> Smb2Command {
    let header = parse_smb2_header(&packet[..0x40]);
    header.command
}

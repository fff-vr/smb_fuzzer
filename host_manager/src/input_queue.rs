use std::collections::VecDeque;

use crate::protocol::smb3::{self, Smb2Command};
use rand::rngs::ThreadRng;
use rand::Rng;
pub struct InputQueue {
    nego_queue: VecDeque<Vec<u8>>,
    setup_queue: VecDeque<Vec<u8>>,
    gettree_queue: VecDeque<Vec<u8>>,
    rng: ThreadRng,
}
impl InputQueue {
    pub fn new() -> Self {
        Self {
            nego_queue: Default::default(),
            setup_queue: Default::default(),
            gettree_queue: Default::default(),
            rng: Default::default(),
        }
    }
    pub fn print_corpus_count(&self) {
        print!("nego_queue = {}", self.nego_queue.len());
        print!(", setup_queue = {}", self.setup_queue.len());
        println!(", gettree_queue = {}", self.gettree_queue.len());
    }
    fn get_nego(&mut self) -> Vec<u8> {
        let idx = self.rng.gen_range(0..self.nego_queue.len());
        self.nego_queue[idx].clone()
    }
    fn get_setup(&mut self) -> Vec<u8> {
        let idx = self.rng.gen_range(0..self.setup_queue.len());
        self.setup_queue[idx].clone()
    }
    fn get_gettree(&mut self) -> Vec<u8> {
        let idx = self.rng.gen_range(0..self.gettree_queue.len());
        self.gettree_queue[idx].clone()
    }
    fn insert_nego(&mut self, new_input: Vec<u8>) {
        self.nego_queue.push_back(new_input);
    }
    fn insert_setup(&mut self, new_input: Vec<u8>) {
        self.setup_queue.push_back(new_input);
    }
    fn insert_gettree(&mut self, new_input: Vec<u8>) {
        self.gettree_queue.push_back(new_input);
    }
    pub fn get_input(&mut self, original_packet: Vec<u8>) -> Vec<u8> {
        let command = smb3::parse_command_from_packet(&original_packet);
        match command {
            Smb2Command::Negotiate => {
                if self.nego_queue.len() == 0 {
                    original_packet
                } else {
                    self.get_nego()
                }
            }
            Smb2Command::SessionSetup => {
                if self.setup_queue.len() == 0 {
                    original_packet
                } else {
                    self.get_setup()
                }
            }
            Smb2Command::TreeConnect => {
                if self.setup_queue.len() == 0 {
                    original_packet
                } else {
                    self.get_gettree()
                }
            }
            _ => panic!("Unknown Command"),
        }
    }
    pub fn insert_input(&mut self, new_input: Vec<u8>) {
        //smb3::parse_smb3_packet(&new_input);

        let command = smb3::parse_command_from_packet(&new_input);
        match command {
            Smb2Command::Negotiate => self.insert_nego(new_input),
            Smb2Command::SessionSetup => self.insert_setup(new_input),
            Smb2Command::TreeConnect => self.insert_gettree(new_input),
            _ => {
                let header = smb3::parse_smb2_header(&new_input);
                //let body = smb3::parse_smb3_packet(&new_input);
                panic!("Unknown Command\n{:?}\n{:?}", header, new_input);
            }
        }
    }
}

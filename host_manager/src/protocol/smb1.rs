struct packet{
    test : u8,
    opcode :u8
}
impl packet{
    pub fn new(raw_packet: Vec<u8>) -> Self {
        let test = raw_packet[0];
        let opcode = raw_packet[1];

        Self {
            test : test,
            opcode : opcode
        }
    }

}

#[repr(C)]
pub struct MutateData {
    offset: u32,
    data: Vec<u8>,
}

#[repr(C)]
pub struct PakcetMutateData {
    //array of MutateData
    pakcet_idx: u32,
    mutate_data: Vec<MutateData>,
}

#[repr(C)]
pub struct PacketMutateDataArray {
    //array of PacketMutateData
    aaa: Vec<PakcetMutateData>,
}

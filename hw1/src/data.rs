#[derive(bincode::Encode, bincode::Decode)]
pub(crate) struct DataHeader {
    pub parts_count: u32,
}

#[derive(bincode::Encode, bincode::Decode)]
pub(crate) struct DataPacket {
    pub part: u32,
    pub data: Vec<u8>,
}

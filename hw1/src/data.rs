use std::fmt::Debug;

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub(crate) struct DataHeader {
    pub parts_count: u32,
}

#[derive(bincode::Encode, bincode::Decode)]
pub(crate) struct DataPacket {
    pub part: u32,
    pub data: Vec<u8>,
}

impl Debug for DataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataPacket")
            .field("part", &self.part)
            .field("data_size", &self.data.len())
            .finish()
    }
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum UdpPacket {
    Header(DataHeader),
    DataPart(DataPacket),
    Reset,
}

#[derive(bincode::Encode, bincode::Decode, PartialEq, Eq, Debug)]
pub enum ConfirmPacketVariant {
    Header,
    Part(u32),
}

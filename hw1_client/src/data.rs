use ring::rand::{SecureRandom, SystemRandom};

#[derive(bincode::Encode, bincode::Decode)]
pub struct DataHeader {
    pub parts_count: u32
}

#[derive(bincode::Encode, bincode::Decode)]
pub struct DataPacket {
    pub part: u32,
    pub data: Vec<u8>
}

pub fn generate_data(length: u32) -> Vec<u8> {
    let sys_random = SystemRandom::new();
    
    // Create a vector to hold the random bytes
    let mut buffer = vec![0u8; length as usize]; // 16 bytes of random data
    
    // Fill the vector with random bytes
    sys_random.fill(&mut buffer).unwrap();

    return buffer;
}

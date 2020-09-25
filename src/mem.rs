use crate::helpers::readFileIntoVec;

pub struct Memory {
    // main. non-IO memory
    ROM: Vec<u8>
}

impl Memory {
    pub fn new(romPath: String) -> Memory {
        Memory {
            ROM: readFileIntoVec(&romPath)
        }
    }
}
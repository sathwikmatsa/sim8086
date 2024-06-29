pub struct Memory([u8; 1024 * 64]);

impl Default for Memory {
    fn default() -> Self {
        Self([0u8; 1024 * 64])
    }
}

impl Memory {
    pub fn raw(&self) -> &[u8] {
        &self.0
    }

    pub fn load_16(&self, addr: u16) -> u16 {
        let low = *self.0.get(addr as usize).expect("addr in range");
        let high = *self.0.get(addr as usize + 1).expect("addr in range");
        u16::from_le_bytes([low, high])
    }

    pub fn load_8(&self, addr: u16) -> u8 {
        *self.0.get(addr as usize).expect("addr in range")
    }

    pub fn store_16(&mut self, addr: u16, val: u16) {
        let [low, high] = val.to_le_bytes();
        *self.0.get_mut(addr as usize).expect("addr in range") = low;
        *self.0.get_mut(addr as usize + 1).expect("addr in range") = high;
    }

    pub fn store_8(&mut self, addr: u16, val: u8) {
        *self.0.get_mut(addr as usize).expect("addr in range") = val;
    }
}

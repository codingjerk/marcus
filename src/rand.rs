// This is fast PRNG, using xorshift
// It's suitable to use with fuzzing only
pub struct FastRng(u64);

impl FastRng {
    #[cfg(test)]
    pub fn from_system_time() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let time = unsafe {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_unchecked()
                .as_secs()
        };

        Self(time)
    }

    pub fn rand_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;

        self.0
    }

    pub fn rand_bool(&mut self) -> bool {
        self.rand_u64() % 2 == 0
    }

    pub fn rand_range_u16(&mut self, min: u16, max: u16) -> u16 {
        let val = self.rand_u64() as u16;

        (val % (max - min)) + min
    }

    pub fn rand_range_u8(&mut self, min: u8, max: u8) -> u8 {
        let val = self.rand_u64() as u8;

        (val % (max - min)) + min
    }
}

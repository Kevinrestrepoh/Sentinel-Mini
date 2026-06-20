pub struct Battery;

const MIN_MV: u32 = 1_700; // ≈ 3.4 V cell × 0.5 divider
const MAX_MV: u32 = 2_100; // ≈ 4.2 V cell × 0.5 divider

impl Battery {
    pub fn new() -> Self {
        Self
    }

    pub fn percent(&self) -> u8 {
        let mv = self.read_raw();
        let clamped = mv.clamp(MIN_MV, MAX_MV);
        ((clamped - MIN_MV) * 100 / (MAX_MV - MIN_MV)) as u8
    }

    fn read_raw(&self) -> u32 {
        2_054
    }
}

impl Default for Battery {
    fn default() -> Self {
        Self::new()
    }
}

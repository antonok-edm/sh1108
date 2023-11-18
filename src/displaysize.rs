//! Display size

/// Display size enumeration
#[derive(Clone, Copy, Debug)]
pub enum DisplaySize {
    /// 64 by 160 pixels
    Display64x160 = 0x0,
    /// 96 by 160 pixels
    Display96x160 = 0x1,
    /// 128 by 160 pixels
    Display128x160 = 0x2,
    /// 160 by 160 pixels
    Display160x160 = 0x3,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    pub fn dimensions(self) -> (u8, u8) {
        match self {
            DisplaySize::Display64x160 => (64, 160),
            DisplaySize::Display96x160 => (96, 160),
            DisplaySize::Display128x160 => (128, 160),
            DisplaySize::Display160x160 => (160, 160),
        }
    }

    /// Get the panel column offset from DisplaySize
    pub fn column_offset(self) -> u8 {
        match self {
            DisplaySize::Display64x160 => 48,
            DisplaySize::Display96x160 => 32,
            DisplaySize::Display128x160 => 16,
            DisplaySize::Display160x160 => 0,
        }
    }
}

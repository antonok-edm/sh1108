//! sh1108 Commands

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

/// Commands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    /// Set contrast. Higher number is higher contrast. Default = 0x7F
    Contrast(u8),
    /// Turn entire display on. If set, all pixels will
    /// be set to on, if not, the value in memory will be used.
    AllOn(bool),
    /// Invert display.
    Invert(bool),
    /// Set display resolution.
    DisplayResolution(crate::displaysize::DisplaySize),
    /// Set the addressing mode.
    /// `false` is page addressing mode.
    /// `true` is vertical addressing mode.
    AddressMode(bool),
    /// Turn display on or off.
    DisplayOn(bool),
    /// Set column address lower 4 bits
    ColumnAddressLow(u8),
    /// Set column address higher 4 bits
    ColumnAddressHigh(u8),
    /// Set page address
    PageAddress(u8),
    /// Reverse columns from 127-0
    SegmentRemap(bool),
    /// Set the scan direction of the output.
    /// `false` scans from COM0 to COM[n-1].
    /// `true` scans from COM[n-1] to COM0.
    SetCommonScanDir(bool),
    /// Set up display clock.
    /// First value is oscillator frequency, increasing with higher value
    /// Second value is divide ratio - 1
    DisplayClockDiv(u8, u8),
    /// Set up phase 1 and 2 of precharge period. each value is from 0-63
    PreChargePeriod(u8, u8),
    /// NOOP
    Noop,
}

impl Command {
    /// Send command to sh1108
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        // Transform command into a fixed size array of 7 u8 and the real length for sending
        let (data, len) = match self {
            Command::Contrast(val) => ([0x81, val, 0, 0, 0, 0, 0], 2),
            Command::AllOn(on) => ([0xA4 | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::Invert(inv) => ([0xA6 | (inv as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayResolution(resolution) => ([0xA9, resolution as u8, 0, 0, 0, 0, 0], 2),
            Command::AddressMode(mode) => ([0x20 | (mode as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayOn(on) => ([0xAE | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::ColumnAddressLow(addr) => ([0xF & addr, 0, 0, 0, 0, 0, 0], 1),
            Command::ColumnAddressHigh(addr) => ([0x10 | (0xF & addr), 0, 0, 0, 0, 0, 0], 1),
            Command::PageAddress(page) => ([0xB0, page, 0, 0, 0, 0, 0], 2),
            Command::SegmentRemap(remap) => ([0xA0 | (remap as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::SetCommonScanDir(rev) => ([0xC0 | ((rev as u8) << 3), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayClockDiv(fosc, div) => {
                ([0xD5, ((0xF & fosc) << 4) | (0xF & div), 0, 0, 0, 0, 0], 2)
            }
            Command::PreChargePeriod(phase1, phase2) => (
                [0xD9, ((0xF & phase2) << 4) | (0xF & phase1), 0, 0, 0, 0, 0],
                2,
            ),
            Command::Noop => ([0xE3, 0, 0, 0, 0, 0, 0], 1),
        };

        // Send command over the interface
        iface.send_commands(DataFormat::U8(&data[0..len]))
    }
}

/// Frame interval
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum NFrames {
    /// 2 Frames
    F2 = 0b111,
    /// 3 Frames
    F3 = 0b100,
    /// 4 Frames
    F4 = 0b101,
    /// 5 Frames
    F5 = 0b000,
    /// 25 Frames
    F25 = 0b110,
    /// 64 Frames
    F64 = 0b001,
    /// 128 Frames
    F128 = 0b010,
    /// 256 Frames
    F256 = 0b011,
}

/// Vcom Deselect level
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum VcomLevel {
    // Note: lots of other values are valid here too
    /// 0.77 * Vcc
    V077 = 0x35,
    /// 0.43 * Vcc
    V043 = 0x00,
    /// 0.43 * Vcc
    V0834 = 0x3f,
}

use std::fmt;

#[repr(C)]
#[derive(Debug)]
pub struct StatusResponse {
   command: u8,
   cart_cgb: u8,
   ver_maj: u8,
   ver_min: u8,
   flash_manufacturer: u8,
   flash_device_id: u8,
   flash_sector_group_protect: u8,
   byte7: u8,
   cart_logo_correct: u8,
   cart_name: [u8;16],
   cart_new_licensee_hi: u8,
   cart_new_licensee_lo: u8,
   cart_sgb: u8,
   cart_type: u8,
   cart_rom_size: u8,
   cart_ram_size: u8,
   cart_dst_code: u8,
   cart_old_licensee: u8,
   cart_mask_rom_version: u8,
   cart_complement: u8,
   cart_checksum_hi: u8,
   cart_checksum_lo: u8,
   // hax so it's small enough to implement debug
   unused: [u8;20],
   unused2: [u8;13],
   response_checksum: [u8;2],
}

impl fmt::Display for StatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Firmware version {:x}{:x}.{:x}{:x}", 
            ((self.ver_maj&0xf0) >> 4), self.ver_maj&0x0f, ((self.ver_min&0xf0) >> 4), self.ver_min&0x0f
        )?;
        writeln!(f, "Game Title: {}", std::str::from_utf8(&self.cart_name).unwrap())?;
        writeln!(f, "CGB Only: {}", self.cart_cgb==0xC0)?;
        writeln!(f, "Flash Manufacturer ID: {:#04x}", self.flash_manufacturer)?;
        writeln!(f, "Flash Manufacturer Name: {:?}", 
            FlashManufacturer::from(self.flash_manufacturer)
        )?;
        writeln!(f, "Flash Device ID: {:#04x}", self.flash_device_id)?;
        writeln!(f, "Nintendo logo correct: {}", self.cart_logo_correct == 0x01)?;
        writeln!(f, "Cartridge licensee: {:#04x}",
            if self.cart_old_licensee == 0x33 {
                u16::from_be_bytes([self.cart_new_licensee_hi, self.cart_new_licensee_lo])
            } else {
                u16::from(self.cart_old_licensee)
            }
        )?;
        writeln!(f, "SGB Enhanced: {}", self.cart_sgb != 0)?;
        writeln!(f, "Cartridge Type: {:?}", 
            CartType::from(self.cart_type)
        )?;
        writeln!(f, "ROM size: {}KB", (32 << self.cart_rom_size))?;
        writeln!(f, "RAM size: {}", 
            match(self.cart_ram_size) {
                0x00 => "0KB",
                0x01 => "2KB",
                0x02 => "8KB",
                0x03 => "32KB",
                0x04 => "128KB",
                0x05 => "64KB",
                _    => "Unknown"
            }
        )?;
        writeln!(f, "Japanese: {}", self.cart_dst_code==0x00)?;
        writeln!(f, "ROM version: {:#04x}", self.cart_mask_rom_version)?;
        writeln!(f, "Header checksum: {:#04x}", self.cart_complement)?;
        writeln!(f, "Cartridge checksum: {:#04x}",
            u16::from_be_bytes([self.cart_checksum_hi, self.cart_checksum_lo])
        )
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum FlashManufacturer {
    ALLIANCE = 0x52,
    AMD = 0x01,
    AMIC = 0x37,
    ATMEL = 0x1F,
    BRIGHT_HYUNDAI = 0xAD,
    CATALYST = 0x31,
    ESMT = 0x8C,
    EON = 0x1C,
    EXCEL = 0x4A,
    FIDELIX = 0xF8,
    FUJITSU = 0x04,
    GIGADEVICE = 0xC8,
    INTEL = 0x89,
    MACRONIX = 0xC2,
    ISSI_NANTRONICS = 0xD5,
    PMC = 0x9D,
    SANYO = 0x62,
    SHARP = 0xB0,
    SST = 0xBF,
    ST = 0x20,
    SM = 0x40,
    TENX = 0x5E,
    TI = 0x97,
    NEXCOM = 0xEF,
    WINBOND = 0xDA,
    ZETTADEVICE = 0xBA,
    UNKNOWN = 0xFF
} 

impl From<u8> for FlashManufacturer {
    fn from(value: u8) -> FlashManufacturer {
        match (value) {
            0x52 => FlashManufacturer::ALLIANCE,
            0x01 => FlashManufacturer::AMD,
            0x37 => FlashManufacturer::AMIC,
            0x1F => FlashManufacturer::ATMEL,
            // These two companies share one code
            0xAD => FlashManufacturer::BRIGHT_HYUNDAI,
            0x31 => FlashManufacturer::CATALYST,
            0x8C => FlashManufacturer::ESMT,
            0x1C => FlashManufacturer::EON,
            0x4A => FlashManufacturer::EXCEL,
            0xF8 => FlashManufacturer::FIDELIX,
            0x04 => FlashManufacturer::FUJITSU,
            0xC8 => FlashManufacturer::GIGADEVICE,
            0x89 => FlashManufacturer::INTEL,
            // Why do they do this?
            0xD5 => FlashManufacturer::ISSI_NANTRONICS,
            0xC2 => FlashManufacturer::MACRONIX,
            0x9D => FlashManufacturer::PMC,
            0x62 => FlashManufacturer::SANYO,
            0xB0 => FlashManufacturer::SHARP,
            0xBF => FlashManufacturer::SST,
            0x20 => FlashManufacturer::ST,
            0x40 => FlashManufacturer::SM,
            0x5E => FlashManufacturer::TENX,
            0x97 => FlashManufacturer::TI,
            0xEF => FlashManufacturer::NEXCOM,
            0xDA => FlashManufacturer::WINBOND,
            0xBA => FlashManufacturer::ZETTADEVICE,
            _    => FlashManufacturer::UNKNOWN,
        }
    }
}

#[repr(u16)]
pub enum OldLicensee {
    NoLicense = 0x00,
    nintendo = 0x01,
    capcom = 0x08,
    Hot_B = 0x09,
    Jaleco = 0x0A,
    Coconuts = 0x0B,
    Elite_Systems = 0x0C,
    EA = 0x13,
    HudsonSoft = 0x18,
    ITC = 0x19,
    Yanoman = 0x1A,
    Clary = 0x1D,
    virgin = 0x1F,
    KSS = 0x20,
    PCM_Complete = 0x24,
    San_X = 0x25,
    Kotobuki_Systems = 0x28,
    Seta = 0x29,
    Infogrames = 0x30,
    Nintendo = 0x31, 
    Bandai = 0x32,
    Konami = 0x34,
    Hector = 0x35,
    Capcom = 0x38,
    Banpresto = 0x39,
    EntertainmentI = 0x3C,
    Gremlin = 0x3E,
    Ubisoft = 0x41,
    Atlus = 0x42,
    malibu = 0x44,
    Angel = 0x46,
    Spectrum_Holoby = 0x47,
    Irem = 0x49,
    Virgin = 0x4A,
    Malibu = 0x4D,
    US_Gold = 0x4F,
    Absolute = 0x50,
    Acclaim = 0x51,
    Activision = 0x52,
    American_Sammy = 0x53,
    Gametek = 0x54,
    Park_Place = 0x55,
    LJN = 0x56,
    Matchbox = 0x57,
    Milton_Bradley = 0x59,
    Mindscape = 0x5A,
    Romstar = 0x5B,
    Naxat_Soft = 0x5C,
    Tradewest = 0x5D,
    Titus = 0x60,
    Virgin_Again = 0x61,
    Ocean = 0x67,
    Electronic_Arts = 0x69,

}

//#[repr(u16)]
pub enum NewLicensee {

}

#[repr(u8)]
#[derive(Debug)]
pub enum CartType {
    ROM = 0x00,
    MBC1 = 0x01,
    MBC1_RAM = 0x02,
    MBC1_RAM_BATTERY = 0x03,
    MBC2 = 0x05,
    MBC2_BATTERY = 0x06,
    ROM_RAM = 0x08,
    ROM_RAM_BATTERY = 0x09,
    MMM01 = 0x0B,
    MMM01_RAM = 0x0C,
    MMM01_RAM_BATTERY = 0x0D,
    MBC3_TIMER_BATTERY = 0x0F,
    MBC3_TIMER_RAM_BATTERY = 0x10,
    MBC3 = 0x11,
    MBC3_RAM = 0x12,
    MBC3_RAM_BATTERY = 0x13,
    MBC5 = 0x19,
    MBC5_RAM = 0x1A,
    MBC5_RAM_BATTERY = 0x1B,
    MBC5_RUMBLE = 0x1C,
    MBC5_RUMBLE_RAM = 0x1D,
    MBC5_RUMBLE_RAM_BATTERY = 0x1E,
    MBC6 = 0x20,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY = 0x22,
    UNKNOWN = 0x23,
    POCKET_CAMERA = 0xFC,
    BANDAI_TAMA5 = 0xFD,
    HuC3 = 0xFE,
    HuC1_RAM_BATTERY = 0xFF
}

impl From<u8> for CartType {
    fn from(value: u8) -> CartType {
        match (value) {
            0x00 => CartType::ROM,
            0x01 => CartType::MBC1,
            0x02 => CartType::MBC1_RAM,
            0x03 => CartType::MBC1_RAM_BATTERY,
            0x05 => CartType::MBC2,
            0x06 => CartType::MBC2_BATTERY,
            0x08 => CartType::ROM_RAM,
            0x09 => CartType::ROM_RAM_BATTERY,
            0x0B => CartType::MMM01,
            0x0C => CartType::MMM01_RAM,
            0x0D => CartType::MMM01_RAM_BATTERY,
            0x0F => CartType::MBC3_TIMER_BATTERY,
            0x10 => CartType::MBC3_TIMER_RAM_BATTERY,
            0x11 => CartType::MBC3,
            0x12 => CartType::MBC3_RAM,
            0x13 => CartType::MBC3_RAM_BATTERY,
            0x19 => CartType::MBC5,
            0x1A => CartType::MBC5_RAM,
            0x1B => CartType::MBC5_RAM_BATTERY,
            0x1C => CartType::MBC5_RUMBLE,
            0x1D => CartType::MBC5_RUMBLE_RAM,
            0x1E => CartType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => CartType::MBC6,
            0x22 => CartType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => CartType::POCKET_CAMERA,
            0xFD => CartType::BANDAI_TAMA5,
            0xFE => CartType::HuC3,
            0xFF => CartType::HuC1_RAM_BATTERY,
            _ => CartType::UNKNOWN
        }
    }
}

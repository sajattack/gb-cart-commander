use enumn::N;
use std::fmt;

#[repr(C)]
#[derive(Debug)]
pub struct InfoResponse {
    command: u8,
    cart_cgb: u8,
    ver_maj: u8,
    ver_min: u8,
    flash_manufacturer: u8,
    flash_device_id: u8,
    flash_sector_group_protect: u8,
    byte7: u8,
    cart_logo_correct: u8,
    cart_name: [u8; 16],
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
    unused: [u8; 20],
    unused2: [u8; 13],
    response_checksum: [u8; 2],
}

impl fmt::Display for InfoResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Firmware version {:x}{:x}.{:x}{:x}",
            ((self.ver_maj & 0xf0) >> 4),
            self.ver_maj & 0x0f,
            ((self.ver_min & 0xf0) >> 4),
            self.ver_min & 0x0f
        )?;
        writeln!(f, "Flash Manufacturer ID: {:#04x}", self.flash_manufacturer)?;
        writeln!(
            f,
            "Flash Manufacturer Name: {:?}",
            FlashManufacturer::n(self.flash_manufacturer).unwrap_or(FlashManufacturer::Unknown)
        )?;
        writeln!(f, "Flash Device ID: {:#04x}", self.flash_device_id)?;
        if self.cart_logo_correct == 1 {
            writeln!(
                f,
                "Game Title: {}",
                std::str::from_utf8(&self.cart_name).unwrap_or("")
            )?;
            writeln!(f, "CGB Only: {}", self.cart_cgb == 0xC0)?;
            if self.cart_old_licensee == 0x33 {
                writeln!(
                    f,
                    "Cartridge Licensee: {:?}",
                    NewLicensee::n(u16::from_be_bytes([
                        self.cart_new_licensee_hi,
                        self.cart_new_licensee_lo
                    ]))
                    .unwrap_or(NewLicensee::Unknown)
                )?;
            } else {
                writeln!(
                    f,
                    "Cartridge Licensee: {:?}",
                    OldLicensee::n(self.cart_old_licensee).unwrap_or(OldLicensee::Unknown)
                )?;
            }
            writeln!(f, "SGB Enhanced: {}", self.cart_sgb != 0)?;
            writeln!(
                f,
                "Cartridge Type: {:?}",
                CartType::n(self.cart_type).unwrap_or(CartType::Unknown)
            )?;
            writeln!(f, "ROM size: {}KB", (32 << self.cart_rom_size))?;
            writeln!(
                f,
                "RAM size: {}",
                match self.cart_ram_size {
                    0x00 => "0KB",
                    0x01 => "2KB",
                    0x02 => "8KB",
                    0x03 => "32KB",
                    0x04 => "128KB",
                    0x05 => "64KB",
                    _ => "Unknown",
                }
            )?;
            writeln!(f, "Japanese: {}", self.cart_dst_code == 0x00)?;
            writeln!(f, "ROM version: {:#04x}", self.cart_mask_rom_version)?;
            writeln!(f, "Header checksum: {:#04x}", self.cart_complement)?;
            writeln!(
                f,
                "Cartridge checksum: {:#04x}",
                u16::from_be_bytes([self.cart_checksum_hi, self.cart_checksum_lo])
            )
        } else {
            write!(f, "Cartridge is blank, damaged, unlicensed, or disconnected (logo incorrect)")
        }
    }
}

#[repr(u8)]
#[derive(Debug, N)]
pub enum FlashManufacturer {
    Alliance = 0x52,
    AMD = 0x01,
    AMIC = 0x37,
    Atmel = 0x1F,
    Bright_Hyundai = 0xAD,
    Catalyst = 0x31,
    ESMT = 0x8C,
    Eon = 0x1C,
    Excel = 0x4A,
    Fidelix = 0xF8,
    Fujitsu = 0x04,
    Gigadevice = 0xC8,
    Intel = 0x89,
    Macronix = 0xC2,
    Issi_Nantronics = 0xD5,
    PMC = 0x9D,
    Sanyo = 0x62,
    Sharp = 0xB0,
    SST = 0xBF,
    ST = 0x20,
    SM = 0x40,
    TenX = 0x5E,
    TI = 0x97,
    Nexcom = 0xEF,
    Winbond = 0xDA,
    ZettaDevice = 0xBA,
    Unknown = 0xFF,
}

// Wew there are a lot of dupes in here
#[repr(u8)]
#[derive(Debug, N)]
pub enum OldLicensee {
    Unlicensed = 0x00,
    nintendo = 0x01,
    Unknown = 0x02,
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
    infogrames = 0x30,
    Nintendo = 0x31,
    Bandai = 0x32,
    konami = 0x34,
    Hector = 0x35,
    Capcom = 0x38,
    Banpresto = 0x39,
    EntertainmentI = 0x3C,
    Gremlin = 0x3E,
    Ubisoft = 0x41,
    Atlus = 0x42,
    malibu = 0x44,
    angel = 0x46,
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
    Virgin_ = 0x61,
    Ocean = 0x67,
    Electronic_Arts = 0x69,
    elite_systems = 0x6E,
    Electro_Brain = 0x6F,
    Infogrames = 0x70,
    Interplay = 0x71,
    Broderbund = 0x72,
    Sculptered_Soft = 0x73,
    The_Sales_Curve = 0x75,
    THQ = 0x78,
    Accolade = 0x79,
    Triffix_Entertainment = 0x7A,
    Microprose = 0x7C,
    Kemco = 0x7F,
    Misawa_Entertainment = 0x80,
    Lozc = 0x83,
    Tokuma_Shoten_Intermedia = 0x86,
    Bullet_Proof_Software = 0x8B,
    Vic_Tokai = 0x8C,
    Ape = 0x8E,
    Imax = 0x8F,
    Chun_Soft = 0x91,
    Video_System = 0x92,
    Tsuburava = 0x93,
    Varie = 0x95,
    Yonezawa = 0x96,
    Kaneko = 0x97,
    Arc = 0x99,
    Nihon_Bussan = 0x9A,
    Tecmo = 0x9B,
    Imagineer = 0x9C,
    banpresto = 0x9D,
    Nova = 0x9F,
    Hori_Electric = 0xA1,
    bandai = 0xA2,
    Konami = 0xA4,
    Kawada = 0xA6,
    Takara = 0xA7,
    Technos_Japan = 0xA9,
    broderbund = 0xAA,
    Toei_Animation = 0xAC,
    Toho = 0xAD,
    Namco = 0xAF,
    acclaim = 0xB0,
    Ascii_or_Nexoft = 0xB1,
    Bandai_ = 0xB2,
    Enix = 0xB4,
    HAL = 0xB6,
    SNK = 0xB7,
    Pony_Canyon = 0xB9,
    Culture_Brain = 0xBA,
    Sunsoft = 0xBB,
    Sony_Imagesoft = 0xBD,
    Sammy = 0xBF,
    Taito = 0xC0,
    kemco = 0xC2,
    Squaresoft = 0xC3,
    tokuma_shoten_intermedia = 0xC4,
    Data_East = 0xC5,
    Tonkin_House = 0xC6,
    Koei = 0xC8,
    UFL = 0xC9,
    Ultra = 0xCA,
    Vap = 0xCB,
    Use = 0xCC,
    Meidac = 0xCD,
    pony_canyon = 0xCE,
    Angel = 0xCF,
    taito = 0xD0,
    Sofel = 0xD1,
    Quest = 0xD2,
    Sigma_Enterprised = 0xD3,
    Ask_Kodansha = 0xD4,
    naxat_soft = 0xD6,
    Copya_Systems = 0xD7,
    Banpresto_ = 0xD9,
    Tomy = 0xDA,
    ljn = 0xD8,
    NCS = 0xDD,
    Human = 0xDE,
    Altron = 0xDF,
    jaleco = 0xE0,
    Towachiki = 0xE1,
    Uutaka = 0xE2,
    varie = 0xE3,
    Epoch = 0xE5,
    Athena = 0xE7,
    Asmik = 0xE8,
    Natsume = 0xE9,
    King_Records = 0xEA,
    atlus = 0xEB,
    Epic_Sony_records = 0xEC,
    IGS = 0xEE,
    A_Wave = 0xF0,
    Extreme_Entertainment = 0xF3,
    LJN_,
}

// This is some retarded ascii encoded bullshit
#[repr(u16)]
#[derive(Debug, N)]
pub enum NewLicensee {
    Unlicensed = to_ascii_u16(0x00),
    Nintendo = to_ascii_u16(0x01),
    Unknown = to_ascii_u16(0x02),
    Capcom = to_ascii_u16(0x08),
    EA = to_ascii_u16(0x13),
    HudsonSoft = to_ascii_u16(0x18),
    b_ai = to_ascii_u16(0x19),
    KSS = to_ascii_u16(0x20),
    Pow = to_ascii_u16(0x22),
    PCM_Complete = to_ascii_u16(0x24),
    San_X = to_ascii_u16(0x25),
    Kemco_Japan = to_ascii_u16(0x28),
    Seta = to_ascii_u16(0x29),
    Viacom = to_ascii_u16(0x30),
    nintendo = to_ascii_u16(0x31),
    Bandai = to_ascii_u16(0x32),
    ocean_acclaim = to_ascii_u16(0x33),
    konami = to_ascii_u16(0x34),
    Hector = to_ascii_u16(0x35),
    Taito = to_ascii_u16(0x37),
    Hudson = to_ascii_u16(0x38),
    Banpresto = to_ascii_u16(0x39),
    Ubisoft = to_ascii_u16(0x41),
    Atlus = to_ascii_u16(0x42),
    Malibu = to_ascii_u16(0x44),
    Angel = to_ascii_u16(0x46),
    Bullet_Proof = to_ascii_u16(0x47),
    Irem = to_ascii_u16(0x49),
    Absolute = to_ascii_u16(0x50),
    Acclaim = to_ascii_u16(0x51),
    Activision = to_ascii_u16(0x52),
    American_Sammy = to_ascii_u16(0x53),
    Konami = to_ascii_u16(0x54),
    Hi_Tech_Entertainment = to_ascii_u16(0x55),
    LJN = to_ascii_u16(0x56),
    Matchbox = to_ascii_u16(0x57),
    Mattel = to_ascii_u16(0x58),
    Milton_Bradley = to_ascii_u16(0x59),
    Titus = to_ascii_u16(0x60),
    Virgin = to_ascii_u16(0x61),
    LucasArts = to_ascii_u16(0x64),
    Ocean = to_ascii_u16(0x67),
    Electronic_Arts = to_ascii_u16(0x69),
    Infogrames = to_ascii_u16(0x70),
    Interplay = to_ascii_u16(0x71),
    Broderbund = to_ascii_u16(0x72),
    Sculptered_Soft = to_ascii_u16(0x73),
    SCI = to_ascii_u16(0x75),
    THQ = to_ascii_u16(0x78),
    Accolade = to_ascii_u16(0x79),
    Misawa = to_ascii_u16(0x80),
    Lozc = to_ascii_u16(0x83),
    Tokuma_Shoten_Intermedia = to_ascii_u16(0x86),
    Tsukada_Original = to_ascii_u16(0x87),
    Chun_Soft = to_ascii_u16(0x91),
    Video_System = to_ascii_u16(0x92),
    Ocean_Acclaim = to_ascii_u16(0x93),
    Varie = to_ascii_u16(0x95),
    Yonezawa = to_ascii_u16(0x96),
    Kaneko = to_ascii_u16(0x97),
    Pack_In_Soft = to_ascii_u16(0x99),
}

const fn to_ascii_u16(value: u8) -> u16 {
    (((value as u16 & 0xf0) << 4) + 0x3000) + ((value as u16 & 0x0f) + 0x30)
}

#[test]
fn test_ascii_u16_1() {
    assert_eq!(to_ascii_u16(0x13), 0x3133)
}
#[test]
fn test_ascii_u16_2() {
    assert_eq!(to_ascii_u16(0x13), u16::from_be_bytes(*b"13"))
}

#[repr(u8)]
#[derive(Debug, N)]
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
    Unknown = 0x23,
    POCKET_CAMERA = 0xFC,
    BANDAI_TAMA5 = 0xFD,
    HuC3 = 0xFE,
    HuC1_RAM_BATTERY = 0xFF,
}

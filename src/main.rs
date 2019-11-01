mod types;

use std::io::prelude::*;
use std::time::Duration;
use std::mem::transmute;

use serialport::prelude::*;
//use crc::{crc16};
use clap::{App, Arg, SubCommand};

use crate::types::StatusResponse;

fn main() {
    let matches = App::new("GB Cart Commander")
        .version("0.1")
        .author("Paul Sajna <sajattack@gmail.com>")
        .about("Command-line tool for ATmega8515-based Gameboy Cartridge Reader/Writers")
        .arg(Arg::with_name("device")
             .help("Sets the device to use for the serial port connect\nEx: COM4 or /dev/ttyUSB0"))
        .arg(Arg::with_name("baudrate")
            .short("b")
            .long("baudrate")
            .help("Sets the baudrate of the serial port connection")
            .default_value("187500")
            .possible_values(&["125000", "187500", "375000"]))
        .subcommand(
            SubCommand::with_name("status")
                .about("Prints info about the currently connected cartridge")
        ).get_matches();

    let mut settings: SerialPortSettings = Default::default();
    settings.baud_rate = u32::from_str_radix(matches.value_of("baudrate").unwrap(), 10).unwrap();
    settings.timeout = Duration::from_millis(500);
    let mut serialport = serialport::open_with_settings(
        matches.value_of("device").unwrap(),
        &settings
    ).unwrap();

    if let Some(_matches) = matches.subcommand_matches("status") {
        serialport.write(&STATUS_COMMAND).expect("failed to write bytes");
        let mut rx_buf = [0u8;72];
        serialport.read(&mut rx_buf).expect("failed to read bytes");
        let stat_rx = unsafe { transmute::<[u8;72], StatusResponse>(rx_buf) };
        println!("{}", stat_rx);
    }
}

static STATUS_COMMAND: [u8;72] = [
    0x55, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x36
];


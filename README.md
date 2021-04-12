# GB Cart Commander 0.1
Paul Sajna <sajattack@gmail.com>
Command-line tool for ATmega8515-based Gameboy Cartridge Reader/Writers

USAGE:
    gb-cart-commander [OPTIONS] <device> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --baudrate <baudrate>    Sets the baudrate of the serial port connection [default: 187500]  [possible values:
                                 125000, 187500, 375000]

ARGS:
    <device>    Sets the device to use for the serial port connect
                Ex: COM4 or /dev/ttyUSB0

SUBCOMMANDS:
    erase    Erase flash or ram on a flash cart
    help     Prints this message or the help of the given subcommand(s)
    info     Prints info about the currently connected cartridge
    read     Read flash or ram on a cart
    write    Write flash or ram on a flash cart

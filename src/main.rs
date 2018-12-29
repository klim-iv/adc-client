mod adc_command;
use self::adc_command::{AdcCommand, Type, Error};

fn main() {
    let CMDS = vec![
        "SUP",
        "DEF",
    ];

    let x = AdcCommand { cmd_type: Type::Udp, from: 0, to: 1 };
    let mut y: AdcCommand;

    y = x;

    println!("DC++ parser on Rust {}", Type::Udp.to_string());

    println!("{} -> {} {}", y.from, y.to, adc_command::Error::SlotsFull as u32);

    println!("{}", y.cmd_type.to_string());
    println!("{}", y.cmd_type as u32);
    y.to = 999;
    println!("y.to = {} {}", y.to, adc_command::Error::InvalidPid.to_string());
}

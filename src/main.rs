use std::fs::File;
use std::io::Write;

mod adc_command;
use self::adc_command::{AdcCommand, Type, Error};

fn main() -> std::io::Result<()> {
    let CMDS = vec![
        "SUP",
        "DEF",
    ];

    let x = AdcCommand::new(Type::Broadcast, 77, 56, "SUP".to_string());
    let mut y: AdcCommand;

    y = x;

    println!("DC++ parser on Rust {}", Type::Udp.to_string());

    println!("{} -> {} {}", y.from, y.to, adc_command::Error::SlotsFull as u32);

    println!("{}", y.cmd_type.to_string());
    println!("{}", y.cmd_type as u32);
    y.to = 999;
    println!("y.to = {} {}", y.to, adc_command::Error::InvalidPid.to_string());

    println!("y.cmd = {} ", y.cmd);
//    println!("y.ToInt = {:?} ", y.to_vec());

    let mut f1 = File::create("/tmp/ccc.111")?;
    f1.write(y.to_vec().as_slice())?;
    Ok(())
}

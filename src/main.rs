extern crate serial;
extern crate bufstream;

mod xdo;

use std::env;
use std::time::Duration;
use serial::prelude::*;
use std::io::prelude::*;
use bufstream::BufStream;
use xdo::XDo;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    for arg in env::args_os().skip(1) {
        println!("opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();

        interact(&mut port).unwrap();
    }
}

fn interact<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    port.configure(&SETTINGS)?;
    port.set_timeout(Duration::from_secs(1))?;
    let stream = BufStream::new(port);


    println!("connected ... ");
    //let buf: Vec<u8> = (0..255).collect();
    //stream.write_all(&buf)?;

    if let Ok(xdo) = XDo::new(None) {

        for l in stream.lines() {
            match l {
                Ok(res) => {
                    println!("{}", res);
                    if res == "c-" {
                        let _res = xdo.send_keysequence("XF86AudioLowerVolume", 0);
                    }
                    if res == "c+" {
                        let _res = xdo.send_keysequence("XF86AudioRaiseVolume", 0);
                    }
                    if res == "c=" {
                        let _res = xdo.send_keysequence("M", 0);
                    }
                    if res == "cc" {
                        let _res = xdo.send_keysequence("F", 0);
                    }
                    if res == "c<" {
                        let _res = xdo.send_keysequence("shift+P", 0);
                    }
                    if res == "c>" {
                        let _res = xdo.send_keysequence("shift+N", 0);
                    }
                    if res == "c#" {
                        let _res = xdo.send_keysequence("K", 0);
                    }
                    if res == "cp" {
                        let _res = xdo.send_keysequence("J", 0);
                    }
                    if res == "cn" {
                        let _res = xdo.send_keysequence("L", 0);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

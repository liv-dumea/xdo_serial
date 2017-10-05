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


fn when_vlc(xdo: &XDo, res: &str) {
    if res == "cc" {
        let _res = xdo.send_keysequence("f", 0);
    }
    if res == "cp" {
        let _res = xdo.send_keysequence("p", 0);
    }
    if res == "cn" {
        let _res = xdo.send_keysequence("n", 0);
    }

    if res == "c<" {
        let _res = xdo.send_keysequence("alt+Left", 0);
    }
    if res == "c>" {
        let _res = xdo.send_keysequence("alt+Right", 0);
    }
    if res == "c#" {
        let _res = xdo.send_keysequence("space", 0);
    }
    if res == "c-" {
        let _res = xdo.send_keysequence("XF86AudioLowerVolume", 0);
    }
    if res == "c+" {
        let _res = xdo.send_keysequence("XF86AudioRaiseVolume", 0);
    }
    if res == "c=" {
        let _res = xdo.send_keysequence("M", 0);
    }
    if res == "c0" {
        let _res = xdo.send_keysequence("super", 0);
    }
    if res == "ch" {
        let _res = xdo.send_keysequence("super+shift+Tab", 0);
    }
    if res == "ct" {
        let _res = xdo.send_keysequence("super+Tab", 0);
    }
    if res == "c1" {
        let _res = xdo.send_keysequence("Tab", 0);
    }
    if res == "c3" {
        let _res = xdo.send_keysequence("ctrl+l", 0);
    }
    if res == "c2" {
        let _res = xdo.send_keysequence("Up", 0);
    }
    if res == "c8" {
        let _res = xdo.send_keysequence("Down", 0);
    }
    if res == "c4" {
        let _res = xdo.send_keysequence("Left", 0);
    }
    if res == "c6" {
        let _res = xdo.send_keysequence("Right", 0);
    }
    if res == "c5" {
        let _res = xdo.send_keysequence("Return", 0);
    }
}
fn when_youtube(xdo: &XDo, res: &str) {
    if res == "cc" {
        let _res = xdo.send_keysequence("F", 0);
    }
    if res == "cp" {
        let _res = xdo.send_keysequence("J", 0);
    }
    if res == "cn" {
        let _res = xdo.send_keysequence("L", 0);
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
    if res == "c-" {
        let _res = xdo.send_keysequence("XF86AudioLowerVolume", 0);
    }
    if res == "c+" {
        let _res = xdo.send_keysequence("XF86AudioRaiseVolume", 0);
    }
    if res == "c=" {
        let _res = xdo.send_keysequence("M", 0);
    }
    if res == "c0" {
        let _res = xdo.send_keysequence("super", 0);
    }
    if res == "ch" {
        let _res = xdo.send_keysequence("super+shift+Tab", 0);
    }
    if res == "ct" {
        let _res = xdo.send_keysequence("super+Tab", 0);
    }
    if res == "c2" {
        let _res = xdo.send_keysequence("Up", 0);
    }
    if res == "c8" {
        let _res = xdo.send_keysequence("Down", 0);
    }
    if res == "c4" {
        let _res = xdo.send_keysequence("Left", 0);
    }
    if res == "c6" {
        let _res = xdo.send_keysequence("Right", 0);
    }
    if res == "c5" {
        let _res = xdo.send_keysequence("Return", 0);
    }
}

fn when_default(xdo: &XDo, res: &str) {
    if res == "cc" {
        let _res = xdo.send_keysequence("super", 0);
    }
    if res == "cp" {
        let _res = xdo.send_keysequence("super+shift+Tab", 0);
    }
    if res == "cn" {
        let _res = xdo.send_keysequence("super+Tab", 0);
    }

    if res == "c-" {
        let _res = xdo.send_keysequence("XF86AudioLowerVolume", 0);
    }
    if res == "c+" {
        let _res = xdo.send_keysequence("XF86AudioRaiseVolume", 0);
    }
    if res == "c=" {
        let _res = xdo.send_keysequence("XF86AudioMute", 0);
    }

    if res == "c<" {
        let _res = xdo.send_keysequence("ctrl+shift+Tab", 0);
    }
    if res == "c>" {
        let _res = xdo.send_keysequence("ctrl+Tab", 0);
    }
    if res == "c#" {
        let _res = xdo.send_keysequence("Return", 0);
    }
    if res == "c0" {
        let _res = xdo.send_keysequence("super", 0);
    }
    if res == "ch" {
        let _res = xdo.send_keysequence("super+shift+Tab", 0);
    }
    if res == "ct" {
        let _res = xdo.send_keysequence("super+Tab", 0);
    }
    if res == "c2" {
        let _res = xdo.send_keysequence("Up", 0);
    }
    if res == "c8" {
        let _res = xdo.send_keysequence("Down", 0);
    }
    if res == "c4" {
        let _res = xdo.send_keysequence("Left", 0);
    }
    if res == "c6" {
        let _res = xdo.send_keysequence("Right", 0);
    }
    if res == "c5" {
        let _res = xdo.send_keysequence("Return", 0);
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
                    if let Ok(window) = xdo.get_window_name() {
                        println!("{}", res);
                        println!("{}", window);
                        if window.contains("YouTube") {
                            when_youtube(&xdo, &res);
                        } else if window.contains("VLC") {
                            when_vlc(&xdo, &res);
                        } else {
                            when_default(&xdo, &res);
                        }
                    }
                }

                _ => {}
            }
        }
    }
    Ok(())
}

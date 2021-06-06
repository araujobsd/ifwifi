/*-
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * BSD 2-Clause License
 *
 * Copyright (c) 2021, Marcelo Araujo <araujobsdport@gmail.com>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use clap::{App, Arg};
use colored::*;
use std::env;
use std::io;
use std::process::exit;
use wifi_rs::prelude::*;
use wifi_rs::WiFi;
use wifiscanner;

#[derive(Debug)]
enum SignalMeasure {
    Maximum,
    Excellent,
    Good,
    Reliable,
    Weak,
    Unreliable,
    Bad,
}

fn scan_table_format(network_info: &wifiscanner::Wifi) {
    let signal_level =
        dBm_signal_measure(network_info.signal_level.parse::<f32>().unwrap_or_default());

    print!(
        "{} \t{:15} {:10} {:4}",
        network_info.mac,
        network_info.ssid.yellow().bold(),
        network_info.channel.white().bold(),
        network_info.signal_level
    );

    match signal_level {
        SignalMeasure::Maximum => {
            print!("\t{}", "Maximum".green().bold().blink())
        }
        SignalMeasure::Excellent => {
            print!("\t{}", "Excellent".green().bold().blink())
        }
        SignalMeasure::Good => {
            print!("\t{}", "Good".green().blink())
        }
        SignalMeasure::Reliable => {
            print!("\t{}", "Reliable".yellow().bold().blink())
        }
        SignalMeasure::Weak => {
            print!("\t{}", "Weak".yellow())
        }
        SignalMeasure::Unreliable => {
            print!("\t{}", "Unreliable".red())
        }
        SignalMeasure::Bad => {
            print!("\t{}", "Bad".red().bold())
        }
    };

    println!("{}", network_info.security);
}

fn dBm_signal_measure(signal: f32) -> SignalMeasure {
    if signal >= -30.00 {
        return SignalMeasure::Maximum;
    } else if signal < -30.00 && signal >= -50.00 {
        return SignalMeasure::Excellent;
    } else if signal < -50.00 && signal >= -60.00 {
        return SignalMeasure::Good;
    } else if signal < -60.00 && signal >= -67.00 {
        return SignalMeasure::Reliable;
    } else if signal < -67.00 && signal >= -70.00 {
        return SignalMeasure::Weak;
    } else if signal < -70.00 && signal >= -80.00 {
        return SignalMeasure::Unreliable;
    } else if signal < -80.00 {
        return SignalMeasure::Bad;
    }
    return SignalMeasure::Bad;
}

fn is_root() -> bool {
    match env::var("USER") {
        Err(e) => {
            println!("Something went very wrong: {:?}", e);
            false
        }
        Ok(name) => {
            if name != "root" {
                println!("{}", "You must be root!".red().bold().blink());
                false
            } else {
                true
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    if is_root() == false {
        exit(2);
    }

    let margs = env::args().nth(1).unwrap_or_default();
    match margs.as_str() {
        "scan" => {
            let networks = wifiscanner::scan().expect("Cannot scan network");
            for network in networks {
                scan_table_format(&network);
            }
        }
        "help" => {
            println!("USAGE: \n\tifwifi help");
            println!("\tifwifi scan");
            println!("\tifwifi --interface <interface> --password <password> --ssid <ssid>");
        }
        _ => {
            let matches = App::new("Wi-Fi")
                .version("0.0.1")
                .author("Tochukwu Nkemdilim")
                .about("Connect to a Wi-Fi network 🎉")
                .arg(
                    Arg::with_name("ssid")
                        .short("s")
                        .long("ssid")
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .help("SSID of wireless network."),
                )
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .multiple(false)
                        .required(true)
                        .takes_value(true)
                        .help("Password of the wireless network."),
                )
                .arg(
                    Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .multiple(false)
                        .default_value("wlan0")
                        .takes_value(true)
                        .help("Wireless interface to connect through."),
                )
                .get_matches();

            // Get Password
            let password = matches.value_of("password").unwrap();

            // Get SSID
            let ssid = matches.value_of("ssid").unwrap();

            // Get Wireless Interface
            let interface = matches.value_of("interface").unwrap();

            let config = Some(Config {
                interface: Some(interface),
            });

            // let wifi = WiFi::new(ssid)?;
            let mut wifi = WiFi::new(config);
            println!("Connection Status: {:?}", wifi.connect(ssid, password));
        }
    }
    Ok(())
}

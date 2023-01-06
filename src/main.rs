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

use clap::{Parser, Subcommand};
use colored::*;
use std::env;
use std::process::exit;
use std::process::Command;
use wifi_rs::prelude::*;
use wifi_rs::WiFi;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Connect to an Access Point
    Connect {
        /// SSID of wireless network
        #[arg(short, long)]
        ssid: String,

        /// Password of the wireless network
        #[arg(short, long)]
        password: String,

        /// Wireless interface to connect through
        #[arg(short, long, default_value = "wlan0")]
        interface: String,
    },
    /// Scan wireless network
    Scan {},
}

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

    if is_connected(&network_info.ssid) {
        print!(
            "{} {} \t{:15} {:10} {:4}",
            "*".green().bold().blink(),
            network_info.mac,
            network_info.ssid.yellow().bold(),
            network_info.channel.white().bold(),
            network_info.signal_level
        );
    } else {
        print!(
            "  {} \t{:15} {:10} {:4}",
            network_info.mac,
            network_info.ssid.yellow().bold(),
            network_info.channel.white().bold(),
            network_info.signal_level
        );
    }

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

fn is_connected(ssid: &str) -> bool {
    let nmcli = Command::new("nmcli")
        .args(&["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .expect("failed to run nmcli");

    let ssid_comp: String = "yes:".to_owned() + ssid;
    let output = String::from_utf8_lossy(&nmcli.stdout);
    let output = output.split('\n').take(1).collect::<Vec<_>>()[0];

    output.to_string().trim().starts_with("yes") && ssid_comp.eq(&output.to_string().trim())
}

#[allow(non_snake_case)]
fn dBm_signal_measure(signal: f32) -> SignalMeasure {
    if signal >= -30.00 {
        SignalMeasure::Maximum
    } else if signal >= -50.00 {
        SignalMeasure::Excellent
    } else if signal >= -60.00 {
        SignalMeasure::Good
    } else if signal >= -67.00 {
        SignalMeasure::Reliable
    } else if signal >= -70.00 {
        SignalMeasure::Weak
    } else if signal >= -80.00 {
        SignalMeasure::Unreliable
    } else {
        SignalMeasure::Bad
    }
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

fn scan() -> Result<(), String> {
    let networks = wifiscanner::scan().expect("Cannot scan network");
    for network in networks {
        scan_table_format(&network);
    }

    Ok(())
}

fn connect(ssid: &str, password: &str, interface: &str) -> Result<(), String> {
    let config = Some(Config {
        interface: Some(interface),
    });

    let mut wifi = WiFi::new(config);
    println!("Connection Status: {:?}", wifi.connect(ssid, password));

    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Scan {}) => {
            if !is_root() {
                exit(2);
            }
            scan()
        }
        Some(Commands::Connect {
            ssid,
            password,
            interface,
        }) => {
            if !is_root() {
                exit(2);
            }
            connect(ssid, password, interface)
        }
        None => Ok(()),
    }
}

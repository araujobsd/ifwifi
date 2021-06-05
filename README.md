## ifwifi - A simple wrapper over nmcli using [wifiscanner](https://crates.io/crates/wifiscanner) made in [rust](https://www.rust-lang.org/).
I felt bothered because I never remember the long and tedious command line to setup my wifi interface. So, I wanted to develop something using rust to simplify the usage of nmcli, and I met the <b>wifiscanner</b> project that gave me almost everything I wanted to create this tool.

## Command line:
* <b>ifwifi help</b> - Help menu with the few options provided by this tool
* <b>ifwifi scan</b> - It will scan the wifi available in your area
* <b>ifwifi --interface IFACE --password PASSWORD --ssid SSID</b> - Set the wifi

[logo]: https://raw.githubusercontent.com/araujobsd/ifwifi/main/gif/terminal.gif "Terminal example"
<b>Termina example:</b>
![alt text][logo]

## How to build:
* <b>cargo build --release</b>

## License:
<b>BSD 2-Clause "Simplified" License</b>


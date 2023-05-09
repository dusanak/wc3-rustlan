# WC3 RustLAN
This a rewrite of Lancraft into Rust. At this point this tool provides only basic functionality and no GUI.
## Build
1. Navigate into project root directory.
2. Run `cargo build --release`
3. The artifact can be found in ./target/release/wc3-rustlan(.exe)
## Usage
`./wc3-rustlan <HOST_IP_ADDRESS:HOST_PORT>`
* Host must have a public IP address and port forwarding done. Warcraft 3 by default hosts its servers at port 6112.
* If you leave the game lobby after connecting, WC3 RustLAN must be restarted. 

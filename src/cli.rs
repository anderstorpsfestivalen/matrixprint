use clap::Parser;
#[derive(Parser, Debug)]
#[clap(setting = clap::AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(short, long, default_value = "/dev/usb/lp0")]
    pub printer_path: String,

    #[clap(
        short,
        long,
        default_value = "wss://mch.anderstorpsfestivalen.se/kernel/pipe"
    )]
    pub websocket: String,

    #[clap(
        long,
        default_value = "https://mch.anderstorpsfestivalen.se/kernel/print"
    )]
    pub stats_url: String,

    #[clap(
        short,
        long,
        default_value = ""
    )]
    pub stats_key: String,

    #[clap(short, long, default_value = "26")]
    pub relaypin: u8,
}

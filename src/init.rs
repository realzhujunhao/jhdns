use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'n', long = "name", help = "domain name")]
    pub domain_name: String,

    #[arg(short = 's', long = "server", help = "dns server ip", default_value_t = String::from("8.8.8.8"))]
    pub dns_server_ip: String,
}

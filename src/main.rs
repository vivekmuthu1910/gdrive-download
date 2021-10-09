use gdrive_download::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    println!("{:?}", config);
}

use r_find_file::Config;
use structopt::StructOpt;
fn main() {
    let args: Config = Config::from_args();

    match args.read_file_read_to_string() {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    }
}

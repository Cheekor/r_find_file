use r_find_file::Config;
use std::env;
use structopt::StructOpt;
fn main() {
    let args: Config = Config::from_args();

    let is_up_case = env::var("IS_UP_CASE");
    println!("is_up_case:{}", is_up_case.is_ok());
    println!("is_up_case:{:?}", is_up_case);
    match args.read_file_read_to_string() {
        Ok(b) => println!("存在字段：{}", b), //println!("ok"),
        Err(e) => println!("{}", e),
    }
}

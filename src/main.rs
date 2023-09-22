use std::env;
use std::fs;
fn main() {
    // println!("Hello, world!");
    // let a: Vec<_> = env::args_os().collect();
    // let p = &a[1..];
    // // let my_args = &a[1..];
    // println!("{:?}", p);
    // a.str
    // for out in a.iter() {
    //     let out_str = out.to_str().unwrap();
    //     println!("{}", out_str);
    //     if out_str.contains(".") {
    //         // read_file(out_str);
    //     }
    // }

    // let std::env::args_os().nth(1);
}

struct Config {
    name: Vec<String>,
    path: Vec<String>,
}

impl Config {
    fn read_file(path: &str) {
        match fs::read_to_string(path) {
            Ok(file) => {
                println!("{}", file)
            }
            Err(e) => println!("{}", e),
        }
    }

    // fn new(config_vec: Vec<_>) -> Config {
    //     Config {
    //         name: config_vec[0].to_vec(),
    //         path: config_vec[1].to_vec(),
    //     }
    // }
}

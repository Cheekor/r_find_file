use std::{
    fs::{self, File},
    io::Error,
    io::Read,
};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Config {
    //// 获取需要查的数据
    #[structopt(short = "n", long = "name")]
    query: Vec<String>,

    //// 自动转换路径
    #[structopt(short = "p", long = "path")]
    path: Vec<std::path::PathBuf>,
}

impl Config {
    pub fn read_file_read_to_string(&self) -> Result<(), Error> {
        for p in self.path.iter() {
            let file_str = fs::read_to_string(p.as_path())?.to_string();
            println!("{}", file_str);
        }
        for query_str in self.query.iter() {
            println!("{}", query_str);
        }

        Ok(())
    }

    pub fn new() -> Config {
        Config::from_args()
    }

    pub fn read_file_std_env_args(&self) -> Result<String, Error> {
        let mut s = String::from("");
        for path_str in self.path.iter() {
            let mut file = File::open(path_str)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            s.push_str(&buffer);
        }
        Ok(s)
    }
}

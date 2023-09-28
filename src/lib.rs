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
    pub fn read_file_read_to_string(&self) -> Result<bool, Error> {
        let mut b = false;
        for p in self.path.iter() {
            let file_str = fs::read_to_string(p.as_path())?.to_string();

            for query_str in self.query.iter() {
                println!("{}", query_str);
                b = Config::search(query_str, &file_str);
            }
        }

        Ok(b)
    }

    fn search(query: &str, file_str: &str) -> bool {
        let b = false;
        for str in file_str.lines() {
            if str.contains(query) {
                return true;
            }
            continue;
        }
        b
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

mod test {
    #[test]
    fn test_search() {
        let query_str = "test";
        let file_str = "test\ntest\ntest";
        let b = super::Config::search(query_str, file_str);
        println!("{}", b);
        assert_eq!(b, true);
    }
}

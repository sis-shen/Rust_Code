use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err|{
        // println!("出错的解析声明:{err}");
        process::exit(-1);
    });

    // println!("search for {}",config.query);
    // println!("In file {}",config.file_path);

    if let Err(e) = minigrep::run(config){
        // println!("应用出错:{e}");
        process::exit(1);
    }
}


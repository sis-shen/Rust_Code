use std::{fs, result::Result};
use std::error::Error;
use std::env;


impl Config {
    // pub fn build(args:&[String])->Result<Config,&'static str>{
    //     if args.len() < 3{
    //         return Err("参数数量不足");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     Ok(Config { query, file_path ,ignore_case})
    // }

    pub fn build(mut args:impl Iterator<Item = String>,)->Result<Config,&'static str>{
        args.next();//跳过第一个参数

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("没有获取到目标子串"),
        };

        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>return Err("没有获取到文件路径"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path ,ignore_case})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }
    Ok(())
}


pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:
safe,fast,productive.
Pick three.";

        assert_eq!(vec!["safe,fast,productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
safe,fast,productive.
Pick three.";
        assert_eq!(vec!["safe,fast,productive."],search_case_insensitive(query,contents));
    }
}

pub fn search<'a>(query:&'a str,contents:&'a str)->Vec<&'a str>{
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(
    query:&'a str,
    contents:&'a str
)->Vec<&'a str>{
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query)
    //     {
    //         results.push(line);
    //     }
    // }

    // results

    let query = &query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(query)).collect()
}

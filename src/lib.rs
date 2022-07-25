use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub path: String,
}

impl Config {
    // pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args. Usage: demo key ./abc.md");
        }

        // args.next();
        // let key = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("not key str")
        // };
        // let path = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("not path str")
        // };

        let key: String = args[1].clone();
        let path: String = args[2].clone(); // 解决所有权问题
        Ok(Config { key, path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);
    let contents: String = fs::read_to_string(config.path)?; // ? 异常返回 Err
    for line in grep(&config.key, &contents) {
        println!("{}", line)
    }
    Ok(())
}

pub fn grep<'a>(key: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(key) {
            results.push(line);
        }
    }

    results

    // 以上代码可以使用如下迭代器替换
    // contents.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep() {
        let key = "abc";
        let contents = "\
abcdefghui
xyz";
        assert_eq!(vec!["abcdefghui"], grep(key, contents));
    }
}

use std::env;
use std::process;
use x_demo::Config;

fn main() {
    // env::args() 是一个迭代器
    // env::args_os() -> OsString
    let args: Vec<String> = env::args().collect();
    // let c = Config::new(&args);
    // match c {
    //     Ok(config) => println!("{:?}, args[0]: {:?}", config, config.name),
    //     Err(msg) => println!("{}", msg)
    // }
    // let c = Config::new(env::args()).unwrap_or_else(|err| {
    let c = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Occur Error: {}", err);
        process::exit(1); // 使 println 的输出无 debug 信息
    });

    if let Err(e) = x_demo::run(c) {
        eprintln!("Run Error: {}", e);
        process::exit(1);
    }
}

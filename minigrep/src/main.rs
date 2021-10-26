use minigrep::Config;
use std::{env, process};

/*
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
*/

// 将错误信息输出到标准错误而不是标准输出
// cargo run THe poem.txt > output.txt 标准输出到文件
// cargo run > output.txt 错误到终端
fn main() {
    let args: Vec<String> = env::args().collect();

    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err); // 标准输出
        eprintln!("Problem parsing arguments: {}", err); // 标准错误
        process::exit(1);
    });

    // 处理main中run返回的错误
    if let Err(e) = minigrep::run(config) {
        // eprintln!("Application error: {}", e); // 标准输出
        eprintln!("Application error: {}", e); // 标准错误
        process::exit(1);
    }
}

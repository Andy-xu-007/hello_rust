use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = args_parse(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments : {err}");
        process::exit(-1);
    });
    // println!("query is {} ", config.query);

    // dbg!(args);

    // vector 的第一个值是”target∕debug∕minigrep”
    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    // let content = fs::read_to_string(file_path)
    // .expect("should have been able able to read!");

    // 只关心错误检测，并不需要unwrap_or_else 来返回未封装的值，因为它只会是()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err is {e}");
        process::exit(-1);
    }

    // println!("With text: \n {content}");
}
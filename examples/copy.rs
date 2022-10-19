use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] == "--help" {
        println!("{ } old-file new-file\n", args[0]);
        return;
    }

    // let from = &args[1];

    let result = fs::copy(&args[1], &args[2]);

    match result {
        Ok(res_num) => println!("copy success! {}", res_num),
        Err(e) => println!("copy error! {:?}", e),
    }

    println!("{:?}", args);
}

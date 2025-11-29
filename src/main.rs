use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let input = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", input.query);

    // read file
    println!("In file {}", input.file_path);

    if let Err(e) = run(input) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }    

}


fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else{
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

mod tests {
    
}

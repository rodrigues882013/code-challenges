use std::{env, fs, io};
use std::io::Read;

struct Config {
    option: String,
    file_path: String,
}

fn main() {
    let config = parse_config().unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        eprintln!("Usage: ccwc [-c|-l|-m|-w] [file_path]");
        std::process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    let contents = read_file(&config.file_path);

    match config.option.as_str() {
        "-c" => println!("{} {}", contents.as_bytes().len(), config.file_path),
        "-l" => println!("{} {}", contents.lines().count(), config.file_path),
        "-m" => println!("{} {}", contents.chars().count(), config.file_path),
        "-w" => println!("{} {}", contents.split_whitespace().count(), config.file_path),
        _ => {
            println!(
                "{} {} {} {}",
                contents.as_bytes().len(),
                contents.lines().count(),
                contents.chars().count(),
                config.file_path
            );
        }
    }
}

fn parse_config() -> Result<Config, &'static str> {
    let mut args = env::args().skip(1); // skip program name

    let option = args.next().ok_or("Missing option flag")?;
    let file_path = args.next().unwrap_or_default(); // empty string for stdin

    Ok(Config { option, file_path })
}

fn read_file(file_path: &str) -> String {
    if file_path.trim().is_empty() {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        fs::read_to_string(file_path).expect("Failed to read from file")
    }
}

use std::error::Error;
use std::process;

enum Option {
    Encode,
    Decode,
}

pub struct Config {
    option: Option,
    text: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let option = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an option"),
        };

        let option = match option.as_str() {
            "-e" => Option::Encode,
            "-d" => Option::Decode,
            _ => return Err("Not a valid option"),
        };

        let text = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get text"),
        };

        Ok(Config { option, text })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let resault = match config.option {
        Option::Encode => encode(&config.text),
        Option::Decode => decode(&config.text),
    };

    let resault = resault.unwrap_or_else(|e| {
        let error: String = e.unsupported_characters.into_iter().collect();
        eprintln!("Transtalion Error: unsupported chatacters: {}", error);
        process::exit(1)
    });

    println!("{}", resault);

    Ok(())
}

fn decode(text: &String) -> Result<String, morse::TranslationError> {
    morse::decode::decode(text)
}

fn encode(text: &String) -> Result<String, morse::TranslationError> {
    morse::encode::encode(text)
}

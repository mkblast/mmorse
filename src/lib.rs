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

        return Ok(Config { option, text });
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let result = match config.option {
        Option::Encode => encode(&config.text),
        Option::Decode => decode(&config.text),
    };

    match result {
        Ok(res) => {
            println!("{}", res);
            return Ok(());
        }

        Err(err) => {
            let error: String = err.unsupported_characters.into_iter().collect();
            return Err(format!(
                "Translation Error: unsupported characters: {}",
                error
            ));
        }
    }
}

fn decode(text: &String) -> Result<String, morse::TranslationError> {
    return morse::decode::decode(text);
}

fn encode(text: &String) -> Result<String, morse::TranslationError> {
    return morse::encode::encode(text);
}

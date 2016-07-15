use std::error::Error;
use yaml_rust::yaml::YamlLoader;

pub struct Config {
    source_dir: String,
}

impl Config {
    pub fn new(config_string: String) -> Self {
        let yaml = match YamlLoader::load_from_str(&config_string) {
            Ok(yaml) => yaml,
            Err(what) => panic!("{}", Error::description(&what))
        };

        let yaml = yaml.get(0).unwrap();

        let mut config = Config {
            source_dir: String::new()
        };

        if let Some(source) = yaml["source"].as_str() {
            config.source_dir = String::from(source);
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        String::from("source: pages\n\
            output: _site")
    }

    #[test]
    fn it_parses_input_directory_option_in_config() {
        let config_string = setup();

        let config = Config::new(config_string);

        assert_eq!("pages", config.source_dir);
    }
}

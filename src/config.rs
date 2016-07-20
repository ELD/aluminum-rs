use std::error::Error;
use yaml_rust::yaml::YamlLoader;

pub struct Config {
    source_dir: String,
    output_dir: String,
}

impl Config {
    pub fn from_string(config_string: String) -> Self {
        let yaml = match YamlLoader::load_from_str(&config_string) {
            Ok(yaml) => yaml,
            Err(what) => panic!("Config file couldn't be read: {}", Error::description(&what))
        };

        let mut config = Self::default();
        if let Some(yaml) = yaml.get(0) {
            if let Some(source) = yaml["source"].as_str() {
                config.source_dir = source.to_string();
            }

            if let Some(output) = yaml["output"].as_str() {
                config.output_dir = output.to_string();
            }
        }

        config
    }

    pub fn get_output_dir(&self) -> &str {
        &self.output_dir
    }

    pub fn get_source_dir(&self) -> &str {
        &self.source_dir
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            source_dir: "./pages".to_string(),
            output_dir: "./_site".to_string(),
        }
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

        let config = Config::from_string(config_string);

        assert_eq!("pages", config.source_dir);
    }

    #[test]
    fn it_parses_output_directory_option_in_config() {
        let config_string = setup();

        let config = Config::from_string(config_string);

        assert_eq!("_site", config.output_dir);
    }
}

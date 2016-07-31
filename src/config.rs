use std::error::Error;
use yaml_rust::yaml::YamlLoader;

pub struct Config {
    pub source_dir: String,
    pub output_dir: String,
    pub port: String,
    pub markdown_options: Vec<String>
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

            if let Some(config_port) = yaml["port"].as_str() {
                config.port = config_port.to_string();
            }

            if let Some(markdown_options) = yaml["markdown_options"].as_vec() {
                config.markdown_options = markdown_options.iter()
                    .filter_map(|option| option.as_str().map(|option| option.to_owned()))
                    .collect();
            }
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            source_dir: "pages".to_string(),
            output_dir: "_site".to_string(),
            port: "4000".to_string(),
            markdown_options: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_setup() -> String {
        "source: pages\n\
         output: _site\n\
         port: 4000\n\
         markdown_options:\n\
           - tables\n\
           - footnotes".to_string()
    }

    fn bad_setup() -> String {
        "source: pages\n\
         \t\toutput: _site".to_string()
    }

    #[test]
    fn it_creates_sensible_defaults() {
        let config = Config::default();

        assert_eq!("pages", config.source_dir);
        assert_eq!("_site", config.output_dir);
        assert_eq!("4000", config.port);
    }

    #[test]
    fn it_parses_input_directory_option_in_config() {
        let config_string = good_setup();

        let config = Config::from_string(config_string);

        assert_eq!("pages", config.source_dir);
    }

    #[test]
    fn it_parses_output_directory_option_in_config() {
        let config_string = good_setup();

        let config = Config::from_string(config_string);

        assert_eq!("_site", config.output_dir);
    }

    #[test]
    fn it_parses_port_number_in_config() {
        let config_string = good_setup();

        let config = Config::from_string(config_string);

        assert_eq!("4000", config.port);
    }

    #[test]
    fn it_parses_markdown_options() {
        let config_string = good_setup();
        let expected_options = vec!["tables", "footnotes"];

        let config = Config::from_string(config_string);

        assert_eq!(expected_options, config.markdown_options);
    }

    #[test]
    #[should_panic]
    fn it_panics_on_poorly_formed_file() {
        let config_string = bad_setup();

        Config::from_string(config_string);
    }
}

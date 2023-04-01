use regex::Regex;
use serde::{de::Visitor, Deserialize};
use std::env;

#[derive(Debug, PartialEq, Clone)]
pub enum MaybeEnv {
    Env(String),
    Value(String),
}

impl MaybeEnv {
    pub fn get(&self) -> Option<String> {
        match self {
            MaybeEnv::Env(key) => env::var(&key).ok(),
            MaybeEnv::Value(value) => Some(value.clone()),
        }
    }
}

impl<'de> Deserialize<'de> for MaybeEnv {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(MaybeEnvVisitor)
    }
}

fn parse_maybe_env(v: &str) -> MaybeEnv {
    let regex_env = Regex::new(r"^\$env\((\w*)\)$").unwrap();

    if let Some(captures) = regex_env.captures(v) {
        if let Some(env_capture) = captures.get(1) {
            let str = env_capture.as_str().to_string();
            return MaybeEnv::Env(str);
        };
    };

    MaybeEnv::Value(v.to_string())
}

struct MaybeEnvVisitor;

impl<'de> Visitor<'de> for MaybeEnvVisitor {
    type Value = MaybeEnv;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with a value or $env() with env key in it")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(parse_maybe_env(v))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maybe_env_parser() {
        let env_string = "$env(ABOBUS)";
        let value_string = "some-value";

        let parsed_env = parse_maybe_env(env_string);
        let parsed_value = parse_maybe_env(value_string);

        let expected_parsed_env = MaybeEnv::Env("ABOBUS".to_owned());
        let expected_parsed_value = MaybeEnv::Value("some-value".to_owned());

        assert_eq!(parsed_env, expected_parsed_env);
        assert_eq!(parsed_value, expected_parsed_value);
    }

    #[test]
    fn test_maybe_env_deserializer() {
        let env_string = "$env(ABOBUS)";
        let value_string = "some-value";

        let deserialized_env: MaybeEnv =
            serde_json::from_str(&format!("\"{}\"", env_string)).unwrap();
        let deserialized_value: MaybeEnv =
            serde_json::from_str(&format!("\"{}\"", value_string)).unwrap();

        let expected_deserialized_env = MaybeEnv::Env("ABOBUS".to_owned());
        let expected_deserialized_value = MaybeEnv::Value("some-value".to_owned());

        assert_eq!(deserialized_env, expected_deserialized_env);
        assert_eq!(deserialized_value, expected_deserialized_value);
    }
}

use std::{collections::HashMap, fs, path::PathBuf};

use dotenv_parser::parse_dotenv;

pub fn parse_env_file(path: PathBuf) -> Option<HashMap<String, String>> {
    let env_content = fs::read_to_string(path).ok()?;

    parse_dotenv(&env_content)
        .map(|btree_map| HashMap::from_iter(btree_map.into_iter()))
        .ok()
}

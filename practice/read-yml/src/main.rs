extern crate serde_yaml;
extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct App {
    app_name: String,
    version: String,
    user_infos: Vec<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    name: String,
    param: String,
    token: String,
}

fn main() {
    let yaml_str = include_str!("../app.yml");
    let user: App = serde_yaml::from_str(yaml_str).expect("app.yaml read failed!");
    println!("{:#?}", user);
}
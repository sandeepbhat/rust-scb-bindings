use reqwest::blocking;
use serde_json::{Result, Value};

pub struct SCB {
    base: String,
    url: String
}

impl SCB {
    pub fn new(language: &str) -> SCB {
        SCB {
            base: format!("https://api.scb.se/OV0104/v1/doris/{}/ssd/", language),
            url: format!("https://api.scb.se/OV0104/v1/doris/{}/ssd/", language)
        }
    }

    pub fn api_base(&self) -> &String {
        &self.base
    }

    pub fn info(&self) -> Result<&serde_json::Value, reqwest::Error> {
        let response = blocking::get(&self.url)?.text()?.ex;
        serde_json::json!(response)
    }
}

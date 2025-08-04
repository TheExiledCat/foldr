use std::io::{Cursor, Read};

use reqwest::Url;

use crate::commands::command::{Iteration, Result, error};
use crate::config::Config;
use crate::globals::FOLDR_MANIFEST_FILE;
use crate::templates::Template;
pub struct NetworkUtil;

impl NetworkUtil {
    pub fn fetch_template(
        config: &Config,
        endpoint: String,
        name: String,
        iteration: Iteration,
    ) -> Result<Template> {
        let http_endpoint;
        if let Ok(url) = Url::parse(&endpoint) {
            if url.scheme() == "http" || url.scheme() == "https" {
                http_endpoint = url;
            } else {
                return Err(error("Endpoint passed is not an http(s) endpoint"));
            }
        } else {
            return Err(error("Invalid endpoint passed"));
        }

        if config.require_https && http_endpoint.scheme() != "https" {
            return Err(error("Non https endpoints not allowed by config"));
        }
        println!("Fetching Template From {}", http_endpoint.to_string());
        let mut response = reqwest::blocking::get(http_endpoint)
            .map_err(|_| error("Network error while fetching template over http"))?;

        if !response.status().is_success() {
            return Err(error(&format!(
                "Failed statuscode recieved from endpoint: {}",
                response.status().as_str()
            )));
        }
        let mut buffer = Vec::new();
        response
            .read_to_end(&mut buffer)
            .map_err(|_| error("Error copying fetched template contents"))?;
        let mut cursor = Cursor::new(&mut buffer);
        let template = Template::store(
            &config,
            name,
            iteration,
            &mut cursor,
            vec![FOLDR_MANIFEST_FILE.into()],
        )?;
        return Ok(template);
    }
}

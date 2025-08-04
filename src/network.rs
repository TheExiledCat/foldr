use std::fs::{self, File};
use std::io::{self, Cursor, Read, copy};
use std::time::Instant;

use itertools::Itertools;
use reqwest::Url;
use sha2::{Digest, Sha256};

use crate::commands::command::{Result, error};
use crate::config::Config;
use crate::globals::FOLDR_MANIFEST_FILE;
use crate::templates::Template;
pub struct NetworkUtil;

impl NetworkUtil {
    pub fn fetch_template(config: &Config, endpoint: String, name: String) -> Result<Template> {
        let existing = Template::get_existing(&config)?;
        if existing
            .iter()
            .map(|t| t.info.name.clone())
            .contains(name.as_str())
        {
            return Err(error("Template name already in use"));
        }
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

        let mut response = reqwest::blocking::get(http_endpoint)
            .map_err(|_| error("Network error while fetching template over http"))?;

        if !response.status().is_success() {
            return Err(error(&format!(
                "Failed statuscode recieved from endpoint: {}",
                response.status().as_str()
            )));
        }
        let mut buffer = Vec::new();
        response.read_to_end(&mut buffer);
        let mut cursor = Cursor::new(&mut buffer);
        let template = Template::store(&config, &mut cursor, vec![FOLDR_MANIFEST_FILE.into()])?;
        return Ok(template);
    }
}

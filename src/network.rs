use std::io::{self, Cursor, Read};

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
        if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
            return Err(error("Endpoint passed is not an http(s) endpoint"));
        }

        if config.require_https && !endpoint.starts_with("https") {
            return Err(error("Non https endpoints not allowed by config"));
        }
        println!("Fetching Template From {}", endpoint.to_string());
        let mut response = ureq::get(endpoint).call().map_err(|e| {
            error(&format!(
                "Network error while fetching template over http: {}",
                e.to_string()
            ))
        })?;

        if !response.status().is_success() {
            return Err(error(&format!(
                "Failed statuscode recieved from endpoint: {}",
                response.status().as_str()
            )));
        }
        let mut buffer = response
            .body_mut()
            .read_to_vec()
            .map_err(|_| error("Error while reading template http stream"))?;
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

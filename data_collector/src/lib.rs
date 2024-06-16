use std::{process::Command, thread, time};

use rand::Rng;
use ureq::Error;
use url::{ParseError, Url};

const PAGE_SIZE: u64 = 1000;

pub struct Client {
    auth_cmd_path: String,
    connect_sid: String,
    debug: bool,
}

pub fn new_client(debug: bool, auth_cmd_path: String) -> Client {
    Client {
        // TODO: Just for test purposes - already authenticated connect token
        connect_sid: "s%3AH519MDgMrFtbrlz4ZXr28kw481eeDRXk.tysjm69u%2BUkCxcDrUjIt56H0zItzk%2FN%2BZD%2FhRi3bZRc".to_string(),
        // connect_sid: String::new(),
        debug,
        auth_cmd_path,
    }
}

pub enum ClimbingCategory {
    SportClimbing,
    Bouldering,
}

impl ClimbingCategory {
    fn url_param(&self) -> &str {
        match self {
            &Self::SportClimbing => "sportclimbing",
            &Self::Bouldering => "bouldering",
        }
    }
}

impl Client {
    pub fn user_ascents(
        &mut self,
        user: &str,
        climbing_category: &ClimbingCategory,
        break_on_not_authenticated: bool,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        // Status codes I've figured out so far:
        // 3xx - can have to do with using HTTP not HTTPs
        // 401 is no connect.sid cookie header or wrong cookie
        // 403 is e.g bad User-Agent header or some other headers (turns out Accept-Encoding is also required).

        let mut rng = rand::thread_rng();
        let mut res = vec![];

        if self.connect_sid.is_empty() {
            self.authenticate()?;
        }

        for page_idx in 0..10 {
            // fetch max 10 * PAGE_SIZE ascents
            let req_url = self.user_ascents_url(user, climbing_category, &page_idx)?;
            if self.debug {
                println!("GET /ascents request URL: {}", req_url);
            }

            let raw_json_body = match self.user_ascents_req(req_url.as_str()).call() {
                Ok(resp) => resp.into_json::<serde_json::Value>()?,
                Err(Error::Status(code, _)) => match code {
                    401 => {
                        if break_on_not_authenticated {
                            if self.debug {
                                println!("GET /ascent/{} 401 - break_on_not_authenticated", user);
                            }
                            return Err(format!("GET /ascent/{} break_on_not_authenticated", user))?;
                        }

                        if self.debug {
                            println!("GET /ascent/{} 401 - reauthenticating", user);
                        }

                        self.authenticate()?;
                        // repeat the whole fetch, but authenticated this time, break if we get 401 again
                        return self.user_ascents(user, climbing_category, true);
                    },
                    _ => {
                        if self.debug {
                            println!("GET /ascent/{} unhandled HTTP error code: {}", user, code);
                        }
                        return Err(format!("GET /ascent/{} unhandled HTTP error code: {}", user, code))?;
                    }
                },
                Err(e) => return Err(e)?,
            };

            let body_obj = match raw_json_body.as_object() {
                Some(b) => b,
                None => return Err(format!("GET /ascent/{} no body in response", user))?,
            };

            match body_obj.get("ascents") {
                Some(ascents) => res.extend(ascents.as_array().unwrap().to_owned()),
                None => (),
            }

            match body_obj.get("totalItems") {
                Some(val) => {
                    if val.as_number().unwrap().as_u64().unwrap() < PAGE_SIZE {
                        break;
                    }
                }
                None => return Err(format!("GET /ascent/{} - totalItems not found in response body", user))?,
            }

            // sleep random 100-500ms between fetching pages as a bot detection caution.
            let sleep_time_millis = time::Duration::from_millis(rng.gen_range(100..500));
            thread::sleep(sleep_time_millis);
        }

        Ok(res)
    }

    pub fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cmdres = Command::new("node")
            .arg(self.auth_cmd_path.as_str())
            .output()?;
        let cmd_out = String::from_utf8(cmdres.stdout)?;
        if !cmdres.status.success() {
            return Err(format!("sideexporter fail: {}", cmd_out))?;
        }
        self.connect_sid = cmd_out.trim().to_string();
        if self.debug {
            println!("authenticate: Got connect_sid: {:?}", self.connect_sid)
        }
        Ok(())
    }

    fn user_ascents_url(
        &self,
        user: &str,
        climbing_category: &ClimbingCategory,
        page_index: &u64,
    ) -> Result<Url, ParseError> {
        let mut u = Url::parse(&format!(
            "https://www.8a.nu/unificationAPI/ascent/v1/web/users/{}/ascents",
            user
        ))?;

        u.query_pairs_mut()
            .append_pair("sortField", "date_desc")
            .append_pair("includeProjects", "false")
            .append_pair("showRepeats", "false")
            .append_pair("showDuplicates", "false")
            .append_pair("category", climbing_category.url_param())
            .append_pair("pageSize", &PAGE_SIZE.to_string())
            .append_pair("pageIndex", &page_index.to_string());

        return Ok(u);
    }

    fn user_ascents_req(&self, url: &str) -> ureq::Request {
        ureq::get(url)
            .set(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
            )
            .set(
                "Cookie",
                format!("connect.sid={};", self.connect_sid).as_str(),
            )
            .set("Host", "www.8a.nu")
            .set("Accept", "*/*")
            .set("Accept-Encoding", "gzip")
    }
}

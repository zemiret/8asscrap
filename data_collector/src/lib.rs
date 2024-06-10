use core::panic;
use std::{arch::x86_64::_mm_cmpord_pd, collections::HashMap, io::{stdout, Write}, process::Command};

use curl::easy::Easy;
use reqwest::header::{HeaderMap, USER_AGENT};

pub struct Client {
    http_client: reqwest::Client,
    connect_sid: String,
    debug: bool,
}

pub fn new_client(debug: bool) -> Client {
    Client {
        http_client: reqwest::Client::new(),
        // connect_sid: String::new(),
        // TODO: Just for test purposes
        connect_sid: "s%3AH519MDgMrFtbrlz4ZXr28kw481eeDRXk.tysjm69u%2BUkCxcDrUjIt56H0zItzk%2FN%2BZD%2FhRi3bZRc".to_string(),
        debug,
    }
}

impl Client {
    #[tokio::main]
    pub async fn example_req(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .http_client
            .get("https://httpbin.org/ip")
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{resp:#?}");
        Ok(())
    }

    pub fn curl_user_ascents(
        &self,
        user: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // We get 401 on sid expired or wrong user-agent

        let mut easy = Easy::new();
        easy.url("https://www.8a.nu/unificationAPI/ascent/v1/web/users/antoni-mleczko/ascents?category=sportclimbing&pageIndex=0&pageSize=1").unwrap();
        easy.useragent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0").unwrap();
        easy.cookie("connect.sid=s%3AH519MDgMrFtbrlz4ZXr28kw481eeDRXk.tysjm69u%2BUkCxcDrUjIt56H0zItzk%2FN%2BZD%2FhRi3bZRc;").unwrap();
        easy.write_function(|data| {
            println!("GOT RES!");
            Ok(stdout().write(data).unwrap())
        }).unwrap();

        easy.perform().unwrap();

        let code = easy.response_code().unwrap();
        if code != 200 {
            panic!("not 200 status code: {}", code);
        }


        Ok(HashMap::new())
    }

    #[tokio::main]
    pub async fn user_ascents(
        &self,
        user: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // We get 401 on sid expired or wrong user-agent

        // Okaaaaay, this works... DAMN. IT KINDA LOOK LIKE IT MIGHT HAVE TO DO WITH SMALL/BIG HEADERS!
        // So i'll need to use some different library as reqwest normalizes all requests to lowercase...
        let mut tmpcmd = Command::new("curl");
        let cmd = tmpcmd
            .args([
                "https://www.8a.nu/unificationAPI/ascent/v1/web/users/antoni-mleczko/ascents?category=sportclimbing&pageIndex=0&pageSize=1",
                "-H",
                "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
                "-H",
                "Cookie: connect.sid=s%3AH519MDgMrFtbrlz4ZXr28kw481eeDRXk.tysjm69u%2BUkCxcDrUjIt56H0zItzk%2FN%2BZD%2FhRi3bZRc;"
            ]);
            // .args([
            //     "https://www.8a.nu/unificationAPI/ascent/v1/web/users/antoni-mleczko/ascents?category=sportclimbing&pageIndex=0&pageSize=1",
            //     "-H",
            //     "user-ugent: Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
            //     "-H",
            //     "cookie: connect.sid=s%3AH519MDgMrFtbrlz4ZXr28kw481eeDRXk.tysjm69u%2BUkCxcDrUjIt56H0zItzk%2FN%2BZD%2FhRi3bZRc;"
            // ]);
        println!("{:#?}", cmd);
        let cmdres = cmd.output()?;

        let cmd_out = String::from_utf8(cmdres.stdout).unwrap(); // TODO: Handle error nicer
        if !cmdres.status.success() {
            panic!("cmd fail: {}", cmd_out); // TODO: HANDLE ERROR NICELY
        }
        println!("{}", cmd_out);


        // TODO: HANDLE PAGING
        // TODO: HANDLE 401 (call authenticate and retry operation)
        // TODO: Bouldering category too

        let mut headers = HeaderMap::new();

        headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0".parse().unwrap());
        headers.insert("Cookie", format!("connect.sid={};", self.connect_sid).parse().unwrap());

        let req = self
            .http_client
            .get(format!(
                "https://www.8a.nu/unificationAPI/ascent/v1/web/users/antoni-mleczko/ascents?category=sportclimbing&pageIndex=0&pageSize=1",
            ))
            .headers(headers)
            // .header(
            //     USER_AGENT,
            //     "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
            // )
            // .header("Cookie", format!("connect.sid={};", self.connect_sid)).build()?;
            .build()?;

        println!("REQUEST:");
        println!("{req:#?}");

        // let resp = req.send().await?;
        let resp = self.http_client.execute(req).await?;

        // let resp = self
        //     .http_client
        //     .get(format!(
        //         "https://www.8a.nu/unificationAPI/ascent/v1/web/users/{}/ascents", user
        //     ))
        //     .query(&[
        //         ("category", "sportclimbing"),
        //         ("pageIndex", "0"),
        //         ("pageSize", "1"),
        //         // ("sortField", "date_desc"),
        //         // ("timeFilter", "0"),
        //         // ("gradeFilter", "0"),
        //         // ("typeFilter", ""),
        //         // ("includeProjects", "false"),
        //         // ("searchQuery", ""),
        //         // ("showRepeats", "false"),
        //         // ("showDuplicates", "false"),
        //     ])
        //     .header(
        //         "User-Agent",
        //         "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
        //     )
        //     .header("Cookie", format!("connect.sid={};", self.connect_sid))
        //     .send()
        //     .await?;
            // .json::<HashMap<String, String>>()
            // .await?;

        // TODO: 403 resp. The question is - Am I doing something wrong here in code or is it the same way I already had it once that it works with curl but not when called from code?
        println!("HELLLOOOOOOOOOO");
        println!("{resp:#?}");

        Ok(HashMap::new())
    }

    pub fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Pass in the path to the exporter exec
        let cmdres = Command::new("node")
            .arg("../../sidexporter/main.mjs")
            .output()?;
        let cmd_out = String::from_utf8(cmdres.stdout).unwrap(); // TODO: Handle error nicer
        if !cmdres.status.success() {
            panic!("sideexporter fail: {}", cmd_out); // TODO: HANDLE ERROR NICELY
        }
        self.connect_sid = cmd_out.trim().to_string();
        if self.debug {
            println!("authenticate: Got connect_sid: {:?}", self.connect_sid)
        }
        Ok(())
    }
}

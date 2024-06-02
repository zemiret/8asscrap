use std::{collections::HashMap, process::Command};

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

    #[tokio::main]
    pub async fn user_ascents(
        &self,
        user: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // We get 401 on sid expired or wrong user-agent


        // TODO: HANDLE PAGING
        // TODO: HANDLE 401 (call authenticate and retry operation)
        // TODO: Bouldering category too
        let resp = self
            .http_client
            .get(format!(
                "https://www.8a.nu/unificationAPI/ascent/v1/web/users/{}/ascents", user
            ))
            .query(&[
                ("category", "sportclimbing"),
                ("pageIndex", "0"),
                ("pageSize", "1000"),
                ("sortField", "date_desc"),
                ("timeFilter", "0"),
                ("gradeFilter", "0"),
                ("typeFilter", ""),
                ("includeProjects", "false"),
                ("searchQuery", ""),
                ("showRepeats", "false"),
                ("showDuplicates", "false"),
            ])
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0",
            )
            .header("Cookie", format!("connect.sid={}", self.connect_sid))
            .send()
            .await?;
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

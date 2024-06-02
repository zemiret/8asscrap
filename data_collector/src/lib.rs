use std::{collections::HashMap, process::Command};

pub struct Client {
    http_client: reqwest::Client,
    connect_sid: String,
    debug: bool,
}

pub fn new_client(debug: bool) -> Client {
    Client {
        http_client: reqwest::Client::new(),
        connect_sid: String::new(),
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
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // self.http_client.get("")
        Ok(HashMap::new())
    }

    pub fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Pass in the path to the exporter exec
        let cmdres = Command::new("node").arg("../../sidexporter/main.mjs").output()?;
        if !cmdres.status.success() {
            panic!("CMD FAIL"); // TODO: HANDLE ERROR NICELY
        }
        self.connect_sid = String::from_utf8(cmdres.stdout).unwrap();
        if self.debug {
            println!("authenticate: Got connect_sid: {:?}", self.connect_sid)
        }
        Ok(())
    }
}

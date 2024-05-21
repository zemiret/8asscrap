use std::collections::HashMap;


pub struct Client {
    http_client: reqwest::Client,
    connect_sid: String,
}

pub fn new_client() -> Client {
    Client {
        http_client: reqwest::Client::new(),
    }
}

impl Client {
    #[tokio::main]
    pub async fn example_req(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self.http_client.get("https://httpbin.org/ip")
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{resp:#?}");
        Ok(())
    }


    #[tokio::main]
    pub async fn user_ascents(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // self.http_client.get("")
    }

    pub fn authenticate(&self) {
        // TODO: Run the sidexporter and store the connect_sid cookie
    }
}





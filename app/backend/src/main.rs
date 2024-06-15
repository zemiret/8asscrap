use data_collector;

fn main() {
    let mut client = data_collector::new_client(true);
    // client.example_req();
    client.authenticate("../../sidexporter/main.mjs").unwrap();
    // client.user_ascents("antoni-mleczko").unwrap();
    client.curl_user_ascents("antoni-mleczko").unwrap();
}

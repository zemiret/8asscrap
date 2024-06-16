use data_collector;

fn main() {
    let mut client = data_collector::new_client(true, "../../sidexporter/main.mjs".to_string());
    // client.example_req();
    // client.authenticate("../../sidexporter/main.mjs").unwrap();
    // client.user_ascents("antoni-mleczko").unwrap();
    // client.curl_user_ascents("antoni-mleczko").unwrap();
    let user_ascents = client.user_ascents("antoni-mleczko", &data_collector::ClimbingCategory::SportClimbing, false).unwrap();

    // let sleep_time_millis = time::Duration::from_millis(200);

    println!("{:?}", user_ascents);
}

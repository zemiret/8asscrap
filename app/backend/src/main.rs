use data_collector;

fn main() {
    let client = data_collector::new_client();
    client.example_req();
}

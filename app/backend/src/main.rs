use data_collector;

fn main() {
    let mut client = data_collector::new_client(true);
    // client.example_req();
    client.authenticate().unwrap();
}

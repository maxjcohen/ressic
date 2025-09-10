use ressic::Client;
use ressic::storage::MockStorage;

#[test]
fn load_client() {
    let storage = MockStorage {};
    let _client = Client::new(storage);
}

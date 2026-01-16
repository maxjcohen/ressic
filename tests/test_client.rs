use ressic::Client;
use ressic::generator::PlainText;
use ressic::storage::MockStorage;

#[test]
fn load_client() {
    let storage = MockStorage::new();
    let generator = PlainText::new();
    let _client = Client::new(storage, generator);
}

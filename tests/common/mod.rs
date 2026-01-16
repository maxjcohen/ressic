use ressic::storage::JsonLocalStorage;
use std::fs;

pub fn with_localfile_storage<F>(test_name: &str, test_fn: F)
where
    F: FnOnce(JsonLocalStorage),
{
    let base = format!("./feeds-test/{}", test_name);
    let _ = fs::remove_dir_all(&base);
    let storage = JsonLocalStorage::new(&base).expect("failed to create LocalFile");
    test_fn(storage);
    let _ = fs::remove_dir_all(&base);
}

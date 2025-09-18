use std::fs;
use ressic::storage::LocalFile;

pub fn with_localfile_storage<F>(test_name: &str, test_fn: F)
where
    F: FnOnce(LocalFile),
{
    let base = format!("./feeds-test/{}", test_name);
    let _ = fs::remove_dir_all(&base);
    let storage = LocalFile::new(&base).expect("failed to create LocalFile");
    test_fn(storage);
    let _ = fs::remove_dir_all(&base);
}
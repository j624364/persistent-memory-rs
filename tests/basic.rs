use persistent_memory_rs::*;

fn get_app_name() -> &'static str {
    "persistent-memory-rs-tester"
}

#[test]
fn store_and_retrieve() {
    let test_message = "This is a test message.";

    let persistent_data_config = PersistentDataConfig::new(
        StorageDirectory::default(),
        get_app_name(),
        "store-and-retrieve",
    );

    persistent_data_config.store(&test_message).unwrap();
    let read_message = persistent_data_config.retrieve::<String>().unwrap();

    assert_eq!(test_message, &read_message);
}

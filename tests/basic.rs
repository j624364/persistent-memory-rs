use persistent_memory_rs::*;

fn get_app_name() -> &'static str {
    "persistent-memory-rs-tester"
}

#[test]
fn store_and_retrieve() {
    let test_message = "This is a test message.";

    let persistent_data = PersistentDataBuilder::new(get_app_name(), "store-and-retrieve").build();

    persistent_data.store(&test_message).unwrap();
    let read_message = persistent_data.retrieve::<String>().unwrap();

    assert_eq!(test_message, &read_message);
}

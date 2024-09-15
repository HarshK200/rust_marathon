pub fn printmap() {
    let mut mymap = std::collections::HashMap::new();
    // NOTE: maps have unique keys i.e. the keys cannot be repeated but the value can be repeated

    mymap.insert(String::from("user1"), String::from("harsh"));
    mymap.insert(String::from("user2"), String::from("mypc"));
    mymap.insert(String::from("user3"), String::from("sera"));

    println!("hashmap: {:?}", mymap);
    println!("user2's details:\n{:?}", mymap.get("user2")); // WARN: the HashMap.get("key") returns the
                                                            // value as an Option(Some || None)
}

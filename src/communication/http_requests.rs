pub trait RequestSerializable {
    fn serialize(&self) -> String;
}

pub fn send(serializable: &dyn RequestSerializable, url: &String) {
    let query = serializable.serialize();

    let newrl = url.to_owned() + &query;

    println!("{}\t{}", newrl, query);

    let client = reqwest::blocking::Client::new();
    match client.post(newrl)
        // .body(serializable.serialize())
        .send() {
        Ok(v) => { println!("SUCCESS! {}", v.text().unwrap()) }
        Err(e) => { println!("ERROR! {}", e.to_string()) }
    };
}
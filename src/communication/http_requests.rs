pub trait RequestSerializable {
    fn serialize(&self) -> String;
}

pub fn send(serializable: &dyn RequestSerializable, url: &String) {

    let query = serializable.serialize();

    let url = url.to_owned() + &query;

    match reqwest::blocking::get(url) {
        Ok(v) => match v.text() {
            Ok(v1) => println!("Message received: {}", v1);
            Err(_) => println!("Could not read message");
        },
        Err(_) => println!("An error occurred."),
    };
}

pub fn test() {
    let url = String::from("http://duckduckgo.com");
    let mut response = reqwest::blocking::get(url).unwrap();

    response.copy_to(&mut std::io::stdout()).unwrap();
}
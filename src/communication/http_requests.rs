pub trait RequestSerializable {
    fn serialize(&self) -> String;
}

pub fn send(serializable: &dyn RequestSerializable, url: &String) {
    let query = serializable.serialize();

    let newrl = url.to_owned() + &query;

    println!("{}\t{}", newrl, query);

    let client = reqwest::blocking::Client::new();
    match client
        .post(newrl)
        // .body(serializable.serialize())
        .send()
    {
        Ok(v) => {
            println!("SUCCESS! {}", v.text().unwrap())
        }
        Err(e) => {
            println!("ERROR! {}", e.to_string())
        }
    };
}

struct Test<'a> {
    s1: &'a str,
    s2: &'a str,
}

pub fn test() {
    let client = reqwest::blocking::Client::new();

    let t = Test {
        s1: "test",
        s2: "test too",
    };

    match client
        .post("http://127.0.0.1/test/")
        // .json(&t)
        // .body(serializable.serialize())
        .send()
    {
        Ok(v) => {
            println!("SUCCESS! {}", v.text().unwrap())
        }
        Err(e) => {
            println!("ERROR! {}", e.to_string())
        }
    };
}

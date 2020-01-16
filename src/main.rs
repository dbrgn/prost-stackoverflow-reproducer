use prost::Message;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/reproducer.rs"));
}

fn parse(bytes: &[u8]) {
    match proto::Empty::decode(bytes) {
        Ok(_) => println!("Parsing succeeded"),
        Err(_) => println!("Parsing failed"),
    }
}

fn main() {
    println!("Hello, world!");
    parse(include_bytes!("../testinput/1.bin"));
    println!("Goodbye, world!");
}

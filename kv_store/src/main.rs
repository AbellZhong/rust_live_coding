mod pb;

use prost::Message;

use crate::pb::abi::RequestGet;


fn main() {
    let request = pb::abi::RequestGet{
        key: "hello".to_string()
    };

    let mut buf = Vec::new();

    request.encode(&mut buf).unwrap();

    println!("encoded: {:?}", buf);
}

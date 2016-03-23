extern crate rustc_serialize;
use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8, 
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };  

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();
    //println!("{:?}",encoded);
    println!("{}",encoded);
    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(&encoded).unwrap();
    println!("{:?}",decoded.data_vector);
}


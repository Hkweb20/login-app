use serde::{Deserialize,Serialize};
use serde_json;
use std::fs::File;
use std::io::{prelude::*, BufReader};
pub mod task;
pub  use task::mytask::*;

fn main(){
    let address = Address{
        city: "Kaura Namoda".to_string(),
        home: "Emier palace".to_string(),
        country: "Nigeria".to_string(),
    };
    let people = People{
        name: "Hassan Kabiru".to_string(),
        age: 23,
        isactive: true,
        address: address,

    };
    let json_string = serde_json::to_string(&people).expect("error");

    let mut file =  File::options().append(true).create(true).open("people.json").expect("error occur");
    file.write_all(json_string.as_bytes()).expect("error");
    file.write(b"\n").expect("error");

    let file= File::open("people.json").expect("error");
    let reader = BufReader::new(file);

    for line in reader.lines()  {
        let line = line.expect("error");

        let string_json:People = serde_json::from_str(&line).expect("error");
        println!("The normal people {:?}", string_json);
        
    }
  
}
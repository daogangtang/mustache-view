extern crate mustache;
extern crate rustc_serialize;

use std::io::Write;
use std::path::Path;
use rustc_serialize::Encodable;

pub struct View;

impl View {
	pub fn render<T: Encodable> (pathstr: &str, data: T) -> Vec<u8> {
		let path = Path::new(pathstr);
                let template = mustache::compile_path(path).unwrap();
                let mut output: Vec<u8> = vec![];
                
                template.render(&mut output, &data).unwrap();
        
                output
	}
}




#[test]
fn it_works() {
    use std::collections::HashMap;
    
    let mut hash = HashMap::new();
    hash.insert("name", "hello world!");

    //let view_instance = View;
    let output =  View::render("test.txt", hash);
    assert!(String::from_utf8(output).unwrap() == "abcdefg hello world! higklmn.");
}

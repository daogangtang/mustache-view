extern crate mustache;
extern crate rustc_serialize;

use std::path::Path;
use rustc_serialize::Encodable;

pub struct View;

impl View {
	pub fn render<T: Encodable> (pathstr: &str, data: T) -> String {
		let path = Path::new(pathstr);
        let template = mustache::compile_path(path).unwrap();
        let mut output: Vec<u8> = vec![];
        
        template.render(&mut output, &data).unwrap();

        String::from_utf8(output).expect(&format!("failed to render template {}", pathstr))
	}
}




#[test]
fn it_works() {
    use std::collections::HashMap;
    
    let mut hash = HashMap::new();
    hash.insert("name", "hello world!");

    //let view_instance = View;
    let output =  View::render("test.txt", hash);
    assert!(output == "abcdefg hello world! higklmn.");
}

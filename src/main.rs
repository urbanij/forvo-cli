extern crate requests;
use std::println;

use std::process::Command;

use regex::Regex;


fn main() {

    let response = requests::get("https://forvo.com/search/jacket/").unwrap();

    let content = response.text().unwrap();

    // println!("{}", content);

//     let content = r#"
// k=\"Play(265113,\'OTAwMDg2My8zOS85MDAwODYzXzM5XzM0MTkwXzIxOTg0Lm1wMw==\',\'OTAwMDg2My8zO
//     "#;

/// ok
// let content = r##"
// k="Play(265113,'OTAwMDg2My8zOS85MDAwODYzXzM5XzM0MTkwXzIxOTg0Lm1wMw==','OTAwMDg2My8zO
//     "##;



    // println!("{:?}",content);

    let regex_sequence_pattern = Regex::new(r"(Play\(\w+,')(\w+=*)").unwrap(); 
    

    for caps in regex_sequence_pattern.captures_iter(content) {
        let code_sequence = caps.get(2).unwrap().as_str();
    }


    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    
    


}

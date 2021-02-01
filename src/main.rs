extern crate requests;
use std::println;

use std::fs::File;
use std::io::Error;

use std::process::{Command, Stdio};

use regex::Regex;
use argparse::{ArgumentParser, StoreTrue, Store};

fn main() -> Result<(), Error> {

    // default values of argparsed arguments
    let mut verbose = false;
    let mut word = String::from("");

    {   // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("forvo-cli");
        ap.refer(&mut word)
            .add_option(&["-w"], 
                        Store,
                        "word")
            .required();
                        
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], 
                        StoreTrue,
                        "be verbose");
        ap.parse_args_or_exit();
    }

    let response = requests::get(format!("https://forvo.com/search/{}/", word)).unwrap();

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

        println!("{}", code_sequence);

        let outputs = File::create("/tmp/forvo.mp3")?;
        let errors = outputs.try_clone()?;
    
        Command::new("curl")
            .args(&["-s", format!("https://forvo.com/player-mp3Handler.php?path={}",code_sequence).as_str()])
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .spawn()?
            .wait_with_output()?;
        
        Command::new("afplay")
            .arg("/tmp/forvo.mp3")
            .spawn()
            .ok()
            .expect("Can't play audio recording");

        break;
        
    }


    // if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(&["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute process")
    // };
    



    Ok(())
}


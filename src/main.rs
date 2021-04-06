/*
 * Author(s):   Francesco Urbani
 * Date:        some evening between January-Februrary 2021
 */

use std::println;

use std::io;
use std::fs::File;
use std::io::Error;

use std::process::{Command, Stdio};

use regex::Regex;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate reqwest;

fn main() -> Result<(), Error> {

    // default values of argparsed arguments
    let mut verbose = false;
    let mut word = "".to_string();

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

    let url = format!("https://forvo.com/search/{}/", word);

    let content = reqwest::get(url.as_str())
                    .expect("Could not make request.")
                    .text()
                    .expect("Could not read text.");
    
    let content = content.as_str();
    // println!("{}", content);

    let regex_num_results_found = Regex::new(r"(>)(\d+)( words found)").unwrap();
    for caps in regex_num_results_found.captures_iter(content) {
        let num_results = caps.get(2).unwrap().as_str();
        println!("{} words found.", num_results);
        println!("\tPress enter (↵) to play them, one after another.");
        println!("\tPress ^C (ctrl-C / cmd-C) to quit prematurely.");
    }

    let regex_sequence_pattern = Regex::new(r"(Play\(\w+,')(\w+=*)").unwrap(); 
    

    for caps in regex_sequence_pattern.captures_iter(content) {
        let code_sequence = caps.get(2).unwrap().as_str();

        // println!("{}", code_sequence);

        let outputs = File::create(format!("/tmp/{}.mp3", word.replace(" ", "_")).as_str())?;
        let errors = outputs.try_clone()?;
    
        Command::new("curl")
            .args(&["-s", format!("https://forvo.com/player-mp3Handler.php?path={}", code_sequence).as_str()])
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .spawn()?
            .wait_with_output()?;
        
        if cfg!(target_os = "macos") {
            Command::new("afplay")
                .arg(format!("/tmp/{}.mp3", word.replace(" ", "_")).as_str())
                .spawn()
                .ok()
                .expect("Can't play audio recording");
        } else {
            println!("Sorry, I'm not able to autoplay the audio recording 
on your system (which is not macOS). 
While I'm working on that you can navigate to {} 
and play that yourself, sorry for the inconvenience.", 
                format!("/tmp/{}.mp3", word.replace(" ", "_")) 
            );
        }

        
        // the user types enter or anything else to listen to the next result.
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => { },
            Err(error) => println!("error: {}", error),
        }
        
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


#[cfg(test)]
mod tests {

    #[test]
    fn test_sqrt() -> Result<(), String> {
        use super::*;

        let content = r##"
k="Play(265113,'OTAwMDg2My8zOS85MDAwODYzXzM5XzM0MTkwXzIxOTg0Lm1wMw==','OTAwMDg2My8zO
"##;
        let regex_sequence_pattern = Regex::new(r"(Play\(\w+,')(\w+=*)").unwrap(); 
    
        for caps in regex_sequence_pattern.captures_iter(content) {
            let code_sequence = caps.get(2).unwrap().as_str();
            assert_eq!(code_sequence, "OTAwMDg2My8zOS85MDAwODYzXzM5XzM0MTkwXzIxOTg0Lm1wMw==");
        }
        Ok(())
    }
}

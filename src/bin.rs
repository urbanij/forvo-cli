use argparse::{ArgumentParser, StoreTrue, Store};
use std::fs::File;

use std::process::{Command, Stdio};



use forvolib::*;


fn main() -> Result<(), std::io::Error> {
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


    for code_sequence in forvolib::retrieve_audios(&word)? {
        if verbose {
            println!("https://forvo.com/player-mp3Handler.php?path={}", code_sequence);
        }


        let outputs = File::create(format!("/tmp/{}.mp3", word.replace(" ", "_")).as_str())?;
        let errors = outputs.try_clone()?;
    
        Command::new("curl")
            .args(&["-s", format!("https://forvo.com/player-mp3Handler.php?path={}", code_sequence).as_str()])
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .spawn()?
            .wait_with_output()?;
        
        
        if cfg!(target_os = "macos") {
            // println!("Hi from macOS");
            Command::new("afplay")
                .arg(format!("/tmp/{}.mp3", word.replace(" ", "_")).as_str())
                .spawn()
                .ok()
                .expect("Can't play audio recording");
        } else if cfg!(target_os = "linux") {
            // println!("Hi from Linux");
            Command::new("play")
                .arg("-s")
                .arg(format!("/tmp/{}.mp3", word.replace(" ", "_")).as_str())
                .spawn()
                .ok()
                .expect("Can't play audio recording");
                // "apt-get install sox" <- play <filename>.mp3 does not work either
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
        match std::io::stdin().read_line(&mut input) {
            Ok(_n) => { },
            Err(error) => println!("error: {}", error),
        }



    }

    
    Ok(())
}
fn main() -> Result<(), std::io::Error> {
    
    let word = "rijksmuseum";
    
    for i in forvolib::retrieve_audios(&word.to_string())? {
        println!("https://forvo.com/player-mp3Handler.php?path={}", i);
    }

    Ok(())
}

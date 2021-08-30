/*
 * Author(s): @urbanij,
 */

fn main() -> Result<(), std::io::Error> {
    
    let word = "rijksmuseum";
    
    let results = forvolib::retrieve_audios(word)?;

    println!("{} pronunciations found for {}:\n", results.len(), word);

    for (index, audio_url) in results.iter().enumerate() {
        println!("{}) https://forvo.com/player-mp3Handler.php?path={}", index+1, audio_url);
    }

    Ok(())
}

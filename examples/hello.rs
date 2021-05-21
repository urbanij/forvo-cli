/*
 * Author(s): @urbanij,
 */

fn main() -> Result<(), std::io::Error> {
    
    let word = "rijksmuseum";
    
    let results = forvolib::retrieve_audios(word)?;

    println!("{} pronunciations found for {}:\n", results.len(), word);

    for audio_url in results {
        println!("https://forvo.com/player-mp3Handler.php?path={}", audio_url);
    }

    Ok(())
}

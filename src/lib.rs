/*
 * Author(s):   Francesco Urbani
 * Date:        some evening between January-Februrary 2021
 */

use regex::Regex;



/// Pass a word to this function and return a list of codes you can use to 
/// listen their pronounciations on [Forvo](https://forvo.com/)
pub fn retrieve_audios(word: &String) -> Result<Vec<String>, std::io::Error> {

    let url = format!("https://forvo.com/search/{}/", word);

    let content = reqwest::get(url.as_str())
                    .expect("Could not make request.")
                    .text()
                    .expect("Could not read text.");
    
    // println!("{}", content);

    
    /*
    let regex_num_results_found = Regex::new(r"(>)(\d+)( words found)").unwrap();
    for caps in regex_num_results_found.captures_iter(content.as_str()) {
        let num_results = caps.get(2).unwrap().as_str();
        println!("{} words found.", num_results);
        println!("\tPress enter (↵) to play them, one after another.");
        println!("\tPress ^C (ctrl-C / cmd-C) to quit prematurely.");
    }
    */


    let mut pronounciations = vec![];

    let regex_sequence_pattern = Regex::new(r"(Play\(\w+,')(\w+=*)").unwrap(); 
    for caps in regex_sequence_pattern.captures_iter(content.as_str()) {
        let code_sequence = caps.get(2).unwrap().as_str();

        pronounciations.push(code_sequence.to_string());

    }

    Ok(pronounciations)
}


#[cfg(test)]
mod tests {
    use super::*;  
    #[test]
    fn test_regex_1() -> Result<(), String> {  
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

    #[test]
    fn test_results_1() -> Result<(), String> {
        assert_eq!(
            retrieve_audios(&"rijksmuseum".to_string()).unwrap(),
            vec![
                String::from("OTM4NDcyNy8xMTgvOTM4NDcyN18xMThfMzk1NTc1Lm1wMw=="),
                String::from("OTI1MDI1OS8xMTgvOTI1MDI1OV8xMThfMjA5NDg0NF8xLm1wMw=="),
                String::from("OTkxNzYyOC8xMTgvOTkxNzYyOF8xMThfNjMwMTQ0Ni5tcDM="),
            ]
        );
        Ok(())
    }
}

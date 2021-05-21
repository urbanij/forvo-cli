
#[cfg(test)]
mod tests {
    
    pub use crate::retrieve_audios;
    use regex::Regex;

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

    #[test]
    fn test_results_2() -> Result<(), String> {
        assert_eq!(
            retrieve_audios(&"llanfairpwllgwyngyllgogerychwyrndrobwllllantysiliogogogoch".to_string()).unwrap(),
            vec![
                String::from("OTY2ODAyMC8zMi85NjY4MDIwXzMyXzcwODQ1MzgubXAz"),
            ]
        );
        Ok(())
    }
}

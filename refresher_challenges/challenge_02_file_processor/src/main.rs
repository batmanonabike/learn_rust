// Challenge 2: File Text Processor
// Your task: implement a text processing tool with file I/O and collections

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct TextStats {
    lines: usize,
    words: usize,
    characters: usize,
    unique_words: usize,
}

struct TextProcessor {
    content: String,
    filename: String,
}

impl TextProcessor {
    // TODO: Implement new method that reads file and handles errors
    fn new(filename: &str) -> Result<TextProcessor, Box<dyn std::error::Error>> {
        todo!("Read file content and create TextProcessor")
    }

    // TODO: Implement get_stats method
    fn get_stats(&self) -> TextStats {
        todo!("Calculate text statistics")
    }

    // TODO: Implement search_lines method
    fn search_lines(&self, pattern: &str) -> Vec<(usize, String)> {
        todo!("Find lines containing the pattern, return line number and content")
    }

    // TODO: Implement replace_text method
    fn replace_text(&self, from: &str, to: &str) -> String {
        todo!("Replace all occurrences of 'from' with 'to'")
    }

    // TODO: Implement to_uppercase method
    fn to_uppercase(&self) -> String {
        todo!("Convert content to uppercase")
    }

    // TODO: Implement word_frequency method
    fn word_frequency(&self) -> HashMap<String, usize> {
        todo!("Count frequency of each word (case-insensitive)")
    }

    // TODO: Implement save_to_file method
    fn save_to_file(&self, filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Save content to a file")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processor = TextProcessor::new("sample.txt")?;
    
    let stats = processor.get_stats();
    println!("File Statistics:");
    println!("Lines: {}", stats.lines);
    println!("Words: {}", stats.words);
    println!("Characters: {}", stats.characters);
    println!("Unique words: {}", stats.unique_words);
    println!();
    
    // Search for lines containing "rust" (case-insensitive)
    let matches = processor.search_lines("rust");
    println!("Lines containing 'rust':");
    for (line_num, line) in matches {
        println!("Line {}: {}", line_num, line);
    }
    println!();
    
    // Show word frequency
    let freq = processor.word_frequency();
    println!("Word frequency (top 5):");
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in freq_vec.iter().take(5) {
        println!("{}: {}", word, count);
    }
    println!();
    
    // Create an uppercase version and save it
    let upper_content = processor.to_uppercase();
    processor.save_to_file("output.txt", &upper_content)?;
    println!("Uppercase version saved to output.txt");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_not_found() {
        let result = TextProcessor::new("nonexistent.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_stats_calculation() {
        // Create a temporary test file
        let content = "Hello world\nThis is a test";
        fs::write("test.txt", content).unwrap();
        
        let processor = TextProcessor::new("test.txt").unwrap();
        let stats = processor.get_stats();
        
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 6);
        assert_eq!(stats.characters, 24); // including newline
        
        // Cleanup
        fs::remove_file("test.txt").unwrap();
    }

    #[test]
    fn test_search_functionality() {
        let content = "Hello world\nworld of Rust\nGoodbye";
        fs::write("search_test.txt", content).unwrap();
        
        let processor = TextProcessor::new("search_test.txt").unwrap();
        let matches = processor.search_lines("world");
        
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].0, 1);
        assert_eq!(matches[1].0, 2);
        
        // Cleanup
        fs::remove_file("search_test.txt").unwrap();
    }

    #[test]
    fn test_word_frequency() {
        let content = "hello world hello rust world";
        fs::write("freq_test.txt", content).unwrap();
        
        let processor = TextProcessor::new("freq_test.txt").unwrap();
        let freq = processor.word_frequency();
        
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&2));
        assert_eq!(freq.get("rust"), Some(&1));
        
        // Cleanup
        fs::remove_file("freq_test.txt").unwrap();
    }
}

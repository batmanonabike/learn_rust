// Challenge 2: File Text Processor
// Your task: implement a text processing tool with file I/O and collections

use std::collections::{HashMap, HashSet};
use std::fs;
use ahash::RandomState;

#[derive(Debug)]
struct TextStats {
    lines: usize,
    words: usize,
    characters: usize,
    unique_words: usize,
}

struct TextProcessor {
    content: String,
    // filename: String,
}

impl TextProcessor {
    fn new(filename: &str) -> Result<TextProcessor, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;        
        Ok(TextProcessor {
            content,
            //filename: filename.to_string(),
        })
    }
    
    fn get_stats(&self) -> TextStats {        
        Self::count_words(&self.content)
    }
    
    fn search_lines(&self, pattern: &str) -> Vec<(usize, String)> {
        let mut result: Vec<(usize, String)> = Vec::new();        
        for (size, line) in self.content.lines().enumerate() {
            if line.contains(pattern) {
                result.push((size, line.to_string()));
            }
        }
        result
    }
    
    fn replace_text(&self, from: &str, to: &str) -> String {
        self.content.replace(from, to)
    }

    fn to_uppercase(&self) -> String {
        self.content.to_uppercase()
    }
    
    #[allow(dead_code)]
    fn word_frequency_crypto_safe(&self) -> HashMap<String, usize> {        
        // Use a temporary map of string slices so that we reduce heap allocations.
        let map: HashMap<&str, usize> = self.content
            .split_whitespace()

            // fold: takes initial value (HashMap) + closure, accumulates across all items
            // map: transforms each item individually, returns same number of items
            // Here we need fold because we're building one HashMap from many words
            .fold(HashMap::new(), |mut acc, word| {

                // Use HashMap's entry API for efficient counting:
                // 1. acc.entry(word) - get Entry enum (Occupied or Vacant)
                // 2. .or_insert(0) - if vacant, insert 0; if occupied, return existing value
                // 3. * - dereference to get the actual usize value
                // 4. += 1 - increment the count
                   *acc.entry(word).or_insert(0) += 1;
                // ^
                // This dereferences the mutable reference to get the actual usize value.
                // These are exactly equivalent...
                // *acc.entry(word).or_insert(0) += 1;                
                // *(acc.entry(word).or_insert(0)) += 1;
                
                acc
                // Our closure has to return the accumulator for the next iteration.
            });

        // Now we can map to the return value that can own this collection.
        let result: HashMap<String, usize> = 
            map.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        result
    }

    // This version uses faster hash map which doesnt care to be crptographically safe.
    // See hashbrown/ahash.
    fn word_frequency_faster(&self) -> HashMap<String, usize> {
       // Use a temporary map of string slices so that we reduce heap allocations.
        let map: HashMap<&str, usize, RandomState> = self.content
            .split_whitespace()

            // fold: takes initial value (HashMap) + closure, accumulates across all items
            // map: transforms each item individually, returns same number of items
            // Here we need fold because we're building one HashMap from many words
            .fold(HashMap::with_hasher(RandomState::new()), |mut acc, word| {

                // Use HashMap's entry API for efficient counting:
                // 1. acc.entry(word) - get Entry enum (Occupied or Vacant)
                // 2. .or_insert(0) - if vacant, insert 0; if occupied, return existing value
                // 3. * - dereference to get the actual usize value
                // 4. += 1 - increment the count
                   *acc.entry(word).or_insert(0) += 1;
                // ^
                // This dereferences the mutable reference to get the actual usize value.
                // These are exactly equivalent...
                // *acc.entry(word).or_insert(0) += 1;                
                // *(acc.entry(word).or_insert(0)) += 1;
                
                acc
                // Our closure has to return the accumulator for the next iteration.
            });

        // Now we can map to the return value that can own this collection.
        let result: HashMap<String, usize> = 
            map.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        result
    }

    // TODO: Implement save_to_file method
    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(filename, &self.content)?;
        Ok(())
    }

    fn count_words(content: &str) -> TextStats {
        let lines = content.lines().count();
        let characters = content.chars().count();

        let words_array: Vec<&str> = content.split_whitespace().collect();
        let words = words_array.len();

        let unique_words_map = words_array.into_iter().map(|word| word.to_lowercase());
        let unique_words_set = unique_words_map.collect::<HashSet<String>>();
        let unique_words = unique_words_set.len();
            
        TextStats {
            lines,
            words,
            characters,
            unique_words
        }
    }
}

// ...existing code...
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
    let freq = processor.word_frequency_faster();
    println!("Word frequency (top 5):");
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in freq_vec.iter().take(5) {
        println!("{}: {}", word, count);
    }
    println!();

    let uppercase_content = processor.to_uppercase();
    std::fs::write("output.txt", uppercase_content)?;
    println!("Uppercase version saved to output.txt");
    
    // Demonstrate save_to_file functionality
    processor.save_to_file("original_copy.txt")?;
    println!("Original content saved to original_copy.txt");
    
    // Demonstrate replace_text functionality
    let replaced_content = processor.replace_text("rust", "Rust");
    std::fs::write("replaced.txt", replaced_content)?;
    println!("Text with replacements saved to replaced.txt");
    
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
        assert_eq!(stats.characters, 26); // including newline
        
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
        assert_eq!(matches[0].0, 0);
        assert_eq!(matches[1].0, 1);
        
        // Cleanup
        fs::remove_file("search_test.txt").unwrap();
    }

    #[test]
    fn test_word_frequency() {
        let content = "hello world hello rust world";
        fs::write("freq_test.txt", content).unwrap();
        
        let processor = TextProcessor::new("freq_test.txt").unwrap();
        let freq = processor.word_frequency_faster();
        
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&2));
        assert_eq!(freq.get("rust"), Some(&1));
        
        // Cleanup
        fs::remove_file("freq_test.txt").unwrap();
    }
}

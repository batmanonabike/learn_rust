# Challenge 2: File Text Processor

**Difficulty:** 🟡 Intermediate  
**Topics:** File I/O, Error Handling, Collections, String Processing, Pattern Matching

## The Challenge

Create a command-line text processing tool that can analyze files and perform various operations on them. This challenge will refresh your knowledge of file I/O, error handling, and collections.

## Requirements

Build a program that can:

1. **Read a text file** and handle file errors gracefully
2. **Count statistics:** lines, words, characters, unique words
3. **Search functionality:** find lines containing specific text
4. **Text transformations:** convert to uppercase/lowercase, replace text
5. **Export results** to a new file

### Core Structure
```rust
use std::collections::HashMap;
use std::fs;
use std::io;

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
```

### Required Methods

1. `new(filename: &str) -> Result<TextProcessor, Box<dyn std::error::Error>>`
2. `get_stats(&self) -> TextStats`
3. `search_lines(&self, pattern: &str) -> Vec<(usize, String)>` (returns line number and content)
4. `replace_text(&self, from: &str, to: &str) -> String`
5. `to_uppercase(&self) -> String`
6. `word_frequency(&self) -> HashMap<String, usize>`
7. `save_to_file(&self, filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>>`

### Example Usage
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processor = TextProcessor::new("sample.txt")?;
    
    let stats = processor.get_stats();
    println!("File Statistics:");
    println!("Lines: {}", stats.lines);
    println!("Words: {}", stats.words);
    println!("Characters: {}", stats.characters);
    println!("Unique words: {}", stats.unique_words);
    
    // Search for lines containing "rust"
    let matches = processor.search_lines("rust");
    for (line_num, line) in matches {
        println!("Line {}: {}", line_num, line);
    }
    
    // Create an uppercase version and save it
    let upper_content = processor.to_uppercase();
    processor.save_to_file("output.txt", &upper_content)?;
    
    Ok(())
}
```

## Learning Objectives

- Practice file I/O with proper error handling
- Work with `Result<T, E>` and the `?` operator
- Use `HashMap` for counting and frequency analysis
- Practice string processing and iteration
- Understand trait objects (`Box<dyn std::error::Error>`)

## Starter Files

Create a `sample.txt` file for testing:
```text
Rust is a systems programming language.
It focuses on safety, speed, and concurrency.
Rust prevents segfaults and guarantees memory safety.
Many developers love Rust for its performance.
Learning Rust can be challenging but rewarding.
```

## Hints

<details>
<summary>Click to see hints</summary>

1. **File reading:** Use `std::fs::read_to_string()`
2. **Error handling:** Use `Box<dyn std::error::Error>` for generic error types
3. **Word counting:** Split by whitespace and filter empty strings
4. **Case-insensitive search:** Convert to lowercase before comparing
5. **HashMap:** Use `entry().or_insert()` for counting

Example word splitting:
```rust
let words: Vec<&str> = text.split_whitespace().collect();
```

</details>

## Bonus Challenges

1. **Command-line arguments:** Use `std::env::args()` to accept filename as argument
2. **Multiple file processing:** Process multiple files and compare statistics
3. **Regular expressions:** Use the `regex` crate for advanced pattern matching
4. **CSV export:** Export word frequency as CSV format
5. **Performance:** Use `BufReader` for large files

## Testing

Include tests for:
- File not found errors
- Empty file handling
- Word frequency counting
- Search functionality

## Next Steps

After completing this challenge, move on to `challenge_03_temperature_sensor` to practice enums, pattern matching, and traits!

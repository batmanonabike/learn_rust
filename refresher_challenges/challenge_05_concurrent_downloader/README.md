# Challenge 5: Concurrent Web Downloader

**Difficulty:** 🔴 Advanced  
**Topics:** Async Programming, Networking, Concurrency, Error Handling, Channels

## The Challenge

Build a high-performance, concurrent web downloader that can fetch multiple URLs simultaneously while providing progress tracking, retry logic, and rate limiting. This challenge combines async programming, networking, and advanced concurrency patterns.

## Requirements

Create a downloader that can:

1. **Download multiple URLs concurrently** with configurable parallelism
2. **Track download progress** with real-time updates
3. **Implement retry logic** with exponential backoff
4. **Rate limiting** to respect server limits
5. **Save downloads** to specified directories with conflict resolution
6. **Provide detailed statistics** and error reporting

### Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["stream"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
indicatif = "0.17"
url = "2.0"
```

### Core Structures

```rust
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use url::Url;

#[derive(Debug, Clone)]
struct DownloadConfig {
    max_concurrent: usize,
    retry_attempts: u32,
    timeout: Duration,
    rate_limit: Option<Duration>, // Delay between requests
    output_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct DownloadRequest {
    url: Url,
    filename: Option<String>,
    headers: Option<HashMap<String, String>>,
}

#[derive(Debug)]
enum DownloadEvent {
    Started(String),
    Progress(String, u64, u64), // url, downloaded, total
    Completed(String, PathBuf),
    Failed(String, String), // url, error
    Retry(String, u32), // url, attempt number
}

#[derive(Debug)]
struct DownloadStats {
    total_files: usize,
    completed: usize,
    failed: usize,
    total_bytes: u64,
    start_time: std::time::Instant,
}
```

### Required Implementation

```rust
struct WebDownloader {
    config: DownloadConfig,
    client: reqwest::Client,
    stats: Arc<Mutex<DownloadStats>>,
}

impl WebDownloader {
    fn new(config: DownloadConfig) -> Self;
    
    async fn download_urls(&self, requests: Vec<DownloadRequest>) -> Result<(), DownloadError>;
    
    async fn download_single(&self, request: DownloadRequest) -> Result<PathBuf, DownloadError>;
    
    fn subscribe_to_events(&self) -> mpsc::Receiver<DownloadEvent>;
    
    async fn get_stats(&self) -> DownloadStats;
}
```

### Example Usage

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DownloadConfig {
        max_concurrent: 5,
        retry_attempts: 3,
        timeout: Duration::from_secs(30),
        rate_limit: Some(Duration::from_millis(200)),
        output_dir: PathBuf::from("downloads"),
    };
    
    let downloader = WebDownloader::new(config);
    
    let requests = vec![
        DownloadRequest {
            url: "https://httpbin.org/json".parse()?,
            filename: Some("test1.json".to_string()),
            headers: None,
        },
        DownloadRequest {
            url: "https://httpbin.org/uuid".parse()?,
            filename: Some("test2.json".to_string()),
            headers: None,
        },
    ];
    
    // Subscribe to events in a separate task
    let mut event_receiver = downloader.subscribe_to_events();
    tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                DownloadEvent::Started(url) => {
                    println!("🚀 Started downloading: {}", url);
                }
                DownloadEvent::Progress(url, downloaded, total) => {
                    let progress = (downloaded as f64 / total as f64) * 100.0;
                    println!("📊 {}: {:.1}% ({}/{})", url, progress, downloaded, total);
                }
                DownloadEvent::Completed(url, path) => {
                    println!("✅ Completed: {} -> {:?}", url, path);
                }
                DownloadEvent::Failed(url, error) => {
                    println!("❌ Failed: {} - {}", url, error);
                }
                DownloadEvent::Retry(url, attempt) => {
                    println!("🔄 Retrying: {} (attempt {})", url, attempt);
                }
            }
        }
    });
    
    // Start downloads
    downloader.download_urls(requests).await?;
    
    // Print final stats
    let stats = downloader.get_stats().await;
    println!("📈 Final stats: {:?}", stats);
    
    Ok(())
}
```

## Advanced Features

1. **Command-line interface:**
   ```rust
   #[derive(Parser)]
   #[clap(name = "webdl")]
   struct Cli {
       #[clap(short, long)]
       urls: Vec<Url>,
       
       #[clap(short, long, default_value = "5")]
       concurrent: usize,
       
       #[clap(short, long, default_value = "downloads")]
       output: PathBuf,
   }
   ```

2. **Progress bars:**
   ```rust
   use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
   
   // Create individual progress bars for each download
   ```

3. **Resume capability:**
   ```rust
   async fn resume_download(&self, url: &Url, partial_file: &Path) -> Result<(), DownloadError>;
   ```

4. **Bandwidth limiting:**
   ```rust
   struct RateLimiter {
       tokens: Arc<Mutex<f64>>,
       last_update: Arc<Mutex<Instant>>,
       rate: f64, // bytes per second
   }
   ```

## Learning Objectives

- Master async/await and futures
- Understand tokio's concurrency primitives
- Implement producer-consumer patterns with channels
- Handle HTTP streaming and large file downloads
- Create robust error handling for network operations
- Build responsive CLI applications

## Hints

<details>
<summary>Click to see hints</summary>

1. **Concurrent downloads:** Use `futures::stream::iter().buffer_unordered()`

2. **Progress tracking:** Stream the response body and report chunks:
   ```rust
   while let Some(chunk) = response.chunk().await? {
       file.write_all(&chunk).await?;
       // Send progress event
   }
   ```

3. **Retry with backoff:**
   ```rust
   let delay = Duration::from_millis(100 * 2_u64.pow(attempt));
   tokio::time::sleep(delay).await;
   ```

4. **Rate limiting:** Use `tokio::time::interval()` or token bucket algorithm

</details>

## Bonus Challenges

1. **Checksum verification:** Verify downloads with MD5/SHA256 hashes
2. **Mirror support:** Try multiple mirrors for the same file
3. **Pause/resume:** Allow pausing and resuming downloads
4. **Web UI:** Create a web interface using `warp` or `axum`
5. **Plugin system:** Support custom download handlers
6. **Database tracking:** Store download history in SQLite

## Testing

- Mock HTTP servers for testing retry logic
- Integration tests with real downloads
- Performance benchmarks for concurrent downloads
- Error injection testing

## Next Steps

After completing this challenge, move on to `challenge_06_json_api_server` to practice building REST APIs!

# Challenge 3: Temperature Sensor Network

**Difficulty:** 🟡 Intermediate  
**Topics:** Enums, Pattern Matching, Traits, Option/Result, Collections

## The Challenge

Build a temperature monitoring system that can handle multiple sensor types, different temperature units, and provide analysis of the collected data. This challenge focuses on Rust's powerful enum and trait system.

## Requirements

Create a system that can:

1. **Handle different sensor types** (Indoor, Outdoor, CPU, etc.)
2. **Support multiple temperature units** (Celsius, Fahrenheit, Kelvin)
3. **Collect and store readings** with timestamps
4. **Analyze temperature data** (averages, min/max, trends)
5. **Handle sensor errors** and invalid readings
6. **Display formatted reports**

### Core Structures

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone)]
enum SensorType {
    Indoor,
    Outdoor,
    CPU,
    GPU,
    Custom(String),
}

#[derive(Debug, Clone)]
struct Temperature {
    value: f64,
    unit: TemperatureUnit,
}

#[derive(Debug)]
enum SensorError {
    OutOfRange,
    ConnectionLost,
    InvalidReading,
}

#[derive(Debug, Clone)]
struct SensorReading {
    sensor_id: String,
    sensor_type: SensorType,
    temperature: Temperature,
    timestamp: u64, // Unix timestamp
}
```

### Required Traits and Methods

1. **TemperatureConversion trait:**
   ```rust
   trait TemperatureConversion {
       fn to_celsius(&self) -> Temperature;
       fn to_fahrenheit(&self) -> Temperature;
       fn to_kelvin(&self) -> Temperature;
   }
   ```

2. **SensorNetwork struct** with methods:
   - `new() -> Self`
   - `add_reading(reading: SensorReading) -> Result<(), SensorError>`
   - `get_readings_by_sensor(&self, sensor_id: &str) -> Vec<&SensorReading>`
   - `calculate_average(&self, sensor_id: &str) -> Option<Temperature>`
   - `find_extreme_readings(&self) -> (Option<&SensorReading>, Option<&SensorReading>)` (min, max)
   - `get_sensor_status(&self, sensor_id: &str) -> SensorStatus`

3. **SensorStatus enum:**
   ```rust
   enum SensorStatus {
       Active,
       Inactive,
       Error(SensorError),
       Unknown,
   }
   ```

### Example Usage
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut network = SensorNetwork::new();
    
    // Add some readings
    let reading1 = SensorReading {
        sensor_id: "sensor_01".to_string(),
        sensor_type: SensorType::Indoor,
        temperature: Temperature { value: 22.5, unit: TemperatureUnit::Celsius },
        timestamp: 1640995200,
    };
    
    network.add_reading(reading1)?;
    
    // Convert temperature
    let temp_f = reading1.temperature.to_fahrenheit();
    println!("Temperature: {:.1}°F", temp_f.value);
    
    // Get average
    if let Some(avg) = network.calculate_average("sensor_01") {
        println!("Average temperature: {:.1}°{:?}", avg.value, avg.unit);
    }
    
    Ok(())
}
```

## Learning Objectives

- Master enum variants and pattern matching
- Implement and use traits effectively
- Handle complex error types with custom enums
- Use `Option` and `Result` for safe programming
- Work with collections and data analysis
- Practice the newtype pattern and type safety

## Hints

<details>
<summary>Click to see hints</summary>

1. **Temperature conversion formulas:**
   - C to F: `(C × 9/5) + 32`
   - F to C: `(F - 32) × 5/9`
   - C to K: `C + 273.15`

2. **Pattern matching example:**
   ```rust
   match sensor_type {
       SensorType::Indoor => "Indoor sensor",
       SensorType::Custom(name) => &name,
       _ => "Unknown sensor",
   }
   ```

3. **Error handling:** Use `Result<T, E>` and implement `std::error::Error` for custom errors

4. **Collections:** Use `HashMap<String, Vec<SensorReading>>` to group by sensor ID

</details>

## Bonus Challenges

1. **Time-based analysis:** Add methods to filter readings by time range
2. **Alert system:** Implement threshold-based alerts for extreme temperatures
3. **Data persistence:** Save/load sensor data to/from JSON files
4. **Statistics:** Add standard deviation, median calculations
5. **Display trait:** Implement pretty-printing for temperature reports
6. **Async sensors:** Simulate real sensors with async data generation

## Testing Requirements

Include tests for:
- Temperature conversions accuracy
- Error handling for invalid readings
- Data analysis correctness
- Edge cases (empty data, single readings)

## Next Steps

After completing this challenge, move on to `challenge_04_generic_cache` to practice generics and lifetimes!

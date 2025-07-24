// Challenge 3: Temperature Sensor Network
// Your task: implement a temperature monitoring system using enums, traits, and pattern matching

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

#[derive(Debug)]
enum SensorStatus {
    Active,
    Inactive,
    Error(SensorError),
    Unknown,
}

// TODO: Implement TemperatureConversion trait
trait TemperatureConversion {
    fn to_celsius(&self) -> Temperature;
    fn to_fahrenheit(&self) -> Temperature;
    fn to_kelvin(&self) -> Temperature;
}

// TODO: Implement TemperatureConversion for Temperature
impl TemperatureConversion for Temperature {
    fn to_celsius(&self) -> Temperature {
        todo!("Convert temperature to Celsius")
    }

    fn to_fahrenheit(&self) -> Temperature {
        todo!("Convert temperature to Fahrenheit")
    }

    fn to_kelvin(&self) -> Temperature {
        todo!("Convert temperature to Kelvin")
    }
}

struct SensorNetwork {
    readings: HashMap<String, Vec<SensorReading>>,
}

impl SensorNetwork {
    // TODO: Implement new method
    fn new() -> Self {
        todo!("Create a new sensor network")
    }

    // TODO: Implement add_reading method
    fn add_reading(&mut self, reading: SensorReading) -> Result<(), SensorError> {
        todo!("Add a reading, validate it first (check for reasonable temperature ranges)")
    }

    // TODO: Implement get_readings_by_sensor method
    fn get_readings_by_sensor(&self, sensor_id: &str) -> Vec<&SensorReading> {
        todo!("Get all readings for a specific sensor")
    }

    // TODO: Implement calculate_average method
    fn calculate_average(&self, sensor_id: &str) -> Option<Temperature> {
        todo!("Calculate average temperature for a sensor (convert all to Celsius first)")
    }

    // TODO: Implement find_extreme_readings method
    fn find_extreme_readings(&self) -> (Option<&SensorReading>, Option<&SensorReading>) {
        todo!("Find the minimum and maximum temperature readings across all sensors")
    }

    // TODO: Implement get_sensor_status method
    fn get_sensor_status(&self, sensor_id: &str) -> SensorStatus {
        todo!("Determine sensor status based on recent readings")
    }

    // TODO: Implement helper method to validate temperature range
    fn is_valid_temperature(&self, temp: &Temperature, sensor_type: &SensorType) -> bool {
        todo!("Validate temperature is within reasonable range for sensor type")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut network = SensorNetwork::new();
    
    // Create some test readings
    let reading1 = SensorReading {
        sensor_id: "living_room".to_string(),
        sensor_type: SensorType::Indoor,
        temperature: Temperature { value: 22.5, unit: TemperatureUnit::Celsius },
        timestamp: 1640995200,
    };
    
    let reading2 = SensorReading {
        sensor_id: "backyard".to_string(),
        sensor_type: SensorType::Outdoor,
        temperature: Temperature { value: 75.0, unit: TemperatureUnit::Fahrenheit },
        timestamp: 1640995260,
    };
    
    let reading3 = SensorReading {
        sensor_id: "cpu_temp".to_string(),
        sensor_type: SensorType::CPU,
        temperature: Temperature { value: 340.0, unit: TemperatureUnit::Kelvin },
        timestamp: 1640995320,
    };
    
    // Add readings
    network.add_reading(reading1.clone())?;
    network.add_reading(reading2)?;
    network.add_reading(reading3)?;
    
    // Demonstrate temperature conversion
    let temp_f = reading1.temperature.to_fahrenheit();
    println!("Living room: {:.1}°F", temp_f.value);
    
    // Get average for a sensor
    if let Some(avg) = network.calculate_average("living_room") {
        println!("Average living room temperature: {:.1}°C", avg.value);
    }
    
    // Find extreme readings
    let (min, max) = network.find_extreme_readings();
    if let (Some(min_reading), Some(max_reading)) = (min, max) {
        let min_celsius = min_reading.temperature.to_celsius();
        let max_celsius = max_reading.temperature.to_celsius();
        println!("Temperature range: {:.1}°C to {:.1}°C", min_celsius.value, max_celsius.value);
    }
    
    // Check sensor statuses
    for sensor_id in &["living_room", "backyard", "cpu_temp", "unknown_sensor"] {
        let status = network.get_sensor_status(sensor_id);
        println!("Sensor {}: {:?}", sensor_id, status);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversion() {
        let temp_c = Temperature { value: 0.0, unit: TemperatureUnit::Celsius };
        let temp_f = temp_c.to_fahrenheit();
        assert!((temp_f.value - 32.0).abs() < 0.1);
        
        let temp_k = temp_c.to_kelvin();
        assert!((temp_k.value - 273.15).abs() < 0.1);
    }

    #[test]
    fn test_add_reading() {
        let mut network = SensorNetwork::new();
        let reading = SensorReading {
            sensor_id: "test".to_string(),
            sensor_type: SensorType::Indoor,
            temperature: Temperature { value: 20.0, unit: TemperatureUnit::Celsius },
            timestamp: 1640995200,
        };
        
        let result = network.add_reading(reading);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_temperature() {
        let mut network = SensorNetwork::new();
        let reading = SensorReading {
            sensor_id: "test".to_string(),
            sensor_type: SensorType::Indoor,
            temperature: Temperature { value: 1000.0, unit: TemperatureUnit::Celsius },
            timestamp: 1640995200,
        };
        
        let result = network.add_reading(reading);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_average() {
        let mut network = SensorNetwork::new();
        
        // Add multiple readings for same sensor
        let readings = vec![
            SensorReading {
                sensor_id: "test".to_string(),
                sensor_type: SensorType::Indoor,
                temperature: Temperature { value: 20.0, unit: TemperatureUnit::Celsius },
                timestamp: 1640995200,
            },
            SensorReading {
                sensor_id: "test".to_string(),
                sensor_type: SensorType::Indoor,
                temperature: Temperature { value: 30.0, unit: TemperatureUnit::Celsius },
                timestamp: 1640995260,
            },
        ];
        
        for reading in readings {
            network.add_reading(reading).unwrap();
        }
        
        let avg = network.calculate_average("test");
        assert!(avg.is_some());
        let avg = avg.unwrap();
        assert!((avg.value - 25.0).abs() < 0.1);
    }
}

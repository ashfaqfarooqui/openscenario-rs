//! Basic usage example for the OpenSCENARIO-rs builder pattern
//!
//! This example demonstrates how to use the builder pattern to create
//! a simple OpenSCENARIO scenario programmatically with type safety
//! and guided construction.

use openscenario_rs::builder::{ScenarioBuilder, BuilderResult};

fn main() -> BuilderResult<()> {
    // Create a basic scenario using the type-safe builder pattern
    let scenario = ScenarioBuilder::new()
        // Step 1: Set file header (required first step)
        .with_simple_header("Highway Test Scenario", "Test Engineer")
        
        // Step 2: Configure catalog locations (can use defaults)
        .with_default_catalogs()
        
        // Step 3: Set road network (required for entity positioning)
        .with_road_network("roads/highway.xodr")
        
        // Step 4: Initialize entities section
        .with_entities()
        
        // Step 5: Build the final scenario (validates all requirements)
        .build()?;

    println!("✅ Successfully created scenario: {}", scenario.file_header.name);
    println!("📅 Date: {}", scenario.file_header.date);
    println!("👤 Author: {}", scenario.file_header.author);
    
    // The scenario can now be serialized to XML or used with the validation system
    Ok(())
}

/// Example showing parameter usage
fn example_with_parameters() -> BuilderResult<()> {
    use openscenario_rs::types::enums::ParameterType;
    
    let scenario = ScenarioBuilder::new()
        .with_header(
            "Parametric Speed Test",
            "1", "0", 
            "2024-01-15T10:00:00",
            "Test scenario with configurable speed parameter",
            "Test Engineer"
        )
        
        // Add parameters for configurability
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_parameter("target_speed", ParameterType::Double, "30.0")
        .add_parameter("weather_condition", ParameterType::String, "clear")
        
        .with_default_catalogs()
        .with_road_network("roads/highway.xodr")
        .with_entities()
        .build()?;

    println!("✅ Created parametric scenario with {} parameters", 
        scenario.parameter_declarations
            .map(|p| p.parameter_declarations.len())
            .unwrap_or(0)
    );
    
    Ok(())
}

/// Example showing advanced road network configuration
fn example_with_scene_graph() -> BuilderResult<()> {
    let scenario = ScenarioBuilder::new()
        .with_simple_header("3D Visualization Test", "Graphics Team")
        .with_default_catalogs()
        
        // Configure road network with 3D scene graph for visualization
        .with_road_network_and_scene(
            "roads/city_intersection.xodr",
            Some("scenes/city_intersection.osgb")
        )
        
        .with_entities()
        .build()?;

    println!("✅ Created scenario with 3D scene graph support");
    Ok(())
}

/// Example showing error handling and validation
fn example_error_handling() {
    // This will fail because we haven't set required elements
    let result = ScenarioBuilder::new().build();
    
    match result {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => {
            println!("❌ Expected error occurred: {}", e);
            // The error message will be helpful:
            // "Required field missing: file_header. Call .with_header() first"
        }
    }
    
    // This will also fail because we try to build before setting entities
    let result = ScenarioBuilder::new()
        .with_simple_header("Incomplete", "Author")
        .build();
        
    match result {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => {
            println!("❌ Another expected error: {}", e);
            // Will suggest which methods to call next
        }
    }
}

/// Demonstrates the type state system preventing invalid operations
fn example_type_safety() {
    let builder = ScenarioBuilder::new();
    
    // This won't compile - can't add entities before setting header:
    // builder.with_entities(); // ❌ Compile error!
    
    let builder_with_header = builder.with_simple_header("Test", "Author");
    
    // This won't compile - can't add entities before setting road network:
    // builder_with_header.with_entities(); // ❌ Compile error!
    
    let builder_with_catalogs = builder_with_header.with_default_catalogs();
    let builder_with_road = builder_with_catalogs.with_road_network("test.xodr");
    
    // Now this is allowed:
    let _builder_with_entities = builder_with_road.with_entities();
    
    println!("✅ Type safety ensures proper construction order");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scenario_creation() {
        let result = ScenarioBuilder::new()
            .with_simple_header("Test Scenario", "Test Author")
            .with_default_catalogs()
            .with_road_network("test.xodr")
            .with_entities()
            .build();

        assert!(result.is_ok());
        let scenario = result.unwrap();
        assert_eq!(scenario.file_header.name, "Test Scenario");
        assert_eq!(scenario.file_header.author, "Test Author");
    }

    #[test]
    fn test_parameter_scenario() {
        let result = example_with_parameters();
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_conditions() {
        // Test missing header
        let result = ScenarioBuilder::new().build();
        assert!(result.is_err());
        
        // Test missing road network
        let result = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .build();
        assert!(result.is_err());
    }
}
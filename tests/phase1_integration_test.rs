//! Integration test for Phase 1 builder implementation

use openscenario_rs::builder::ScenarioBuilder;

#[test]
fn test_phase1_basic_scenario_construction() {
    let scenario = ScenarioBuilder::new()
        .with_simple_header("Test Scenario", "Test Author")
        .with_default_catalogs()
        .expect("Should set default catalogs")
        .with_road_network("test.xodr")
        .with_entities()
        .build()
        .expect("Should build valid scenario");

    // Verify structure
    assert!(scenario.file_header.description.as_literal().unwrap().contains("Test"));
    assert!(scenario.catalog_locations.is_some());
    assert!(scenario.road_network.is_some());
    assert!(scenario.entities.is_some());
}

#[test]
fn test_phase1_state_transitions() {
    // Test that state transitions work correctly
    let builder = ScenarioBuilder::new();
    
    // Empty -> HasHeader
    let builder = builder.with_simple_header("Test", "Author");
    
    // HasHeader -> HasCatalogLocations
    let builder = builder.with_default_catalogs().unwrap();
    
    // HasCatalogLocations -> HasRoadNetwork
    let builder = builder.with_road_network("test.xodr");
    
    // HasRoadNetwork -> HasEntities
    let builder = builder.with_entities();
    
    // HasEntities -> Complete (via build)
    let scenario = builder.build().unwrap();
    
    assert!(scenario.is_scenario());
}

#[test]
fn test_phase1_error_handling() {
    // Test strict validation
    let builder = ScenarioBuilder::new()
        .with_simple_header("Test", "Author")
        .with_default_catalogs().unwrap()
        .with_road_network("test.xodr")
        .with_entities();
    
    // Should fail strict validation with no entities
    let result = builder.build_validated();
    assert!(result.is_err());
    
    // Should succeed with normal build
    let builder = ScenarioBuilder::new()
        .with_simple_header("Test", "Author")
        .with_default_catalogs().unwrap()
        .with_road_network("test.xodr")
        .with_entities();
    
    let result = builder.build();
    assert!(result.is_ok());
}
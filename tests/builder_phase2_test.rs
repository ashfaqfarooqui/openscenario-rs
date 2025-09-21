//! Tests for Phase 2 builder functionality
//!
//! This test file verifies that the CatalogBuilder and ParameterVariationBuilder
//! work correctly and produce valid OpenSCENARIO documents.

use openscenario_rs::builder::{CatalogBuilder, ParameterVariationBuilder, BuilderResult};
use openscenario_rs::types::enums::{VehicleCategory, ControllerType, PedestrianCategory};

#[test]
fn test_catalog_builder_basic() -> BuilderResult<()> {
    let catalog = CatalogBuilder::new("TestCatalog")
        .with_simple_header("Test Catalog", "Test Author")
        .add_vehicle("test_car")
            .with_category(VehicleCategory::Car)
            .with_dimensions(4.5, 1.8, 1.4)
            .finish_vehicle()?
        .build()?;

    assert!(catalog.is_catalog());
    assert_eq!(catalog.file_header.author.as_literal(), Some(&"Test Author".to_string()));
    
    if let Some(catalog_def) = &catalog.catalog {
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.vehicles[0].name, "test_car");
    } else {
        panic!("Catalog definition should be present");
    }

    Ok(())
}

#[test]
fn test_catalog_builder_multiple_entities() -> BuilderResult<()> {
    let catalog = CatalogBuilder::new("MixedCatalog")
        .with_simple_header("Mixed Catalog", "Test Author")
        .add_vehicle("car")
            .with_category(VehicleCategory::Car)
            .finish_vehicle()?
        .add_controller("ai_driver")
            .with_type(ControllerType::Movement)
            .finish_controller()?
        .add_pedestrian("walker")
            .with_category(PedestrianCategory::Pedestrian)
            .with_dimensions(0.6, 0.6, 1.8)
            .finish_pedestrian()?
        .add_misc_object("cone", 0.3, 0.3, 0.7)
        .add_environment("sunny")
        .build()?;

    if let Some(catalog_def) = &catalog.catalog {
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.controllers.len(), 1);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
        assert_eq!(catalog_def.catalog.misc_objects.len(), 1);
        assert_eq!(catalog_def.catalog.environments.len(), 1);
        assert_eq!(catalog_def.catalog.entity_count(), 5);
    } else {
        panic!("Catalog definition should be present");
    }

    Ok(())
}

#[test]
fn test_parameter_variation_builder_deterministic() -> BuilderResult<()> {
    let param_variation = ParameterVariationBuilder::new("test_scenario.xosc")
        .with_simple_header("Speed Study", "Test Engineer")
        .with_deterministic_distribution()
            .single_parameter("speed")
                .distribution_range(20.0, 40.0, 5.0)
                .finish_parameter()?
            .finish_distribution()?
        .build()?;

    assert!(param_variation.is_parameter_variation());
    assert_eq!(param_variation.file_header.author.as_literal(), Some(&"Test Engineer".to_string()));
    
    if let Some(dist) = &param_variation.parameter_value_distribution {
        assert!(dist.deterministic.is_some());
        assert!(dist.stochastic.is_none());
        
        let deterministic = dist.deterministic.as_ref().unwrap();
        assert_eq!(deterministic.single_distributions.len(), 1);
        assert_eq!(deterministic.multi_distributions.len(), 0);
    } else {
        panic!("Parameter value distribution should be present");
    }

    Ok(())
}

#[test]
fn test_parameter_variation_builder_multi_parameter() -> BuilderResult<()> {
    let param_variation = ParameterVariationBuilder::new("complex.xosc")
        .with_simple_header("Complex Study", "Test Engineer")
        .with_deterministic_distribution()
            .multi_parameter()
                .add_parameter_set()
                    .assign("speed", "30.0")
                    .assign("weather", "dry")
                    .finish_set()?
                .add_parameter_set()
                    .assign("speed", "25.0")
                    .assign("weather", "wet")
                    .finish_set()?
                .finish_multi_parameter()?
            .finish_distribution()?
        .build()?;

    if let Some(dist) = &param_variation.parameter_value_distribution {
        let deterministic = dist.deterministic.as_ref().unwrap();
        assert_eq!(deterministic.single_distributions.len(), 0);
        assert_eq!(deterministic.multi_distributions.len(), 1);
        
        let multi_dist = &deterministic.multi_distributions[0];
        match &multi_dist.distribution_type {
            openscenario_rs::types::distributions::deterministic::DeterministicMultiParameterDistributionType::ValueSetDistribution(vsd) => {
                assert_eq!(vsd.parameter_value_sets.len(), 2);
            }
        }
    } else {
        panic!("Parameter value distribution should be present");
    }

    Ok(())
}

#[test]
fn test_parameter_variation_builder_stochastic() -> BuilderResult<()> {
    let param_variation = ParameterVariationBuilder::new("stochastic.xosc")
        .with_simple_header("Stochastic Study", "Test Engineer")
        .with_stochastic_distribution()
            .normal_distribution("speed", 30.0, 5.0)
            .finish_distribution()?
        .build()?;

    if let Some(dist) = &param_variation.parameter_value_distribution {
        assert!(dist.deterministic.is_none());
        assert!(dist.stochastic.is_some());
        
        let stochastic = dist.stochastic.as_ref().unwrap();
        assert_eq!(stochastic.distributions.len(), 1);
    } else {
        panic!("Parameter value distribution should be present");
    }

    Ok(())
}

#[test]
fn test_catalog_builder_validation_errors() {
    // Test building without entities
    let result = CatalogBuilder::new("EmptyCatalog")
        .with_simple_header("Empty", "Author")
        .build();
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("catalog must have entities"));
}

#[test]
fn test_parameter_variation_builder_validation_errors() {
    // Test building without distribution
    let result = ParameterVariationBuilder::new("test.xosc")
        .with_simple_header("Empty", "Author")
        .build();
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("parameter_value_distribution"));
}

#[test]
fn test_catalog_builder_vehicle_configurations() -> BuilderResult<()> {
    let catalog = CatalogBuilder::new("VehicleCatalog")
        .with_simple_header("Vehicle Test", "Test Author")
        .add_vehicle("sedan")
            .with_category(VehicleCategory::Car)
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(200.0, 8.0, 10.0)
            .with_standard_axles()
            .finish_vehicle()?
        .add_vehicle("truck")
            .with_category(VehicleCategory::Truck)
            .with_dimensions(8.0, 2.5, 3.0)
            .with_performance(120.0, 4.0, 6.0)
            .with_custom_axles(0.4, 0.8, 2.0, 4.0)
            .finish_vehicle()?
        .build()?;

    if let Some(catalog_def) = &catalog.catalog {
        assert_eq!(catalog_def.catalog.vehicles.len(), 2);
        
        let sedan = &catalog_def.catalog.vehicles[0];
        assert_eq!(sedan.name, "sedan");
        assert_eq!(sedan.performance.max_speed.as_literal(), Some(&200.0));
        
        let truck = &catalog_def.catalog.vehicles[1];
        assert_eq!(truck.name, "truck");
        assert_eq!(truck.axles.front_axle.max_steering.as_literal(), Some(&0.4));
    }

    Ok(())
}
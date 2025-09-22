//! Phase 2 integration tests for document type builders
//!
//! This module tests the complete functionality of catalog and parameter variation builders
//! as specified in the Phase 2 requirements.

use openscenario_rs::builder::{CatalogBuilder, ParameterVariationBuilder};

#[cfg(test)]
mod phase2_integration_tests {
    use super::*;

    #[test]
    fn test_catalog_document_construction() {
        let catalog = CatalogBuilder::new("TestCatalog")
            .with_simple_header("Test Catalog", "Developer")
            .add_vehicle("test_car")
                .with_model("TestModel")
                .finish_vehicle()
            .build()
            .expect("Should build valid catalog");

        assert!(catalog.is_catalog());
        assert!(catalog.catalog.is_some());
        
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.vehicles[0].name, "test_car");
    }

    #[test]
    fn test_parameter_variation_construction() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Test Variations", "Developer")
            .add_deterministic_distribution("test_param")
                .with_values(vec!["1.0", "2.0", "3.0"])
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        assert!(param_var.parameter_value_distribution.is_some());
        
        let param_dist = param_var.parameter_value_distribution.unwrap();
        assert!(param_dist.deterministic.is_some());
        
        let deterministic = param_dist.deterministic.unwrap();
        assert_eq!(deterministic.single_distributions.len(), 1);
        assert_eq!(deterministic.single_distributions[0].parameter_name.as_literal().unwrap(), "test_param");
    }

    #[test]
    fn test_catalog_with_multiple_entity_types() {
        let catalog = CatalogBuilder::new("MultiEntityCatalog")
            .with_simple_header("Multi Entity Test", "Developer")
            .add_vehicle("car1")
                .with_model("Sedan")
                .with_dimensions(4.5, 1.8, 1.4)
                .finish_vehicle()
            .add_pedestrian("ped1")
                .with_model("Adult")
                .finish_pedestrian()
            .add_controller("ctrl1")
                .with_type("movement")
                .finish_controller()
            .build()
            .expect("Should build valid multi-entity catalog");

        assert!(catalog.is_catalog());
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
        assert_eq!(catalog_def.catalog.controllers.len(), 1);
    }

    #[test]
    fn test_parameter_variation_with_multiple_distributions() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Multi Distribution Test", "Developer")
            .add_deterministic_distribution("speed")
                .with_range(30.0, 60.0, 10.0)
                .finish()
            .add_deterministic_distribution("distance")
                .with_values(vec!["100", "200", "300"])
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        let deterministic = param_dist.deterministic.unwrap();
        assert_eq!(deterministic.single_distributions.len(), 2);
    }

    #[test]
    fn test_stochastic_parameter_variation() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Stochastic Test", "Developer")
            .add_stochastic_distribution("weather_temp")
                .normal_distribution(25.0, 5.0)
                .finish()
            .build()
            .expect("Should build valid stochastic parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        assert!(param_dist.stochastic.is_some());
        assert!(param_dist.deterministic.is_none());
    }

    #[test]
    fn test_catalog_validation_empty_fails() {
        let result = CatalogBuilder::new("EmptyCatalog")
            .with_simple_header("Empty Test", "Developer")
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("No entities defined"));
    }

    #[test]
    fn test_parameter_variation_validation_empty_fails() {
        let result = ParameterVariationBuilder::new()
            .with_simple_header("Empty Test", "Developer")
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("No distributions defined"));
    }

    #[test]
    fn test_catalog_vehicle_configuration() {
        let catalog = CatalogBuilder::new("VehicleTestCatalog")
            .with_simple_header("Vehicle Configuration Test", "Developer")
            .add_vehicle("sports_car")
                .with_model("SportsCar")
                .with_dimensions(4.2, 1.9, 1.3)
                .with_performance(250.0, 10.0, 12.0)
                .finish_vehicle()
            .build()
            .expect("Should build valid catalog");

        let catalog_def = catalog.catalog.unwrap();
        let vehicle = &catalog_def.catalog.vehicles[0];
        
        assert_eq!(vehicle.name, "sports_car");
        assert_eq!(vehicle.bounding_box.dimensions.length.as_literal().unwrap(), &4.2);
        assert_eq!(vehicle.bounding_box.dimensions.width.as_literal().unwrap(), &1.9);
        assert_eq!(vehicle.bounding_box.dimensions.height.as_literal().unwrap(), &1.3);
        assert_eq!(vehicle.performance.max_speed.as_literal().unwrap(), &250.0);
    }

    #[test]
    fn test_parameter_variation_range_distribution() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Range Test", "Developer")
            .add_deterministic_distribution("initial_speed")
                .with_range(20.0, 60.0, 5.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        let param_dist = param_var.parameter_value_distribution.unwrap();
        let deterministic = param_dist.deterministic.unwrap();
        let single_dist = &deterministic.single_distributions[0];
        
        assert!(single_dist.distribution_range.is_some());
        let range = single_dist.distribution_range.as_ref().unwrap();
        assert_eq!(range.step_width.as_literal().unwrap(), &5.0);
        assert_eq!(range.range.lower_limit.as_literal().unwrap(), &20.0);
        assert_eq!(range.range.upper_limit.as_literal().unwrap(), &60.0);
    }

    #[test]
    fn test_parameter_variation_uniform_distribution() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Uniform Test", "Developer")
            .add_stochastic_distribution("random_factor")
                .uniform_distribution(0.0, 1.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        let param_dist = param_var.parameter_value_distribution.unwrap();
        let stochastic = param_dist.stochastic.unwrap();
        assert_eq!(stochastic.distributions.len(), 1);
        
        let dist = &stochastic.distributions[0];
        assert_eq!(dist.parameter_name.as_literal().unwrap(), "random_factor");
    }

    #[test]
    fn test_document_type_detection() {
        // Test scenario document detection (from Phase 1)
        // This would require a scenario builder, but we're focusing on Phase 2 documents
        
        // Test catalog document detection
        let catalog = CatalogBuilder::new("TestCatalog")
            .with_simple_header("Test", "Developer")
            .add_vehicle("test")
                .finish_vehicle()
            .build()
            .expect("Should build valid catalog");
        
        assert!(catalog.is_catalog());
        assert!(!catalog.is_scenario());
        assert!(!catalog.is_parameter_variation());

        // Test parameter variation document detection
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Test", "Developer")
            .add_deterministic_distribution("test")
                .with_values(vec!["1"])
                .finish()
            .build()
            .expect("Should build valid parameter variation");
        
        assert!(param_var.is_parameter_variation());
        assert!(!param_var.is_scenario());
        assert!(!param_var.is_catalog());
    }

    #[test]
    fn test_builder_fluent_api_chaining() {
        // Test that the fluent API allows for natural chaining
        let catalog = CatalogBuilder::new("FluentTestCatalog")
            .with_simple_header("Fluent API Test", "Developer")
            .add_vehicle("vehicle1")
                .with_model("Model1")
                .with_dimensions(4.0, 1.8, 1.5)
                .finish_vehicle()
            .add_vehicle("vehicle2")
                .with_model("Model2")
                .with_performance(180.0, 7.0, 9.0)
                .finish_vehicle()
            .add_pedestrian("pedestrian1")
                .with_model("Adult")
                .finish_pedestrian()
            .build()
            .expect("Should build valid catalog with fluent API");

        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 2);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
    }

    #[test]
    fn test_parameter_variation_scenario_file_setting() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Scenario File Test", "Developer")
            .with_scenario_file("custom_scenario.xosc")
            .add_deterministic_distribution("test_param")
                .with_values(vec!["1.0"])
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        let param_dist = param_var.parameter_value_distribution.unwrap();
        assert_eq!(param_dist.scenario_file.filepath, "custom_scenario.xosc");
    }
}
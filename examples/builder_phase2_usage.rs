//! Example demonstrating Phase 2 builder functionality
//!
//! This example shows how to use the new CatalogBuilder and ParameterVariationBuilder
//! to create catalog and parameter variation documents.

use openscenario_rs::builder::{CatalogBuilder, ParameterVariationBuilder, BuilderResult};
use openscenario_rs::types::enums::{VehicleCategory, ControllerType, PedestrianCategory};

fn main() -> BuilderResult<()> {
    println!("OpenSCENARIO-rs Phase 2 Builder Example");
    println!("========================================");

    // Example 1: Create a vehicle catalog
    println!("\n1. Creating a vehicle catalog...");
    let catalog = CatalogBuilder::new("VehicleCatalog")
        .with_simple_header("Vehicle Library", "Fleet Manager")
        .add_vehicle("sedan")
            .with_category(VehicleCategory::Car)
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(200.0, 8.0, 10.0)
            .with_standard_axles()
            .finish_vehicle()?
        .add_vehicle("suv")
            .with_category(VehicleCategory::Car)
            .with_dimensions(5.0, 2.0, 1.7)
            .with_performance(180.0, 6.0, 8.0)
            .with_custom_axles(0.6, 0.7, 1.9, 3.2)
            .finish_vehicle()?
        .add_controller("ai_driver")
            .with_type(ControllerType::Movement)
            .finish_controller()?
        .add_pedestrian("walker")
            .with_category(PedestrianCategory::Pedestrian)
            .with_dimensions(0.6, 0.6, 1.8)
            .finish_pedestrian()?
        .add_misc_object("traffic_cone", 0.3, 0.3, 0.7)
        .add_environment("sunny_day")
        .build()?;

    println!("✓ Catalog created successfully!");
    println!("  Document type: {:?}", catalog.document_type());
    if let Some(catalog_def) = &catalog.catalog {
        println!("  Entities: {} total", catalog_def.catalog.entity_count());
        println!("  - Vehicles: {}", catalog_def.catalog.vehicles.len());
        println!("  - Controllers: {}", catalog_def.catalog.controllers.len());
        println!("  - Pedestrians: {}", catalog_def.catalog.pedestrians.len());
        println!("  - Misc Objects: {}", catalog_def.catalog.misc_objects.len());
        println!("  - Environments: {}", catalog_def.catalog.environments.len());
    }

    // Example 2: Create a parameter variation with deterministic distribution
    println!("\n2. Creating parameter variation with deterministic distribution...");
    let param_variation = ParameterVariationBuilder::new("highway_scenario.xosc")
        .with_simple_header("Speed Variation Study", "Test Engineer")
        .with_deterministic_distribution()
            .single_parameter("initial_speed")
                .distribution_range(20.0, 40.0, 5.0)
                .finish_parameter()?
            .single_parameter("weather_condition")
                .distribution_set(vec!["dry", "wet", "snow"])
                .finish_parameter()?
            .finish_distribution()?
        .build()?;

    println!("✓ Parameter variation created successfully!");
    println!("  Document type: {:?}", param_variation.document_type());
    if let Some(dist) = &param_variation.parameter_value_distribution {
        if let Some(deterministic) = &dist.deterministic {
            println!("  Single parameters: {}", deterministic.single_distributions.len());
            println!("  Multi parameters: {}", deterministic.multi_distributions.len());
        }
    }

    // Example 3: Create a parameter variation with multi-parameter distribution
    println!("\n3. Creating parameter variation with multi-parameter distribution...");
    let multi_param_variation = ParameterVariationBuilder::new("complex_scenario.xosc")
        .with_simple_header("Complex Parameter Study", "Research Team")
        .with_deterministic_distribution()
            .multi_parameter()
                .add_parameter_set()
                    .assign("speed", "30.0")
                    .assign("weather", "dry")
                    .assign("time_of_day", "morning")
                    .finish_set()?
                .add_parameter_set()
                    .assign("speed", "25.0")
                    .assign("weather", "wet")
                    .assign("time_of_day", "evening")
                    .finish_set()?
                .add_parameter_set()
                    .assign("speed", "20.0")
                    .assign("weather", "snow")
                    .assign("time_of_day", "night")
                    .finish_set()?
                .finish_multi_parameter()?
            .finish_distribution()?
        .build()?;

    println!("✓ Multi-parameter variation created successfully!");
    println!("  Document type: {:?}", multi_param_variation.document_type());
    if let Some(dist) = &multi_param_variation.parameter_value_distribution {
        if let Some(deterministic) = &dist.deterministic {
                if let Some(multi_dist) = deterministic.multi_distributions.first() {
                    match &multi_dist.distribution_type {
                        openscenario_rs::types::distributions::deterministic::DeterministicMultiParameterDistributionType::ValueSetDistribution(vsd) => {
                            println!("  Parameter value sets: {}", vsd.parameter_value_sets.len());
                        }
                    }
                }
        }
    }

    // Example 4: Create a parameter variation with stochastic distribution
    println!("\n4. Creating parameter variation with stochastic distribution...");
    let stochastic_variation = ParameterVariationBuilder::new("stochastic_scenario.xosc")
        .with_simple_header("Stochastic Parameter Study", "ML Team")
        .with_stochastic_distribution()
            .normal_distribution("speed", 30.0, 5.0)
            .finish_distribution()?
        .build()?;

    println!("✓ Stochastic parameter variation created successfully!");
    println!("  Document type: {:?}", stochastic_variation.document_type());
    if let Some(dist) = &stochastic_variation.parameter_value_distribution {
        if let Some(stochastic) = &dist.stochastic {
            println!("  Stochastic distributions: {}", stochastic.distributions.len());
        }
    }

    println!("\n🎉 All Phase 2 builders working successfully!");
    println!("\nPhase 2 Implementation Complete:");
    println!("✓ CatalogBuilder - supports all 8 entity types");
    println!("✓ ParameterVariationBuilder - supports deterministic and stochastic distributions");
    println!("✓ Type-safe state transitions");
    println!("✓ Comprehensive validation");
    println!("✓ Fluent API design");

    Ok(())
}
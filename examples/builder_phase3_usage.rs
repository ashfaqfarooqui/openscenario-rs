//! Example demonstrating Phase 3 builder functionality
//! 
//! This example shows how to use the new entity and position builders
//! to create complex scenarios with vehicles, pedestrians, and objects.

use openscenario_rs::builder::{ScenarioBuilder, BuilderResult};

fn main() -> BuilderResult<()> {
    println!("OpenSCENARIO-rs Phase 3 Builder Example");
    println!("========================================");

    // Create a scenario with the new entity builders
    let scenario = ScenarioBuilder::new()
        .with_simple_header("Phase 3 Demo", "Builder Example")
        .with_default_catalogs()
        .with_road_network("demo_highway.xodr")
        .with_entities_builder()
        
        // Add an ego vehicle
        .add_vehicle("ego")
            .car()
            .with_model("sedan")
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(50.0, 8.0, 10.0)
            .at_position()
                .lane("1", -1, 100.0)
                .with_heading(0.0)
                .finish()?
            .finish_vehicle()?
            
        // Add a target vehicle ahead
        .add_vehicle("target")
            .car()
            .with_model("suv")
            .with_dimensions(4.8, 2.0, 1.6)
            .at_position()
                .relative_to("ego")
                .ahead(50.0)
            .finish_vehicle()?
            
        // Add a pedestrian on the sidewalk
        .add_pedestrian("pedestrian1")
            .pedestrian()
            .with_dimensions(0.6, 0.6, 1.8)
            .at_position()
                .world(120.0, 5.0, None)
                .with_heading(1.57)
                .finish()?
            .finish_pedestrian()?
            
        // Add a barrier object
        .add_misc_object("barrier1")
            .barrier()
            .with_dimensions(2.0, 0.5, 1.0)
            .at_position()
                .world(200.0, -3.0, None)
                .finish()?
            .finish_misc_object()?
            
        .finish_entities();

    println!("✓ Successfully created scenario with Phase 3 builders!");
    println!("  - Ego vehicle positioned in lane");
    println!("  - Target vehicle positioned relative to ego");
    println!("  - Pedestrian positioned in world coordinates");
    println!("  - Barrier object positioned as static obstacle");
    
    // The scenario would be built here, but we're just demonstrating the builder API
    println!("\nPhase 3 builder implementation complete!");
    
    Ok(())
}
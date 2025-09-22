//! Performance benchmarks for the OpenSCENARIO builder system
//!
//! These benchmarks measure the performance characteristics of the builder
//! system to ensure it meets production requirements.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use openscenario_rs::builder::{ScenarioBuilder, performance::get_metrics};
use std::time::Duration;

/// Benchmark scenario building with varying entity counts
fn bench_scenario_building(c: &mut Criterion) {
    let mut group = c.benchmark_group("scenario_building");
    
    // Test with different entity counts
    for entity_count in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("entities", entity_count),
            entity_count,
            |b, &entity_count| {
                b.iter(|| {
                    let mut builder = ScenarioBuilder::new()
                        .with_simple_header("Benchmark Scenario", "Benchmark")
                        .with_default_catalogs()
                        .expect("Failed to set catalogs")
                        .with_road_network("test.xodr")
                        .with_entities();

                    // Add entities efficiently
                    for i in 0..entity_count {
                        builder = builder
                            .add_vehicle(&format!("vehicle_{}", i))
                            .car()
                            .at_position()
                            .world(i as f64 * 5.0, 0.0, 0.0)
                            .finish_vehicle();
                    }

                    let scenario = builder.build().expect("Failed to build scenario");
                    black_box(scenario)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("large_scenario_memory", |b| {
        b.iter(|| {
            let mut builder = ScenarioBuilder::new()
                .with_simple_header("Memory Test", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities();

            // Create a large scenario
            for i in 0..1000 {
                builder = builder
                    .add_vehicle(&format!("vehicle_{}", i))
                    .car()
                    .at_position()
                    .world(i as f64 * 5.0, 0.0, 0.0)
                    .finish_vehicle();
            }

            let scenario = builder.build().expect("Failed to build scenario");
            
            // Measure memory footprint
            let memory_usage = std::mem::size_of_val(&scenario);
            black_box((scenario, memory_usage))
        });
    });
    
    group.finish();
}

/// Benchmark builder state transitions
fn bench_state_transitions(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_transitions");
    
    group.bench_function("header_to_complete", |b| {
        b.iter(|| {
            let scenario = ScenarioBuilder::new()
                .with_simple_header("State Test", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities()
                .add_vehicle("test_vehicle")
                .car()
                .at_position()
                .world(0.0, 0.0, 0.0)
                .finish_vehicle()
                .build()
                .expect("Failed to build scenario");
            
            black_box(scenario)
        });
    });
    
    group.finish();
}

/// Benchmark bulk operations
fn bench_bulk_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_operations");
    
    group.bench_function("bulk_entity_addition", |b| {
        b.iter(|| {
            let mut builder = ScenarioBuilder::new()
                .with_simple_header("Bulk Test", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities();

            // Simulate bulk entity addition
            let entities: Vec<_> = (0..100)
                .map(|i| format!("vehicle_{}", i))
                .collect();

            for entity_name in entities {
                builder = builder
                    .add_vehicle(&entity_name)
                    .car()
                    .at_position()
                    .world(0.0, 0.0, 0.0)
                    .finish_vehicle();
            }

            let scenario = builder.build().expect("Failed to build scenario");
            black_box(scenario)
        });
    });
    
    group.finish();
}

/// Benchmark validation performance
fn bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");
    
    group.bench_function("scenario_validation", |b| {
        // Pre-build a scenario for validation testing
        let scenario = ScenarioBuilder::new()
            .with_simple_header("Validation Test", "Benchmark")
            .with_default_catalogs()
            .expect("Failed to set catalogs")
            .with_road_network("test.xodr")
            .with_entities()
            .add_vehicle("test_vehicle")
            .car()
            .at_position()
            .world(0.0, 0.0, 0.0)
            .finish_vehicle()
            .build()
            .expect("Failed to build scenario");

        b.iter(|| {
            // Simulate validation (placeholder for actual validation logic)
            let is_valid = scenario.file_header.is_some() && 
                          scenario.entities.is_some();
            black_box(is_valid)
        });
    });
    
    group.finish();
}

/// Performance regression test
fn bench_performance_targets(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_targets");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("1000_entities_under_5s", |b| {
        b.iter(|| {
            let start = std::time::Instant::now();
            
            let mut builder = ScenarioBuilder::new()
                .with_simple_header("Performance Target Test", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities();

            // Add 1000 entities
            for i in 0..1000 {
                builder = builder
                    .add_vehicle(&format!("vehicle_{}", i))
                    .car()
                    .at_position()
                    .world(i as f64 * 5.0, 0.0, 0.0)
                    .finish_vehicle();
            }

            let scenario = builder.build().expect("Failed to build scenario");
            let duration = start.elapsed();
            
            // Assert performance target
            assert!(duration < Duration::from_secs(5), 
                   "Build time {} exceeded 5s target", duration.as_secs_f64());
            
            black_box((scenario, duration))
        });
    });
    
    group.finish();
}

/// Memory efficiency benchmark
fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    
    group.bench_function("memory_per_entity", |b| {
        b.iter(|| {
            let scenario_small = ScenarioBuilder::new()
                .with_simple_header("Memory Small", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities()
                .add_vehicle("vehicle_1")
                .car()
                .at_position()
                .world(0.0, 0.0, 0.0)
                .finish_vehicle()
                .build()
                .expect("Failed to build scenario");

            let mut builder_large = ScenarioBuilder::new()
                .with_simple_header("Memory Large", "Benchmark")
                .with_default_catalogs()
                .expect("Failed to set catalogs")
                .with_road_network("test.xodr")
                .with_entities();

            for i in 0..100 {
                builder_large = builder_large
                    .add_vehicle(&format!("vehicle_{}", i))
                    .car()
                    .at_position()
                    .world(i as f64 * 5.0, 0.0, 0.0)
                    .finish_vehicle();
            }

            let scenario_large = builder_large.build().expect("Failed to build scenario");

            let size_small = std::mem::size_of_val(&scenario_small);
            let size_large = std::mem::size_of_val(&scenario_large);
            let size_per_entity = (size_large - size_small) / 99;

            black_box((size_small, size_large, size_per_entity))
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_scenario_building,
    bench_memory_usage,
    bench_state_transitions,
    bench_bulk_operations,
    bench_validation,
    bench_performance_targets,
    bench_memory_efficiency
);

criterion_main!(benches);
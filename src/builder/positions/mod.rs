//! Position builders for programmatic position construction
//!
//! This module provides fluent APIs for creating all types of positions in OpenSCENARIO
//! scenarios with comprehensive validation and type safety.

pub mod world;
pub mod relative;
pub mod road;
pub mod lane;

pub use world::WorldPositionBuilder;
pub use relative::RelativePositionBuilder;
pub use road::RoadPositionBuilder;
pub use lane::LanePositionBuilder;

use crate::types::positions::Position;
use crate::builder::{BuilderError, BuilderResult};

/// Trait for position builders that can be finished and converted to Position
pub trait PositionBuilder {
    /// Finish building the position and return it
    fn finish(self) -> BuilderResult<Position>;
    
    /// Validate the position configuration
    fn validate(&self) -> BuilderResult<()>;
}

/// Unified position builder interface for dynamic position type selection
pub struct UnifiedPositionBuilder {
    position_type: PositionType,
}

/// Enum representing different position types
pub enum PositionType {
    World(WorldPositionBuilder),
    Relative(RelativePositionBuilder),
    Road(RoadPositionBuilder),
    Lane(LanePositionBuilder),
}

impl UnifiedPositionBuilder {
    /// Create a new world position builder
    pub fn world() -> WorldPositionBuilder {
        WorldPositionBuilder::new()
    }
    
    /// Create a new relative position builder
    pub fn relative() -> RelativePositionBuilder {
        RelativePositionBuilder::new()
    }
    
    /// Create a new road position builder
    pub fn road() -> RoadPositionBuilder {
        RoadPositionBuilder::new()
    }
    
    /// Create a new lane position builder
    pub fn lane() -> LanePositionBuilder {
        LanePositionBuilder::new()
    }
}

/// Helper function to validate coordinate values
pub(crate) fn validate_coordinate(value: f64, name: &str) -> BuilderResult<()> {
    if value.is_nan() || value.is_infinite() {
        return Err(BuilderError::validation_error(
            &format!("{} coordinate is invalid (NaN or infinite)", name),
            &format!("Provide a valid finite number for {}", name)
        ));
    }
    Ok(())
}

/// Helper function to validate angle values (in radians)
pub(crate) fn validate_angle(value: f64, name: &str) -> BuilderResult<()> {
    if value.is_nan() || value.is_infinite() {
        return Err(BuilderError::validation_error(
            &format!("{} angle is invalid (NaN or infinite)", name),
            &format!("Provide a valid finite angle for {}", name)
        ));
    }
    Ok(())
}

/// Helper function to validate entity reference
pub(crate) fn validate_entity_ref(entity_ref: &str) -> BuilderResult<()> {
    if entity_ref.trim().is_empty() {
        return Err(BuilderError::validation_error(
            "Entity reference cannot be empty",
            "Provide a valid entity name"
        ));
    }
    Ok(())
}

/// Helper function to validate road ID
pub(crate) fn validate_road_id(road_id: &str) -> BuilderResult<()> {
    if road_id.trim().is_empty() {
        return Err(BuilderError::validation_error(
            "Road ID cannot be empty",
            "Provide a valid road identifier"
        ));
    }
    Ok(())
}
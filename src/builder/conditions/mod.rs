//! Condition builders for programmatic condition construction
//!
//! This module provides fluent APIs for creating all types of conditions in OpenSCENARIO
//! scenarios with comprehensive validation and type safety.

pub mod entity;
pub mod value;
pub mod spatial;

pub use entity::{
    EndOfRoadConditionBuilder, CollisionConditionBuilder, OffroadConditionBuilder,
    TimeHeadwayConditionBuilder, TimeToCollisionConditionBuilder, AccelerationConditionBuilder,
    StandStillConditionBuilder, SpeedConditionBuilder, RelativeSpeedConditionBuilder,
};
pub use value::{
    ParameterConditionBuilder, VariableConditionBuilder, SimulationTimeConditionBuilder,
    StoryboardElementConditionBuilder,
};
pub use spatial::{
    ReachPositionConditionBuilder, DistanceConditionBuilder, RelativeDistanceConditionBuilder,
    TraveledDistanceConditionBuilder,
};

use crate::types::conditions::{Condition, ConditionWrapper};
use crate::types::enums::ConditionEdge;
use crate::types::basic::Double;
use crate::builder::{BuilderError, BuilderResult};

/// Trait for condition builders that can be finished and converted to Condition
pub trait ConditionBuilder {
    /// The specific condition type this builder creates
    type ConditionType;
    
    /// Finish building the condition and return it
    fn finish(self) -> BuilderResult<Self::ConditionType>;
    
    /// Validate the condition configuration
    fn validate(&self) -> BuilderResult<()>;
    
    /// Get the entity reference this condition applies to (if any)
    fn get_entity_ref(&self) -> Option<&str>;
}

/// Unified condition builder interface for dynamic condition type selection
pub struct UnifiedConditionBuilder {
    name: Option<String>,
    edge: ConditionEdge,
    delay: f64,
}

impl UnifiedConditionBuilder {
    /// Create a new unified condition builder
    pub fn new() -> Self {
        Self {
            name: None,
            edge: ConditionEdge::Rising,
            delay: 0.0,
        }
    }
    
    /// Set the condition name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the condition edge
    pub fn edge(mut self, edge: ConditionEdge) -> Self {
        self.edge = edge;
        self
    }
    
    /// Set the condition delay
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }
    
    /// Create a simulation time condition builder
    pub fn simulation_time() -> SimulationTimeConditionBuilder {
        SimulationTimeConditionBuilder::new()
    }
    
    /// Create a speed condition builder
    pub fn speed() -> SpeedConditionBuilder {
        SpeedConditionBuilder::new()
    }
    
    /// Create a distance condition builder
    pub fn distance() -> DistanceConditionBuilder {
        DistanceConditionBuilder::new()
    }
    
    /// Create a collision condition builder
    pub fn collision() -> CollisionConditionBuilder {
        CollisionConditionBuilder::new()
    }
    
    /// Create a reach position condition builder
    pub fn reach_position() -> ReachPositionConditionBuilder {
        ReachPositionConditionBuilder::new()
    }
    
    /// Create a time headway condition builder
    pub fn time_headway() -> TimeHeadwayConditionBuilder {
        TimeHeadwayConditionBuilder::new()
    }
    
    /// Create a time to collision condition builder
    pub fn time_to_collision() -> TimeToCollisionConditionBuilder {
        TimeToCollisionConditionBuilder::new()
    }
    
    /// Create an acceleration condition builder
    pub fn acceleration() -> AccelerationConditionBuilder {
        AccelerationConditionBuilder::new()
    }
    
    /// Create a standstill condition builder
    pub fn standstill() -> StandStillConditionBuilder {
        StandStillConditionBuilder::new()
    }
    
    /// Create an end of road condition builder
    pub fn end_of_road() -> EndOfRoadConditionBuilder {
        EndOfRoadConditionBuilder::new()
    }
    
    /// Create an offroad condition builder
    pub fn offroad() -> OffroadConditionBuilder {
        OffroadConditionBuilder::new()
    }
}

impl Default for UnifiedConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
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

/// Helper function to validate timing values
pub(crate) fn validate_timing(value: f64, name: &str) -> BuilderResult<()> {
    if value.is_nan() || value.is_infinite() {
        return Err(BuilderError::validation_error(
            &format!("{} timing value is invalid (NaN or infinite)", name),
            &format!("Provide a valid finite number for {}", name)
        ));
    }
    if value < 0.0 {
        return Err(BuilderError::validation_error(
            &format!("{} timing value cannot be negative", name),
            &format!("Provide a non-negative value for {}", name)
        ));
    }
    Ok(())
}

/// Helper function to validate distance values
pub(crate) fn validate_distance(value: f64, name: &str) -> BuilderResult<()> {
    if value.is_nan() || value.is_infinite() {
        return Err(BuilderError::validation_error(
            &format!("{} distance value is invalid (NaN or infinite)", name),
            &format!("Provide a valid finite number for {}", name)
        ));
    }
    Ok(())
}

/// Helper function to validate speed values
pub(crate) fn validate_speed(value: f64, name: &str) -> BuilderResult<()> {
    if value.is_nan() || value.is_infinite() {
        return Err(BuilderError::validation_error(
            &format!("{} speed value is invalid (NaN or infinite)", name),
            &format!("Provide a valid finite number for {}", name)
        ));
    }
    Ok(())
}
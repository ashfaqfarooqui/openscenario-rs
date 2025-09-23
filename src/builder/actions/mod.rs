//! Action builders for programmatic action construction
//!
//! This module provides fluent APIs for creating all types of actions in OpenSCENARIO
//! scenarios with comprehensive validation and type safety.

pub mod movement;
pub mod control;
pub mod appearance;
pub mod traffic;

pub use movement::{
    TeleportActionBuilder, SynchronizeActionBuilder, FollowTrajectoryActionBuilder, 
    SpeedActionBuilder, LaneChangeActionBuilder, LaneOffsetActionBuilder,
};
pub use control::{
    ActivateControllerActionBuilder, ControllerActionBuilder, OverrideControllerActionBuilder,
};
pub use appearance::{
    AppearanceActionBuilder, LightActionBuilder, AnimationActionBuilder,
};
pub use traffic::{
    TrafficSignalActionBuilder, TrafficSwarmActionBuilder, TrafficSourceActionBuilder,
};

use crate::types::actions::Action;
use crate::builder::{BuilderError, BuilderResult};

/// Trait for action builders that can be finished and converted to Action
pub trait ActionBuilder {
    /// The specific action type this builder creates
    type ActionType;
    
    /// Finish building the action and return it
    fn finish(self) -> BuilderResult<Self::ActionType>;
    
    /// Validate the action configuration
    fn validate(&self) -> BuilderResult<()>;
    
    /// Get the entity reference this action applies to (if any)
    fn get_entity_ref(&self) -> Option<&str>;
}

/// Unified action builder interface for dynamic action type selection
pub struct UnifiedActionBuilder {
    action_type: ActionType,
}

/// Enum representing different action types
pub enum ActionType {
    Movement(MovementActionType),
    Control(ControlActionType),
    Appearance(AppearanceActionType),
    Traffic(TrafficActionType),
}

/// Movement action subtypes
pub enum MovementActionType {
    Teleport(TeleportActionBuilder),
    Synchronize(SynchronizeActionBuilder),
    FollowTrajectory(FollowTrajectoryActionBuilder),
    Speed(SpeedActionBuilder),
    LaneChange(LaneChangeActionBuilder),
    LaneOffset(LaneOffsetActionBuilder),
}

/// Control action subtypes
pub enum ControlActionType {
    ActivateController(ActivateControllerActionBuilder),
    Controller(ControllerActionBuilder),
    OverrideController(OverrideControllerActionBuilder),
}

/// Appearance action subtypes
pub enum AppearanceActionType {
    Appearance(AppearanceActionBuilder),
    Light(LightActionBuilder),
    Animation(AnimationActionBuilder),
}

/// Traffic action subtypes
pub enum TrafficActionType {
    TrafficSignal(TrafficSignalActionBuilder),
    TrafficSwarm(TrafficSwarmActionBuilder),
    TrafficSource(TrafficSourceActionBuilder),
}

impl UnifiedActionBuilder {
    /// Create a new teleport action builder
    pub fn teleport() -> TeleportActionBuilder {
        TeleportActionBuilder::new()
    }
    

    
    /// Create a new speed action builder
    pub fn speed() -> SpeedActionBuilder {
        SpeedActionBuilder::new()
    }
    
    /// Create a new lane change action builder
    pub fn lane_change() -> LaneChangeActionBuilder {
        LaneChangeActionBuilder::new()
    }
    
    /// Create a new controller action builder
    pub fn controller() -> ControllerActionBuilder {
        ControllerActionBuilder::new()
    }
    
    /// Create a new appearance action builder
    pub fn appearance() -> AppearanceActionBuilder {
        AppearanceActionBuilder::new()
    }
    
    /// Create a new traffic signal action builder
    pub fn traffic_signal() -> TrafficSignalActionBuilder {
        TrafficSignalActionBuilder::new()
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
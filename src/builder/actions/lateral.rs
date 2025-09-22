//! Lateral action builders for lane changes, offsets, and lateral distance actions
//!
//! This module provides builders for actions that affect movement perpendicular
//! to the entity's forward direction, including lane changes, lateral offsets,
//! and lateral distance maintenance.

use crate::types::actions::{
    Action, LateralAction, LateralActionChoice, LaneChangeAction, LaneChangeTarget,
    LaneChangeTargetChoice, LaneOffsetAction, LaneOffsetTarget, LaneOffsetTargetChoice,
    LateralDistanceAction, AbsoluteTargetLane, RelativeTargetLane,
    AbsoluteTargetLaneOffset, RelativeTargetLaneOffset, LaneOffsetActionDynamics,
    DynamicConstraints
};
use crate::types::basic::{Double, OSString, Value, Int};
use crate::types::enums::{DynamicsShape, DynamicsDimension};
use super::{ActionBuilderTrait, ActionUtils};
use crate::builder::error::{BuilderError, BuilderResult};
use crate::builder::states::*;
use std::marker::PhantomData;

/// Builder for lateral actions (lane changes, offsets, lateral distance)
/// 
/// This builder provides a type-safe way to construct lateral actions
/// that affect movement perpendicular to the entity's forward direction.
/// 
/// # Type Parameters
/// * `S` - Current builder state
/// 
/// # Example
/// ```rust
/// let action = LateralActionBuilder::new(Some("lane_change".to_string()))
///     .lane_change_action()
///     .target_lane_relative(1)
///     .dynamics_shape(DynamicsShape::Sinusoidal)
///     .dynamics_dimension(DynamicsDimension::Time)
///     .dynamics_value(3.0)
///     .build()?;
/// ```
pub struct LateralActionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Action name for identification
    name: Option<String>,
    
    /// Partially constructed action data
    action_data: PartialLateralData,
}

/// Internal data structure for building lateral actions
#[derive(Debug, Default)]
struct PartialLateralData {
    /// Action type being built
    action_type: Option<LateralActionType>,
    
    /// Lane change action data
    lane_change_data: Option<LaneChangeActionData>,
    
    /// Lane offset action data
    lane_offset_data: Option<LaneOffsetActionData>,
    
    /// Lateral distance action data
    lateral_distance_data: Option<LateralDistanceActionData>,
}

/// Types of lateral actions
#[derive(Debug, Clone)]
enum LateralActionType {
    LaneChange,
    LaneOffset,
    LateralDistance,
}

/// Lane change action configuration data
#[derive(Debug, Default)]
struct LaneChangeActionData {
    target_lane_type: Option<LaneChangeTargetType>,
    target_lane_absolute: Option<i32>,
    target_lane_relative: Option<i32>,
    target_lane_offset_absolute: Option<f64>,
    target_lane_offset_relative: Option<f64>,
    dynamics_shape: Option<DynamicsShape>,
    dynamics_dimension: Option<DynamicsDimension>,
    dynamics_value: Option<Double>,
    freespace: Option<bool>,
}

/// Lane offset action configuration data
#[derive(Debug, Default)]
struct LaneOffsetActionData {
    target_type: Option<LaneOffsetTargetType>,
    absolute_value: Option<f64>,
    relative_value: Option<f64>,
    entity_ref: Option<String>,
    dynamics_shape: Option<DynamicsShape>,
    dynamics_dimension: Option<DynamicsDimension>,
    dynamics_value: Option<Double>,
}

/// Lateral distance action configuration data
#[derive(Debug, Default)]
struct LateralDistanceActionData {
    entity_ref: Option<String>,
    distance: Option<Double>,
    freespace: Option<bool>,
    continuous: Option<bool>,
    displacement: Option<String>,
    coordinate_system: Option<String>,
}

/// Types of lane change targets
#[derive(Debug, Clone)]
enum LaneChangeTargetType {
    AbsoluteLane,
    RelativeLane,
    AbsoluteLaneOffset,
    RelativeLaneOffset,
}

/// Types of lane offset targets
#[derive(Debug, Clone)]
enum LaneOffsetTargetType {
    Absolute,
    Relative,
}

// Core builder implementation for Empty state
impl LateralActionBuilder<Empty> {
    /// Create a new lateral action builder
    /// 
    /// # Arguments
    /// * `name` - Optional action name
    /// 
    /// # Returns
    /// A new LateralActionBuilder in Empty state
    pub fn new(name: Option<String>) -> Self {
        Self {
            _state: PhantomData,
            name,
            action_data: PartialLateralData::default(),
        }
    }

    /// Start building a lane change action
    /// 
    /// Lane change actions move an entity from one lane to another with
    /// specified dynamics and timing.
    /// 
    /// # Returns
    /// LateralActionBuilder in HasType state for lane change configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LateralActionBuilder::new(None)
    ///     .lane_change_action()
    ///     .target_lane_relative(1);
    /// ```
    pub fn lane_change_action(mut self) -> LateralActionBuilder<HasType> {
        self.action_data.action_type = Some(LateralActionType::LaneChange);
        self.action_data.lane_change_data = Some(LaneChangeActionData::default());
        
        LateralActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a lane offset action
    /// 
    /// Lane offset actions maintain a lateral offset from the lane center
    /// or from another entity's position.
    /// 
    /// # Returns
    /// LateralActionBuilder in HasType state for lane offset configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LateralActionBuilder::new(None)
    ///     .lane_offset_action()
    ///     .absolute_offset(1.5);
    /// ```
    pub fn lane_offset_action(mut self) -> LateralActionBuilder<HasType> {
        self.action_data.action_type = Some(LateralActionType::LaneOffset);
        self.action_data.lane_offset_data = Some(LaneOffsetActionData::default());
        
        LateralActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a lateral distance action
    /// 
    /// Lateral distance actions maintain a specific lateral distance to another entity.
    /// 
    /// # Returns
    /// LateralActionBuilder in HasType state for lateral distance configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LateralActionBuilder::new(None)
    ///     .lateral_distance_action()
    ///     .target_entity("adjacent_vehicle")
    ///     .distance(2.0);
    /// ```
    pub fn lateral_distance_action(mut self) -> LateralActionBuilder<HasType> {
        self.action_data.action_type = Some(LateralActionType::LateralDistance);
        self.action_data.lateral_distance_data = Some(LateralDistanceActionData::default());
        
        LateralActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }
}

// Methods for configuring lateral actions
impl LateralActionBuilder<HasType> {
    /// Set the target lane using absolute lane ID
    /// 
    /// # Arguments
    /// * `lane_id` - Absolute lane ID to change to
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_lane_absolute(2)
    /// ```
    pub fn target_lane_absolute(mut self, lane_id: i32) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.target_lane_type = Some(LaneChangeTargetType::AbsoluteLane);
            lane_change_data.target_lane_absolute = Some(lane_id);
        }
        self
    }

    /// Set the target lane using relative lane offset
    /// 
    /// # Arguments
    /// * `lane_offset` - Relative lane offset (positive = right, negative = left)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_lane_relative(1) // Change to right lane
    /// ```
    pub fn target_lane_relative(mut self, lane_offset: i32) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.target_lane_type = Some(LaneChangeTargetType::RelativeLane);
            lane_change_data.target_lane_relative = Some(lane_offset);
        }
        self
    }

    /// Set the target lane offset using absolute value
    /// 
    /// # Arguments
    /// * `offset` - Absolute lane offset in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_lane_offset_absolute(1.5)
    /// ```
    pub fn target_lane_offset_absolute(mut self, offset: f64) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.target_lane_type = Some(LaneChangeTargetType::AbsoluteLaneOffset);
            lane_change_data.target_lane_offset_absolute = Some(offset);
        }
        self
    }

    /// Set the target lane offset using relative value
    /// 
    /// # Arguments
    /// * `offset` - Relative lane offset in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_lane_offset_relative(0.5)
    /// ```
    pub fn target_lane_offset_relative(mut self, offset: f64) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.target_lane_type = Some(LaneChangeTargetType::RelativeLaneOffset);
            lane_change_data.target_lane_offset_relative = Some(offset);
        }
        self
    }

    /// Set the dynamics shape for lateral movements
    /// 
    /// # Arguments
    /// * `shape` - Dynamics shape (Linear, Cubic, Step, Sinusoidal)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_shape(DynamicsShape::Sinusoidal)
    /// ```
    pub fn dynamics_shape(mut self, shape: DynamicsShape) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.dynamics_shape = Some(shape);
        }
        if let Some(ref mut lane_offset_data) = self.action_data.lane_offset_data {
            lane_offset_data.dynamics_shape = Some(shape);
        }
        self
    }

    /// Set the dynamics dimension for lateral movements
    /// 
    /// # Arguments
    /// * `dimension` - Dynamics dimension (Rate, Time, Distance)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_dimension(DynamicsDimension::Time)
    /// ```
    pub fn dynamics_dimension(mut self, dimension: DynamicsDimension) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.dynamics_dimension = Some(dimension);
        }
        if let Some(ref mut lane_offset_data) = self.action_data.lane_offset_data {
            lane_offset_data.dynamics_dimension = Some(dimension);
        }
        self
    }

    /// Set the dynamics value for lateral movements
    /// 
    /// # Arguments
    /// * `value` - Dynamics value (rate, time, or distance depending on dimension)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_value(3.0) // 3 seconds for time dimension
    /// ```
    pub fn dynamics_value(mut self, value: f64) -> Self {
        let double_value = ActionUtils::double(value);
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.dynamics_value = Some(double_value.clone());
        }
        if let Some(ref mut lane_offset_data) = self.action_data.lane_offset_data {
            lane_offset_data.dynamics_value = Some(double_value);
        }
        self
    }

    /// Set freespace flag for distance calculations
    /// 
    /// # Arguments
    /// * `freespace` - Whether to use freespace (true) or reference point (false) distance
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn freespace(mut self, freespace: bool) -> Self {
        if let Some(ref mut lane_change_data) = self.action_data.lane_change_data {
            lane_change_data.freespace = Some(freespace);
        }
        if let Some(ref mut lateral_distance_data) = self.action_data.lateral_distance_data {
            lateral_distance_data.freespace = Some(freespace);
        }
        self
    }

    /// Set absolute offset for lane offset actions
    /// 
    /// # Arguments
    /// * `offset` - Absolute offset from lane center in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.absolute_offset(1.5)
    /// ```
    pub fn absolute_offset(mut self, offset: f64) -> Self {
        if let Some(ref mut lane_offset_data) = self.action_data.lane_offset_data {
            lane_offset_data.target_type = Some(LaneOffsetTargetType::Absolute);
            lane_offset_data.absolute_value = Some(offset);
        }
        self
    }

    /// Set relative offset for lane offset actions
    /// 
    /// # Arguments
    /// * `offset` - Relative offset in meters
    /// * `entity_ref` - Reference entity for relative offset
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.relative_offset(0.5, "lead_vehicle")
    /// ```
    pub fn relative_offset(mut self, offset: f64, entity_ref: &str) -> Self {
        if let Some(ref mut lane_offset_data) = self.action_data.lane_offset_data {
            lane_offset_data.target_type = Some(LaneOffsetTargetType::Relative);
            lane_offset_data.relative_value = Some(offset);
            lane_offset_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }

    /// Set the target entity for lateral distance actions
    /// 
    /// # Arguments
    /// * `entity_ref` - Name of the target entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_entity("adjacent_vehicle")
    /// ```
    pub fn target_entity(mut self, entity_ref: &str) -> Self {
        if let Some(ref mut lateral_distance_data) = self.action_data.lateral_distance_data {
            lateral_distance_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }

    /// Set the distance for lateral distance actions
    /// 
    /// # Arguments
    /// * `distance` - Target lateral distance in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.distance(2.0)
    /// ```
    pub fn distance(mut self, distance: f64) -> Self {
        if let Some(ref mut lateral_distance_data) = self.action_data.lateral_distance_data {
            lateral_distance_data.distance = Some(ActionUtils::double(distance));
        }
        self
    }

    /// Set the continuous flag for lateral distance actions
    /// 
    /// # Arguments
    /// * `continuous` - Whether the action should be continuous
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn continuous(mut self, continuous: bool) -> Self {
        if let Some(ref mut lateral_distance_data) = self.action_data.lateral_distance_data {
            lateral_distance_data.continuous = Some(continuous);
        }
        self
    }
}

// Build implementation
impl<S: BuilderState> LateralActionBuilder<S> {
    /// Build the final lateral action
    /// 
    /// # Returns
    /// Complete Action or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<Action> {
        match self.action_data.action_type {
            Some(LateralActionType::LaneChange) => self.build_lane_change_action(),
            Some(LateralActionType::LaneOffset) => self.build_lane_offset_action(),
            Some(LateralActionType::LateralDistance) => self.build_lateral_distance_action(),
            None => Err(BuilderError::missing_field(
                "action_type",
                "Call one of the action type methods first (lane_change_action, etc.)"
            )),
        }
    }

    fn build_lane_change_action(self) -> BuilderResult<Action> {
        let lane_change_data = self.action_data.lane_change_data
            .ok_or_else(|| BuilderError::missing_field("lane_change_data", "Internal error"))?;

        let target_lane_type = lane_change_data.target_lane_type
            .ok_or_else(|| BuilderError::missing_field(
                "target_lane_type",
                "Call one of the target lane methods (target_lane_absolute, target_lane_relative, etc.)"
            ))?;

        let target = match target_lane_type {
            LaneChangeTargetType::AbsoluteLane => {
                let lane_id = lane_change_data.target_lane_absolute
                    .ok_or_else(|| BuilderError::missing_field("target_lane_absolute", "Internal error"))?;
                LaneChangeTargetChoice::AbsoluteTargetLane(AbsoluteTargetLane {
                    value: ActionUtils::os_string(&lane_id.to_string()),
                })
            }
            LaneChangeTargetType::RelativeLane => {
                let lane_offset = lane_change_data.target_lane_relative
                    .ok_or_else(|| BuilderError::missing_field("target_lane_relative", "Internal error"))?;
                LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane {
                    entity_ref: ActionUtils::os_string(""), // Will need entity reference
                    value: Value::literal(lane_offset),
                })
            }
            LaneChangeTargetType::AbsoluteLaneOffset => {
                let offset = lane_change_data.target_lane_offset_absolute
                    .ok_or_else(|| BuilderError::missing_field("target_lane_offset_absolute", "Internal error"))?;
                LaneChangeTargetChoice::AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset {
                    value: ActionUtils::double(offset),
                })
            }
            LaneChangeTargetType::RelativeLaneOffset => {
                let offset = lane_change_data.target_lane_offset_relative
                    .ok_or_else(|| BuilderError::missing_field("target_lane_offset_relative", "Internal error"))?;
                LaneChangeTargetChoice::RelativeTargetLaneOffset(RelativeTargetLaneOffset {
                    entity_ref: ActionUtils::os_string(""), // Will need entity reference
                    value: ActionUtils::double(offset),
                })
            }
        };

        let lane_change_action = LaneChangeAction {
            target_lane: LaneChangeTarget {
                choice: target,
            },
            lane_change_action_dynamics: DynamicConstraints {
                max_deceleration: None,
                max_acceleration: None,
                max_speed: None,
            },
            freespace: lane_change_data.freespace.unwrap_or(false),
        };

        let lateral_action = LateralAction {
            choice: LateralActionChoice::LaneChangeAction(lane_change_action),
        };

        Ok(Action::Lateral(lateral_action))
    }

    fn build_lane_offset_action(self) -> BuilderResult<Action> {
        let lane_offset_data = self.action_data.lane_offset_data
            .ok_or_else(|| BuilderError::missing_field("lane_offset_data", "Internal error"))?;

        let target_type = lane_offset_data.target_type
            .ok_or_else(|| BuilderError::missing_field(
                "target_type",
                "Call absolute_offset() or relative_offset() to set the target"
            ))?;

        let target = match target_type {
            LaneOffsetTargetType::Absolute => {
                let value = lane_offset_data.absolute_value
                    .ok_or_else(|| BuilderError::missing_field("absolute_value", "Internal error"))?;
                LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset {
                    value: ActionUtils::double(value),
                })
            }
            LaneOffsetTargetType::Relative => {
                let value = lane_offset_data.relative_value
                    .ok_or_else(|| BuilderError::missing_field("relative_value", "Internal error"))?;
                let entity_ref = lane_offset_data.entity_ref
                    .ok_or_else(|| BuilderError::missing_field("entity_ref", "Internal error"))?;
                LaneOffsetTargetChoice::RelativeTargetLaneOffset(RelativeTargetLaneOffset {
                    entity_ref: ActionUtils::os_string(&entity_ref),
                    value: ActionUtils::double(value),
                })
            }
        };

        let lane_offset_action = LaneOffsetAction {
            continuous: true, // Default to continuous
            lane_offset_target: LaneOffsetTarget {
                choice: target,
            },
            lane_offset_action_dynamics: LaneOffsetActionDynamics {
                max_lateral_acc: None,
                dynamics_shape: lane_offset_data.dynamics_shape.unwrap_or(DynamicsShape::Linear),
                dynamics_dimension: lane_offset_data.dynamics_dimension.unwrap_or(DynamicsDimension::Time),
                dynamics_value: lane_offset_data.dynamics_value.unwrap_or_else(|| ActionUtils::double(1.0)),
            },
        };

        let lateral_action = LateralAction {
            choice: LateralActionChoice::LaneOffsetAction(lane_offset_action),
        };

        Ok(Action::Lateral(lateral_action))
    }

    fn build_lateral_distance_action(self) -> BuilderResult<Action> {
        let lateral_distance_data = self.action_data.lateral_distance_data
            .ok_or_else(|| BuilderError::missing_field("lateral_distance_data", "Internal error"))?;

        let entity_ref = lateral_distance_data.entity_ref
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Call .target_entity() to set the target entity"
            ))?;

        let distance = lateral_distance_data.distance
            .ok_or_else(|| BuilderError::missing_field(
                "distance",
                "Call .distance() to set the target distance"
            ))?;

        let lateral_distance_action = LateralDistanceAction {
            entity_ref: ActionUtils::os_string(&entity_ref),
            distance,
            freespace: lateral_distance_data.freespace.unwrap_or(false),
            continuous: lateral_distance_data.continuous.unwrap_or(true),
            displacement: lateral_distance_data.displacement.clone(),
            coordinate_system: lateral_distance_data.coordinate_system.clone(),
            dynamics: DynamicConstraints {
                max_deceleration: None,
                max_acceleration: None,
                max_speed: None,
            },
        };

        Ok(Action::LateralDistance(lateral_distance_action))
    }
}

impl<S: BuilderState> ActionBuilderTrait for LateralActionBuilder<S> {
    fn build_action(self) -> BuilderResult<Action> {
        self.build()
    }

    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::{DynamicsShape, DynamicsDimension};

    #[test]
    fn test_lane_change_action_relative() {
        let action = LateralActionBuilder::new(Some("lane_change".to_string()))
            .lane_change_action()
            .target_lane_relative(1)
            .dynamics_shape(DynamicsShape::Sinusoidal)
            .dynamics_dimension(DynamicsDimension::Time)
            .dynamics_value(3.0)
            .freespace(false)
            .build()
            .unwrap();

        match action {
            Action::Lateral(lateral_action) => {
                match lateral_action.choice {
                    LateralActionChoice::LaneChangeAction(lane_change) => {
                        assert!(!lane_change.freespace);
                        // Additional assertions for lane change target
                    }
                    _ => panic!("Expected LaneChangeAction"),
                }
            }
            _ => panic!("Expected Lateral action"),
        }
    }

    #[test]
    fn test_lane_offset_action_absolute() {
        let action = LateralActionBuilder::new(None)
            .lane_offset_action()
            .absolute_offset(1.5)
            .dynamics_shape(DynamicsShape::Linear)
            .dynamics_dimension(DynamicsDimension::Rate)
            .dynamics_value(0.5)
            .build()
            .unwrap();

        match action {
            Action::Lateral(lateral_action) => {
                match lateral_action.choice {
                    LateralActionChoice::LaneOffsetAction(lane_offset) => {
                        assert!(lane_offset.continuous);
                        assert_eq!(lane_offset.lane_offset_action_dynamics.dynamics_shape, DynamicsShape::Linear);
                        assert_eq!(lane_offset.lane_offset_action_dynamics.dynamics_dimension, DynamicsDimension::Rate);
                    }
                    _ => panic!("Expected LaneOffsetAction"),
                }
            }
            _ => panic!("Expected Lateral action"),
        }
    }

    #[test]
    fn test_lateral_distance_action() {
        let action = LateralActionBuilder::new(None)
            .lateral_distance_action()
            .target_entity("adjacent_vehicle")
            .distance(2.0)
            .freespace(true)
            .continuous(false)
            .build()
            .unwrap();

        match action {
            Action::LateralDistance(lateral_distance) => {
                assert_eq!(lateral_distance.entity_ref.as_literal().unwrap(), "adjacent_vehicle");
                assert_eq!(lateral_distance.distance.as_literal().unwrap(), &2.0);
                assert!(lateral_distance.freespace);
                assert!(!lateral_distance.continuous);
            }
            _ => panic!("Expected LateralDistance action"),
        }
    }

    #[test]
    fn test_lane_change_missing_target() {
        let result = LateralActionBuilder::new(None)
            .lane_change_action()
            .dynamics_shape(DynamicsShape::Linear)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("target_lane_type"));
    }

    #[test]
    fn test_lane_offset_relative() {
        let action = LateralActionBuilder::new(None)
            .lane_offset_action()
            .relative_offset(0.5, "lead_vehicle")
            .build()
            .unwrap();

        match action {
            Action::Lateral(lateral_action) => {
                match lateral_action.choice {
                    LateralActionChoice::LaneOffsetAction(lane_offset) => {
                        // Verify relative offset configuration
                        assert!(lane_offset.continuous);
                    }
                    _ => panic!("Expected LaneOffsetAction"),
                }
            }
            _ => panic!("Expected Lateral action"),
        }
    }

    #[test]
    fn test_builder_with_name() {
        let builder = LateralActionBuilder::new(Some("test_lateral".to_string()));
        assert_eq!(builder.get_name().unwrap(), "test_lateral");
    }
}
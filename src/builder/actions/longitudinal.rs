//! Longitudinal action builders for speed, acceleration, and following actions
//!
//! This module provides builders for actions that affect movement along the
//! entity's forward direction, including speed changes, acceleration profiles,
//! and following behaviors.

use crate::types::actions::{
    Action, SpeedAction, LongitudinalAction, LongitudinalDistanceAction,
    SynchronizeAction, FollowTrajectoryAction, AcquirePositionAction,
    DynamicConstraints, FinalSpeed, Trajectory, TrajectoryFollowingMode
};
use crate::types::basic::{Double, OSString, Value};
use crate::types::enums::{DynamicsShape, DynamicsDimension, SpeedTargetValueType};
use crate::types::positions::Position;
use super::{ActionBuilderTrait, ActionUtils};
use crate::builder::error::{BuilderError, BuilderResult};
use crate::builder::states::*;
use std::marker::PhantomData;

/// Builder for longitudinal actions (speed, acceleration, following)
/// 
/// This builder provides a type-safe way to construct longitudinal actions
/// that affect movement along the entity's forward direction.
/// 
/// # Type Parameters
/// * `S` - Current builder state
/// 
/// # Example
/// ```rust
/// let action = LongitudinalActionBuilder::new(Some("accelerate".to_string()))
///     .speed_action()
///     .target_speed(30.0)
///     .dynamics_shape(DynamicsShape::Linear)
///     .dynamics_dimension(DynamicsDimension::Rate)
///     .dynamics_value(2.0)
///     .build()?;
/// ```
pub struct LongitudinalActionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Action name for identification
    name: Option<String>,
    
    /// Partially constructed action data
    action_data: PartialLongitudinalData,
}

/// Internal data structure for building longitudinal actions
#[derive(Debug, Default)]
struct PartialLongitudinalData {
    /// Action type being built
    action_type: Option<LongitudinalActionType>,
    
    /// Speed action data
    speed_data: Option<SpeedActionData>,
    
    /// Longitudinal distance action data
    distance_data: Option<LongitudinalDistanceData>,
    
    /// Synchronize action data
    synchronize_data: Option<SynchronizeActionData>,
    
    /// Follow trajectory action data
    trajectory_data: Option<TrajectoryActionData>,
    
    /// Acquire position action data
    acquire_position_data: Option<AcquirePositionData>,
}

/// Types of longitudinal actions
#[derive(Debug, Clone)]
enum LongitudinalActionType {
    Speed,
    LongitudinalDistance,
    Synchronize,
    FollowTrajectory,
    AcquirePosition,
}

/// Speed action configuration data
#[derive(Debug, Default)]
struct SpeedActionData {
    target_speed: Option<Double>,
    speed_target_type: Option<SpeedTargetValueType>,
    dynamics_shape: Option<DynamicsShape>,
    dynamics_dimension: Option<DynamicsDimension>,
    dynamics_value: Option<Double>,
    freespace: Option<bool>,
}

/// Longitudinal distance action configuration data
#[derive(Debug, Default)]
struct LongitudinalDistanceData {
    entity_ref: Option<String>,
    distance: Option<Double>,
    freespace: Option<bool>,
    continuous: Option<bool>,
    displacement: Option<String>,
    coordinate_system: Option<String>,
}

/// Synchronize action configuration data
#[derive(Debug, Default)]
struct SynchronizeActionData {
    entity_ref: Option<String>,
    target_position: Option<Position>,
    master_entity_position: Option<Position>,
    freespace: Option<bool>,
}

/// Follow trajectory action configuration data
#[derive(Debug, Default)]
struct TrajectoryActionData {
    trajectory: Option<Trajectory>,
    following_mode: Option<TrajectoryFollowingMode>,
}

/// Acquire position action configuration data
#[derive(Debug, Default)]
struct AcquirePositionData {
    position: Option<Position>,
}

// Core builder implementation for Empty state
impl LongitudinalActionBuilder<Empty> {
    /// Create a new longitudinal action builder
    /// 
    /// # Arguments
    /// * `name` - Optional action name
    /// 
    /// # Returns
    /// A new LongitudinalActionBuilder in Empty state
    pub fn new(name: Option<String>) -> Self {
        Self {
            _state: PhantomData,
            name,
            action_data: PartialLongitudinalData::default(),
        }
    }

    /// Start building a speed action
    /// 
    /// Speed actions control the target speed of an entity with specified dynamics.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder in HasType state for speed configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LongitudinalActionBuilder::new(None)
    ///     .speed_action()
    ///     .target_speed(25.0);
    /// ```
    pub fn speed_action(mut self) -> LongitudinalActionBuilder<HasType> {
        self.action_data.action_type = Some(LongitudinalActionType::Speed);
        self.action_data.speed_data = Some(SpeedActionData::default());
        
        LongitudinalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a longitudinal distance action
    /// 
    /// Longitudinal distance actions maintain a specific distance to another entity.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder in HasType state for distance configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LongitudinalActionBuilder::new(None)
    ///     .longitudinal_distance_action()
    ///     .target_entity("lead_vehicle")
    ///     .distance(50.0);
    /// ```
    pub fn longitudinal_distance_action(mut self) -> LongitudinalActionBuilder<HasType> {
        self.action_data.action_type = Some(LongitudinalActionType::LongitudinalDistance);
        self.action_data.distance_data = Some(LongitudinalDistanceData::default());
        
        LongitudinalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a synchronize action
    /// 
    /// Synchronize actions coordinate entity movement with another entity.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder in HasType state for synchronize configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LongitudinalActionBuilder::new(None)
    ///     .synchronize_action()
    ///     .master_entity("lead_vehicle")
    ///     .target_position(position);
    /// ```
    pub fn synchronize_action(mut self) -> LongitudinalActionBuilder<HasType> {
        self.action_data.action_type = Some(LongitudinalActionType::Synchronize);
        self.action_data.synchronize_data = Some(SynchronizeActionData::default());
        
        LongitudinalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a follow trajectory action
    /// 
    /// Follow trajectory actions make an entity follow a predefined path.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder in HasType state for trajectory configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LongitudinalActionBuilder::new(None)
    ///     .follow_trajectory_action()
    ///     .trajectory(trajectory)
    ///     .following_mode(TrajectoryFollowingMode::Follow);
    /// ```
    pub fn follow_trajectory_action(mut self) -> LongitudinalActionBuilder<HasType> {
        self.action_data.action_type = Some(LongitudinalActionType::FollowTrajectory);
        self.action_data.trajectory_data = Some(TrajectoryActionData::default());
        
        LongitudinalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building an acquire position action
    /// 
    /// Acquire position actions move an entity to a specific position.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder in HasType state for position configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = LongitudinalActionBuilder::new(None)
    ///     .acquire_position_action()
    ///     .position(target_position);
    /// ```
    pub fn acquire_position_action(mut self) -> LongitudinalActionBuilder<HasType> {
        self.action_data.action_type = Some(LongitudinalActionType::AcquirePosition);
        self.action_data.acquire_position_data = Some(AcquirePositionData::default());
        
        LongitudinalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }
}

// Methods for configuring speed actions
impl LongitudinalActionBuilder<HasType> {
    /// Set the target speed for a speed action
    /// 
    /// # Arguments
    /// * `speed` - Target speed in m/s
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_speed(25.0)
    /// ```
    pub fn target_speed(mut self, speed: f64) -> Self {
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.target_speed = Some(ActionUtils::double(speed));
        }
        self
    }

    /// Set the speed target type
    /// 
    /// # Arguments
    /// * `target_type` - Speed target value type (Absolute, Relative, etc.)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn speed_target_type(mut self, target_type: SpeedTargetValueType) -> Self {
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.speed_target_type = Some(target_type);
        }
        self
    }

    /// Set the dynamics shape for speed changes
    /// 
    /// # Arguments
    /// * `shape` - Dynamics shape (Linear, Cubic, Step, Sinusoidal)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_shape(DynamicsShape::Linear)
    /// ```
    pub fn dynamics_shape(mut self, shape: DynamicsShape) -> Self {
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.dynamics_shape = Some(shape);
        }
        self
    }

    /// Set the dynamics dimension for speed changes
    /// 
    /// # Arguments
    /// * `dimension` - Dynamics dimension (Rate, Time, Distance)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_dimension(DynamicsDimension::Rate)
    /// ```
    pub fn dynamics_dimension(mut self, dimension: DynamicsDimension) -> Self {
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.dynamics_dimension = Some(dimension);
        }
        self
    }

    /// Set the dynamics value for speed changes
    /// 
    /// # Arguments
    /// * `value` - Dynamics value (rate, time, or distance depending on dimension)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.dynamics_value(2.0) // 2 m/s² for rate dimension
    /// ```
    pub fn dynamics_value(mut self, value: f64) -> Self {
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.dynamics_value = Some(ActionUtils::double(value));
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
        if let Some(ref mut speed_data) = self.action_data.speed_data {
            speed_data.freespace = Some(freespace);
        }
        if let Some(ref mut distance_data) = self.action_data.distance_data {
            distance_data.freespace = Some(freespace);
        }
        if let Some(ref mut sync_data) = self.action_data.synchronize_data {
            sync_data.freespace = Some(freespace);
        }
        self
    }

    /// Set the target entity for distance-based actions
    /// 
    /// # Arguments
    /// * `entity_ref` - Name of the target entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.target_entity("lead_vehicle")
    /// ```
    pub fn target_entity(mut self, entity_ref: &str) -> Self {
        if let Some(ref mut distance_data) = self.action_data.distance_data {
            distance_data.entity_ref = Some(entity_ref.to_string());
        }
        if let Some(ref mut sync_data) = self.action_data.synchronize_data {
            sync_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }

    /// Set the distance for longitudinal distance actions
    /// 
    /// # Arguments
    /// * `distance` - Target distance in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.distance(50.0)
    /// ```
    pub fn distance(mut self, distance: f64) -> Self {
        if let Some(ref mut distance_data) = self.action_data.distance_data {
            distance_data.distance = Some(ActionUtils::double(distance));
        }
        self
    }

    /// Set the continuous flag for distance actions
    /// 
    /// # Arguments
    /// * `continuous` - Whether the action should be continuous
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn continuous(mut self, continuous: bool) -> Self {
        if let Some(ref mut distance_data) = self.action_data.distance_data {
            distance_data.continuous = Some(continuous);
        }
        self
    }

    /// Set the master entity for synchronize actions
    /// 
    /// # Arguments
    /// * `entity_ref` - Name of the master entity to synchronize with
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.master_entity("lead_vehicle")
    /// ```
    pub fn master_entity(mut self, entity_ref: &str) -> Self {
        self.target_entity(entity_ref)
    }

    /// Set the target position for synchronize actions
    /// 
    /// # Arguments
    /// * `position` - Target position to reach
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn target_position(mut self, position: Position) -> Self {
        if let Some(ref mut sync_data) = self.action_data.synchronize_data {
            sync_data.target_position = Some(position);
        }
        self
    }

    /// Set the master entity position for synchronize actions
    /// 
    /// # Arguments
    /// * `position` - Position of the master entity
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn master_entity_position(mut self, position: Position) -> Self {
        if let Some(ref mut sync_data) = self.action_data.synchronize_data {
            sync_data.master_entity_position = Some(position);
        }
        self
    }

    /// Set the trajectory for follow trajectory actions
    /// 
    /// # Arguments
    /// * `trajectory` - Trajectory to follow
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn trajectory(mut self, trajectory: Trajectory) -> Self {
        if let Some(ref mut traj_data) = self.action_data.trajectory_data {
            traj_data.trajectory = Some(trajectory);
        }
        self
    }

    /// Set the following mode for trajectory actions
    /// 
    /// # Arguments
    /// * `mode` - Trajectory following mode
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn following_mode(mut self, mode: TrajectoryFollowingMode) -> Self {
        if let Some(ref mut traj_data) = self.action_data.trajectory_data {
            traj_data.following_mode = Some(mode);
        }
        self
    }

    /// Set the position for acquire position actions
    /// 
    /// # Arguments
    /// * `position` - Position to acquire
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn position(mut self, position: Position) -> Self {
        if let Some(ref mut pos_data) = self.action_data.acquire_position_data {
            pos_data.position = Some(position);
        }
        self
    }
}

// Build implementation
impl<S: BuilderState> LongitudinalActionBuilder<S> {
    /// Build the final longitudinal action
    /// 
    /// # Returns
    /// Complete Action or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<Action> {
        match self.action_data.action_type {
            Some(LongitudinalActionType::Speed) => self.build_speed_action(),
            Some(LongitudinalActionType::LongitudinalDistance) => self.build_distance_action(),
            Some(LongitudinalActionType::Synchronize) => self.build_synchronize_action(),
            Some(LongitudinalActionType::FollowTrajectory) => self.build_trajectory_action(),
            Some(LongitudinalActionType::AcquirePosition) => self.build_acquire_position_action(),
            None => Err(BuilderError::missing_field(
                "action_type",
                "Call one of the action type methods first (speed_action, etc.)"
            )),
        }
    }

    fn build_speed_action(self) -> BuilderResult<Action> {
        let speed_data = self.action_data.speed_data
            .ok_or_else(|| BuilderError::missing_field("speed_data", "Internal error"))?;

        let target_speed = speed_data.target_speed
            .ok_or_else(|| BuilderError::missing_field(
                "target_speed", 
                "Call .target_speed() to set the target speed"
            ))?;

        let dynamics_shape = speed_data.dynamics_shape
            .unwrap_or(DynamicsShape::Linear);
        let dynamics_dimension = speed_data.dynamics_dimension
            .unwrap_or(DynamicsDimension::Rate);
        let dynamics_value = speed_data.dynamics_value
            .unwrap_or_else(|| ActionUtils::double(1.0));

        let speed_action = SpeedAction {
            speed_target_value_type: speed_data.speed_target_type
                .unwrap_or(SpeedTargetValueType::Absolute),
            value: target_speed,
            freespace: speed_data.freespace.unwrap_or(false),
            dynamics: DynamicConstraints {
                max_deceleration: None,
                max_acceleration: None,
                max_speed: None,
            },
        };

        Ok(Action::Speed(speed_action))
    }

    fn build_distance_action(self) -> BuilderResult<Action> {
        let distance_data = self.action_data.distance_data
            .ok_or_else(|| BuilderError::missing_field("distance_data", "Internal error"))?;

        let entity_ref = distance_data.entity_ref
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Call .target_entity() to set the target entity"
            ))?;

        let distance = distance_data.distance
            .ok_or_else(|| BuilderError::missing_field(
                "distance",
                "Call .distance() to set the target distance"
            ))?;

        let distance_action = LongitudinalDistanceAction {
            entity_ref: ActionUtils::os_string(&entity_ref),
            distance,
            freespace: distance_data.freespace.unwrap_or(false),
            continuous: distance_data.continuous.unwrap_or(true),
            displacement: distance_data.displacement.clone(),
            coordinate_system: distance_data.coordinate_system.clone(),
            dynamics: DynamicConstraints {
                max_deceleration: None,
                max_acceleration: None,
                max_speed: None,
            },
        };

        Ok(Action::LongitudinalDistance(distance_action))
    }

    fn build_synchronize_action(self) -> BuilderResult<Action> {
        let sync_data = self.action_data.synchronize_data
            .ok_or_else(|| BuilderError::missing_field("synchronize_data", "Internal error"))?;

        let entity_ref = sync_data.entity_ref
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Call .master_entity() to set the master entity"
            ))?;

        let target_position = sync_data.target_position
            .ok_or_else(|| BuilderError::missing_field(
                "target_position",
                "Call .target_position() to set the target position"
            ))?;

        let sync_action = SynchronizeAction {
            master_entity_ref: ActionUtils::os_string(&entity_ref),
            target_position_master: sync_data.master_entity_position,
            target_position,
            freespace: sync_data.freespace.unwrap_or(false),
            final_speed: None,
        };

        Ok(Action::Synchronize(sync_action))
    }

    fn build_trajectory_action(self) -> BuilderResult<Action> {
        let traj_data = self.action_data.trajectory_data
            .ok_or_else(|| BuilderError::missing_field("trajectory_data", "Internal error"))?;

        let trajectory = traj_data.trajectory
            .ok_or_else(|| BuilderError::missing_field(
                "trajectory",
                "Call .trajectory() to set the trajectory"
            ))?;

        let traj_action = FollowTrajectoryAction {
            trajectory,
            following_mode: traj_data.following_mode,
            reference_domain: None,
            scale: None,
            offset: None,
        };

        Ok(Action::FollowTrajectory(traj_action))
    }

    fn build_acquire_position_action(self) -> BuilderResult<Action> {
        let pos_data = self.action_data.acquire_position_data
            .ok_or_else(|| BuilderError::missing_field("acquire_position_data", "Internal error"))?;

        let position = pos_data.position
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .position() to set the target position"
            ))?;

        let acquire_action = AcquirePositionAction {
            position,
        };

        Ok(Action::AcquirePosition(acquire_action))
    }
}

impl<S: BuilderState> ActionBuilderTrait for LongitudinalActionBuilder<S> {
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
    use crate::types::enums::{DynamicsShape, DynamicsDimension, SpeedTargetValueType};

    #[test]
    fn test_speed_action_builder() {
        let action = LongitudinalActionBuilder::new(Some("accelerate".to_string()))
            .speed_action()
            .target_speed(30.0)
            .dynamics_shape(DynamicsShape::Linear)
            .dynamics_dimension(DynamicsDimension::Rate)
            .dynamics_value(2.0)
            .build()
            .unwrap();

        match action {
            Action::Speed(speed_action) => {
                assert_eq!(speed_action.value.as_literal().unwrap(), &30.0);
                assert_eq!(speed_action.speed_target_value_type, SpeedTargetValueType::Absolute);
                assert!(!speed_action.freespace);
            }
            _ => panic!("Expected Speed action"),
        }
    }

    #[test]
    fn test_longitudinal_distance_action_builder() {
        let action = LongitudinalActionBuilder::new(None)
            .longitudinal_distance_action()
            .target_entity("lead_vehicle")
            .distance(50.0)
            .freespace(true)
            .continuous(false)
            .build()
            .unwrap();

        match action {
            Action::LongitudinalDistance(distance_action) => {
                assert_eq!(distance_action.entity_ref.as_literal().unwrap(), "lead_vehicle");
                assert_eq!(distance_action.distance.as_literal().unwrap(), &50.0);
                assert!(distance_action.freespace);
                assert!(!distance_action.continuous);
            }
            _ => panic!("Expected LongitudinalDistance action"),
        }
    }

    #[test]
    fn test_speed_action_missing_target_speed() {
        let result = LongitudinalActionBuilder::new(None)
            .speed_action()
            .dynamics_shape(DynamicsShape::Linear)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("target_speed"));
    }

    #[test]
    fn test_distance_action_missing_entity() {
        let result = LongitudinalActionBuilder::new(None)
            .longitudinal_distance_action()
            .distance(50.0)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("entity_ref"));
    }

    #[test]
    fn test_builder_with_name() {
        let builder = LongitudinalActionBuilder::new(Some("test_action".to_string()));
        assert_eq!(builder.get_name().unwrap(), "test_action");
    }
}
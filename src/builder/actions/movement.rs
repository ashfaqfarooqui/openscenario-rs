//! Movement action builders for programmatic movement action construction

use crate::types::{
    basic::{Double, Boolean, OSString, Int},
    actions::{
        SpeedAction, TeleportAction, LongitudinalAction, LateralAction,
        SynchronizeAction, FollowTrajectoryAction, LaneChangeAction,
        LaneOffsetAction, TransitionDynamics, SpeedActionTarget, 
        AbsoluteTargetSpeed, RelativeTargetSpeed, LaneChangeTarget, 
        LaneChangeTargetChoice, RelativeTargetLane, AbsoluteTargetLane,
        LaneOffsetActionDynamics, LaneOffsetTarget, LaneOffsetTargetChoice,
        AbsoluteTargetLaneOffset, RelativeTargetLaneOffset,
        LateralActionChoice, LongitudinalActionChoice,
        Trajectory, TrajectoryFollowingMode,
    },
    positions::Position,
    enums::{SpeedTargetValueType, DynamicsShape, DynamicsDimension, FollowingMode},
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ActionBuilder, validate_entity_ref, validate_speed, validate_timing, validate_distance};

/// Builder for creating teleport actions
#[derive(Debug, Clone)]
pub struct TeleportActionBuilder {
    entity_ref: Option<String>,
    position: Option<Position>,
}

impl TeleportActionBuilder {
    /// Create a new teleport action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            position: None,
        }
    }
    
    /// Set the entity to teleport
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target position
    pub fn to_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}

impl ActionBuilder for TeleportActionBuilder {
    type ActionType = TeleportAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for teleport action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.position.is_none() {
            return Err(BuilderError::validation_error(
                "Position is required for teleport action",
                "Call to_position() to set the target position"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(TeleportAction {
            position: self.position.unwrap(),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating speed actions
#[derive(Debug, Clone)]
pub struct SpeedActionBuilder {
    entity_ref: Option<String>,
    target_speed: Option<f64>,
    speed_target_type: SpeedTargetValueType,
    target_entity_ref: Option<String>,
    dynamics_shape: DynamicsShape,
    dynamics_dimension: DynamicsDimension,
    dynamics_value: Option<f64>,
}

impl SpeedActionBuilder {
    /// Create a new speed action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_speed: None,
            speed_target_type: SpeedTargetValueType::Absolute,
            target_entity_ref: None,
            dynamics_shape: DynamicsShape::Linear,
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_value: Some(1.0), // Default 1 second
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set absolute target speed
    pub fn absolute_target(mut self, speed: f64) -> Self {
        self.target_speed = Some(speed);
        self.speed_target_type = SpeedTargetValueType::Absolute;
        self
    }
    
    /// Set relative target speed
    pub fn relative_target(mut self, speed: f64, entity_ref: impl Into<String>) -> Self {
        self.target_speed = Some(speed);
        self.target_entity_ref = Some(entity_ref.into());
        self.speed_target_type = SpeedTargetValueType::Relative;
        self
    }
    
    /// Set dynamics shape
    pub fn dynamics_shape(mut self, shape: DynamicsShape) -> Self {
        self.dynamics_shape = shape;
        self
    }
    
    /// Set dynamics dimension and value
    pub fn dynamics(mut self, dimension: DynamicsDimension, value: f64) -> Self {
        self.dynamics_dimension = dimension;
        self.dynamics_value = Some(value);
        self
    }
    
    /// Set time-based dynamics
    pub fn over_time(mut self, time: f64) -> Self {
        self.dynamics_dimension = DynamicsDimension::Time;
        self.dynamics_value = Some(time);
        self
    }
    
    /// Set distance-based dynamics
    pub fn over_distance(mut self, distance: f64) -> Self {
        self.dynamics_dimension = DynamicsDimension::Distance;
        self.dynamics_value = Some(distance);
        self
    }
}

impl ActionBuilder for SpeedActionBuilder {
    type ActionType = SpeedAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for speed action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_speed = self.target_speed.ok_or_else(|| BuilderError::validation_error(
            "Target speed is required for speed action",
            "Call absolute_target() or relative_target() to set the speed"
        ))?;
        
        validate_speed(target_speed, "target speed")?;
        
        if self.speed_target_type == SpeedTargetValueType::Relative && self.target_entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Target entity reference is required for relative speed",
                "Call relative_target() with an entity reference"
            ));
        }
        
        if let Some(dynamics_value) = self.dynamics_value {
            match self.dynamics_dimension {
                DynamicsDimension::Time => validate_timing(dynamics_value, "dynamics time")?,
                DynamicsDimension::Distance => validate_distance(dynamics_value, "dynamics distance")?,
                _ => {} // Other dimensions don't need specific validation
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        let dynamics = TransitionDynamics {
            dynamics_dimension: self.dynamics_dimension,
            dynamics_shape: self.dynamics_shape,
            value: Double::literal(self.dynamics_value.unwrap_or(1.0)),
        };
        
        let target = match self.speed_target_type {
            SpeedTargetValueType::Absolute => SpeedActionTarget {
                absolute: Some(AbsoluteTargetSpeed {
                    value: Double::literal(self.target_speed.unwrap()),
                }),
                relative: None,
            },
            _ => SpeedActionTarget {
                absolute: None,
                relative: Some(RelativeTargetSpeed {
                    value: Double::literal(self.target_speed.unwrap()),
                    entity_ref: self.target_entity_ref.unwrap_or_else(|| "DefaultEntity".to_string()),
                    value_type: self.speed_target_type,
                    continuous: false,
                }),
            },
        };
        
        Ok(SpeedAction {
            speed_action_dynamics: dynamics,
            speed_action_target: target,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating synchronize actions
#[derive(Debug, Clone)]
pub struct SynchronizeActionBuilder {
    entity_ref: Option<String>,
    target_entity_ref: Option<String>,
    target_position: Option<Position>,
    target_position_master: Option<Position>,
}

impl SynchronizeActionBuilder {
    /// Create a new synchronize action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity_ref: None,
            target_position: None,
            target_position_master: None,
        }
    }
    
    /// Set the entity to synchronize
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity to synchronize with
    pub fn with_entity(mut self, target_entity_ref: impl Into<String>) -> Self {
        self.target_entity_ref = Some(target_entity_ref.into());
        self
    }
    
    /// Set the target position
    pub fn at_position(mut self, position: Position) -> Self {
        self.target_position = Some(position.clone());
        self.target_position_master = Some(position);
        self
    }
    
    /// Set separate target positions
    pub fn at_positions(mut self, target_position: Position, master_position: Position) -> Self {
        self.target_position = Some(target_position);
        self.target_position_master = Some(master_position);
        self
    }
}

impl ActionBuilder for SynchronizeActionBuilder {
    type ActionType = SynchronizeAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for synchronize action",
            "Call entity() to set the entity to synchronize"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_entity_ref = self.target_entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Target entity reference is required for synchronize action",
            "Call with_entity() to set the target entity"
        ))?;
        
        validate_entity_ref(target_entity_ref)?;
        
        if self.target_position.is_none() {
            return Err(BuilderError::validation_error(
                "Target position is required for synchronize action",
                "Call at_position() to set the target position"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(SynchronizeAction {
            master_entity_ref: OSString::literal(self.target_entity_ref.unwrap()),
            target_position_master: self.target_position_master.unwrap_or_else(|| self.target_position.clone().unwrap()),
            target_position: self.target_position.unwrap(),
            final_speed: None,
            target_tolerance_master: None,
            target_tolerance: None,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating follow trajectory actions
#[derive(Debug, Clone)]
pub struct FollowTrajectoryActionBuilder {
    entity_ref: Option<String>,
    trajectory: Option<Trajectory>,
    following_mode: Option<TrajectoryFollowingMode>,
}

impl FollowTrajectoryActionBuilder {
    /// Create a new follow trajectory action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            trajectory: None,
            following_mode: None,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the trajectory to follow
    pub fn trajectory(mut self, trajectory: Trajectory) -> Self {
        self.trajectory = Some(trajectory);
        self
    }
    
    /// Set the following mode
    pub fn following_mode(mut self, mode: FollowingMode) -> Self {
        self.following_mode = Some(TrajectoryFollowingMode { following_mode: mode });
        self
    }
}

impl ActionBuilder for FollowTrajectoryActionBuilder {
    type ActionType = FollowTrajectoryAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for follow trajectory action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.trajectory.is_none() {
            return Err(BuilderError::validation_error(
                "Trajectory is required for follow trajectory action",
                "Call trajectory() to set the trajectory to follow"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(FollowTrajectoryAction {
            trajectory: self.trajectory,
            catalog_reference: None,
            trajectory_following_mode: self.following_mode.unwrap_or_default(),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating lane change actions
#[derive(Debug, Clone)]
pub struct LaneChangeActionBuilder {
    entity_ref: Option<String>,
    target_lane_offset: Option<f64>,
    target_entity_ref: Option<String>,
    target_lane_value: Option<i32>,
    absolute_lane_id: Option<String>,
    dynamics_shape: DynamicsShape,
    dynamics_dimension: DynamicsDimension,
    dynamics_value: Option<f64>,
}

impl LaneChangeActionBuilder {
    /// Create a new lane change action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_lane_offset: None,
            target_entity_ref: None,
            target_lane_value: None,
            absolute_lane_id: None,
            dynamics_shape: DynamicsShape::Linear,
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_value: Some(3.0), // Default 3 seconds
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target lane offset
    pub fn target_lane_offset(mut self, offset: f64) -> Self {
        self.target_lane_offset = Some(offset);
        self
    }
    
    /// Set relative target lane
    pub fn relative_target_lane(mut self, entity_ref: impl Into<String>, lane_value: i32) -> Self {
        self.target_entity_ref = Some(entity_ref.into());
        self.target_lane_value = Some(lane_value);
        self.absolute_lane_id = None;
        self
    }
    
    /// Set absolute target lane
    pub fn absolute_target_lane(mut self, lane_id: impl Into<String>) -> Self {
        self.absolute_lane_id = Some(lane_id.into());
        self.target_entity_ref = None;
        self.target_lane_value = None;
        self
    }
    
    /// Set dynamics parameters
    pub fn dynamics(mut self, shape: DynamicsShape, dimension: DynamicsDimension, value: f64) -> Self {
        self.dynamics_shape = shape;
        self.dynamics_dimension = dimension;
        self.dynamics_value = Some(value);
        self
    }
}

impl ActionBuilder for LaneChangeActionBuilder {
    type ActionType = LaneChangeAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for lane change action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.target_entity_ref.is_none() && self.absolute_lane_id.is_none() {
            return Err(BuilderError::validation_error(
                "Either relative or absolute target lane is required",
                "Call relative_target_lane() or absolute_target_lane()"
            ));
        }
        
        if let Some(dynamics_value) = self.dynamics_value {
            match self.dynamics_dimension {
                DynamicsDimension::Time => validate_timing(dynamics_value, "lane change time")?,
                DynamicsDimension::Distance => validate_distance(dynamics_value, "lane change distance")?,
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        let dynamics = TransitionDynamics {
            dynamics_dimension: self.dynamics_dimension,
            dynamics_shape: self.dynamics_shape,
            value: Double::literal(self.dynamics_value.unwrap_or(3.0)),
        };
        
        let target = if let Some(entity_ref) = self.target_entity_ref {
            LaneChangeTarget {
                target_choice: LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane {
                    entity_ref: OSString::literal(entity_ref),
                    value: Int::literal(self.target_lane_value.unwrap_or(1)),
                }),
            }
        } else {
            LaneChangeTarget {
                target_choice: LaneChangeTargetChoice::AbsoluteTargetLane(AbsoluteTargetLane {
                    value: OSString::literal(self.absolute_lane_id.unwrap_or_else(|| "1".to_string())),
                }),
            }
        };
        
        Ok(LaneChangeAction {
            target_lane_offset: self.target_lane_offset.map(|v| Double::literal(v)),
            lane_change_action_dynamics: dynamics,
            lane_change_target: target,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating lane offset actions
#[derive(Debug, Clone)]
pub struct LaneOffsetActionBuilder {
    entity_ref: Option<String>,
    continuous: bool,
    offset: Option<f64>,
    relative_entity_ref: Option<String>,
    dynamics_shape: DynamicsShape,
    max_lateral_acc: Option<f64>,
}

impl LaneOffsetActionBuilder {
    /// Create a new lane offset action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            continuous: true,
            offset: None,
            relative_entity_ref: None,
            dynamics_shape: DynamicsShape::Linear,
            max_lateral_acc: None,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set whether the offset is continuous
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = continuous;
        self
    }
    
    /// Set absolute lane offset value
    pub fn absolute_offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self.relative_entity_ref = None;
        self
    }
    
    /// Set relative lane offset value
    pub fn relative_offset(mut self, offset: f64, entity_ref: impl Into<String>) -> Self {
        self.offset = Some(offset);
        self.relative_entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set dynamics parameters
    pub fn dynamics(mut self, shape: DynamicsShape, max_lateral_acc: Option<f64>) -> Self {
        self.dynamics_shape = shape;
        self.max_lateral_acc = max_lateral_acc;
        self
    }
}

impl ActionBuilder for LaneOffsetActionBuilder {
    type ActionType = LaneOffsetAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for lane offset action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.offset.is_none() {
            return Err(BuilderError::validation_error(
                "Offset value is required for lane offset action",
                "Call absolute_offset() or relative_offset() to set the lane offset value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        let dynamics = LaneOffsetActionDynamics {
            dynamics_shape: self.dynamics_shape,
            max_lateral_acc: self.max_lateral_acc.map(|v| Double::literal(v)),
        };
        
        let target = if let Some(entity_ref) = self.relative_entity_ref {
            LaneOffsetTarget {
                target_choice: LaneOffsetTargetChoice::RelativeTargetLaneOffset(RelativeTargetLaneOffset {
                    entity_ref: OSString::literal(entity_ref),
                    value: Double::literal(self.offset.unwrap()),
                }),
            }
        } else {
            LaneOffsetTarget {
                target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset {
                    value: Double::literal(self.offset.unwrap()),
                }),
            }
        };
        
        Ok(LaneOffsetAction {
            continuous: Boolean::literal(self.continuous),
            dynamics,
            target,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for TeleportActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SpeedActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SynchronizeActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FollowTrajectoryActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LaneChangeActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LaneOffsetActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::{Position, WorldPosition};
    
    #[test]
    fn test_teleport_action_builder() {
        let position = Position {
            world_position: Some(WorldPosition {
                x: Double::literal(10.0),
                y: Double::literal(20.0),
                z: Double::literal(0.0),
                h: Double::literal(0.0),
                p: Double::literal(0.0),
                r: Double::literal(0.0),
            }),
            ..Position::empty()
        };
        
        let action = TeleportActionBuilder::new()
            .entity("test_vehicle")
            .to_position(position.clone())
            .finish()
            .unwrap();
        
        assert_eq!(action.position.world_position.as_ref().unwrap().x.as_literal().unwrap(), &10.0);
    }
    
    #[test]
    fn test_speed_action_builder() {
        let action = SpeedActionBuilder::new()
            .entity("test_vehicle")
            .absolute_target(25.0)
            .over_time(3.0)
            .finish()
            .unwrap();
        
        assert_eq!(action.speed_action_target.absolute.unwrap().value.as_literal().unwrap(), &25.0);
        assert_eq!(action.speed_action_dynamics.dynamics_dimension, DynamicsDimension::Time);
        assert_eq!(action.speed_action_dynamics.value.as_literal().unwrap(), &3.0);
    }
    
    #[test]
    fn test_speed_action_builder_validation() {
        let result = SpeedActionBuilder::new()
            .entity("test_vehicle")
            // Missing target speed
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Target speed"));
    }
    
    #[test]
    fn test_synchronize_action_builder() {
        let position = Position {
            world_position: Some(WorldPosition {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.0),
                h: Double::literal(0.0),
                p: Double::literal(0.0),
                r: Double::literal(0.0),
            }),
            ..Position::empty()
        };
        
        let action = SynchronizeActionBuilder::new()
            .entity("ego")
            .with_entity("target")
            .at_position(position)
            .finish()
            .unwrap();
        
        assert_eq!(action.master_entity_ref.as_literal().unwrap(), &"target".to_string());
    }
    
    #[test]
    fn test_lane_change_action_builder() {
        let action = LaneChangeActionBuilder::new()
            .entity("test_vehicle")
            .relative_target_lane("ego", -1)
            .target_lane_offset(0.5)
            .finish()
            .unwrap();
        
        assert_eq!(action.target_lane_offset.unwrap().as_literal().unwrap(), &0.5);
        
        if let LaneChangeTargetChoice::RelativeTargetLane(rel) = action.lane_change_target.target_choice {
            assert_eq!(rel.entity_ref.as_literal().unwrap(), &"ego".to_string());
            assert_eq!(rel.value.as_literal().unwrap(), &-1);
        } else {
            panic!("Expected RelativeTargetLane");
        }
    }
    
    #[test]
    fn test_lane_offset_action_builder() {
        let action = LaneOffsetActionBuilder::new()
            .entity("test_vehicle")
            .absolute_offset(1.5)
            .continuous(false)
            .finish()
            .unwrap();
        
        assert_eq!(action.continuous.as_literal().unwrap(), &false);
        
        if let LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(abs) = action.target.target_choice {
            assert_eq!(abs.value.as_literal().unwrap(), &1.5);
        } else {
            panic!("Expected AbsoluteTargetLaneOffset");
        }
    }
}
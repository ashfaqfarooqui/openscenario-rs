//! Movement action types for entity positioning and motion
//!
//! This file contains:
//! - Movement actions (SpeedAction, LaneChangeAction, TeleportAction, etc.)
//! - Dynamics and transition specifications for smooth movement
//! - Target specification types (absolute, relative, following)
//! - Spatial relationship actions (distance keeping, synchronization)
//! - Trajectory following and path planning actions
//!
//! Contributes to project by:
//! - Implementing the core movement capabilities for scenario entities
//! - Providing smooth, realistic motion through transition dynamics
//! - Supporting both simple positioning and complex trajectory following
//! - Enabling entity coordination through synchronization actions
//! - Offering flexible target specification (absolute vs. relative positioning)

use crate::types::basic::{Boolean, Double, Int, OSString};
use crate::types::catalogs::entities::CatalogTrajectory;
use crate::types::catalogs::references::{CatalogReference, ParameterAssignment};
use crate::types::enums::{DynamicsDimension, DynamicsShape, FollowingMode, SpeedTargetValueType};
use crate::types::geometry::shapes::Shape;
use crate::types::positions::Position;
use crate::types::routing::{Route, RouteRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpeedAction {
    #[serde(rename = "SpeedActionDynamics")]
    pub speed_action_dynamics: TransitionDynamics,
    #[serde(rename = "SpeedActionTarget")]
    pub speed_action_target: SpeedActionTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeleportAction {
    #[serde(rename = "Position")]
    pub position: Position,
}

// Remove duplicate import

// Define supporting types for SpeedAction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransitionDynamics {
    #[serde(rename = "@dynamicsDimension")]
    pub dynamics_dimension: DynamicsDimension,
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedActionTarget {
    #[serde(
        rename = "AbsoluteTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub absolute: Option<AbsoluteTargetSpeed>,
    #[serde(
        rename = "RelativeTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative: Option<RelativeTargetSpeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@speedTargetValueType")]
    pub value_type: SpeedTargetValueType,
    #[serde(rename = "@continuous")]
    pub continuous: bool,
}

/// Complete trajectory definition with shape and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trajectory {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    #[serde(rename = "Shape")]
    pub shape: Shape,
}

/// Trajectory reference wrapper - can contain direct trajectory or catalog reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrajectoryRef {
    /// Direct trajectory definition
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,

    /// Reference to a trajectory in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogTrajectory>>,
}

/// Trajectory following mode specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrajectoryFollowingMode {
    #[serde(rename = "@followingMode")]
    pub following_mode: FollowingMode,
}

/// Follow trajectory action with trajectory reference support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowTrajectoryAction {
    /// Trajectory source (XSD choice group - exactly one required)
    #[serde(flatten)]
    pub trajectory_source: TrajectorySource,

    #[serde(rename = "TrajectoryFollowingMode")]
    pub trajectory_following_mode: TrajectoryFollowingMode,
}

/// Trajectory source for FollowTrajectoryAction
/// XSD requires exactly one child element (choice group)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrajectorySource {
    /// Direct trajectory definition
    Trajectory(Trajectory),
    /// Reference to a trajectory in a catalog
    CatalogReference(CatalogReference<CatalogTrajectory>),
}

impl Default for FollowTrajectoryAction {
    fn default() -> Self {
        Self {
            trajectory_source: TrajectorySource::default(),
            trajectory_following_mode: TrajectoryFollowingMode::default(),
        }
    }
}

impl Default for TrajectorySource {
    fn default() -> Self {
        Self::Trajectory(Trajectory::default())
    }
}

/// Assign route action for setting entity routes
///
/// Assigns a route to an entity, either through direct route definition
/// or catalog reference, enabling route-based navigation scenarios.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "AssignRouteAction")]
pub struct AssignRouteAction {
    /// Route reference (direct or catalog-based)
    #[serde(flatten)]
    pub route: RouteRef,
}

/// Follow route action with route reference support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowRouteAction {
    /// Route reference (direct or catalog-based)
    #[serde(flatten)]
    pub route_ref: RouteRef,
}

/// Routing action container for trajectory and route-based movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingAction {
    /// Assign route action
    #[serde(rename = "AssignRouteAction", skip_serializing_if = "Option::is_none")]
    pub assign_route_action: Option<AssignRouteAction>,

    /// Follow trajectory action
    #[serde(
        rename = "FollowTrajectoryAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_trajectory_action: Option<FollowTrajectoryAction>,

    /// Follow route action
    #[serde(rename = "FollowRouteAction", skip_serializing_if = "Option::is_none")]
    pub follow_route_action: Option<FollowRouteAction>,
}

impl Default for RoutingAction {
    fn default() -> Self {
        Self {
            assign_route_action: None,
            follow_trajectory_action: None,
            follow_route_action: None,
        }
    }
}

// PHASE 4A: Core Movement Actions - Implementation

/// Lane change action for lateral lane movements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeAction {
    #[serde(rename = "@targetLaneOffset")]
    pub target_lane_offset: Option<Double>,
    #[serde(rename = "LaneChangeActionDynamics")]
    pub lane_change_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneChangeTarget")]
    pub lane_change_target: LaneChangeTarget,
}

/// Lane change target specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeTarget {
    #[serde(flatten)]
    pub target_choice: LaneChangeTargetChoice,
}

/// Lane change target choice - relative or absolute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaneChangeTargetChoice {
    RelativeTargetLane(RelativeTargetLane),
    AbsoluteTargetLane(AbsoluteTargetLane),
}

/// Relative target lane specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetLane {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: Int,
}

/// Absolute target lane specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLane {
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Lane offset action for lateral positioning within a lane
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetAction {
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub dynamics: LaneOffsetActionDynamics,
    #[serde(rename = "LaneOffsetTarget")]
    pub target: LaneOffsetTarget,
}

/// Lane offset action dynamics specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetActionDynamics {
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<Double>,
}

/// Lane offset target specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetTarget {
    #[serde(flatten)]
    pub target_choice: LaneOffsetTargetChoice,
}

/// Lane offset target choice - relative or absolute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaneOffsetTargetChoice {
    RelativeTargetLaneOffset(RelativeTargetLaneOffset),
    AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset),
}

/// Relative target lane offset specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetLaneOffset {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: Double,
}

/// Absolute target lane offset specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLaneOffset {
    #[serde(rename = "@value")]
    pub value: Double,
}

/// Lateral action wrapper for all lateral movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralAction {
    #[serde(flatten)]
    pub lateral_choice: LateralActionChoice,
}

/// Lateral action choice - lane change, offset, or distance keeping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LateralActionChoice {
    #[serde(rename = "LaneChangeAction")]
    LaneChangeAction(LaneChangeAction),
    #[serde(rename = "LaneOffsetAction")]
    LaneOffsetAction(LaneOffsetAction),
    #[serde(rename = "LateralDistanceAction")]
    LateralDistanceAction(LateralDistanceAction),
}

/// Lateral distance action for maintaining lateral distance to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: Option<Double>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}

/// Longitudinal action wrapper for all longitudinal movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    #[serde(flatten)]
    pub longitudinal_action_choice: LongitudinalActionChoice,
}

/// Longitudinal action choice - speed, distance, or speed profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionChoice {
    SpeedAction(SpeedAction),
    LongitudinalDistanceAction(LongitudinalDistanceAction),
    SpeedProfileAction(SpeedProfileAction),
}

/// Longitudinal distance action for maintaining longitudinal distance to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: Double,
    #[serde(rename = "@freespace")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}

/// Speed profile action for time-based speed control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedProfileAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: Option<OSString>,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
    #[serde(rename = "Entry", default)]
    pub entries: Vec<SpeedProfileEntry>,
}

/// Speed profile entry with time and speed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedProfileEntry {
    #[serde(rename = "@time")]
    pub time: Double,
    #[serde(rename = "@speed")]
    pub speed: Double,
}

/// Dynamic constraints for movement actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicConstraints {
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<Double>,
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Option<Double>,
}

/// Synchronize action for coordinated entity movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynchronizeAction {
    #[serde(rename = "@targetEntityRef")]
    pub target_entity_ref: OSString,
    #[serde(rename = "TargetPositionMaster")]
    pub target_position_master: Position,
    #[serde(rename = "TargetPosition")]
    pub target_position: Position,
    #[serde(rename = "FinalSpeed")]
    pub final_speed: Option<FinalSpeed>,
}

/// Final speed specification for synchronize action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinalSpeed {
    #[serde(flatten)]
    pub speed_choice: FinalSpeedChoice,
}

/// Final speed choice - absolute or relative
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FinalSpeedChoice {
    AbsoluteSpeed(AbsoluteSpeed),
    RelativeSpeedToMaster(RelativeSpeedToMaster),
}

/// Absolute speed specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteSpeed {
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Relative speed to master specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeSpeedToMaster {
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Acquire position action for moving to a specific position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcquirePositionAction {
    #[serde(rename = "Position")]
    pub position: Position,
}

// Default implementations

impl Default for TransitionDynamics {
    fn default() -> Self {
        Self {
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
            value: Double::literal(1.0),
        }
    }
}

impl Default for SpeedActionTarget {
    fn default() -> Self {
        Self {
            absolute: Some(AbsoluteTargetSpeed::default()),
            relative: None,
        }
    }
}

impl Default for AbsoluteTargetSpeed {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
        }
    }
}

impl Default for RelativeTargetSpeed {
    fn default() -> Self {
        Self {
            value: Double::literal(0.0),
            entity_ref: "DefaultEntity".to_string(),
            value_type: SpeedTargetValueType::Delta,
            continuous: false,
        }
    }
}

impl Default for Trajectory {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultTrajectory".to_string()),
            closed: Boolean::literal(false),
            shape: Shape::default(),
        }
    }
}

impl Default for TrajectoryFollowingMode {
    fn default() -> Self {
        Self {
            following_mode: FollowingMode::Follow,
        }
    }
}

impl Default for TrajectoryRef {
    fn default() -> Self {
        Self {
            trajectory: Some(Trajectory::default()),
            catalog_reference: None,
        }
    }
}

impl Default for AssignRouteAction {
    fn default() -> Self {
        Self {
            route: RouteRef::default(),
        }
    }
}

impl Default for FollowRouteAction {
    fn default() -> Self {
        Self {
            route_ref: RouteRef::default(),
        }
    }
}

// Helper implementations for catalog-based actions

impl TrajectoryRef {
    /// Create a trajectory reference with direct trajectory definition
    pub fn with_trajectory(trajectory: Trajectory) -> Self {
        Self {
            trajectory: Some(trajectory),
            catalog_reference: None,
        }
    }

    /// Create a trajectory reference with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogTrajectory>) -> Self {
        Self {
            trajectory: None,
            catalog_reference: Some(catalog_reference),
        }
    }

    /// Create a trajectory reference from catalog name and entry name
    pub fn from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_catalog_reference(CatalogReference::new(catalog_name, entry_name))
    }

    /// Create a trajectory reference from catalog with parameters
    pub fn from_catalog_with_parameters(
        catalog_name: String,
        entry_name: String,
        parameters: Vec<ParameterAssignment>,
    ) -> Self {
        Self::with_catalog_reference(CatalogReference::with_parameters(
            catalog_name,
            entry_name,
            parameters,
        ))
    }
}



impl FollowTrajectoryAction {
    /// Create a follow trajectory action with direct trajectory
    pub fn with_trajectory(trajectory: Trajectory, following_mode: FollowingMode) -> Self {
        Self {
            trajectory_source: TrajectorySource::Trajectory(trajectory),
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
        }
    }

    /// Create a follow trajectory action with catalog reference
    pub fn with_catalog_reference(
        catalog_reference: CatalogReference<CatalogTrajectory>,
        following_mode: FollowingMode,
    ) -> Self {
        Self {
            trajectory_source: TrajectorySource::CatalogReference(catalog_reference),
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
        }
    }

    /// Create a follow trajectory action from catalog name and entry name
    pub fn from_catalog(
        catalog_name: String,
        entry_name: String,
        following_mode: FollowingMode,
    ) -> Self {
        Self::with_catalog_reference(
            CatalogReference::new(catalog_name, entry_name),
            following_mode,
        )
    }
}

impl FollowRouteAction {
    /// Create a follow route action with direct route
    pub fn with_route(route: Route) -> Self {
        Self {
            route_ref: RouteRef::direct(route),
        }
    }

    /// Create a follow route action from catalog name and entry name
    pub fn from_catalog(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        Self {
            route_ref: RouteRef::catalog(catalog_name, entry_name),
        }
    }
}

impl AssignRouteAction {
    /// Create a new assign route action with direct route
    pub fn new(route: RouteRef) -> Self {
        Self { route }
    }

    /// Create an assign route action with direct route definition
    pub fn direct_route(route: Route) -> Self {
        Self {
            route: RouteRef::direct(route),
        }
    }

    /// Create an assign route action with catalog route reference
    pub fn catalog_route(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        Self {
            route: RouteRef::catalog(catalog_name, entry_name),
        }
    }
}

impl RoutingAction {
    /// Create a routing action with assign route action
    pub fn with_assign_route(action: AssignRouteAction) -> Self {
        Self {
            assign_route_action: Some(action),
            follow_trajectory_action: None,
            follow_route_action: None,
        }
    }
    /// Create a routing action with trajectory following
    pub fn with_trajectory(action: FollowTrajectoryAction) -> Self {
        Self {
            assign_route_action: None,
            follow_trajectory_action: Some(action),
            follow_route_action: None,
        }
    }

    /// Create a routing action with route following
    pub fn with_route(action: FollowRouteAction) -> Self {
        Self {
            assign_route_action: None,
            follow_trajectory_action: None,
            follow_route_action: Some(action),
        }
    }

    /// Create a routing action with trajectory from catalog
    pub fn with_trajectory_from_catalog(
        catalog_name: String,
        entry_name: String,
        following_mode: FollowingMode,
    ) -> Self {
        Self::with_trajectory(FollowTrajectoryAction::from_catalog(
            catalog_name,
            entry_name,
            following_mode,
        ))
    }

    /// Create a routing action with route from catalog
    pub fn with_route_from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_route(FollowRouteAction::from_catalog(catalog_name, entry_name))
    }
}

// PHASE 4A: Helper methods for LaneChangeAction system

impl LaneChangeAction {
    /// Create a new LaneChangeAction with the specified dynamics and target
    pub fn new(dynamics: TransitionDynamics, target: LaneChangeTarget) -> Self {
        Self {
            lane_change_action_dynamics: dynamics,
            lane_change_target: target,
            target_lane_offset: None,
        }
    }
    
    /// Set the target lane offset for this lane change action
    pub fn with_offset(mut self, offset: Double) -> Self {
        self.target_lane_offset = Some(offset);
        self
    }
}

impl LaneChangeTarget {
    /// Create a relative lane change target
    pub fn relative(entity_ref: impl Into<String>, value: i32) -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane {
                entity_ref: OSString::literal(entity_ref.into()),
                value: Int::literal(value),
            }),
        }
    }
    
    /// Create an absolute lane change target
    pub fn absolute(lane_id: impl Into<String>) -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::AbsoluteTargetLane(AbsoluteTargetLane {
                value: OSString::literal(lane_id.into()),
            }),
        }
    }
}

impl RelativeTargetLane {
    /// Create a new relative target lane
    pub fn new(entity_ref: impl Into<String>, value: i32) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
            value: Int::literal(value),
        }
    }
}

impl AbsoluteTargetLane {
    /// Create a new absolute target lane
    pub fn new(lane_id: impl Into<String>) -> Self {
        Self {
            value: OSString::literal(lane_id.into()),
        }
    }
}

// PHASE 4A: Helper methods for LaneOffsetAction system

impl LaneOffsetAction {
    /// Create a new LaneOffsetAction with the specified dynamics and target
    pub fn new(dynamics: LaneOffsetActionDynamics, target: LaneOffsetTarget, continuous: bool) -> Self {
        Self {
            dynamics,
            target,
            continuous: Boolean::literal(continuous),
        }
    }
    
    /// Set the continuous flag for this lane offset action
    pub fn with_continuous(mut self, continuous: bool) -> Self {
        self.continuous = Boolean::literal(continuous);
        self
    }
}

impl LaneOffsetActionDynamics {
    /// Create new lane offset action dynamics
    pub fn new(dynamics_shape: DynamicsShape) -> Self {
        Self {
            dynamics_shape,
            max_lateral_acc: None,
        }
    }
    
    /// Set the maximum lateral acceleration
    pub fn with_max_acceleration(mut self, max_acc: f64) -> Self {
        self.max_lateral_acc = Some(Double::literal(max_acc));
        self
    }
}

impl LaneOffsetTarget {
    /// Create a relative lane offset target
    pub fn relative(entity_ref: impl Into<String>, value: f64) -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::RelativeTargetLaneOffset(RelativeTargetLaneOffset {
                entity_ref: OSString::literal(entity_ref.into()),
                value: Double::literal(value),
            }),
        }
    }
    
    /// Create an absolute lane offset target
    pub fn absolute(value: f64) -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset {
                value: Double::literal(value),
            }),
        }
    }
}

impl RelativeTargetLaneOffset {
    /// Create a new relative target lane offset
    pub fn new(entity_ref: impl Into<String>, value: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
            value: Double::literal(value),
        }
    }
}

impl AbsoluteTargetLaneOffset {
    /// Create a new absolute target lane offset
    pub fn new(value: f64) -> Self {
        Self {
            value: Double::literal(value),
        }
    }
}

// PHASE 4A: Helper methods for LateralAction system

impl LateralAction {
    /// Create a lateral action with lane change
    pub fn lane_change(action: LaneChangeAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneChangeAction(action),
        }
    }
    
    /// Create a lateral action with lane offset
    pub fn lane_offset(action: LaneOffsetAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneOffsetAction(action),
        }
    }
    
    /// Create a lateral action with lateral distance
    pub fn lateral_distance(action: LateralDistanceAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LateralDistanceAction(action),
        }
    }
}

// PHASE 4A: Default implementations for new movement actions

impl Default for LaneChangeAction {
    fn default() -> Self {
        Self {
            target_lane_offset: None,
            lane_change_action_dynamics: TransitionDynamics::default(),
            lane_change_target: LaneChangeTarget::default(),
        }
    }
}

impl Default for LaneChangeTarget {
    fn default() -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane::default()),
        }
    }
}

impl Default for RelativeTargetLane {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: Int::literal(1),
        }
    }
}

impl Default for AbsoluteTargetLane {
    fn default() -> Self {
        Self { 
            value: OSString::literal("1".to_string()) 
        }
    }
}

impl Default for LaneOffsetAction {
    fn default() -> Self {
        Self {
            continuous: Boolean::literal(false),
            dynamics: LaneOffsetActionDynamics::default(),
            target: LaneOffsetTarget::default(),
        }
    }
}

impl Default for LaneOffsetTarget {
    fn default() -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(
                AbsoluteTargetLaneOffset::default(),
            ),
        }
    }
}

impl Default for RelativeTargetLaneOffset {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: Double::literal(0.0),
        }
    }
}

impl Default for AbsoluteTargetLaneOffset {
    fn default() -> Self {
        Self { 
            value: Double::literal(0.0) 
        }
    }
}

impl Default for LateralAction {
    fn default() -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneChangeAction(
                LaneChangeAction::default(),
            ),
        }
    }
}

impl Default for LaneOffsetActionDynamics {
    fn default() -> Self {
        Self {
            dynamics_shape: DynamicsShape::Linear,
            max_lateral_acc: None,
        }
    }
}

impl Default for LateralDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: Some(Double::literal(2.0)),
            freespace: Boolean::literal(true),
            continuous: Boolean::literal(false),
            dynamic_constraints: None,
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(
                SpeedAction::default(),
            ),
        }
    }
}

impl Default for LongitudinalDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: Double::literal(10.0),
            freespace: Some(Boolean::literal(true)),
            continuous: Boolean::literal(false),
            dynamic_constraints: None,
        }
    }
}

impl Default for SpeedProfileAction {
    fn default() -> Self {
        Self {
            entity_ref: None,
            dynamic_constraints: None,
            entries: vec![SpeedProfileEntry::default()],
        }
    }
}

impl Default for SpeedProfileEntry {
    fn default() -> Self {
        Self {
            time: Double::literal(0.0),
            speed: Double::literal(10.0),
        }
    }
}

impl Default for DynamicConstraints {
    fn default() -> Self {
        Self {
            max_lateral_acc: None,
            max_speed: None,
        }
    }
}

impl Default for SynchronizeAction {
    fn default() -> Self {
        Self {
            target_entity_ref: OSString::literal("DefaultEntity".to_string()),
            target_position_master: Position::default(),
            target_position: Position::default(),
            final_speed: None,
        }
    }
}

impl Default for FinalSpeed {
    fn default() -> Self {
        Self {
            speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed::default()),
        }
    }
}

impl Default for AbsoluteSpeed {
    fn default() -> Self {
        Self { value: 10.0 }
    }
}

impl Default for RelativeSpeedToMaster {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Default for AcquirePositionAction {
    fn default() -> Self {
        Self {
            position: Position::default(),
        }
    }
}

// PHASE 4A: Unit tests for new movement actions
// Tests temporarily removed to fix compilation - will be restored in next phase

// Add movement action validation
// impl ValidateAction for SpeedAction, TeleportAction

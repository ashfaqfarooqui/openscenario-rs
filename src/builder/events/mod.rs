//! Event builders for programmatic event construction
//!
//! This module provides fluent APIs for creating events with actions and triggers
//! with comprehensive validation and type safety.

use crate::types::scenario::story::{Event, StoryAction, StoryPrivateAction};
use crate::types::scenario::triggers::Trigger;
use crate::types::basic::{OSString, UnsignedInt};
use crate::types::enums::Priority;
use crate::builder::{BuilderError, BuilderResult};
use crate::builder::triggers::TriggerBuilder;
use crate::builder::actions::UnifiedActionBuilder;

/// Builder for creating events with actions and triggers
pub struct EventBuilder {
    name: Option<String>,
    maximum_execution_count: Option<u32>,
    priority: Option<Priority>,
    action: Option<StoryAction>,
    start_trigger: Option<Trigger>,
}

impl EventBuilder {
    /// Create a new event builder
    pub fn new() -> Self {
        Self {
            name: None,
            maximum_execution_count: None,
            priority: None,
            action: None,
            start_trigger: None,
        }
    }

    /// Set the event name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the maximum execution count
    pub fn max_execution_count(mut self, count: u32) -> Self {
        self.maximum_execution_count = Some(count);
        self
    }

    /// Set the event priority
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Add an action to this event
    pub fn add_action(mut self) -> EventActionBuilder {
        EventActionBuilder::new(self)
    }

    /// Set a pre-built action
    pub fn with_action(mut self, action: StoryAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Add a start trigger to this event
    pub fn with_start_trigger(mut self) -> EventTriggerBuilder {
        EventTriggerBuilder::new(self)
    }

    /// Set a pre-built start trigger
    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    /// Finish building the event
    pub fn finish(self) -> BuilderResult<Event> {
        self.validate()?;

        let name = self.name.unwrap_or_else(|| "DefaultEvent".to_string());
        let action = self.action.unwrap_or_else(|| StoryAction {
            name: OSString::literal("DefaultAction".to_string()),
            private_action: Some(StoryPrivateAction::default()),
        });

        Ok(Event {
            name: OSString::literal(name),
            maximum_execution_count: self.maximum_execution_count.map(|c| UnsignedInt::literal(c)),
            priority: self.priority,
            action,
            start_trigger: self.start_trigger,
        })
    }

    /// Validate the event configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.action.is_none() {
            return Err(BuilderError::validation_error(
                "Event must have an action",
                "Add an action using add_action() or with_action()"
            ));
        }

        Ok(())
    }

    /// Internal method to set the action
    fn set_action(mut self, action: StoryAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Internal method to set the start trigger
    fn set_start_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }
}

impl Default for EventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating actions within events
pub struct EventActionBuilder {
    parent: EventBuilder,
    action_name: Option<String>,
    private_action: StoryPrivateAction,
}

impl EventActionBuilder {
    /// Create a new event action builder
    fn new(parent: EventBuilder) -> Self {
        Self {
            parent,
            action_name: None,
            private_action: StoryPrivateAction::default(),
        }
    }

    /// Set the action name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.action_name = Some(name.into());
        self
    }

    /// Add a teleport action
    pub fn teleport(mut self, entity_ref: impl Into<String>) -> TeleportActionBuilder {
        TeleportActionBuilder::new(self, entity_ref.into())
    }

    /// Add a longitudinal action
    pub fn longitudinal(mut self, entity_ref: impl Into<String>) -> LongitudinalActionBuilder {
        LongitudinalActionBuilder::new(self, entity_ref.into())
    }

    /// Add a lateral action
    pub fn lateral(mut self, entity_ref: impl Into<String>) -> LateralActionBuilder {
        LateralActionBuilder::new(self, entity_ref.into())
    }

    /// Add a controller action
    pub fn controller(mut self, entity_ref: impl Into<String>) -> ControllerActionBuilder {
        ControllerActionBuilder::new(self, entity_ref.into())
    }

    /// Add an appearance action
    pub fn appearance(mut self, entity_ref: impl Into<String>) -> AppearanceActionBuilder {
        AppearanceActionBuilder::new(self, entity_ref.into())
    }

    /// Finish building the action and return to event builder
    pub fn finish_action(self) -> EventBuilder {
        let action = StoryAction {
            name: OSString::literal(self.action_name.unwrap_or_else(|| "DefaultAction".to_string())),
            private_action: Some(self.private_action),
        };
        self.parent.set_action(action)
    }

    /// Internal method to set the private action
    fn set_private_action(mut self, private_action: StoryPrivateAction) -> Self {
        self.private_action = private_action;
        self
    }
}

/// Builder for teleport actions within events
pub struct TeleportActionBuilder {
    parent: EventActionBuilder,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl TeleportActionBuilder {
    fn new(parent: EventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            position: None,
        }
    }

    /// Set the target position
    pub fn to_position(mut self) -> crate::builder::positions::UnifiedPositionBuilder<Self> {
        crate::builder::positions::UnifiedPositionBuilder::new(self)
    }

    /// Set a pre-built position
    pub fn with_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Finish building the teleport action
    pub fn finish_action(self) -> BuilderResult<EventActionBuilder> {
        let position = self.position.ok_or_else(|| {
            BuilderError::validation_error(
                "Teleport action requires a position",
                "Use to_position() to specify the target position"
            )
        })?;

        use crate::types::actions::movement::TeleportAction;
        let teleport_action = TeleportAction {
            position,
        };

        let mut private_action = self.parent.private_action.clone();
        private_action.teleport_action = Some(teleport_action);

        Ok(self.parent.set_private_action(private_action))
    }
}

// Implement PositionTarget for TeleportActionBuilder
impl crate::builder::positions::PositionTarget for TeleportActionBuilder {
    fn set_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// Builder for longitudinal actions within events
pub struct LongitudinalActionBuilder {
    parent: EventActionBuilder,
    entity_ref: String,
    action_type: Option<LongitudinalActionType>,
}

#[derive(Debug, Clone)]
enum LongitudinalActionType {
    Speed { target: f64, dynamics: Option<crate::types::actions::movement::TransitionDynamics> },
    Distance { target: f64, dynamics: Option<crate::types::actions::movement::TransitionDynamics> },
}

impl LongitudinalActionBuilder {
    fn new(parent: EventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            action_type: None,
        }
    }

    /// Create a speed action
    pub fn speed_action(mut self) -> SpeedActionBuilder {
        SpeedActionBuilder::new(self)
    }

    /// Create a distance action
    pub fn distance_action(mut self) -> DistanceActionBuilder {
        DistanceActionBuilder::new(self)
    }

    /// Finish building the longitudinal action
    pub fn finish_action(self) -> BuilderResult<EventActionBuilder> {
        let action_type = self.action_type.ok_or_else(|| {
            BuilderError::validation_error(
                "Longitudinal action requires an action type",
                "Use speed_action() or distance_action()"
            )
        })?;

        use crate::types::scenario::init::LongitudinalAction;
        use crate::types::actions::movement::{SpeedAction, LongitudinalDistanceAction, SpeedActionDynamics, AbsoluteSpeedAction};

        let longitudinal_action = match action_type {
            LongitudinalActionType::Speed { target, dynamics } => {
                let speed_action = SpeedAction {
                    speed_action_dynamics: SpeedActionDynamics {
                        dynamics_shape: crate::types::enums::DynamicsShape::Linear,
                        value: crate::types::basic::Value::literal(1.0), // Default rate
                        following_mode: None,
                    },
                    speed_target: crate::types::actions::movement::SpeedTarget {
                        relative_speed_to_master: None,
                        absolute_speed: Some(AbsoluteSpeedAction {
                            value: crate::types::basic::Value::literal(target),
                        }),
                    },
                };
                LongitudinalAction {
                    speed_action: Some(speed_action),
                    longitudinal_distance_action: None,
                }
            },
            LongitudinalActionType::Distance { target, dynamics } => {
                let distance_action = LongitudinalDistanceAction {
                    value: crate::types::basic::Value::literal(target),
                    freespace: crate::types::basic::Value::literal(true),
                    continuous: crate::types::basic::Value::literal(true),
                    displacement: None,
                    coordinate_system: None,
                    dynamics: dynamics,
                };
                LongitudinalAction {
                    speed_action: None,
                    longitudinal_distance_action: Some(distance_action),
                }
            },
        };

        let mut private_action = self.parent.private_action.clone();
        private_action.longitudinal_action = Some(longitudinal_action);

        Ok(self.parent.set_private_action(private_action))
    }

    /// Internal method to set the action type
    fn set_action_type(mut self, action_type: LongitudinalActionType) -> Self {
        self.action_type = Some(action_type);
        self
    }
}

/// Builder for speed actions
pub struct SpeedActionBuilder {
    parent: LongitudinalActionBuilder,
    target: Option<f64>,
    dynamics: Option<crate::types::actions::movement::TransitionDynamics>,
}

impl SpeedActionBuilder {
    fn new(parent: LongitudinalActionBuilder) -> Self {
        Self {
            parent,
            target: None,
            dynamics: None,
        }
    }

    /// Set absolute target speed
    pub fn absolute_target(mut self, speed: f64) -> Self {
        self.target = Some(speed);
        self
    }

    /// Set transition dynamics
    pub fn with_dynamics(mut self, dynamics: crate::types::actions::movement::TransitionDynamics) -> Self {
        self.dynamics = Some(dynamics);
        self
    }

    /// Finish building the speed action
    pub fn finish_action(self) -> BuilderResult<LongitudinalActionBuilder> {
        let target = self.target.ok_or_else(|| {
            BuilderError::validation_error(
                "Speed action requires a target speed",
                "Use absolute_target() to set the target speed"
            )
        })?;

        let action_type = LongitudinalActionType::Speed {
            target,
            dynamics: self.dynamics,
        };

        Ok(self.parent.set_action_type(action_type))
    }
}

/// Builder for distance actions
pub struct DistanceActionBuilder {
    parent: LongitudinalActionBuilder,
    target: Option<f64>,
    dynamics: Option<crate::types::actions::movement::TransitionDynamics>,
}

impl DistanceActionBuilder {
    fn new(parent: LongitudinalActionBuilder) -> Self {
        Self {
            parent,
            target: None,
            dynamics: None,
        }
    }

    /// Set target distance
    pub fn target_distance(mut self, distance: f64) -> Self {
        self.target = Some(distance);
        self
    }

    /// Set transition dynamics
    pub fn with_dynamics(mut self, dynamics: crate::types::actions::movement::TransitionDynamics) -> Self {
        self.dynamics = Some(dynamics);
        self
    }

    /// Finish building the distance action
    pub fn finish_action(self) -> BuilderResult<LongitudinalActionBuilder> {
        let target = self.target.ok_or_else(|| {
            BuilderError::validation_error(
                "Distance action requires a target distance",
                "Use target_distance() to set the target distance"
            )
        })?;

        let action_type = LongitudinalActionType::Distance {
            target,
            dynamics: self.dynamics,
        };

        Ok(self.parent.set_action_type(action_type))
    }
}

/// Builder for lateral actions within events
pub struct LateralActionBuilder {
    parent: EventActionBuilder,
    entity_ref: String,
}

impl LateralActionBuilder {
    fn new(parent: EventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the lateral action (placeholder)
    pub fn finish_action(self) -> EventActionBuilder {
        // For now, just return the parent without adding a lateral action
        // This can be expanded later with specific lateral action types
        self.parent
    }
}

/// Builder for controller actions within events
pub struct ControllerActionBuilder {
    parent: EventActionBuilder,
    entity_ref: String,
}

impl ControllerActionBuilder {
    fn new(parent: EventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the controller action (placeholder)
    pub fn finish_action(self) -> EventActionBuilder {
        // For now, just return the parent without adding a controller action
        // This can be expanded later with specific controller action types
        self.parent
    }
}

/// Builder for appearance actions within events
pub struct AppearanceActionBuilder {
    parent: EventActionBuilder,
    entity_ref: String,
}

impl AppearanceActionBuilder {
    fn new(parent: EventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the appearance action (placeholder)
    pub fn finish_action(self) -> EventActionBuilder {
        // For now, just return the parent without adding an appearance action
        // This can be expanded later with specific appearance action types
        self.parent
    }
}

/// Builder for creating triggers within events
pub struct EventTriggerBuilder {
    parent: EventBuilder,
    trigger_builder: TriggerBuilder,
}

impl EventTriggerBuilder {
    /// Create a new event trigger builder
    fn new(parent: EventBuilder) -> Self {
        Self {
            parent,
            trigger_builder: TriggerBuilder::new(),
        }
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> EventSimulationTimeConditionBuilder {
        EventSimulationTimeConditionBuilder::new(self)
    }

    /// Add a speed condition
    pub fn speed(self) -> EventSpeedConditionBuilder {
        EventSpeedConditionBuilder::new(self)
    }

    /// Add a distance condition
    pub fn distance(self) -> EventDistanceConditionBuilder {
        EventDistanceConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<EventBuilder> {
        let trigger = self.trigger_builder.finish()?;
        Ok(self.parent.set_start_trigger(trigger))
    }

    /// Internal method to add a condition
    fn add_condition(mut self, condition: crate::types::scenario::triggers::Condition) -> Self {
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

/// Builder for simulation time conditions in event triggers
pub struct EventSimulationTimeConditionBuilder {
    parent: EventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    value: Option<f64>,
    rule: Option<String>,
}

impl EventSimulationTimeConditionBuilder {
    fn new(parent: EventTriggerBuilder) -> Self {
        Self {
            parent,
            name: None,
            edge: crate::types::enums::ConditionEdge::Rising,
            delay: 0.0,
            value: None,
            rule: None,
        }
    }

    /// Set the condition name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the condition edge
    pub fn edge(mut self, edge: crate::types::enums::ConditionEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Set the condition delay
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("greaterThan".to_string());
        self
    }

    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("lessThan".to_string());
        self
    }

    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("equalTo".to_string());
        self
    }

    /// Finish building the condition
    pub fn finish_condition(self) -> BuilderResult<EventTriggerBuilder> {
        let condition = self.build_condition()?;
        Ok(self.parent.add_condition(condition))
    }

    fn build_condition(&self) -> BuilderResult<crate::types::scenario::triggers::Condition> {
        let value = self.value.ok_or_else(|| {
            BuilderError::validation_error(
                "Simulation time condition requires a value",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        let rule = self.rule.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Simulation time condition requires a rule",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        // Create the condition using the existing condition types
        use crate::types::conditions::{ByValueCondition, SimulationTimeCondition};
        use crate::types::enums::Rule;

        let rule_enum = match rule.as_str() {
            "greaterThan" => Rule::GreaterThan,
            "lessThan" => Rule::LessThan,
            "equalTo" => Rule::EqualTo,
            _ => return Err(BuilderError::validation_error(
                "Invalid rule for simulation time condition",
                "Use greater_than(), less_than(), or equal_to()"
            )),
        };

        let sim_time_condition = SimulationTimeCondition {
            value: crate::types::basic::Value::literal(value),
            rule: rule_enum,
        };

        let by_value_condition = ByValueCondition {
            simulation_time_condition: Some(sim_time_condition),
            parameter_condition: None,
            variable_condition: None,
            storyboard_element_state_condition: None,
            user_defined_value_condition: None,
            traffic_signal_condition: None,
            traffic_signal_controller_condition: None,
        };

        Ok(crate::types::scenario::triggers::Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "SimTimeCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(crate::types::basic::Double::literal(self.delay)) } else { None },
            by_value_condition: Some(by_value_condition),
            by_entity_condition: None,
        })
    }
}

/// Builder for speed conditions in event triggers
pub struct EventSpeedConditionBuilder {
    parent: EventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Option<String>,
}

impl EventSpeedConditionBuilder {
    fn new(parent: EventTriggerBuilder) -> Self {
        Self {
            parent,
            name: None,
            edge: crate::types::enums::ConditionEdge::Rising,
            delay: 0.0,
            entity_ref: None,
            value: None,
            rule: None,
        }
    }

    /// Set the condition name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the condition edge
    pub fn edge(mut self, edge: crate::types::enums::ConditionEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Set the condition delay
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Set the entity reference
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }

    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("greaterThan".to_string());
        self
    }

    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("lessThan".to_string());
        self
    }

    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("equalTo".to_string());
        self
    }

    /// Finish building the condition
    pub fn finish_condition(self) -> BuilderResult<EventTriggerBuilder> {
        let condition = self.build_condition()?;
        Ok(self.parent.add_condition(condition))
    }

    fn build_condition(&self) -> BuilderResult<crate::types::scenario::triggers::Condition> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Speed condition requires an entity reference",
                "Use entity() to specify which entity to monitor"
            )
        })?;

        let value = self.value.ok_or_else(|| {
            BuilderError::validation_error(
                "Speed condition requires a value",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        let rule = self.rule.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Speed condition requires a rule",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        // Create the condition using the existing condition types
        use crate::types::conditions::{ByEntityCondition, SpeedCondition};
        use crate::types::enums::Rule;
        use crate::types::scenario::triggers::{TriggeringEntities, EntityRef};

        let rule_enum = match rule.as_str() {
            "greaterThan" => Rule::GreaterThan,
            "lessThan" => Rule::LessThan,
            "equalTo" => Rule::EqualTo,
            _ => return Err(BuilderError::validation_error(
                "Invalid rule for speed condition",
                "Use greater_than(), less_than(), or equal_to()"
            )),
        };

        let speed_condition = SpeedCondition {
            value: crate::types::basic::Value::literal(*value),
            rule: rule_enum,
        };

        let triggering_entities = TriggeringEntities {
            triggering_entities_rule: crate::types::enums::TriggeringEntitiesRule::Any,
            entity_refs: vec![EntityRef {
                entity_ref: OSString::literal(entity_ref.clone()),
            }],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            time_headway_condition: None,
            time_to_collision_condition: None,
            acceleration_condition: None,
            stand_still_condition: None,
            speed_condition: Some(speed_condition),
            relative_speed_condition: None,
            traveled_distance_condition: None,
            reach_position_condition: None,
            distance_condition: None,
            relative_distance_condition: None,
            end_of_road_condition: None,
            collision_condition: None,
            offroad_condition: None,
            user_defined_value_condition: None,
        };

        Ok(crate::types::scenario::triggers::Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "SpeedCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(crate::types::basic::Double::literal(self.delay)) } else { None },
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for distance conditions in event triggers
pub struct EventDistanceConditionBuilder {
    parent: EventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    position: Option<crate::types::positions::Position>,
    value: Option<f64>,
    rule: Option<String>,
    freespace: bool,
}

impl EventDistanceConditionBuilder {
    fn new(parent: EventTriggerBuilder) -> Self {
        Self {
            parent,
            name: None,
            edge: crate::types::enums::ConditionEdge::Rising,
            delay: 0.0,
            entity_ref: None,
            position: None,
            value: None,
            rule: None,
            freespace: true,
        }
    }

    /// Set the condition name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the condition edge
    pub fn edge(mut self, edge: crate::types::enums::ConditionEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Set the condition delay
    pub fn delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Set the entity reference
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }

    /// Set the target position
    pub fn to_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Set whether to use freespace calculation
    pub fn freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }

    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("greaterThan".to_string());
        self
    }

    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("lessThan".to_string());
        self
    }

    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some("equalTo".to_string());
        self
    }

    /// Finish building the condition
    pub fn finish_condition(self) -> BuilderResult<EventTriggerBuilder> {
        let condition = self.build_condition()?;
        Ok(self.parent.add_condition(condition))
    }

    fn build_condition(&self) -> BuilderResult<crate::types::scenario::triggers::Condition> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Distance condition requires an entity reference",
                "Use entity() to specify which entity to monitor"
            )
        })?;

        let position = self.position.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Distance condition requires a target position",
                "Use to_position() to specify the target position"
            )
        })?;

        let value = self.value.ok_or_else(|| {
            BuilderError::validation_error(
                "Distance condition requires a value",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        let rule = self.rule.as_ref().ok_or_else(|| {
            BuilderError::validation_error(
                "Distance condition requires a rule",
                "Use greater_than(), less_than(), or equal_to()"
            )
        })?;

        // Create the condition using the existing condition types
        use crate::types::conditions::{ByEntityCondition, DistanceCondition};
        use crate::types::enums::Rule;
        use crate::types::scenario::triggers::{TriggeringEntities, EntityRef};

        let rule_enum = match rule.as_str() {
            "greaterThan" => Rule::GreaterThan,
            "lessThan" => Rule::LessThan,
            "equalTo" => Rule::EqualTo,
            _ => return Err(BuilderError::validation_error(
                "Invalid rule for distance condition",
                "Use greater_than(), less_than(), or equal_to()"
            )),
        };

        let distance_condition = DistanceCondition {
            value: crate::types::basic::Value::literal(value),
            freespace: crate::types::basic::Value::literal(self.freespace),
            rule: rule_enum,
            position: position.clone(),
        };

        let triggering_entities = TriggeringEntities {
            triggering_entities_rule: crate::types::enums::TriggeringEntitiesRule::Any,
            entity_refs: vec![EntityRef {
                entity_ref: OSString::literal(entity_ref.clone()),
            }],
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            time_headway_condition: None,
            time_to_collision_condition: None,
            acceleration_condition: None,
            stand_still_condition: None,
            speed_condition: None,
            relative_speed_condition: None,
            traveled_distance_condition: None,
            reach_position_condition: None,
            distance_condition: Some(distance_condition),
            relative_distance_condition: None,
            end_of_road_condition: None,
            collision_condition: None,
            offroad_condition: None,
            user_defined_value_condition: None,
        };

        Ok(crate::types::scenario::triggers::Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "DistanceCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(crate::types::basic::Double::literal(self.delay)) } else { None },
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}
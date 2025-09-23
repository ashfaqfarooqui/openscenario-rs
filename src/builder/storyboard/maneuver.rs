//! Maneuver builder for programmatic maneuver construction
//!
//! This module provides fluent APIs for creating maneuvers with events and parameters
//! with comprehensive validation and type safety.

use crate::types::scenario::story::{Maneuver, Event};
use crate::types::basic::{OSString, ParameterDeclarations};
use crate::builder::{BuilderError, BuilderResult};
use crate::builder::events::EventBuilder;
use super::act::ManeuverGroupBuilder;

/// Builder for creating maneuvers within maneuver groups
pub struct ManeuverBuilder {
    parent: ManeuverGroupBuilder,
    name: String,
    parameter_declarations: Option<ParameterDeclarations>,
    events: Vec<Event>,
}

impl ManeuverBuilder {
    /// Create a new maneuver builder
    pub(super) fn new(parent: ManeuverGroupBuilder, name: String) -> Self {
        Self {
            parent,
            name,
            parameter_declarations: None,
            events: Vec::new(),
        }
    }

    /// Set parameter declarations for this maneuver
    pub fn with_parameters(mut self, parameters: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(parameters);
        self
    }

    /// Add an event to this maneuver
    pub fn add_event(mut self, name: impl Into<String>) -> ManeuverEventBuilder {
        ManeuverEventBuilder::new(self, name.into())
    }

    /// Add a pre-built event
    pub fn with_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    /// Finish building the maneuver and return to maneuver group builder
    pub fn finish_maneuver(self) -> BuilderResult<ManeuverGroupBuilder> {
        self.validate()?;

        let maneuver = Maneuver {
            name: OSString::literal(self.name),
            parameter_declarations: self.parameter_declarations,
            events: self.events,
        };

        Ok(self.parent.add_completed_maneuver(maneuver))
    }

    /// Validate the maneuver configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.events.is_empty() {
            return Err(BuilderError::validation_error(
                &format!("Maneuver '{}' has no events", self.name),
                "Add at least one event using add_event()"
            ));
        }

        // Validate event names are unique
        let mut event_names = std::collections::HashSet::new();
        for event in &self.events {
            let event_name = event.name.value();
            if !event_names.insert(event_name.clone()) {
                return Err(BuilderError::validation_error(
                    &format!("Duplicate event name '{}' in maneuver '{}'", event_name, self.name),
                    "Ensure all event names within a maneuver are unique"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed event
    pub(super) fn add_completed_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }
}

/// Builder for creating events within maneuvers
pub struct ManeuverEventBuilder {
    parent: ManeuverBuilder,
    event_builder: EventBuilder,
}

impl ManeuverEventBuilder {
    /// Create a new maneuver event builder
    fn new(parent: ManeuverBuilder, name: String) -> Self {
        Self {
            parent,
            event_builder: EventBuilder::new().name(name),
        }
    }

    /// Set the maximum execution count
    pub fn max_execution_count(mut self, count: u32) -> Self {
        self.event_builder = self.event_builder.max_execution_count(count);
        self
    }

    /// Set the event priority
    pub fn priority(mut self, priority: crate::types::enums::Priority) -> Self {
        self.event_builder = self.event_builder.priority(priority);
        self
    }

    /// Add an action to this event
    pub fn add_action(mut self) -> ManeuverEventActionBuilder {
        ManeuverEventActionBuilder::new(self)
    }

    /// Add a start trigger to this event
    pub fn with_start_trigger(mut self) -> ManeuverEventTriggerBuilder {
        ManeuverEventTriggerBuilder::new(self)
    }

    /// Finish building the event and return to maneuver builder
    pub fn finish_event(self) -> BuilderResult<ManeuverBuilder> {
        let event = self.event_builder.finish()?;
        Ok(self.parent.add_completed_event(event))
    }

    /// Internal method to set the event builder
    fn set_event_builder(mut self, event_builder: EventBuilder) -> Self {
        self.event_builder = event_builder;
        self
    }
}

/// Builder for creating actions within maneuver events
pub struct ManeuverEventActionBuilder {
    parent: ManeuverEventBuilder,
    action_name: Option<String>,
}

impl ManeuverEventActionBuilder {
    /// Create a new maneuver event action builder
    fn new(parent: ManeuverEventBuilder) -> Self {
        Self {
            parent,
            action_name: None,
        }
    }

    /// Set the action name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.action_name = Some(name.into());
        self
    }

    /// Add a teleport action
    pub fn teleport(mut self, entity_ref: impl Into<String>) -> ManeuverTeleportActionBuilder {
        ManeuverTeleportActionBuilder::new(self, entity_ref.into())
    }

    /// Add a longitudinal action
    pub fn longitudinal(mut self, entity_ref: impl Into<String>) -> ManeuverLongitudinalActionBuilder {
        ManeuverLongitudinalActionBuilder::new(self, entity_ref.into())
    }

    /// Add a lateral action
    pub fn lateral(mut self, entity_ref: impl Into<String>) -> ManeuverLateralActionBuilder {
        ManeuverLateralActionBuilder::new(self, entity_ref.into())
    }

    /// Add a controller action
    pub fn controller(mut self, entity_ref: impl Into<String>) -> ManeuverControllerActionBuilder {
        ManeuverControllerActionBuilder::new(self, entity_ref.into())
    }

    /// Add an appearance action
    pub fn appearance(mut self, entity_ref: impl Into<String>) -> ManeuverAppearanceActionBuilder {
        ManeuverAppearanceActionBuilder::new(self, entity_ref.into())
    }

    /// Finish building the action and return to event builder
    pub fn finish_action(self) -> ManeuverEventBuilder {
        // For now, just return the parent
        // This will be expanded when we implement specific action builders
        self.parent
    }
}

/// Builder for teleport actions within maneuver events
pub struct ManeuverTeleportActionBuilder {
    parent: ManeuverEventActionBuilder,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl ManeuverTeleportActionBuilder {
    fn new(parent: ManeuverEventActionBuilder, entity_ref: String) -> Self {
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
    pub fn finish_action(self) -> BuilderResult<ManeuverEventBuilder> {
        let position = self.position.ok_or_else(|| {
            BuilderError::validation_error(
                "Teleport action requires a position",
                "Use to_position() to specify the target position"
            )
        })?;

        use crate::types::scenario::story::{StoryAction, StoryPrivateAction};
        use crate::types::actions::movement::TeleportAction;

        let teleport_action = TeleportAction {
            position,
        };

        let mut private_action = StoryPrivateAction::default();
        private_action.teleport_action = Some(teleport_action);

        let action = StoryAction {
            name: OSString::literal(self.parent.action_name.unwrap_or_else(|| "TeleportAction".to_string())),
            private_action: Some(private_action),
        };

        let event_builder = self.parent.parent.event_builder.with_action(action);
        Ok(self.parent.parent.set_event_builder(event_builder))
    }
}

// PositionTarget trait implementation removed - trait doesn't exist

/// Builder for longitudinal actions within maneuver events
pub struct ManeuverLongitudinalActionBuilder {
    parent: ManeuverEventActionBuilder,
    entity_ref: String,
    action_type: Option<ManeuverLongitudinalActionType>,
}

#[derive(Debug, Clone)]
enum ManeuverLongitudinalActionType {
    Speed { target: f64, dynamics: Option<crate::types::actions::movement::TransitionDynamics> },
    Distance { target: f64, dynamics: Option<crate::types::actions::movement::TransitionDynamics> },
}

impl ManeuverLongitudinalActionBuilder {
    fn new(parent: ManeuverEventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            action_type: None,
        }
    }

    /// Create a speed action
    pub fn speed_action(mut self) -> ManeuverSpeedActionBuilder {
        ManeuverSpeedActionBuilder::new(self)
    }

    /// Create a distance action
    pub fn distance_action(mut self) -> ManeuverDistanceActionBuilder {
        ManeuverDistanceActionBuilder::new(self)
    }

    /// Finish building the longitudinal action
    pub fn finish_action(self) -> BuilderResult<ManeuverEventBuilder> {
        let action_type = self.action_type.ok_or_else(|| {
            BuilderError::validation_error(
                "Longitudinal action requires an action type",
                "Use speed_action() or distance_action()"
            )
        })?;

        use crate::types::scenario::story::{StoryAction, StoryPrivateAction};
        use crate::types::scenario::init::LongitudinalAction;
        use crate::types::actions::movement::{SpeedAction, LongitudinalDistanceAction, TransitionDynamics, SpeedActionTarget, AbsoluteTargetSpeed};

        let longitudinal_action = match action_type {
            ManeuverLongitudinalActionType::Speed { target, dynamics } => {
                let speed_action = SpeedAction {
                    speed_action_dynamics: TransitionDynamics {
                        dynamics_dimension: crate::types::enums::DynamicsDimension::Time,
                        dynamics_shape: crate::types::enums::DynamicsShape::Linear,
                        value: crate::types::basic::Double::literal(1.0), // Default rate
                    },
                    speed_action_target: SpeedActionTarget {
                        absolute: Some(AbsoluteTargetSpeed {
                            value: crate::types::basic::Double::literal(target),
                        }),
                        relative: None,
                    },
                };
                LongitudinalAction {
                    speed_action: Some(speed_action),
                    longitudinal_distance_action: None,
                }
            },
            ManeuverLongitudinalActionType::Distance { target, dynamics } => {
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

        let mut private_action = StoryPrivateAction::default();
        private_action.longitudinal_action = Some(longitudinal_action);

        let action = StoryAction {
            name: OSString::literal(self.parent.action_name.unwrap_or_else(|| "LongitudinalAction".to_string())),
            private_action: Some(private_action),
        };

        let event_builder = self.parent.parent.event_builder.with_action(action);
        Ok(self.parent.parent.set_event_builder(event_builder))
    }

    /// Internal method to set the action type
    fn set_action_type(mut self, action_type: ManeuverLongitudinalActionType) -> Self {
        self.action_type = Some(action_type);
        self
    }
}

/// Builder for speed actions within maneuver longitudinal actions
pub struct ManeuverSpeedActionBuilder {
    parent: ManeuverLongitudinalActionBuilder,
    target: Option<f64>,
    dynamics: Option<crate::types::actions::movement::TransitionDynamics>,
}

impl ManeuverSpeedActionBuilder {
    fn new(parent: ManeuverLongitudinalActionBuilder) -> Self {
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
    pub fn finish_action(self) -> BuilderResult<ManeuverLongitudinalActionBuilder> {
        let target = self.target.ok_or_else(|| {
            BuilderError::validation_error(
                "Speed action requires a target speed",
                "Use absolute_target() to set the target speed"
            )
        })?;

        let action_type = ManeuverLongitudinalActionType::Speed {
            target,
            dynamics: self.dynamics,
        };

        Ok(self.parent.set_action_type(action_type))
    }
}

/// Builder for distance actions within maneuver longitudinal actions
pub struct ManeuverDistanceActionBuilder {
    parent: ManeuverLongitudinalActionBuilder,
    target: Option<f64>,
    dynamics: Option<crate::types::actions::movement::TransitionDynamics>,
}

impl ManeuverDistanceActionBuilder {
    fn new(parent: ManeuverLongitudinalActionBuilder) -> Self {
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
    pub fn finish_action(self) -> BuilderResult<ManeuverLongitudinalActionBuilder> {
        let target = self.target.ok_or_else(|| {
            BuilderError::validation_error(
                "Distance action requires a target distance",
                "Use target_distance() to set the target distance"
            )
        })?;

        let action_type = ManeuverLongitudinalActionType::Distance {
            target,
            dynamics: self.dynamics,
        };

        Ok(self.parent.set_action_type(action_type))
    }
}

/// Builder for lateral actions within maneuver events
pub struct ManeuverLateralActionBuilder {
    parent: ManeuverEventActionBuilder,
    entity_ref: String,
}

impl ManeuverLateralActionBuilder {
    fn new(parent: ManeuverEventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the lateral action (placeholder)
    pub fn finish_action(self) -> ManeuverEventBuilder {
        // For now, just return the parent without adding a lateral action
        // This can be expanded later with specific lateral action types
        self.parent.parent
    }
}

/// Builder for controller actions within maneuver events
pub struct ManeuverControllerActionBuilder {
    parent: ManeuverEventActionBuilder,
    entity_ref: String,
}

impl ManeuverControllerActionBuilder {
    fn new(parent: ManeuverEventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the controller action (placeholder)
    pub fn finish_action(self) -> ManeuverEventBuilder {
        // For now, just return the parent without adding a controller action
        // This can be expanded later with specific controller action types
        self.parent.parent
    }
}

/// Builder for appearance actions within maneuver events
pub struct ManeuverAppearanceActionBuilder {
    parent: ManeuverEventActionBuilder,
    entity_ref: String,
}

impl ManeuverAppearanceActionBuilder {
    fn new(parent: ManeuverEventActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
        }
    }

    /// Finish building the appearance action (placeholder)
    pub fn finish_action(self) -> ManeuverEventBuilder {
        // For now, just return the parent without adding an appearance action
        // This can be expanded later with specific appearance action types
        self.parent.parent
    }
}

/// Builder for creating triggers within maneuver events
pub struct ManeuverEventTriggerBuilder {
    parent: ManeuverEventBuilder,
    trigger_builder: crate::builder::triggers::TriggerBuilder,
}

impl ManeuverEventTriggerBuilder {
    /// Create a new maneuver event trigger builder
    fn new(parent: ManeuverEventBuilder) -> Self {
        Self {
            parent,
            trigger_builder: crate::builder::triggers::TriggerBuilder::new(),
        }
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> ManeuverSimulationTimeConditionBuilder {
        ManeuverSimulationTimeConditionBuilder::new(self)
    }

    /// Add a speed condition
    pub fn speed(self) -> ManeuverSpeedConditionBuilder {
        ManeuverSpeedConditionBuilder::new(self)
    }

    /// Add a distance condition
    pub fn distance(self) -> ManeuverDistanceConditionBuilder {
        ManeuverDistanceConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<ManeuverEventBuilder> {
        let trigger = self.trigger_builder.finish()?;
        let event_builder = self.parent.event_builder.with_trigger(trigger);
        Ok(self.parent.set_event_builder(event_builder))
    }

    /// Internal method to add a condition
    fn add_condition(mut self, condition: crate::types::scenario::triggers::Condition) -> Self {
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

/// Builder for simulation time conditions in maneuver event triggers
pub struct ManeuverSimulationTimeConditionBuilder {
    parent: ManeuverEventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    value: Option<f64>,
    rule: Option<String>,
}

impl ManeuverSimulationTimeConditionBuilder {
    fn new(parent: ManeuverEventTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<ManeuverEventTriggerBuilder> {
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

/// Builder for speed conditions in maneuver event triggers
pub struct ManeuverSpeedConditionBuilder {
    parent: ManeuverEventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Option<String>,
}

impl ManeuverSpeedConditionBuilder {
    fn new(parent: ManeuverEventTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<ManeuverEventTriggerBuilder> {
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

/// Builder for distance conditions in maneuver event triggers
pub struct ManeuverDistanceConditionBuilder {
    parent: ManeuverEventTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    position: Option<crate::types::positions::Position>,
    value: Option<f64>,
    rule: Option<String>,
    freespace: bool,
}

impl ManeuverDistanceConditionBuilder {
    fn new(parent: ManeuverEventTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<ManeuverEventTriggerBuilder> {
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
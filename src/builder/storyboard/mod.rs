//! Storyboard builders for programmatic storyboard construction
//!
//! This module provides fluent APIs for creating complete storyboards with stories,
//! acts, maneuvers, and events with comprehensive validation and type safety.

pub mod story;
pub mod act;
pub mod maneuver;

pub use story::StoryBuilder;
pub use act::ActBuilder;
pub use maneuver::ManeuverBuilder;

use crate::types::scenario::storyboard::Storyboard;
use crate::types::scenario::init::Init;
use crate::types::scenario::story::ScenarioStory;
use crate::types::scenario::triggers::Trigger;
use crate::builder::{BuilderError, BuilderResult};
use crate::builder::triggers::TriggerBuilder;

/// Builder for creating complete storyboards
pub struct StoryboardBuilder {
    init: Init,
    stories: Vec<ScenarioStory>,
    stop_trigger: Option<Trigger>,
}

impl StoryboardBuilder {
    /// Create a new storyboard builder
    pub fn new() -> Self {
        Self {
            init: Init::default(),
            stories: Vec::new(),
            stop_trigger: None,
        }
    }

    /// Add an init action to the storyboard
    pub fn add_init_action(mut self) -> InitActionBuilder {
        InitActionBuilder::new(self)
    }

    /// Set the init section directly
    pub fn with_init(mut self, init: Init) -> Self {
        self.init = init;
        self
    }

    /// Add a story to the storyboard
    pub fn add_story(mut self, name: impl Into<String>) -> StoryBuilder {
        StoryBuilder::new(self, name.into())
    }

    /// Add a pre-built story
    pub fn with_story(mut self, story: ScenarioStory) -> Self {
        self.stories.push(story);
        self
    }

    /// Add a stop trigger to the storyboard
    pub fn with_stop_trigger(mut self) -> StoryboardTriggerBuilder {
        StoryboardTriggerBuilder::new(self)
    }

    /// Set a pre-built stop trigger
    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }

    /// Finish building the storyboard
    pub fn finish_storyboard(self) -> BuilderResult<Storyboard> {
        self.validate()?;

        Ok(Storyboard {
            init: self.init,
            stories: self.stories,
            stop_trigger: self.stop_trigger,
        })
    }

    /// Validate the storyboard configuration
    pub fn validate(&self) -> BuilderResult<()> {
        // Storyboard can be valid with no stories (minimal scenario)
        // but if there are stories, they should be valid
        for story in &self.stories {
            if story.acts.is_empty() {
                return Err(BuilderError::validation_error(
                    &format!("Story '{}' has no acts", story.name.value()),
                    "Add at least one act to each story"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed story
    fn add_completed_story(mut self, story: ScenarioStory) -> Self {
        self.stories.push(story);
        self
    }

    /// Internal method to set the init section
    fn set_init(mut self, init: Init) -> Self {
        self.init = init;
        self
    }

    /// Internal method to set the stop trigger
    fn set_stop_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }
}

impl Default for StoryboardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating init actions
pub struct InitActionBuilder {
    parent: StoryboardBuilder,
    actions: Vec<crate::types::scenario::init::InitAction>,
}

impl InitActionBuilder {
    /// Create a new init action builder
    fn new(parent: StoryboardBuilder) -> Self {
        Self {
            parent,
            actions: Vec::new(),
        }
    }

    /// Add a teleport action
    pub fn teleport(mut self, entity_ref: impl Into<String>) -> InitTeleportActionBuilder {
        InitTeleportActionBuilder::new(self, entity_ref.into())
    }

    /// Add a speed action
    pub fn speed(mut self, entity_ref: impl Into<String>) -> InitSpeedActionBuilder {
        InitSpeedActionBuilder::new(self, entity_ref.into())
    }

    /// Add a position action
    pub fn position(mut self, entity_ref: impl Into<String>) -> InitPositionActionBuilder {
        InitPositionActionBuilder::new(self, entity_ref.into())
    }

    /// Finish building the init actions
    pub fn finish_init(self) -> StoryboardBuilder {
        let init = Init {
            actions: self.actions,
        };
        self.parent.set_init(init)
    }

    /// Internal method to add a completed action
    fn add_completed_action(mut self, action: crate::types::scenario::init::InitAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// Builder for teleport actions in init
pub struct InitTeleportActionBuilder {
    parent: InitActionBuilder,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl InitTeleportActionBuilder {
    fn new(parent: InitActionBuilder, entity_ref: String) -> Self {
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
    pub fn finish_action(self) -> BuilderResult<InitActionBuilder> {
        let position = self.position.ok_or_else(|| {
            BuilderError::validation_error(
                "Teleport action requires a position",
                "Use to_position() to specify the target position"
            )
        })?;

        use crate::types::scenario::init::{InitAction, PrivateAction};
        use crate::types::actions::movement::TeleportAction;
        use crate::types::basic::OSString;

        let teleport_action = TeleportAction {
            position,
        };

        let private_action = PrivateAction {
            entity_ref: OSString::literal(self.entity_ref),
            longitudinal_action: None,
            lateral_action: None,
            visibility_action: None,
            synchronize_action: None,
            controller_action: None,
            teleport_action: Some(teleport_action),
            routing_action: None,
            appearance_action: None,
            trailer_action: None,
        };

        let init_action = InitAction {
            private_action: Some(private_action),
            global_action: None,
            user_defined_action: None,
        };

        Ok(self.parent.add_completed_action(init_action))
    }
}

// Implement PositionTarget for InitTeleportActionBuilder
impl crate::builder::positions::PositionTarget for InitTeleportActionBuilder {
    fn set_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// Builder for speed actions in init
pub struct InitSpeedActionBuilder {
    parent: InitActionBuilder,
    entity_ref: String,
    speed: Option<f64>,
}

impl InitSpeedActionBuilder {
    fn new(parent: InitActionBuilder, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            speed: None,
        }
    }

    /// Set the initial speed
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    /// Finish building the speed action
    pub fn finish_action(self) -> BuilderResult<InitActionBuilder> {
        let speed = self.speed.ok_or_else(|| {
            BuilderError::validation_error(
                "Speed action requires a speed value",
                "Use speed() to specify the initial speed"
            )
        })?;

        use crate::types::scenario::init::{InitAction, PrivateAction, LongitudinalAction};
        use crate::types::actions::movement::{SpeedAction, SpeedActionDynamics, SpeedTarget, AbsoluteSpeedAction};
        use crate::types::basic::OSString;
        use crate::types::enums::DynamicsShape;

        let speed_action = SpeedAction {
            speed_action_dynamics: SpeedActionDynamics {
                dynamics_shape: DynamicsShape::Step,
                value: crate::types::basic::Value::literal(0.0), // Instant
                following_mode: None,
            },
            speed_target: SpeedTarget {
                relative_speed_to_master: None,
                absolute_speed: Some(AbsoluteSpeedAction {
                    value: crate::types::basic::Value::literal(speed),
                }),
            },
        };

        let longitudinal_action = LongitudinalAction {
            speed_action: Some(speed_action),
            longitudinal_distance_action: None,
        };

        let private_action = PrivateAction {
            entity_ref: OSString::literal(self.entity_ref),
            longitudinal_action: Some(longitudinal_action),
            lateral_action: None,
            visibility_action: None,
            synchronize_action: None,
            controller_action: None,
            teleport_action: None,
            routing_action: None,
            appearance_action: None,
            trailer_action: None,
        };

        let init_action = InitAction {
            private_action: Some(private_action),
            global_action: None,
            user_defined_action: None,
        };

        Ok(self.parent.add_completed_action(init_action))
    }
}

/// Builder for position actions in init
pub struct InitPositionActionBuilder {
    parent: InitActionBuilder,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl InitPositionActionBuilder {
    fn new(parent: InitActionBuilder, entity_ref: String) -> Self {
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

    /// Finish building the position action
    pub fn finish_action(self) -> BuilderResult<InitActionBuilder> {
        let position = self.position.ok_or_else(|| {
            BuilderError::validation_error(
                "Position action requires a position",
                "Use to_position() to specify the target position"
            )
        })?;

        use crate::types::scenario::init::{InitAction, PrivateAction};
        use crate::types::actions::movement::TeleportAction;
        use crate::types::basic::OSString;

        let teleport_action = TeleportAction {
            position,
        };

        let private_action = PrivateAction {
            entity_ref: OSString::literal(self.entity_ref),
            longitudinal_action: None,
            lateral_action: None,
            visibility_action: None,
            synchronize_action: None,
            controller_action: None,
            teleport_action: Some(teleport_action),
            routing_action: None,
            appearance_action: None,
            trailer_action: None,
        };

        let init_action = InitAction {
            private_action: Some(private_action),
            global_action: None,
            user_defined_action: None,
        };

        Ok(self.parent.add_completed_action(init_action))
    }
}

// Implement PositionTarget for InitPositionActionBuilder
impl crate::builder::positions::PositionTarget for InitPositionActionBuilder {
    fn set_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// Builder for creating triggers within storyboards
pub struct StoryboardTriggerBuilder {
    parent: StoryboardBuilder,
    trigger_builder: TriggerBuilder,
}

impl StoryboardTriggerBuilder {
    /// Create a new storyboard trigger builder
    fn new(parent: StoryboardBuilder) -> Self {
        Self {
            parent,
            trigger_builder: TriggerBuilder::new(),
        }
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> StoryboardSimulationTimeConditionBuilder {
        StoryboardSimulationTimeConditionBuilder::new(self)
    }

    /// Add a speed condition
    pub fn speed(self) -> StoryboardSpeedConditionBuilder {
        StoryboardSpeedConditionBuilder::new(self)
    }

    /// Add a distance condition
    pub fn distance(self) -> StoryboardDistanceConditionBuilder {
        StoryboardDistanceConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<StoryboardBuilder> {
        let trigger = self.trigger_builder.finish()?;
        Ok(self.parent.set_stop_trigger(trigger))
    }

    /// Internal method to add a condition
    fn add_condition(mut self, condition: crate::types::scenario::triggers::Condition) -> Self {
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

/// Builder for simulation time conditions in storyboard triggers
pub struct StoryboardSimulationTimeConditionBuilder {
    parent: StoryboardTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    value: Option<f64>,
    rule: Option<String>,
}

impl StoryboardSimulationTimeConditionBuilder {
    fn new(parent: StoryboardTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<StoryboardTriggerBuilder> {
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
        use crate::types::basic::OSString;

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

/// Builder for speed conditions in storyboard triggers
pub struct StoryboardSpeedConditionBuilder {
    parent: StoryboardTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Option<String>,
}

impl StoryboardSpeedConditionBuilder {
    fn new(parent: StoryboardTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<StoryboardTriggerBuilder> {
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
        use crate::types::basic::OSString;

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

/// Builder for distance conditions in storyboard triggers
pub struct StoryboardDistanceConditionBuilder {
    parent: StoryboardTriggerBuilder,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    position: Option<crate::types::positions::Position>,
    value: Option<f64>,
    rule: Option<String>,
    freespace: bool,
}

impl StoryboardDistanceConditionBuilder {
    fn new(parent: StoryboardTriggerBuilder) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<StoryboardTriggerBuilder> {
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
        use crate::types::basic::OSString;

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
//! Act builder for programmatic act construction
//!
//! This module provides fluent APIs for creating acts with maneuver groups and triggers
//! with comprehensive validation and type safety.

use crate::types::scenario::story::{Act, ManeuverGroup};
use crate::types::scenario::triggers::Trigger;
use crate::types::basic::OSString;
use crate::builder::{BuilderError, BuilderResult};
use crate::builder::triggers::TriggerBuilder;
use super::{StoryBuilder, ManeuverBuilder};

/// Builder for creating acts within stories
pub struct ActBuilder {
    parent: StoryBuilder,
    name: String,
    maneuver_groups: Vec<ManeuverGroup>,
    start_trigger: Option<Trigger>,
    stop_trigger: Option<Trigger>,
}

impl ActBuilder {
    /// Create a new act builder
    pub(super) fn new(parent: StoryBuilder, name: String) -> Self {
        Self {
            parent,
            name,
            maneuver_groups: Vec::new(),
            start_trigger: None,
            stop_trigger: None,
        }
    }

    /// Add a maneuver group to this act
    pub fn add_maneuver_group(mut self, name: impl Into<String>) -> ManeuverGroupBuilder {
        ManeuverGroupBuilder::new(self, name.into())
    }

    /// Add a pre-built maneuver group
    pub fn with_maneuver_group(mut self, group: ManeuverGroup) -> Self {
        self.maneuver_groups.push(group);
        self
    }

    /// Add a start trigger to this act
    pub fn with_start_trigger(mut self) -> ActStartTriggerBuilder {
        ActStartTriggerBuilder::new(self)
    }

    /// Set a pre-built start trigger
    pub fn with_start_trigger_direct(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    /// Add a stop trigger to this act
    pub fn with_stop_trigger(mut self) -> ActStopTriggerBuilder {
        ActStopTriggerBuilder::new(self)
    }

    /// Set a pre-built stop trigger
    pub fn with_stop_trigger_direct(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }

    /// Finish building the act and return to story builder
    pub fn finish_act(self) -> BuilderResult<StoryBuilder> {
        self.validate()?;

        let act = Act {
            name: OSString::literal(self.name),
            maneuver_groups: self.maneuver_groups,
            start_trigger: self.start_trigger,
            stop_trigger: self.stop_trigger,
        };

        Ok(self.parent.add_completed_act(act))
    }

    /// Validate the act configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.maneuver_groups.is_empty() {
            return Err(BuilderError::validation_error(
                &format!("Act '{}' has no maneuver groups", self.name),
                "Add at least one maneuver group using add_maneuver_group()"
            ));
        }

        // Validate maneuver group names are unique
        let mut group_names = std::collections::HashSet::new();
        for group in &self.maneuver_groups {
            let group_name = group.name.value();
            if !group_names.insert(group_name.clone()) {
                return Err(BuilderError::validation_error(
                    &format!("Duplicate maneuver group name '{}' in act '{}'", group_name, self.name),
                    "Ensure all maneuver group names within an act are unique"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed maneuver group
    pub(super) fn add_completed_maneuver_group(mut self, group: ManeuverGroup) -> Self {
        self.maneuver_groups.push(group);
        self
    }

    /// Internal method to set the start trigger
    fn set_start_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    /// Internal method to set the stop trigger
    fn set_stop_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }
}

/// Builder for creating maneuver groups within acts
pub struct ManeuverGroupBuilder {
    parent: ActBuilder,
    name: String,
    maximum_execution_count: Option<u32>,
    actors: crate::types::scenario::story::Actors,
    maneuvers: Vec<crate::types::scenario::story::Maneuver>,
}

impl ManeuverGroupBuilder {
    /// Create a new maneuver group builder
    fn new(parent: ActBuilder, name: String) -> Self {
        Self {
            parent,
            name,
            maximum_execution_count: None,
            actors: crate::types::scenario::story::Actors::default(),
            maneuvers: Vec::new(),
        }
    }

    /// Set the maximum execution count
    pub fn max_execution_count(mut self, count: u32) -> Self {
        self.maximum_execution_count = Some(count);
        self
    }

    /// Add an entity actor to this maneuver group
    pub fn add_actor(mut self, entity_ref: impl Into<String>) -> Self {
        use crate::types::scenario::story::EntityRef;
        use crate::types::basic::OSString;

        let entity_ref = EntityRef {
            entity_ref: OSString::literal(entity_ref.into()),
        };
        self.actors.entity_refs.push(entity_ref);
        self
    }

    /// Set whether to select triggering entities
    pub fn select_triggering_entities(mut self, select: bool) -> Self {
        self.actors.select_triggering_entities = Some(select);
        self
    }

    /// Add a maneuver to this group
    pub fn add_maneuver(mut self, name: impl Into<String>) -> ManeuverBuilder {
        ManeuverBuilder::new(self, name.into())
    }

    /// Add a pre-built maneuver
    pub fn with_maneuver(mut self, maneuver: crate::types::scenario::story::Maneuver) -> Self {
        self.maneuvers.push(maneuver);
        self
    }

    /// Finish building the maneuver group and return to act builder
    pub fn finish_maneuver_group(self) -> BuilderResult<ActBuilder> {
        self.validate()?;

        use crate::types::basic::UnsignedInt;

        let group = ManeuverGroup {
            name: OSString::literal(self.name),
            maximum_execution_count: self.maximum_execution_count.map(|c| UnsignedInt::literal(c)),
            actors: self.actors,
            catalog_reference: None,
            maneuvers: self.maneuvers,
        };

        Ok(self.parent.add_completed_maneuver_group(group))
    }

    /// Validate the maneuver group configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.actors.entity_refs.is_empty() && self.actors.select_triggering_entities != Some(true) {
            return Err(BuilderError::validation_error(
                &format!("Maneuver group '{}' has no actors", self.name),
                "Add at least one actor using add_actor() or enable select_triggering_entities()"
            ));
        }

        if self.maneuvers.is_empty() {
            return Err(BuilderError::validation_error(
                &format!("Maneuver group '{}' has no maneuvers", self.name),
                "Add at least one maneuver using add_maneuver()"
            ));
        }

        // Validate maneuver names are unique
        let mut maneuver_names = std::collections::HashSet::new();
        for maneuver in &self.maneuvers {
            let maneuver_name = maneuver.name.value();
            if !maneuver_names.insert(maneuver_name.clone()) {
                return Err(BuilderError::validation_error(
                    &format!("Duplicate maneuver name '{}' in group '{}'", maneuver_name, self.name),
                    "Ensure all maneuver names within a group are unique"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed maneuver
    pub(super) fn add_completed_maneuver(mut self, maneuver: crate::types::scenario::story::Maneuver) -> Self {
        self.maneuvers.push(maneuver);
        self
    }
}

/// Builder for creating start triggers within acts
pub struct ActStartTriggerBuilder {
    parent: ActBuilder,
    trigger_builder: TriggerBuilder,
}

impl ActStartTriggerBuilder {
    /// Create a new act start trigger builder
    fn new(parent: ActBuilder) -> Self {
        Self {
            parent,
            trigger_builder: TriggerBuilder::new(),
        }
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> ActSimulationTimeConditionBuilder {
        ActSimulationTimeConditionBuilder::new(self, false)
    }

    /// Add a speed condition
    pub fn speed(self) -> ActSpeedConditionBuilder {
        ActSpeedConditionBuilder::new(self, false)
    }

    /// Add a distance condition
    pub fn distance(self) -> ActDistanceConditionBuilder {
        ActDistanceConditionBuilder::new(self, false)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<ActBuilder> {
        let trigger = self.trigger_builder.finish()?;
        Ok(self.parent.set_start_trigger(trigger))
    }

    /// Internal method to add a condition
    fn add_condition(mut self, condition: crate::types::scenario::triggers::Condition) -> Self {
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

/// Builder for creating stop triggers within acts
pub struct ActStopTriggerBuilder {
    parent: ActBuilder,
    trigger_builder: TriggerBuilder,
}

impl ActStopTriggerBuilder {
    /// Create a new act stop trigger builder
    fn new(parent: ActBuilder) -> Self {
        Self {
            parent,
            trigger_builder: TriggerBuilder::new(),
        }
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> ActSimulationTimeConditionBuilder {
        ActSimulationTimeConditionBuilder::new_for_stop(self)
    }

    /// Add a speed condition
    pub fn speed(self) -> ActSpeedConditionBuilder {
        ActSpeedConditionBuilder::new_for_stop(self)
    }

    /// Add a distance condition
    pub fn distance(self) -> ActDistanceConditionBuilder {
        ActDistanceConditionBuilder::new_for_stop(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<ActBuilder> {
        let trigger = self.trigger_builder.finish()?;
        Ok(self.parent.set_stop_trigger(trigger))
    }

    /// Internal method to add a condition
    fn add_condition(mut self, condition: crate::types::scenario::triggers::Condition) -> Self {
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

/// Builder for simulation time conditions in act triggers
pub struct ActSimulationTimeConditionBuilder {
    start_parent: Option<ActStartTriggerBuilder>,
    stop_parent: Option<ActStopTriggerBuilder>,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    value: Option<f64>,
    rule: Option<String>,
}

impl ActSimulationTimeConditionBuilder {
    fn new(parent: ActStartTriggerBuilder, _is_stop: bool) -> Self {
        Self {
            start_parent: Some(parent),
            stop_parent: None,
            name: None,
            edge: crate::types::enums::ConditionEdge::Rising,
            delay: 0.0,
            value: None,
            rule: None,
        }
    }

    fn new_for_stop(parent: ActStopTriggerBuilder) -> Self {
        Self {
            start_parent: None,
            stop_parent: Some(parent),
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
    pub fn finish_condition(self) -> BuilderResult<ActStartTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.start_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActSimulationTimeConditionBuilder must have a start parent for finish_condition")
        }
    }

    /// Finish building the condition for stop trigger
    pub fn finish_condition_for_stop(self) -> BuilderResult<ActStopTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.stop_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActSimulationTimeConditionBuilder must have a stop parent for finish_condition_for_stop")
        }
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

/// Builder for speed conditions in act triggers
pub struct ActSpeedConditionBuilder {
    start_parent: Option<ActStartTriggerBuilder>,
    stop_parent: Option<ActStopTriggerBuilder>,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Option<String>,
}

impl ActSpeedConditionBuilder {
    fn new(parent: ActStartTriggerBuilder, _is_stop: bool) -> Self {
        Self {
            start_parent: Some(parent),
            stop_parent: None,
            name: None,
            edge: crate::types::enums::ConditionEdge::Rising,
            delay: 0.0,
            entity_ref: None,
            value: None,
            rule: None,
        }
    }

    fn new_for_stop(parent: ActStopTriggerBuilder) -> Self {
        Self {
            start_parent: None,
            stop_parent: Some(parent),
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
    pub fn finish_condition(self) -> BuilderResult<ActStartTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.start_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActSpeedConditionBuilder must have a start parent for finish_condition")
        }
    }

    /// Finish building the condition for stop trigger
    pub fn finish_condition_for_stop(self) -> BuilderResult<ActStopTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.stop_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActSpeedConditionBuilder must have a stop parent for finish_condition_for_stop")
        }
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

/// Builder for distance conditions in act triggers
pub struct ActDistanceConditionBuilder {
    start_parent: Option<ActStartTriggerBuilder>,
    stop_parent: Option<ActStopTriggerBuilder>,
    name: Option<String>,
    edge: crate::types::enums::ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    position: Option<crate::types::positions::Position>,
    value: Option<f64>,
    rule: Option<String>,
    freespace: bool,
}

impl ActDistanceConditionBuilder {
    fn new(parent: ActStartTriggerBuilder, _is_stop: bool) -> Self {
        Self {
            start_parent: Some(parent),
            stop_parent: None,
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

    fn new_for_stop(parent: ActStopTriggerBuilder) -> Self {
        Self {
            start_parent: None,
            stop_parent: Some(parent),
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
    pub fn finish_condition(self) -> BuilderResult<ActStartTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.start_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActDistanceConditionBuilder must have a start parent for finish_condition")
        }
    }

    /// Finish building the condition for stop trigger
    pub fn finish_condition_for_stop(self) -> BuilderResult<ActStopTriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.stop_parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("ActDistanceConditionBuilder must have a stop parent for finish_condition_for_stop")
        }
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
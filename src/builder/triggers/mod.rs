//! Trigger builders for programmatic trigger construction
//!
//! This module provides fluent APIs for creating triggers and condition groups
//! with comprehensive validation and type safety.

use crate::types::scenario::triggers::{Trigger, ConditionGroup, Condition, TriggeringEntities, EntityRef};
use crate::types::enums::{ConditionEdge, TriggeringEntitiesRule};
use crate::types::basic::{OSString, Double};
use crate::builder::{BuilderError, BuilderResult};
use crate::builder::conditions::{ConditionBuilder, UnifiedConditionBuilder};

/// Builder for creating triggers with condition groups
pub struct TriggerBuilder {
    condition_groups: Vec<ConditionGroup>,
}

impl TriggerBuilder {
    /// Create a new trigger builder
    pub fn new() -> Self {
        Self {
            condition_groups: Vec::new(),
        }
    }

    /// Add a condition group to this trigger
    pub fn add_condition_group(mut self) -> ConditionGroupBuilder {
        ConditionGroupBuilder::new(self)
    }

    /// Add a simple condition directly (creates a single-condition group)
    pub fn add_condition(mut self, condition: Condition) -> Self {
        let group = ConditionGroup {
            conditions: vec![condition],
        };
        self.condition_groups.push(group);
        self
    }

    /// Add a simulation time condition (convenience method)
    pub fn simulation_time(self) -> SimulationTimeConditionBuilder {
        SimulationTimeConditionBuilder::new(self)
    }

    /// Add a speed condition (convenience method)
    pub fn speed(self) -> SpeedConditionBuilder {
        SpeedConditionBuilder::new(self)
    }

    /// Add a distance condition (convenience method)
    pub fn distance(self) -> DistanceConditionBuilder {
        DistanceConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish(self) -> BuilderResult<Trigger> {
        self.validate()?;
        Ok(Trigger {
            condition_groups: self.condition_groups,
        })
    }

    /// Validate the trigger configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.condition_groups.is_empty() {
            return Err(BuilderError::validation_error(
                "Trigger must have at least one condition group",
                "Add a condition group using add_condition_group()"
            ));
        }

        for group in &self.condition_groups {
            if group.conditions.is_empty() {
                return Err(BuilderError::validation_error(
                    "Condition group cannot be empty",
                    "Add at least one condition to each group"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed condition group
    fn add_completed_group(mut self, group: ConditionGroup) -> Self {
        self.condition_groups.push(group);
        self
    }
}

impl Default for TriggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating condition groups with AND logic
pub struct ConditionGroupBuilder {
    parent: TriggerBuilder,
    conditions: Vec<Condition>,
}

impl ConditionGroupBuilder {
    /// Create a new condition group builder
    fn new(parent: TriggerBuilder) -> Self {
        Self {
            parent,
            conditions: Vec::new(),
        }
    }

    /// Add a condition to this group
    pub fn add_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Add a simulation time condition
    pub fn simulation_time(self) -> SimulationTimeConditionBuilder {
        SimulationTimeConditionBuilder::new_for_group(self)
    }

    /// Add a speed condition
    pub fn speed(self) -> SpeedConditionBuilder {
        SpeedConditionBuilder::new_for_group(self)
    }

    /// Add a distance condition
    pub fn distance(self) -> DistanceConditionBuilder {
        DistanceConditionBuilder::new_for_group(self)
    }

    /// Finish this condition group and return to trigger builder
    pub fn finish_group(self) -> TriggerBuilder {
        let group = ConditionGroup {
            conditions: self.conditions,
        };
        self.parent.add_completed_group(group)
    }

    /// Internal method to add a completed condition
    fn add_completed_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }
}

/// Builder for simulation time conditions
pub struct SimulationTimeConditionBuilder {
    parent: Option<TriggerBuilder>,
    group_parent: Option<ConditionGroupBuilder>,
    name: Option<String>,
    edge: ConditionEdge,
    delay: f64,
    value: Option<f64>,
    rule: Option<String>,
}

impl SimulationTimeConditionBuilder {
    fn new(parent: TriggerBuilder) -> Self {
        Self {
            parent: Some(parent),
            group_parent: None,
            name: None,
            edge: ConditionEdge::Rising,
            delay: 0.0,
            value: None,
            rule: None,
        }
    }

    fn new_for_group(parent: ConditionGroupBuilder) -> Self {
        Self {
            parent: None,
            group_parent: Some(parent),
            name: None,
            edge: ConditionEdge::Rising,
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
    pub fn edge(mut self, edge: ConditionEdge) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<TriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("SimulationTimeConditionBuilder must have a parent")
        }
    }

    /// Finish building the condition for a group
    pub fn finish_condition_for_group(self) -> BuilderResult<ConditionGroupBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.group_parent {
            Ok(parent.add_completed_condition(condition))
        } else {
            unreachable!("SimulationTimeConditionBuilder must have a group parent")
        }
    }

    fn build_condition(&self) -> BuilderResult<Condition> {
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

        Ok(Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "SimTimeCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(Double::literal(self.delay)) } else { None },
            by_value_condition: Some(by_value_condition),
            by_entity_condition: None,
        })
    }
}

/// Builder for speed conditions
pub struct SpeedConditionBuilder {
    parent: Option<TriggerBuilder>,
    group_parent: Option<ConditionGroupBuilder>,
    name: Option<String>,
    edge: ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Option<String>,
}

impl SpeedConditionBuilder {
    fn new(parent: TriggerBuilder) -> Self {
        Self {
            parent: Some(parent),
            group_parent: None,
            name: None,
            edge: ConditionEdge::Rising,
            delay: 0.0,
            entity_ref: None,
            value: None,
            rule: None,
        }
    }

    fn new_for_group(parent: ConditionGroupBuilder) -> Self {
        Self {
            parent: None,
            group_parent: Some(parent),
            name: None,
            edge: ConditionEdge::Rising,
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
    pub fn edge(mut self, edge: ConditionEdge) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<TriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("SpeedConditionBuilder must have a parent")
        }
    }

    /// Finish building the condition for a group
    pub fn finish_condition_for_group(self) -> BuilderResult<ConditionGroupBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.group_parent {
            Ok(parent.add_completed_condition(condition))
        } else {
            unreachable!("SpeedConditionBuilder must have a group parent")
        }
    }

    fn build_condition(&self) -> BuilderResult<Condition> {
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
            triggering_entities_rule: TriggeringEntitiesRule::Any,
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

        Ok(Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "SpeedCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(Double::literal(self.delay)) } else { None },
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for distance conditions
pub struct DistanceConditionBuilder {
    parent: Option<TriggerBuilder>,
    group_parent: Option<ConditionGroupBuilder>,
    name: Option<String>,
    edge: ConditionEdge,
    delay: f64,
    entity_ref: Option<String>,
    position: Option<crate::types::positions::Position>,
    value: Option<f64>,
    rule: Option<String>,
    freespace: bool,
}

impl DistanceConditionBuilder {
    fn new(parent: TriggerBuilder) -> Self {
        Self {
            parent: Some(parent),
            group_parent: None,
            name: None,
            edge: ConditionEdge::Rising,
            delay: 0.0,
            entity_ref: None,
            position: None,
            value: None,
            rule: None,
            freespace: true,
        }
    }

    fn new_for_group(parent: ConditionGroupBuilder) -> Self {
        Self {
            parent: None,
            group_parent: Some(parent),
            name: None,
            edge: ConditionEdge::Rising,
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
    pub fn edge(mut self, edge: ConditionEdge) -> Self {
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
    pub fn finish_condition(self) -> BuilderResult<TriggerBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.parent {
            Ok(parent.add_condition(condition))
        } else {
            unreachable!("DistanceConditionBuilder must have a parent")
        }
    }

    /// Finish building the condition for a group
    pub fn finish_condition_for_group(self) -> BuilderResult<ConditionGroupBuilder> {
        let condition = self.build_condition()?;
        
        if let Some(parent) = self.group_parent {
            Ok(parent.add_completed_condition(condition))
        } else {
            unreachable!("DistanceConditionBuilder must have a group parent")
        }
    }

    fn build_condition(&self) -> BuilderResult<Condition> {
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
            triggering_entities_rule: TriggeringEntitiesRule::Any,
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

        Ok(Condition {
            name: OSString::literal(self.name.clone().unwrap_or_else(|| "DistanceCondition".to_string())),
            condition_edge: self.edge,
            delay: if self.delay > 0.0 { Some(Double::literal(self.delay)) } else { None },
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}
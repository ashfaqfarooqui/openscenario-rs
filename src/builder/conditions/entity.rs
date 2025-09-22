//! Entity condition builders for entity-based conditions
//!
//! This module provides builders for conditions that evaluate properties
//! of specific entities, such as speed, acceleration, distance, and collision states.

use crate::types::conditions::{
    ByEntityCondition, ByEntityConditionSchema, EntityCondition, SpeedCondition,
    AccelerationCondition, DistanceCondition, RelativeDistanceCondition,
    TimeHeadwayCondition, TimeToCollisionCondition, TimeToCollisionTarget,
    CollisionCondition, CollisionTarget, EndOfRoadCondition, OffRoadCondition,
    StandStillCondition, ReachPositionCondition
};
use crate::types::scenario::triggers::{ConditionType, TriggeringEntities, EntityRef};
use crate::types::basic::{OSString, Double, Value};
use crate::types::enums::{ConditionEdge, Rule, TriggeringEntitiesRule, CoordinateSystem};
use crate::types::positions::Position;
use super::{ConditionBuilderTrait, ConditionUtils};
use crate::builder::error::{BuilderError, BuilderResult};
use crate::builder::states::*;
use std::marker::PhantomData;

/// Builder for entity-based conditions
/// 
/// This builder provides a type-safe way to construct entity-based conditions
/// that evaluate properties of specific entities.
/// 
/// # Type Parameters
/// * `S` - Current builder state
/// 
/// # Example
/// ```rust
/// let condition = EntityConditionBuilder::new(Some("speed_check".to_string()), None, None)
///     .speed_condition()
///     .entity_ref("ego_vehicle")
///     .value(25.0)
///     .rule(Rule::GreaterThan)
///     .build()?;
/// ```
pub struct EntityConditionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Condition name for identification
    name: Option<String>,
    
    /// Condition edge detection mode
    edge: Option<ConditionEdge>,
    
    /// Optional delay before condition fires
    delay: Option<Double>,
    
    /// Partially constructed condition data
    condition_data: PartialEntityConditionData,
}

/// Internal data structure for building entity conditions
#[derive(Debug, Default)]
struct PartialEntityConditionData {
    /// Condition type being built
    condition_type: Option<EntityConditionType>,
    
    /// Triggering entities configuration
    triggering_entities: Option<TriggeringEntitiesData>,
    
    /// Speed condition data
    speed_data: Option<SpeedConditionData>,
    
    /// Acceleration condition data
    acceleration_data: Option<AccelerationConditionData>,
    
    /// Distance condition data
    distance_data: Option<DistanceConditionData>,
    
    /// Relative distance condition data
    relative_distance_data: Option<RelativeDistanceConditionData>,
    
    /// Time headway condition data
    time_headway_data: Option<TimeHeadwayConditionData>,
    
    /// Time to collision condition data
    time_to_collision_data: Option<TimeToCollisionConditionData>,
    
    /// Collision condition data
    collision_data: Option<CollisionConditionData>,
    
    /// Reach position condition data
    reach_position_data: Option<ReachPositionConditionData>,
    
    /// Stand still condition data
    stand_still_data: Option<StandStillConditionData>,
    
    /// End of road condition data
    end_of_road_data: Option<EndOfRoadConditionData>,
    
    /// Off road condition data
    off_road_data: Option<OffRoadConditionData>,
}

/// Types of entity conditions
#[derive(Debug, Clone)]
enum EntityConditionType {
    Speed,
    Acceleration,
    Distance,
    RelativeDistance,
    TimeHeadway,
    TimeToCollision,
    Collision,
    ReachPosition,
    StandStill,
    EndOfRoad,
    OffRoad,
}

/// Triggering entities configuration data
#[derive(Debug, Default)]
struct TriggeringEntitiesData {
    rule: Option<TriggeringEntitiesRule>,
    entity_refs: Vec<String>,
}

/// Speed condition configuration data
#[derive(Debug, Default)]
struct SpeedConditionData {
    value: Option<Double>,
    rule: Option<Rule>,
    direction: Option<String>,
}

/// Acceleration condition configuration data
#[derive(Debug, Default)]
struct AccelerationConditionData {
    value: Option<Double>,
    rule: Option<Rule>,
    direction: Option<String>,
}

/// Distance condition configuration data
#[derive(Debug, Default)]
struct DistanceConditionData {
    value: Option<Double>,
    rule: Option<Rule>,
    position: Option<Position>,
    freespace: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
}

/// Relative distance condition configuration data
#[derive(Debug, Default)]
struct RelativeDistanceConditionData {
    entity_ref: Option<String>,
    value: Option<Double>,
    rule: Option<Rule>,
    freespace: Option<bool>,
    relative_distance_type: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
}

/// Time headway condition configuration data
#[derive(Debug, Default)]
struct TimeHeadwayConditionData {
    entity_ref: Option<String>,
    value: Option<Double>,
    rule: Option<Rule>,
    freespace: Option<bool>,
    along_route: Option<bool>,
}

/// Time to collision condition configuration data
#[derive(Debug, Default)]
struct TimeToCollisionConditionData {
    value: Option<Double>,
    rule: Option<Rule>,
    target: Option<TimeToCollisionTargetData>,
    freespace: Option<bool>,
}

/// Time to collision target data
#[derive(Debug, Default)]
struct TimeToCollisionTargetData {
    entity_ref: Option<String>,
    position: Option<Position>,
}

/// Collision condition configuration data
#[derive(Debug, Default)]
struct CollisionConditionData {
    target: Option<CollisionTargetData>,
}

/// Collision target data
#[derive(Debug, Default)]
struct CollisionTargetData {
    entity_ref: Option<String>,
    collision_type: Option<String>,
}

/// Reach position condition configuration data
#[derive(Debug, Default)]
struct ReachPositionConditionData {
    position: Option<Position>,
    tolerance: Option<Double>,
}

/// Stand still condition configuration data
#[derive(Debug, Default)]
struct StandStillConditionData {
    duration: Option<Double>,
}

/// End of road condition configuration data
#[derive(Debug, Default)]
struct EndOfRoadConditionData {
    duration: Option<Double>,
}

/// Off road condition configuration data
#[derive(Debug, Default)]
struct OffRoadConditionData {
    duration: Option<Double>,
}

// Core builder implementation for Empty state
impl EntityConditionBuilder<Empty> {
    /// Create a new entity condition builder
    /// 
    /// # Arguments
    /// * `name` - Optional condition name
    /// * `edge` - Optional edge detection mode
    /// * `delay` - Optional delay
    /// 
    /// # Returns
    /// A new EntityConditionBuilder in Empty state
    pub fn new(
        name: Option<String>, 
        edge: Option<ConditionEdge>, 
        delay: Option<Double>
    ) -> Self {
        Self {
            _state: PhantomData,
            name,
            edge,
            delay,
            condition_data: PartialEntityConditionData::default(),
        }
    }

    /// Start building a speed condition
    /// 
    /// Speed conditions evaluate the speed of an entity against a threshold.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for speed configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .speed_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .value(25.0)
    ///     .rule(Rule::GreaterThan);
    /// ```
    pub fn speed_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::Speed);
        self.condition_data.speed_data = Some(SpeedConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building an acceleration condition
    /// 
    /// Acceleration conditions evaluate the acceleration of an entity.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for acceleration configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .acceleration_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .value(2.0)
    ///     .rule(Rule::GreaterThan);
    /// ```
    pub fn acceleration_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::Acceleration);
        self.condition_data.acceleration_data = Some(AccelerationConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building a distance condition
    /// 
    /// Distance conditions evaluate the distance from an entity to a position.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for distance configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .distance_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .position(target_position)
    ///     .value(10.0)
    ///     .rule(Rule::LessThan);
    /// ```
    pub fn distance_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::Distance);
        self.condition_data.distance_data = Some(DistanceConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building a relative distance condition
    /// 
    /// Relative distance conditions evaluate the distance between two entities.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for relative distance configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .relative_distance_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .target_entity("lead_vehicle")
    ///     .value(50.0)
    ///     .rule(Rule::LessThan);
    /// ```
    pub fn relative_distance_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::RelativeDistance);
        self.condition_data.relative_distance_data = Some(RelativeDistanceConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building a collision condition
    /// 
    /// Collision conditions detect collisions with other entities or objects.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for collision configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .collision_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .collision_target("obstacle");
    /// ```
    pub fn collision_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::Collision);
        self.condition_data.collision_data = Some(CollisionConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building a reach position condition
    /// 
    /// Reach position conditions detect when an entity reaches a specific position.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for reach position configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .reach_position_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .position(target_position)
    ///     .tolerance(2.0);
    /// ```
    pub fn reach_position_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::ReachPosition);
        self.condition_data.reach_position_data = Some(ReachPositionConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }

    /// Start building a stand still condition
    /// 
    /// Stand still conditions detect when an entity remains stationary.
    /// 
    /// # Returns
    /// EntityConditionBuilder in HasType state for stand still configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = EntityConditionBuilder::new(None, None, None)
    ///     .stand_still_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .duration(3.0);
    /// ```
    pub fn stand_still_condition(mut self) -> EntityConditionBuilder<HasType> {
        self.condition_data.condition_type = Some(EntityConditionType::StandStill);
        self.condition_data.stand_still_data = Some(StandStillConditionData::default());
        
        EntityConditionBuilder {
            _state: PhantomData,
            name: self.name,
            edge: self.edge,
            delay: self.delay,
            condition_data: self.condition_data,
        }
    }
}

// Methods for configuring entity conditions
impl EntityConditionBuilder<HasType> {
    /// Set the triggering entity reference
    /// 
    /// # Arguments
    /// * `entity_ref` - Name of the entity to evaluate
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.entity_ref("ego_vehicle")
    /// ```
    pub fn entity_ref(mut self, entity_ref: &str) -> Self {
        if self.condition_data.triggering_entities.is_none() {
            self.condition_data.triggering_entities = Some(TriggeringEntitiesData::default());
        }
        if let Some(ref mut triggering_entities) = self.condition_data.triggering_entities {
            triggering_entities.entity_refs.push(entity_ref.to_string());
        }
        self
    }

    /// Set the triggering entities rule
    /// 
    /// # Arguments
    /// * `rule` - Rule for combining multiple triggering entities (All, Any)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.triggering_entities_rule(TriggeringEntitiesRule::Any)
    /// ```
    pub fn triggering_entities_rule(mut self, rule: TriggeringEntitiesRule) -> Self {
        if self.condition_data.triggering_entities.is_none() {
            self.condition_data.triggering_entities = Some(TriggeringEntitiesData::default());
        }
        if let Some(ref mut triggering_entities) = self.condition_data.triggering_entities {
            triggering_entities.rule = Some(rule);
        }
        self
    }

    /// Set the condition value
    /// 
    /// # Arguments
    /// * `value` - Threshold value for the condition
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.value(25.0)
    /// ```
    pub fn value(mut self, value: f64) -> Self {
        let double_value = ConditionUtils::double(value);
        
        if let Some(ref mut speed_data) = self.condition_data.speed_data {
            speed_data.value = Some(double_value.clone());
        }
        if let Some(ref mut acceleration_data) = self.condition_data.acceleration_data {
            acceleration_data.value = Some(double_value.clone());
        }
        if let Some(ref mut distance_data) = self.condition_data.distance_data {
            distance_data.value = Some(double_value.clone());
        }
        if let Some(ref mut relative_distance_data) = self.condition_data.relative_distance_data {
            relative_distance_data.value = Some(double_value.clone());
        }
        if let Some(ref mut time_headway_data) = self.condition_data.time_headway_data {
            time_headway_data.value = Some(double_value.clone());
        }
        if let Some(ref mut time_to_collision_data) = self.condition_data.time_to_collision_data {
            time_to_collision_data.value = Some(double_value);
        }
        
        self
    }

    /// Set the comparison rule
    /// 
    /// # Arguments
    /// * `rule` - Comparison rule (GreaterThan, LessThan, EqualTo, etc.)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.rule(Rule::GreaterThan)
    /// ```
    pub fn rule(mut self, rule: Rule) -> Self {
        if let Some(ref mut speed_data) = self.condition_data.speed_data {
            speed_data.rule = Some(rule);
        }
        if let Some(ref mut acceleration_data) = self.condition_data.acceleration_data {
            acceleration_data.rule = Some(rule);
        }
        if let Some(ref mut distance_data) = self.condition_data.distance_data {
            distance_data.rule = Some(rule);
        }
        if let Some(ref mut relative_distance_data) = self.condition_data.relative_distance_data {
            relative_distance_data.rule = Some(rule);
        }
        if let Some(ref mut time_headway_data) = self.condition_data.time_headway_data {
            time_headway_data.rule = Some(rule);
        }
        if let Some(ref mut time_to_collision_data) = self.condition_data.time_to_collision_data {
            time_to_collision_data.rule = Some(rule);
        }
        
        self
    }

    /// Set the position for distance conditions
    /// 
    /// # Arguments
    /// * `position` - Target position
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.position(target_position)
    /// ```
    pub fn position(mut self, position: Position) -> Self {
        if let Some(ref mut distance_data) = self.condition_data.distance_data {
            distance_data.position = Some(position.clone());
        }
        if let Some(ref mut reach_position_data) = self.condition_data.reach_position_data {
            reach_position_data.position = Some(position);
        }
        self
    }

    /// Set the target entity for relative conditions
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
        if let Some(ref mut relative_distance_data) = self.condition_data.relative_distance_data {
            relative_distance_data.entity_ref = Some(entity_ref.to_string());
        }
        if let Some(ref mut time_headway_data) = self.condition_data.time_headway_data {
            time_headway_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }

    /// Set the collision target
    /// 
    /// # Arguments
    /// * `target` - Name of the collision target entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.collision_target("obstacle")
    /// ```
    pub fn collision_target(mut self, target: &str) -> Self {
        if let Some(ref mut collision_data) = self.condition_data.collision_data {
            if collision_data.target.is_none() {
                collision_data.target = Some(CollisionTargetData::default());
            }
            if let Some(ref mut target_data) = collision_data.target {
                target_data.entity_ref = Some(target.to_string());
            }
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
    /// 
    /// # Example
    /// ```rust
    /// builder.freespace(true)
    /// ```
    pub fn freespace(mut self, freespace: bool) -> Self {
        if let Some(ref mut distance_data) = self.condition_data.distance_data {
            distance_data.freespace = Some(freespace);
        }
        if let Some(ref mut relative_distance_data) = self.condition_data.relative_distance_data {
            relative_distance_data.freespace = Some(freespace);
        }
        if let Some(ref mut time_headway_data) = self.condition_data.time_headway_data {
            time_headway_data.freespace = Some(freespace);
        }
        if let Some(ref mut time_to_collision_data) = self.condition_data.time_to_collision_data {
            time_to_collision_data.freespace = Some(freespace);
        }
        self
    }

    /// Set the tolerance for reach position conditions
    /// 
    /// # Arguments
    /// * `tolerance` - Position tolerance in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.tolerance(2.0)
    /// ```
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        if let Some(ref mut reach_position_data) = self.condition_data.reach_position_data {
            reach_position_data.tolerance = Some(ConditionUtils::double(tolerance));
        }
        self
    }

    /// Set the duration for time-based conditions
    /// 
    /// # Arguments
    /// * `duration` - Duration in seconds
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.duration(3.0)
    /// ```
    pub fn duration(mut self, duration: f64) -> Self {
        let double_value = ConditionUtils::double(duration);
        
        if let Some(ref mut stand_still_data) = self.condition_data.stand_still_data {
            stand_still_data.duration = Some(double_value.clone());
        }
        if let Some(ref mut end_of_road_data) = self.condition_data.end_of_road_data {
            end_of_road_data.duration = Some(double_value.clone());
        }
        if let Some(ref mut off_road_data) = self.condition_data.off_road_data {
            off_road_data.duration = Some(double_value);
        }
        
        self
    }

    /// Set the coordinate system for distance calculations
    /// 
    /// # Arguments
    /// * `coordinate_system` - Coordinate system to use
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.coordinate_system(CoordinateSystem::Entity)
    /// ```
    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        if let Some(ref mut distance_data) = self.condition_data.distance_data {
            distance_data.coordinate_system = Some(coordinate_system);
        }
        if let Some(ref mut relative_distance_data) = self.condition_data.relative_distance_data {
            relative_distance_data.coordinate_system = Some(coordinate_system);
        }
        self
    }
}

// Build implementation
impl<S: BuilderState> EntityConditionBuilder<S> {
    /// Build the final entity condition
    /// 
    /// # Returns
    /// Complete ConditionType or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<ConditionType> {
        let triggering_entities = self.build_triggering_entities()?;
        
        let entity_condition = match self.condition_data.condition_type {
            Some(EntityConditionType::Speed) => self.build_speed_condition()?,
            Some(EntityConditionType::Acceleration) => self.build_acceleration_condition()?,
            Some(EntityConditionType::Distance) => self.build_distance_condition()?,
            Some(EntityConditionType::RelativeDistance) => self.build_relative_distance_condition()?,
            Some(EntityConditionType::Collision) => self.build_collision_condition()?,
            Some(EntityConditionType::ReachPosition) => self.build_reach_position_condition()?,
            Some(EntityConditionType::StandStill) => self.build_stand_still_condition()?,
            _ => return Err(BuilderError::missing_field(
                "condition_type",
                "Call one of the condition type methods first (speed_condition, etc.)"
            )),
        };

        let by_entity_condition = ByEntityCondition {
            triggering_entities,
            condition: ByEntityConditionSchema {
                choice: entity_condition,
            },
        };

        Ok(ConditionType::ByEntity(by_entity_condition))
    }

    fn build_triggering_entities(&self) -> BuilderResult<TriggeringEntities> {
        let triggering_entities_data = self.condition_data.triggering_entities
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "triggering_entities",
                "Call .entity_ref() to set the triggering entity"
            ))?;

        if triggering_entities_data.entity_refs.is_empty() {
            return Err(BuilderError::missing_field(
                "entity_refs",
                "At least one entity reference is required"
            ));
        }

        let entity_refs: Vec<EntityRef> = triggering_entities_data.entity_refs
            .iter()
            .map(|entity_ref| EntityRef {
                entity_ref: ConditionUtils::os_string(entity_ref),
            })
            .collect();

        Ok(TriggeringEntities {
            triggering_entities_rule: triggering_entities_data.rule
                .unwrap_or(TriggeringEntitiesRule::Any),
            entity_refs,
        })
    }

    fn build_speed_condition(&self) -> BuilderResult<EntityCondition> {
        let speed_data = self.condition_data.speed_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("speed_data", "Internal error"))?;

        let value = speed_data.value
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "value",
                "Call .value() to set the speed threshold"
            ))?;

        let rule = speed_data.rule
            .ok_or_else(|| BuilderError::missing_field(
                "rule",
                "Call .rule() to set the comparison rule"
            ))?;

        let speed_condition = SpeedCondition {
            value: value.clone(),
            rule,
            direction: speed_data.direction.clone(),
        };

        Ok(EntityCondition::SpeedCondition(speed_condition))
    }

    fn build_acceleration_condition(&self) -> BuilderResult<EntityCondition> {
        let acceleration_data = self.condition_data.acceleration_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("acceleration_data", "Internal error"))?;

        let value = acceleration_data.value
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "value",
                "Call .value() to set the acceleration threshold"
            ))?;

        let rule = acceleration_data.rule
            .ok_or_else(|| BuilderError::missing_field(
                "rule",
                "Call .rule() to set the comparison rule"
            ))?;

        let acceleration_condition = AccelerationCondition {
            value: value.clone(),
            rule,
            direction: acceleration_data.direction.clone(),
        };

        Ok(EntityCondition::AccelerationCondition(acceleration_condition))
    }

    fn build_distance_condition(&self) -> BuilderResult<EntityCondition> {
        let distance_data = self.condition_data.distance_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("distance_data", "Internal error"))?;

        let value = distance_data.value
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "value",
                "Call .value() to set the distance threshold"
            ))?;

        let rule = distance_data.rule
            .ok_or_else(|| BuilderError::missing_field(
                "rule",
                "Call .rule() to set the comparison rule"
            ))?;

        let position = distance_data.position
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .position() to set the target position"
            ))?;

        let distance_condition = DistanceCondition {
            value: value.clone(),
            rule,
            position: position.clone(),
            freespace: distance_data.freespace.unwrap_or(false),
            coordinate_system: distance_data.coordinate_system.unwrap_or(CoordinateSystem::Entity),
        };

        Ok(EntityCondition::DistanceCondition(distance_condition))
    }

    fn build_relative_distance_condition(&self) -> BuilderResult<EntityCondition> {
        let relative_distance_data = self.condition_data.relative_distance_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("relative_distance_data", "Internal error"))?;

        let entity_ref = relative_distance_data.entity_ref
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Call .target_entity() to set the target entity"
            ))?;

        let value = relative_distance_data.value
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "value",
                "Call .value() to set the distance threshold"
            ))?;

        let rule = relative_distance_data.rule
            .ok_or_else(|| BuilderError::missing_field(
                "rule",
                "Call .rule() to set the comparison rule"
            ))?;

        let relative_distance_condition = RelativeDistanceCondition {
            entity_ref: ConditionUtils::os_string(entity_ref),
            value: value.clone(),
            rule,
            freespace: relative_distance_data.freespace.unwrap_or(false),
            relative_distance_type: relative_distance_data.relative_distance_type.clone(),
            coordinate_system: relative_distance_data.coordinate_system.unwrap_or(CoordinateSystem::Entity),
        };

        Ok(EntityCondition::RelativeDistanceCondition(relative_distance_condition))
    }

    fn build_collision_condition(&self) -> BuilderResult<EntityCondition> {
        let collision_data = self.condition_data.collision_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("collision_data", "Internal error"))?;

        let target_data = collision_data.target
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "target",
                "Call .collision_target() to set the collision target"
            ))?;

        let entity_ref = target_data.entity_ref
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Collision target entity reference is required"
            ))?;

        let collision_condition = CollisionCondition {
            collision_target: CollisionTarget {
                entity_ref: ConditionUtils::os_string(entity_ref),
                collision_type: target_data.collision_type.clone(),
            },
        };

        Ok(EntityCondition::CollisionCondition(collision_condition))
    }

    fn build_reach_position_condition(&self) -> BuilderResult<EntityCondition> {
        let reach_position_data = self.condition_data.reach_position_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("reach_position_data", "Internal error"))?;

        let position = reach_position_data.position
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .position() to set the target position"
            ))?;

        let reach_position_condition = ReachPositionCondition {
            position: position.clone(),
            tolerance: reach_position_data.tolerance
                .as_ref()
                .cloned()
                .unwrap_or_else(|| ConditionUtils::double(1.0)),
        };

        Ok(EntityCondition::ReachPositionCondition(reach_position_condition))
    }

    fn build_stand_still_condition(&self) -> BuilderResult<EntityCondition> {
        let stand_still_data = self.condition_data.stand_still_data
            .as_ref()
            .ok_or_else(|| BuilderError::missing_field("stand_still_data", "Internal error"))?;

        let duration = stand_still_data.duration
            .as_ref()
            .cloned()
            .unwrap_or_else(|| ConditionUtils::double(0.0));

        let stand_still_condition = StandStillCondition {
            duration,
        };

        Ok(EntityCondition::StandStillCondition(stand_still_condition))
    }
}

impl<S: BuilderState> ConditionBuilderTrait for EntityConditionBuilder<S> {
    fn build_condition(self) -> BuilderResult<ConditionType> {
        self.build()
    }

    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn get_edge(&self) -> Option<ConditionEdge> {
        self.edge
    }

    fn get_delay(&self) -> Option<&Double> {
        self.delay.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::{Rule, TriggeringEntitiesRule, CoordinateSystem};
    use crate::types::positions::{Position, WorldPosition};
    use crate::types::basic::Value;

    #[test]
    fn test_speed_condition_builder() {
        let condition_type = EntityConditionBuilder::new(
            Some("speed_check".to_string()), 
            None, 
            None
        )
        .speed_condition()
        .entity_ref("ego_vehicle")
        .value(25.0)
        .rule(Rule::GreaterThan)
        .build()
        .unwrap();

        match condition_type {
            ConditionType::ByEntity(by_entity) => {
                assert_eq!(by_entity.triggering_entities.entity_refs.len(), 1);
                assert_eq!(
                    by_entity.triggering_entities.entity_refs[0].entity_ref.as_literal().unwrap(),
                    "ego_vehicle"
                );
                
                match by_entity.condition.choice {
                    EntityCondition::SpeedCondition(speed_condition) => {
                        assert_eq!(speed_condition.value.as_literal().unwrap(), &25.0);
                        assert_eq!(speed_condition.rule, Rule::GreaterThan);
                    }
                    _ => panic!("Expected SpeedCondition"),
                }
            }
            _ => panic!("Expected ByEntity condition"),
        }
    }

    #[test]
    fn test_distance_condition_builder() {
        let position = Position::WorldPosition(WorldPosition {
            x: Value::literal(100.0),
            y: Value::literal(50.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let condition_type = EntityConditionBuilder::new(None, None, None)
            .distance_condition()
            .entity_ref("ego_vehicle")
            .position(position)
            .value(10.0)
            .rule(Rule::LessThan)
            .freespace(true)
            .coordinate_system(CoordinateSystem::Entity)
            .build()
            .unwrap();

        match condition_type {
            ConditionType::ByEntity(by_entity) => {
                match by_entity.condition.choice {
                    EntityCondition::DistanceCondition(distance_condition) => {
                        assert_eq!(distance_condition.value.as_literal().unwrap(), &10.0);
                        assert_eq!(distance_condition.rule, Rule::LessThan);
                        assert!(distance_condition.freespace);
                        assert_eq!(distance_condition.coordinate_system, CoordinateSystem::Entity);
                    }
                    _ => panic!("Expected DistanceCondition"),
                }
            }
            _ => panic!("Expected ByEntity condition"),
        }
    }

    #[test]
    fn test_collision_condition_builder() {
        let condition_type = EntityConditionBuilder::new(None, None, None)
            .collision_condition()
            .entity_ref("ego_vehicle")
            .collision_target("obstacle")
            .build()
            .unwrap();

        match condition_type {
            ConditionType::ByEntity(by_entity) => {
                match by_entity.condition.choice {
                    EntityCondition::CollisionCondition(collision_condition) => {
                        assert_eq!(
                            collision_condition.collision_target.entity_ref.as_literal().unwrap(),
                            "obstacle"
                        );
                    }
                    _ => panic!("Expected CollisionCondition"),
                }
            }
            _ => panic!("Expected ByEntity condition"),
        }
    }

    #[test]
    fn test_speed_condition_missing_value() {
        let result = EntityConditionBuilder::new(None, None, None)
            .speed_condition()
            .entity_ref("ego_vehicle")
            .rule(Rule::GreaterThan)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("value"));
    }

    #[test]
    fn test_distance_condition_missing_position() {
        let result = EntityConditionBuilder::new(None, None, None)
            .distance_condition()
            .entity_ref("ego_vehicle")
            .value(10.0)
            .rule(Rule::LessThan)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("position"));
    }

    #[test]
    fn test_multiple_entity_refs() {
        let condition_type = EntityConditionBuilder::new(None, None, None)
            .speed_condition()
            .entity_ref("ego_vehicle")
            .entity_ref("target_vehicle")
            .triggering_entities_rule(TriggeringEntitiesRule::All)
            .value(25.0)
            .rule(Rule::GreaterThan)
            .build()
            .unwrap();

        match condition_type {
            ConditionType::ByEntity(by_entity) => {
                assert_eq!(by_entity.triggering_entities.entity_refs.len(), 2);
                assert_eq!(
                    by_entity.triggering_entities.triggering_entities_rule,
                    TriggeringEntitiesRule::All
                );
            }
            _ => panic!("Expected ByEntity condition"),
        }
    }

    #[test]
    fn test_builder_with_attributes() {
        let builder = EntityConditionBuilder::new(
            Some("test_condition".to_string()),
            Some(ConditionEdge::Falling),
            Some(ConditionUtils::double(2.0))
        );
        
        assert_eq!(builder.get_name().unwrap(), "test_condition");
        assert_eq!(builder.get_edge().unwrap(), ConditionEdge::Falling);
        assert_eq!(builder.get_delay().unwrap().as_literal().unwrap(), &2.0);
    }
}
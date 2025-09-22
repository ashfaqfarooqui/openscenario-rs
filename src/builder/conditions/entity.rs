//! Entity condition builders for programmatic entity condition construction

use crate::types::{
    basic::{Double, Boolean, OSString},
    conditions::{
        SpeedCondition, AccelerationCondition, StandStillCondition, CollisionCondition,
        OffRoadCondition, EndOfRoadCondition, TimeHeadwayCondition, TimeToCollisionCondition,
        TimeToCollisionTarget, CollisionTarget,
    },
    enums::{Rule, DirectionalDimension, CoordinateSystem, RelativeDistanceType, RoutingAlgorithm},
    positions::Position,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ConditionBuilder, validate_entity_ref, validate_timing, validate_speed, validate_distance};

/// Builder for creating speed conditions
#[derive(Debug, Clone)]
pub struct SpeedConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl SpeedConditionBuilder {
    /// Create a new speed condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the speed value to compare against
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
}

impl ConditionBuilder for SpeedConditionBuilder {
    type ConditionType = SpeedCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for speed condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Speed value is required for speed condition",
            "Call value(), greater_than(), less_than(), or equal_to() to set the speed value"
        ))?;
        
        validate_speed(value, "speed")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(SpeedCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            entity_ref: self.entity_ref.unwrap(),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating acceleration conditions
#[derive(Debug, Clone)]
pub struct AccelerationConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
    direction: Option<DirectionalDimension>,
}

impl AccelerationConditionBuilder {
    /// Create a new acceleration condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
            direction: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the acceleration value to compare against
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set the direction of acceleration measurement
    pub fn direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(direction);
        self
    }
    
    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
}

impl ConditionBuilder for AccelerationConditionBuilder {
    type ConditionType = AccelerationCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for acceleration condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Acceleration value is required for acceleration condition",
            "Call value(), greater_than(), or less_than() to set the acceleration value"
        ))?;
        
        if value.is_nan() || value.is_infinite() {
            return Err(BuilderError::validation_error(
                "Acceleration value is invalid (NaN or infinite)",
                "Provide a valid finite acceleration value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(AccelerationCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            direction: self.direction,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating standstill conditions
#[derive(Debug, Clone)]
pub struct StandStillConditionBuilder {
    entity_ref: Option<String>,
    duration: Option<f64>,
}

impl StandStillConditionBuilder {
    /// Create a new standstill condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            duration: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the duration the entity must be stationary
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl ConditionBuilder for StandStillConditionBuilder {
    type ConditionType = StandStillCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for standstill condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let duration = self.duration.ok_or_else(|| BuilderError::validation_error(
            "Duration is required for standstill condition",
            "Call duration() to set the standstill duration"
        ))?;
        
        validate_timing(duration, "standstill duration")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(StandStillCondition {
            duration: Double::literal(self.duration.unwrap()),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating collision conditions
#[derive(Debug, Clone)]
pub struct CollisionConditionBuilder {
    entity_ref: Option<String>,
    target_entity: Option<String>,
    target_type: Option<String>,
    position: Option<Position>,
}

impl CollisionConditionBuilder {
    /// Create a new collision condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity: None,
            target_type: None,
            position: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity for collision detection
    pub fn with_entity(mut self, target_entity: impl Into<String>) -> Self {
        self.target_entity = Some(target_entity.into());
        self.target_type = None;
        self.position = None;
        self
    }
    
    /// Set the target type for collision detection
    pub fn with_type(mut self, target_type: impl Into<String>) -> Self {
        self.target_type = Some(target_type.into());
        self.target_entity = None;
        self.position = None;
        self
    }
    
    /// Set the target position for collision detection
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self.target_entity = None;
        self.target_type = None;
        self
    }
}

impl ConditionBuilder for CollisionConditionBuilder {
    type ConditionType = CollisionCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for collision condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_count = [
            self.target_entity.is_some(),
            self.target_type.is_some(),
            self.position.is_some(),
        ].iter().filter(|&&x| x).count();
        
        if target_count == 0 {
            return Err(BuilderError::validation_error(
                "At least one collision target is required",
                "Call with_entity(), with_type(), or with_position()"
            ));
        }
        
        if target_count > 1 {
            return Err(BuilderError::validation_error(
                "Only one collision target type can be specified",
                "Choose either entity, type, or position target"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(CollisionCondition {
            target: self.target_entity.map(|e| OSString::literal(e)),
            by_type: self.target_type.map(|t| CollisionTarget {
                target_type: OSString::literal(t),
            }),
            position: self.position,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating offroad conditions
#[derive(Debug, Clone)]
pub struct OffroadConditionBuilder {
    entity_ref: Option<String>,
    duration: Option<f64>,
}

impl OffroadConditionBuilder {
    /// Create a new offroad condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            duration: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the duration the entity must be off-road
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl ConditionBuilder for OffroadConditionBuilder {
    type ConditionType = OffRoadCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for offroad condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let duration = self.duration.ok_or_else(|| BuilderError::validation_error(
            "Duration is required for offroad condition",
            "Call duration() to set the offroad duration"
        ))?;
        
        validate_timing(duration, "offroad duration")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(OffRoadCondition {
            duration: Double::literal(self.duration.unwrap()),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating end of road conditions
#[derive(Debug, Clone)]
pub struct EndOfRoadConditionBuilder {
    entity_ref: Option<String>,
    duration: Option<f64>,
}

impl EndOfRoadConditionBuilder {
    /// Create a new end of road condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            duration: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the duration the entity must be at end of road
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl ConditionBuilder for EndOfRoadConditionBuilder {
    type ConditionType = EndOfRoadCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for end of road condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let duration = self.duration.ok_or_else(|| BuilderError::validation_error(
            "Duration is required for end of road condition",
            "Call duration() to set the end of road duration"
        ))?;
        
        validate_timing(duration, "end of road duration")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(EndOfRoadCondition {
            duration: Double::literal(self.duration.unwrap()),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating time headway conditions
#[derive(Debug, Clone)]
pub struct TimeHeadwayConditionBuilder {
    entity_ref: Option<String>,
    target_entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
    freespace: bool,
    coordinate_system: Option<CoordinateSystem>,
    relative_distance_type: Option<RelativeDistanceType>,
    routing_algorithm: Option<RoutingAlgorithm>,
}

impl TimeHeadwayConditionBuilder {
    /// Create a new time headway condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity_ref: None,
            value: None,
            rule: Rule::LessThan,
            freespace: true,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity for headway measurement
    pub fn target_entity(mut self, target_entity_ref: impl Into<String>) -> Self {
        self.target_entity_ref = Some(target_entity_ref.into());
        self
    }
    
    /// Set the headway time value
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set freespace measurement
    pub fn freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }
    
    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
}

impl ConditionBuilder for TimeHeadwayConditionBuilder {
    type ConditionType = TimeHeadwayCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for time headway condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_entity_ref = self.target_entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Target entity reference is required for time headway condition",
            "Call target_entity() to set the target entity"
        ))?;
        
        validate_entity_ref(target_entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Headway time value is required for time headway condition",
            "Call value(), less_than(), or greater_than() to set the headway time"
        ))?;
        
        validate_timing(value, "headway time")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(TimeHeadwayCondition {
            entity_ref: OSString::literal(self.target_entity_ref.unwrap()),
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            freespace: Boolean::literal(self.freespace),
            coordinate_system: self.coordinate_system,
            relative_distance_type: self.relative_distance_type,
            routing_algorithm: self.routing_algorithm,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating time to collision conditions
#[derive(Debug, Clone)]
pub struct TimeToCollisionConditionBuilder {
    entity_ref: Option<String>,
    target_entity_ref: Option<String>,
    target_position: Option<Position>,
    value: Option<f64>,
    rule: Rule,
    freespace: bool,
    coordinate_system: Option<CoordinateSystem>,
    relative_distance_type: Option<RelativeDistanceType>,
    routing_algorithm: Option<RoutingAlgorithm>,
}

impl TimeToCollisionConditionBuilder {
    /// Create a new time to collision condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity_ref: None,
            target_position: None,
            value: None,
            rule: Rule::LessThan,
            freespace: true,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity for collision prediction
    pub fn target_entity(mut self, target_entity_ref: impl Into<String>) -> Self {
        self.target_entity_ref = Some(target_entity_ref.into());
        self.target_position = None;
        self
    }
    
    /// Set the target position for collision prediction
    pub fn target_position(mut self, position: Position) -> Self {
        self.target_position = Some(position);
        self.target_entity_ref = None;
        self
    }
    
    /// Set the time to collision value
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set freespace measurement
    pub fn freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }
    
    /// Set less than rule
    pub fn less_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
}

impl ConditionBuilder for TimeToCollisionConditionBuilder {
    type ConditionType = TimeToCollisionCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for time to collision condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.target_entity_ref.is_none() && self.target_position.is_none() {
            return Err(BuilderError::validation_error(
                "Either target entity or target position is required",
                "Call target_entity() or target_position()"
            ));
        }
        
        if let Some(target_entity_ref) = &self.target_entity_ref {
            validate_entity_ref(target_entity_ref)?;
        }
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Time to collision value is required",
            "Call value(), less_than(), or greater_than() to set the time to collision"
        ))?;
        
        validate_timing(value, "time to collision")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        let target = TimeToCollisionTarget {
            entity_ref: self.target_entity_ref.map(|e| OSString::literal(e)),
            position: self.target_position,
        };
        
        Ok(TimeToCollisionCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            freespace: Boolean::literal(self.freespace),
            coordinate_system: self.coordinate_system,
            relative_distance_type: self.relative_distance_type,
            routing_algorithm: self.routing_algorithm,
            target,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Placeholder for relative speed condition builder
#[derive(Debug, Clone)]
pub struct RelativeSpeedConditionBuilder {
    entity_ref: Option<String>,
    target_entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl RelativeSpeedConditionBuilder {
    /// Create a new relative speed condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity for relative speed measurement
    pub fn target_entity(mut self, target_entity_ref: impl Into<String>) -> Self {
        self.target_entity_ref = Some(target_entity_ref.into());
        self
    }
    
    /// Set the relative speed value
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
}

impl ConditionBuilder for RelativeSpeedConditionBuilder {
    type ConditionType = SpeedCondition; // Simplified for now
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for relative speed condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_entity_ref = self.target_entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Target entity reference is required for relative speed condition",
            "Call target_entity() to set the target entity"
        ))?;
        
        validate_entity_ref(target_entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Relative speed value is required",
            "Call value() to set the relative speed value"
        ))?;
        
        validate_speed(value, "relative speed")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        // This is a simplified implementation - in reality we'd need a proper RelativeSpeedCondition type
        Ok(SpeedCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            entity_ref: self.entity_ref.unwrap(),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for SpeedConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AccelerationConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StandStillConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CollisionConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OffroadConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EndOfRoadConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TimeHeadwayConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TimeToCollisionConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RelativeSpeedConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_speed_condition_builder() {
        let condition = SpeedConditionBuilder::new()
            .entity("test_vehicle")
            .greater_than(25.0)
            .finish()
            .unwrap();
        
        assert_eq!(condition.value.as_literal().unwrap(), &25.0);
        assert_eq!(condition.rule, Rule::GreaterThan);
        assert_eq!(condition.entity_ref, "test_vehicle");
    }
    
    #[test]
    fn test_speed_condition_builder_validation() {
        let result = SpeedConditionBuilder::new()
            .entity("test_vehicle")
            // Missing speed value
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Speed value"));
    }
    
    #[test]
    fn test_collision_condition_builder() {
        let condition = CollisionConditionBuilder::new()
            .entity("test_vehicle")
            .with_entity("target_vehicle")
            .finish()
            .unwrap();
        
        assert_eq!(condition.target.unwrap().as_literal().unwrap(), &"target_vehicle".to_string());
        assert!(condition.by_type.is_none());
        assert!(condition.position.is_none());
    }
    
    #[test]
    fn test_standstill_condition_builder() {
        let condition = StandStillConditionBuilder::new()
            .entity("test_vehicle")
            .duration(5.0)
            .finish()
            .unwrap();
        
        assert_eq!(condition.duration.as_literal().unwrap(), &5.0);
    }
    
    #[test]
    fn test_time_headway_condition_builder() {
        let condition = TimeHeadwayConditionBuilder::new()
            .entity("ego")
            .target_entity("lead_vehicle")
            .less_than(2.0)
            .freespace(true)
            .finish()
            .unwrap();
        
        assert_eq!(condition.entity_ref.as_literal().unwrap(), &"lead_vehicle".to_string());
        assert_eq!(condition.value.as_literal().unwrap(), &2.0);
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(condition.freespace.as_literal().unwrap(), &true);
    }
}
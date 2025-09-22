//! Spatial condition builders for programmatic spatial condition construction

use crate::types::{
    basic::{Double, Boolean, OSString},
    conditions::{
        ReachPositionCondition, DistanceCondition, RelativeDistanceCondition,
    },
    enums::{Rule, CoordinateSystem, RelativeDistanceType, RoutingAlgorithm},
    positions::Position,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ConditionBuilder, validate_entity_ref, validate_distance};

/// Builder for creating reach position conditions
#[derive(Debug, Clone)]
pub struct ReachPositionConditionBuilder {
    entity_ref: Option<String>,
    position: Option<Position>,
    tolerance: Option<f64>,
}

impl ReachPositionConditionBuilder {
    /// Create a new reach position condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            position: None,
            tolerance: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target position
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    /// Set the tolerance for reaching the position
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }
}

impl ConditionBuilder for ReachPositionConditionBuilder {
    type ConditionType = ReachPositionCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for reach position condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.position.is_none() {
            return Err(BuilderError::validation_error(
                "Position is required for reach position condition",
                "Call position() to set the target position"
            ));
        }
        
        let tolerance = self.tolerance.ok_or_else(|| BuilderError::validation_error(
            "Tolerance is required for reach position condition",
            "Call tolerance() to set the position tolerance"
        ))?;
        
        validate_distance(tolerance, "tolerance")?;
        
        if tolerance < 0.0 {
            return Err(BuilderError::validation_error(
                "Tolerance cannot be negative",
                "Provide a non-negative tolerance value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(ReachPositionCondition {
            position: self.position.unwrap(),
            tolerance: Double::literal(self.tolerance.unwrap()),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating distance conditions
#[derive(Debug, Clone)]
pub struct DistanceConditionBuilder {
    entity_ref: Option<String>,
    position: Option<Position>,
    value: Option<f64>,
    rule: Rule,
    freespace: bool,
    along_route: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    relative_distance_type: Option<RelativeDistanceType>,
    routing_algorithm: Option<RoutingAlgorithm>,
}

impl DistanceConditionBuilder {
    /// Create a new distance condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            position: None,
            value: None,
            rule: Rule::LessThan,
            freespace: true,
            along_route: None,
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
    
    /// Set the reference position
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    /// Set the distance value to compare against
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
    
    /// Set along route measurement
    pub fn along_route(mut self, along_route: bool) -> Self {
        self.along_route = Some(along_route);
        self
    }
    
    /// Set coordinate system
    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }
    
    /// Set relative distance type
    pub fn relative_distance_type(mut self, relative_distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(relative_distance_type);
        self
    }
    
    /// Set routing algorithm
    pub fn routing_algorithm(mut self, routing_algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(routing_algorithm);
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
    
    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
}

impl ConditionBuilder for DistanceConditionBuilder {
    type ConditionType = DistanceCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for distance condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.position.is_none() {
            return Err(BuilderError::validation_error(
                "Position is required for distance condition",
                "Call position() to set the reference position"
            ));
        }
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Distance value is required for distance condition",
            "Call value(), less_than(), greater_than(), or equal_to() to set the distance value"
        ))?;
        
        validate_distance(value, "distance")?;
        
        if value < 0.0 {
            return Err(BuilderError::validation_error(
                "Distance value cannot be negative",
                "Provide a non-negative distance value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(DistanceCondition {
            position: self.position.unwrap(),
            value: Double::literal(self.value.unwrap()),
            freespace: Boolean::literal(self.freespace),
            rule: self.rule,
            along_route: self.along_route.map(|v| Boolean::literal(v)),
            coordinate_system: self.coordinate_system,
            relative_distance_type: self.relative_distance_type,
            routing_algorithm: self.routing_algorithm,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating relative distance conditions
#[derive(Debug, Clone)]
pub struct RelativeDistanceConditionBuilder {
    entity_ref: Option<String>,
    target_entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
    freespace: bool,
    relative_distance_type: RelativeDistanceType,
    coordinate_system: Option<CoordinateSystem>,
    routing_algorithm: Option<RoutingAlgorithm>,
}

impl RelativeDistanceConditionBuilder {
    /// Create a new relative distance condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_entity_ref: None,
            value: None,
            rule: Rule::LessThan,
            freespace: true,
            relative_distance_type: RelativeDistanceType::EuclideanDistance,
            coordinate_system: None,
            routing_algorithm: None,
        }
    }
    
    /// Set the entity to monitor
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the target entity for distance measurement
    pub fn target_entity(mut self, target_entity_ref: impl Into<String>) -> Self {
        self.target_entity_ref = Some(target_entity_ref.into());
        self
    }
    
    /// Set the distance value to compare against
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
    
    /// Set relative distance type
    pub fn relative_distance_type(mut self, relative_distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = relative_distance_type;
        self
    }
    
    /// Set coordinate system
    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }
    
    /// Set routing algorithm
    pub fn routing_algorithm(mut self, routing_algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(routing_algorithm);
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
    
    /// Set equal to rule
    pub fn equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
}

impl ConditionBuilder for RelativeDistanceConditionBuilder {
    type ConditionType = RelativeDistanceCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for relative distance condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let target_entity_ref = self.target_entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Target entity reference is required for relative distance condition",
            "Call target_entity() to set the target entity"
        ))?;
        
        validate_entity_ref(target_entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Distance value is required for relative distance condition",
            "Call value(), less_than(), greater_than(), or equal_to() to set the distance value"
        ))?;
        
        validate_distance(value, "relative distance")?;
        
        if value < 0.0 {
            return Err(BuilderError::validation_error(
                "Distance value cannot be negative",
                "Provide a non-negative distance value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(RelativeDistanceCondition {
            entity_ref: OSString::literal(self.target_entity_ref.unwrap()),
            value: Double::literal(self.value.unwrap()),
            freespace: Boolean::literal(self.freespace),
            relative_distance_type: self.relative_distance_type,
            rule: self.rule,
            coordinate_system: self.coordinate_system,
            routing_algorithm: self.routing_algorithm,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating traveled distance conditions
#[derive(Debug, Clone)]
pub struct TraveledDistanceConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl TraveledDistanceConditionBuilder {
    /// Create a new traveled distance condition builder
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
    
    /// Set the distance value to compare against
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

impl ConditionBuilder for TraveledDistanceConditionBuilder {
    type ConditionType = DistanceCondition; // Simplified for now - would need TraveledDistanceCondition type
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for traveled distance condition",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Distance value is required for traveled distance condition",
            "Call value(), greater_than(), less_than(), or equal_to() to set the distance value"
        ))?;
        
        validate_distance(value, "traveled distance")?;
        
        if value < 0.0 {
            return Err(BuilderError::validation_error(
                "Distance value cannot be negative",
                "Provide a non-negative distance value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        // This is a simplified implementation - in reality we'd need a proper TraveledDistanceCondition type
        Ok(DistanceCondition {
            position: Position::default(), // Would need to be properly implemented
            value: Double::literal(self.value.unwrap()),
            freespace: Boolean::literal(true),
            rule: self.rule,
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for ReachPositionConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DistanceConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RelativeDistanceConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TraveledDistanceConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::{Position, WorldPosition};
    
    #[test]
    fn test_reach_position_condition_builder() {
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
        
        let condition = ReachPositionConditionBuilder::new()
            .entity("test_vehicle")
            .position(position)
            .tolerance(2.0)
            .finish()
            .unwrap();
        
        assert_eq!(condition.tolerance.as_literal().unwrap(), &2.0);
        assert!(condition.position.world_position.is_some());
    }
    
    #[test]
    fn test_distance_condition_builder() {
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
        
        let condition = DistanceConditionBuilder::new()
            .entity("test_vehicle")
            .position(position)
            .less_than(50.0)
            .freespace(true)
            .finish()
            .unwrap();
        
        assert_eq!(condition.value.as_literal().unwrap(), &50.0);
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(condition.freespace.as_literal().unwrap(), &true);
    }
    
    #[test]
    fn test_relative_distance_condition_builder() {
        let condition = RelativeDistanceConditionBuilder::new()
            .entity("ego")
            .target_entity("lead_vehicle")
            .less_than(10.0)
            .freespace(false)
            .finish()
            .unwrap();
        
        assert_eq!(condition.entity_ref.as_literal().unwrap(), &"lead_vehicle".to_string());
        assert_eq!(condition.value.as_literal().unwrap(), &10.0);
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(condition.freespace.as_literal().unwrap(), &false);
    }
    
    #[test]
    fn test_distance_condition_builder_validation() {
        let result = DistanceConditionBuilder::new()
            .entity("test_vehicle")
            // Missing position and value
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Position"));
    }
}
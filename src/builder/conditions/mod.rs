//! Condition builder framework for programmatic condition construction
//!
//! This module provides type-safe, ergonomic builders for constructing
//! OpenSCENARIO conditions programmatically. The builder pattern ensures
//! compile-time safety and guides users through the required construction steps.

pub mod entity;
pub mod value;
pub mod spatial;

// Re-export builders for convenience
pub use entity::EntityConditionBuilder;
pub use value::ValueConditionBuilder;
pub use spatial::SpatialConditionBuilder;

use crate::types::conditions::{ByEntityCondition, ByValueCondition};
use crate::types::scenario::triggers::{Condition, ConditionType};
use crate::types::basic::{OSString, Double, Value};
use crate::types::enums::ConditionEdge;
use super::error::{BuilderError, BuilderResult};
use super::states::*;
use std::marker::PhantomData;

/// Main condition builder for constructing OpenSCENARIO conditions
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO conditions. The type parameter S tracks the current
/// construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasType, etc.)
/// 
/// # Example
/// ```rust
/// let condition = ConditionBuilder::new()
///     .with_name("speed_check")
///     .entity()
///     .speed_condition()
///     .entity_ref("ego_vehicle")
///     .value(25.0)
///     .rule(Rule::GreaterThan)
///     .build()?;
/// ```
pub struct ConditionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Condition name for identification
    name: Option<String>,
    
    /// Condition edge detection mode
    edge: Option<ConditionEdge>,
    
    /// Optional delay before condition fires
    delay: Option<Double>,
    
    /// Partially constructed condition data
    condition_data: Option<ConditionType>,
}

// Core builder implementation for Empty state
impl ConditionBuilder<Empty> {
    /// Create a new condition builder
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// a condition type as the first required element.
    /// 
    /// # Returns
    /// A new ConditionBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            name: None,
            edge: None,
            delay: None,
            condition_data: None,
        }
    }

    /// Set the condition name
    /// 
    /// # Arguments
    /// * `name` - Condition name for identification
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new().with_name("speed_check");
    /// ```
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set the condition edge detection mode
    /// 
    /// # Arguments
    /// * `edge` - Edge detection mode (Rising, Falling, RisingOrFalling, None)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new().with_edge(ConditionEdge::Rising);
    /// ```
    pub fn with_edge(mut self, edge: ConditionEdge) -> Self {
        self.edge = Some(edge);
        self
    }

    /// Set a delay before the condition fires
    /// 
    /// # Arguments
    /// * `delay` - Delay in seconds
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new().with_delay(2.0);
    /// ```
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(ConditionUtils::double(delay));
        self
    }

    /// Start building an entity-based condition
    /// 
    /// Entity-based conditions evaluate properties of specific entities,
    /// such as speed, position, distance, or collision states.
    /// 
    /// # Returns
    /// EntityConditionBuilder for configuring entity conditions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new()
    ///     .entity()
    ///     .speed_condition()
    ///     .entity_ref("ego_vehicle");
    /// ```
    pub fn entity(self) -> EntityConditionBuilder<Empty> {
        EntityConditionBuilder::new(self.name, self.edge, self.delay)
    }

    /// Start building a value-based condition
    /// 
    /// Value-based conditions evaluate simulation state, parameters,
    /// variables, or time-based conditions.
    /// 
    /// # Returns
    /// ValueConditionBuilder for configuring value conditions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new()
    ///     .value()
    ///     .simulation_time_condition()
    ///     .value(10.0)
    ///     .rule(Rule::GreaterThan);
    /// ```
    pub fn value(self) -> ValueConditionBuilder<Empty> {
        ValueConditionBuilder::new(self.name, self.edge, self.delay)
    }

    /// Start building a spatial condition
    /// 
    /// Spatial conditions evaluate geometric relationships between entities
    /// and positions, such as reach position, collision, or area conditions.
    /// 
    /// # Returns
    /// SpatialConditionBuilder for configuring spatial conditions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ConditionBuilder::new()
    ///     .spatial()
    ///     .reach_position_condition()
    ///     .entity_ref("ego_vehicle")
    ///     .position(target_position);
    /// ```
    pub fn spatial(self) -> SpatialConditionBuilder<Empty> {
        SpatialConditionBuilder::new(self.name, self.edge, self.delay)
    }
}

/// Condition wrapper with name and timing attributes
#[derive(Debug, Clone)]
pub struct ConditionWrapper {
    /// Condition name for identification
    pub name: String,
    
    /// Edge detection mode
    pub edge: ConditionEdge,
    
    /// Optional delay
    pub delay: Option<Double>,
    
    /// The actual condition content
    pub condition: ConditionType,
}

impl ConditionWrapper {
    /// Create a new condition wrapper
    pub fn new(
        name: String, 
        edge: ConditionEdge, 
        delay: Option<Double>, 
        condition: ConditionType
    ) -> Self {
        Self { name, edge, delay, condition }
    }

    /// Convert to the scenario trigger Condition type
    pub fn to_trigger_condition(self) -> Condition {
        let (by_value_condition, by_entity_condition) = match self.condition {
            ConditionType::ByValue(cond) => (Some(cond), None),
            ConditionType::ByEntity(cond) => (None, Some(cond)),
        };

        Condition {
            name: ConditionUtils::os_string(&self.name),
            condition_edge: self.edge,
            delay: self.delay,
            by_value_condition,
            by_entity_condition,
        }
    }
}

/// Trait for building conditions with common functionality
pub trait ConditionBuilderTrait {
    /// Build the final condition
    fn build_condition(self) -> BuilderResult<ConditionType>;
    
    /// Get the condition name if set
    fn get_name(&self) -> Option<&str>;
    
    /// Get the condition edge if set
    fn get_edge(&self) -> Option<ConditionEdge>;
    
    /// Get the condition delay if set
    fn get_delay(&self) -> Option<&Double>;
}

/// Common condition building utilities
pub struct ConditionUtils;

impl ConditionUtils {
    /// Validate condition name
    pub fn validate_name(name: &str) -> BuilderResult<()> {
        if name.is_empty() {
            return Err(BuilderError::validation_error(
                "Condition name cannot be empty"
            ));
        }
        
        if name.len() > 255 {
            return Err(BuilderError::validation_error(
                "Condition name cannot exceed 255 characters"
            ));
        }
        
        Ok(())
    }
    
    /// Create an OSString value from a string
    pub fn os_string(value: &str) -> OSString {
        Value::literal(value.to_string())
    }
    
    /// Create a double value
    pub fn double(value: f64) -> Double {
        Value::literal(value)
    }
    
    /// Create an unsigned int value
    pub fn unsigned_int(value: u32) -> crate::types::basic::UnsignedInt {
        Value::literal(value)
    }
    
    /// Create an int value
    pub fn int(value: i32) -> crate::types::basic::Int {
        Value::literal(value)
    }
    
    /// Create a boolean value
    pub fn boolean(value: bool) -> crate::types::basic::Boolean {
        Value::literal(value)
    }
}

/// Convenience functions for common condition patterns
impl ConditionBuilder<Empty> {
    /// Create a simulation time condition quickly
    /// 
    /// # Arguments
    /// * `name` - Condition name
    /// * `time` - Time value in seconds
    /// * `rule` - Comparison rule
    /// 
    /// # Returns
    /// Complete condition wrapper
    /// 
    /// # Example
    /// ```rust
    /// let condition = ConditionBuilder::simulation_time("start_time", 5.0, Rule::GreaterThan)?;
    /// ```
    pub fn simulation_time(
        name: &str, 
        time: f64, 
        rule: crate::types::enums::Rule
    ) -> BuilderResult<ConditionWrapper> {
        let condition_type = Self::new()
            .value()
            .simulation_time_condition()
            .value(time)
            .rule(rule)
            .build_condition()?;

        Ok(ConditionWrapper::new(
            name.to_string(),
            ConditionEdge::Rising,
            None,
            condition_type,
        ))
    }

    /// Create a speed condition quickly
    /// 
    /// # Arguments
    /// * `name` - Condition name
    /// * `entity_ref` - Entity reference
    /// * `speed` - Speed value in m/s
    /// * `rule` - Comparison rule
    /// 
    /// # Returns
    /// Complete condition wrapper
    /// 
    /// # Example
    /// ```rust
    /// let condition = ConditionBuilder::speed("speed_check", "ego", 25.0, Rule::GreaterThan)?;
    /// ```
    pub fn speed(
        name: &str, 
        entity_ref: &str, 
        speed: f64, 
        rule: crate::types::enums::Rule
    ) -> BuilderResult<ConditionWrapper> {
        let condition_type = Self::new()
            .entity()
            .speed_condition()
            .entity_ref(entity_ref)
            .value(speed)
            .rule(rule)
            .build_condition()?;

        Ok(ConditionWrapper::new(
            name.to_string(),
            ConditionEdge::Rising,
            None,
            condition_type,
        ))
    }

    /// Create a distance condition quickly
    /// 
    /// # Arguments
    /// * `name` - Condition name
    /// * `entity_ref` - Entity reference
    /// * `target_position` - Target position
    /// * `distance` - Distance value in meters
    /// * `rule` - Comparison rule
    /// 
    /// # Returns
    /// Complete condition wrapper
    /// 
    /// # Example
    /// ```rust
    /// let condition = ConditionBuilder::distance("near_target", "ego", position, 10.0, Rule::LessThan)?;
    /// ```
    pub fn distance(
        name: &str, 
        entity_ref: &str, 
        target_position: crate::types::positions::Position,
        distance: f64, 
        rule: crate::types::enums::Rule
    ) -> BuilderResult<ConditionWrapper> {
        let condition_type = Self::new()
            .entity()
            .distance_condition()
            .entity_ref(entity_ref)
            .position(target_position)
            .value(distance)
            .rule(rule)
            .build_condition()?;

        Ok(ConditionWrapper::new(
            name.to_string(),
            ConditionEdge::Rising,
            None,
            condition_type,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::{ConditionEdge, Rule};

    #[test]
    fn test_condition_builder_creation() {
        let builder = ConditionBuilder::new();
        assert!(builder.name.is_none());
        assert!(builder.edge.is_none());
        assert!(builder.delay.is_none());
    }

    #[test]
    fn test_condition_builder_with_attributes() {
        let builder = ConditionBuilder::new()
            .with_name("test_condition")
            .with_edge(ConditionEdge::Falling)
            .with_delay(2.5);
        
        assert_eq!(builder.name.as_ref().unwrap(), "test_condition");
        assert_eq!(builder.edge.unwrap(), ConditionEdge::Falling);
        assert_eq!(builder.delay.as_ref().unwrap().as_literal().unwrap(), &2.5);
    }

    #[test]
    fn test_condition_utils_validate_name() {
        assert!(ConditionUtils::validate_name("valid_name").is_ok());
        assert!(ConditionUtils::validate_name("").is_err());
        assert!(ConditionUtils::validate_name(&"x".repeat(256)).is_err());
    }

    #[test]
    fn test_condition_utils_value_creation() {
        let os_string = ConditionUtils::os_string("test");
        assert_eq!(os_string.as_literal().unwrap(), "test");
        
        let double_val = ConditionUtils::double(25.5);
        assert_eq!(double_val.as_literal().unwrap(), &25.5);
        
        let uint_val = ConditionUtils::unsigned_int(42);
        assert_eq!(uint_val.as_literal().unwrap(), &42);
        
        let int_val = ConditionUtils::int(-10);
        assert_eq!(int_val.as_literal().unwrap(), &-10);
        
        let bool_val = ConditionUtils::boolean(true);
        assert_eq!(bool_val.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_condition_wrapper_to_trigger_condition() {
        let condition_type = ConditionType::ByValue(ByValueCondition::default());
        let wrapper = ConditionWrapper::new(
            "test".to_string(),
            ConditionEdge::Rising,
            Some(ConditionUtils::double(1.0)),
            condition_type,
        );

        let trigger_condition = wrapper.to_trigger_condition();
        assert_eq!(trigger_condition.name.as_literal().unwrap(), "test");
        assert_eq!(trigger_condition.condition_edge, ConditionEdge::Rising);
        assert_eq!(trigger_condition.delay.as_ref().unwrap().as_literal().unwrap(), &1.0);
        assert!(trigger_condition.by_value_condition.is_some());
        assert!(trigger_condition.by_entity_condition.is_none());
    }

    #[test]
    fn test_simulation_time_convenience_function() {
        let condition = ConditionBuilder::simulation_time("start_time", 5.0, Rule::GreaterThan)
            .unwrap();
        
        assert_eq!(condition.name, "start_time");
        assert_eq!(condition.edge, ConditionEdge::Rising);
        assert!(condition.delay.is_none());
        
        match condition.condition {
            ConditionType::ByValue(_) => {
                // Expected
            }
            _ => panic!("Expected ByValue condition"),
        }
    }
}
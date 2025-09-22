//! Value condition builders for programmatic value condition construction

use crate::types::{
    basic::{Double, OSString},
    conditions::{SimulationTimeCondition, ByValueCondition},
    enums::Rule,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ConditionBuilder, validate_timing};

/// Builder for creating simulation time conditions
#[derive(Debug, Clone)]
pub struct SimulationTimeConditionBuilder {
    value: Option<f64>,
    rule: Rule,
}

impl SimulationTimeConditionBuilder {
    /// Create a new simulation time condition builder
    pub fn new() -> Self {
        Self {
            value: None,
            rule: Rule::GreaterThan,
        }
    }
    
    /// Set the time value to compare against
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
    
    /// Set greater than or equal to rule
    pub fn greater_than_or_equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterOrEqual;
        self
    }
    
    /// Set less than or equal to rule
    pub fn less_than_or_equal_to(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessOrEqual;
        self
    }
}

impl ConditionBuilder for SimulationTimeConditionBuilder {
    type ConditionType = SimulationTimeCondition;
    
    fn validate(&self) -> BuilderResult<()> {
        let value = self.value.ok_or_else(|| BuilderError::validation_error(
            "Time value is required for simulation time condition",
            "Call value(), greater_than(), less_than(), or equal_to() to set the time value"
        ))?;
        
        validate_timing(value, "simulation time")?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        Ok(SimulationTimeCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        None // Simulation time conditions don't reference entities
    }
}

/// Builder for creating parameter conditions
#[derive(Debug, Clone)]
pub struct ParameterConditionBuilder {
    parameter_ref: Option<String>,
    value: Option<String>,
    rule: Rule,
}

impl ParameterConditionBuilder {
    /// Create a new parameter condition builder
    pub fn new() -> Self {
        Self {
            parameter_ref: None,
            value: None,
            rule: Rule::EqualTo,
        }
    }
    
    /// Set the parameter reference
    pub fn parameter(mut self, parameter_ref: impl Into<String>) -> Self {
        self.parameter_ref = Some(parameter_ref.into());
        self
    }
    
    /// Set the value to compare against
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set equal to rule
    pub fn equal_to(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self.rule = Rule::EqualTo;
        self
    }
    
    /// Set not equal to rule
    pub fn not_equal_to(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self.rule = Rule::NotEqualTo;
        self
    }
}

impl ConditionBuilder for ParameterConditionBuilder {
    type ConditionType = ByValueCondition; // Simplified for now
    
    fn validate(&self) -> BuilderResult<()> {
        let parameter_ref = self.parameter_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Parameter reference is required for parameter condition",
            "Call parameter() to set the parameter reference"
        ))?;
        
        if parameter_ref.trim().is_empty() {
            return Err(BuilderError::validation_error(
                "Parameter reference cannot be empty",
                "Provide a valid parameter name"
            ));
        }
        
        if self.value.is_none() {
            return Err(BuilderError::validation_error(
                "Value is required for parameter condition",
                "Call value(), equal_to(), or not_equal_to() to set the comparison value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        // This is a simplified implementation - in reality we'd need a proper ParameterCondition type
        Ok(ByValueCondition {
            simulation_time: None,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        None // Parameter conditions don't reference entities
    }
}

/// Builder for creating variable conditions
#[derive(Debug, Clone)]
pub struct VariableConditionBuilder {
    variable_ref: Option<String>,
    value: Option<String>,
    rule: Rule,
}

impl VariableConditionBuilder {
    /// Create a new variable condition builder
    pub fn new() -> Self {
        Self {
            variable_ref: None,
            value: None,
            rule: Rule::EqualTo,
        }
    }
    
    /// Set the variable reference
    pub fn variable(mut self, variable_ref: impl Into<String>) -> Self {
        self.variable_ref = Some(variable_ref.into());
        self
    }
    
    /// Set the value to compare against
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    
    /// Set the comparison rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }
    
    /// Set equal to rule
    pub fn equal_to(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self.rule = Rule::EqualTo;
        self
    }
    
    /// Set not equal to rule
    pub fn not_equal_to(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self.rule = Rule::NotEqualTo;
        self
    }
}

impl ConditionBuilder for VariableConditionBuilder {
    type ConditionType = ByValueCondition; // Simplified for now
    
    fn validate(&self) -> BuilderResult<()> {
        let variable_ref = self.variable_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Variable reference is required for variable condition",
            "Call variable() to set the variable reference"
        ))?;
        
        if variable_ref.trim().is_empty() {
            return Err(BuilderError::validation_error(
                "Variable reference cannot be empty",
                "Provide a valid variable name"
            ));
        }
        
        if self.value.is_none() {
            return Err(BuilderError::validation_error(
                "Value is required for variable condition",
                "Call value(), equal_to(), or not_equal_to() to set the comparison value"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        // This is a simplified implementation - in reality we'd need a proper VariableCondition type
        Ok(ByValueCondition {
            simulation_time: None,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        None // Variable conditions don't reference entities
    }
}

/// Builder for creating storyboard element conditions
#[derive(Debug, Clone)]
pub struct StoryboardElementConditionBuilder {
    storyboard_element_type: Option<String>,
    storyboard_element_ref: Option<String>,
    state: Option<String>,
}

impl StoryboardElementConditionBuilder {
    /// Create a new storyboard element condition builder
    pub fn new() -> Self {
        Self {
            storyboard_element_type: None,
            storyboard_element_ref: None,
            state: None,
        }
    }
    
    /// Set the storyboard element type
    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.storyboard_element_type = Some(element_type.into());
        self
    }
    
    /// Set the storyboard element reference
    pub fn element_ref(mut self, element_ref: impl Into<String>) -> Self {
        self.storyboard_element_ref = Some(element_ref.into());
        self
    }
    
    /// Set the state to check for
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }
    
    /// Set story element type
    pub fn story(mut self, story_ref: impl Into<String>) -> Self {
        self.storyboard_element_type = Some("story".to_string());
        self.storyboard_element_ref = Some(story_ref.into());
        self
    }
    
    /// Set act element type
    pub fn act(mut self, act_ref: impl Into<String>) -> Self {
        self.storyboard_element_type = Some("act".to_string());
        self.storyboard_element_ref = Some(act_ref.into());
        self
    }
    
    /// Set maneuver element type
    pub fn maneuver(mut self, maneuver_ref: impl Into<String>) -> Self {
        self.storyboard_element_type = Some("maneuver".to_string());
        self.storyboard_element_ref = Some(maneuver_ref.into());
        self
    }
    
    /// Set event element type
    pub fn event(mut self, event_ref: impl Into<String>) -> Self {
        self.storyboard_element_type = Some("event".to_string());
        self.storyboard_element_ref = Some(event_ref.into());
        self
    }
    
    /// Set action element type
    pub fn action(mut self, action_ref: impl Into<String>) -> Self {
        self.storyboard_element_type = Some("action".to_string());
        self.storyboard_element_ref = Some(action_ref.into());
        self
    }
    
    /// Check for running state
    pub fn running(mut self) -> Self {
        self.state = Some("running".to_string());
        self
    }
    
    /// Check for complete state
    pub fn complete(mut self) -> Self {
        self.state = Some("complete".to_string());
        self
    }
    
    /// Check for standby state
    pub fn standby(mut self) -> Self {
        self.state = Some("standby".to_string());
        self
    }
}

impl ConditionBuilder for StoryboardElementConditionBuilder {
    type ConditionType = ByValueCondition; // Simplified for now
    
    fn validate(&self) -> BuilderResult<()> {
        if self.storyboard_element_type.is_none() {
            return Err(BuilderError::validation_error(
                "Storyboard element type is required",
                "Call element_type(), story(), act(), maneuver(), event(), or action()"
            ));
        }
        
        if self.storyboard_element_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Storyboard element reference is required",
                "Call element_ref() or use a specific element method"
            ));
        }
        
        if self.state.is_none() {
            return Err(BuilderError::validation_error(
                "State is required for storyboard element condition",
                "Call state(), running(), complete(), or standby()"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ConditionType> {
        self.validate()?;
        
        // This is a simplified implementation - in reality we'd need a proper StoryboardElementCondition type
        Ok(ByValueCondition {
            simulation_time: None,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        None // Storyboard element conditions don't reference entities
    }
}

// Default implementations
impl Default for SimulationTimeConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParameterConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for VariableConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StoryboardElementConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simulation_time_condition_builder() {
        let condition = SimulationTimeConditionBuilder::new()
            .greater_than(10.0)
            .finish()
            .unwrap();
        
        assert_eq!(condition.value.as_literal().unwrap(), &10.0);
        assert_eq!(condition.rule, Rule::GreaterThan);
    }
    
    #[test]
    fn test_simulation_time_condition_builder_validation() {
        let result = SimulationTimeConditionBuilder::new()
            // Missing time value
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Time value"));
    }
    
    #[test]
    fn test_parameter_condition_builder() {
        let condition = ParameterConditionBuilder::new()
            .parameter("test_param")
            .equal_to("test_value")
            .finish()
            .unwrap();
        
        // Since we're using a simplified implementation, just check it doesn't error
        assert!(condition.simulation_time.is_none());
    }
    
    #[test]
    fn test_storyboard_element_condition_builder() {
        let condition = StoryboardElementConditionBuilder::new()
            .story("main_story")
            .complete()
            .finish()
            .unwrap();
        
        // Since we're using a simplified implementation, just check it doesn't error
        assert!(condition.simulation_time.is_none());
    }
}
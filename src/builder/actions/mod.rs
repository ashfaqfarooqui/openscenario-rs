//! Action builder framework for programmatic action construction
//!
//! This module provides type-safe, ergonomic builders for constructing
//! OpenSCENARIO actions programmatically. The builder pattern ensures
//! compile-time safety and guides users through the required construction steps.

pub mod longitudinal;
pub mod lateral;
pub mod private;
pub mod global;

// Re-export builders for convenience
pub use longitudinal::LongitudinalActionBuilder;
pub use lateral::LateralActionBuilder;
pub use private::PrivateActionBuilder;
pub use global::GlobalActionBuilder;

use crate::types::actions::Action;
use crate::types::basic::{OSString, Value};
use super::error::{BuilderError, BuilderResult};
use super::states::*;
use std::marker::PhantomData;

/// Main action builder for constructing OpenSCENARIO actions
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO actions. The type parameter S tracks the current
/// construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasType, etc.)
/// 
/// # Example
/// ```rust
/// let action = ActionBuilder::new()
///     .longitudinal()
///     .speed_action()
///     .target_speed(30.0)
///     .dynamics_shape(DynamicsShape::Linear)
///     .dynamics_dimension(DynamicsDimension::Rate)
///     .dynamics_value(2.0)
///     .build()?;
/// ```
pub struct ActionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Action name for identification
    name: Option<String>,
    
    /// Partially constructed action data
    action_data: Option<Action>,
}

/// Internal data structure for building actions
/// 
/// This structure holds the partially constructed action data as it's
/// being built. Fields start as None and are populated as the builder
/// progresses through states.
#[derive(Debug, Default)]
struct PartialActionData {
    /// Action type and configuration
    action: Option<Action>,
}

// Core builder implementation for Empty state
impl ActionBuilder<Empty> {
    /// Create a new action builder
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// an action type as the first required element.
    /// 
    /// # Returns
    /// A new ActionBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            name: None,
            action_data: None,
        }
    }

    /// Set the action name
    /// 
    /// # Arguments
    /// * `name` - Action name for identification
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new().with_name("accelerate");
    /// ```
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Start building a longitudinal action
    /// 
    /// Longitudinal actions affect movement along the entity's forward direction,
    /// such as speed changes, acceleration, and following behaviors.
    /// 
    /// # Returns
    /// LongitudinalActionBuilder for configuring longitudinal actions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new()
    ///     .longitudinal()
    ///     .speed_action()
    ///     .target_speed(25.0);
    /// ```
    pub fn longitudinal(self) -> LongitudinalActionBuilder<Empty> {
        LongitudinalActionBuilder::new(self.name)
    }

    /// Start building a lateral action
    /// 
    /// Lateral actions affect movement perpendicular to the entity's forward direction,
    /// such as lane changes, lateral offsets, and lateral distance actions.
    /// 
    /// # Returns
    /// LateralActionBuilder for configuring lateral actions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new()
    ///     .lateral()
    ///     .lane_change_action()
    ///     .target_lane_relative(1);
    /// ```
    pub fn lateral(self) -> LateralActionBuilder<Empty> {
        LateralActionBuilder::new(self.name)
    }

    /// Start building a private action
    /// 
    /// Private actions are entity-specific actions that can include
    /// longitudinal, lateral, visibility, synchronization, and other behaviors.
    /// 
    /// # Returns
    /// PrivateActionBuilder for configuring private actions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new()
    ///     .private()
    ///     .teleport_action()
    ///     .to_position(position);
    /// ```
    pub fn private(self) -> PrivateActionBuilder<Empty> {
        PrivateActionBuilder::new(self.name)
    }

    /// Start building a global action
    /// 
    /// Global actions affect the entire scenario environment,
    /// such as weather changes, traffic signals, and infrastructure actions.
    /// 
    /// # Returns
    /// GlobalActionBuilder for configuring global actions
    /// 
    /// # Example
    /// ```rust
    /// let builder = ActionBuilder::new()
    ///     .global()
    ///     .traffic_signal_action()
    ///     .controller_id("signal_1");
    /// ```
    pub fn global(self) -> GlobalActionBuilder<Empty> {
        GlobalActionBuilder::new(self.name)
    }
}

/// Action wrapper with name and timing attributes
#[derive(Debug, Clone)]
pub struct ActionWrapper {
    /// Action name for identification
    pub name: String,
    
    /// The actual action content
    pub action: Action,
}

impl ActionWrapper {
    /// Create a new action wrapper
    pub fn new(name: String, action: Action) -> Self {
        Self { name, action }
    }
}

/// Trait for building actions with common functionality
pub trait ActionBuilderTrait {
    /// Build the final action
    fn build_action(self) -> BuilderResult<Action>;
    
    /// Get the action name if set
    fn get_name(&self) -> Option<&str>;
}

/// Common action building utilities
pub struct ActionUtils;

impl ActionUtils {
    /// Validate action name
    pub fn validate_name(name: &str) -> BuilderResult<()> {
        if name.is_empty() {
            return Err(BuilderError::validation_error(
                "Action name cannot be empty"
            ));
        }
        
        if name.len() > 255 {
            return Err(BuilderError::validation_error(
                "Action name cannot exceed 255 characters"
            ));
        }
        
        Ok(())
    }
    
    /// Create an OSString value from a string
    pub fn os_string(value: &str) -> OSString {
        Value::literal(value.to_string())
    }
    
    /// Create a double value
    pub fn double(value: f64) -> crate::types::basic::Double {
        Value::literal(value)
    }
    
    /// Create an unsigned int value
    pub fn unsigned_int(value: u32) -> crate::types::basic::UnsignedInt {
        Value::literal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_builder_creation() {
        let builder = ActionBuilder::new();
        assert!(builder.name.is_none());
        assert!(builder.action_data.is_none());
    }

    #[test]
    fn test_action_builder_with_name() {
        let builder = ActionBuilder::new().with_name("test_action");
        assert_eq!(builder.name.as_ref().unwrap(), "test_action");
    }

    #[test]
    fn test_action_utils_validate_name() {
        assert!(ActionUtils::validate_name("valid_name").is_ok());
        assert!(ActionUtils::validate_name("").is_err());
        assert!(ActionUtils::validate_name(&"x".repeat(256)).is_err());
    }

    #[test]
    fn test_action_utils_value_creation() {
        let os_string = ActionUtils::os_string("test");
        assert_eq!(os_string.as_literal().unwrap(), "test");
        
        let double_val = ActionUtils::double(25.5);
        assert_eq!(double_val.as_literal().unwrap(), &25.5);
        
        let uint_val = ActionUtils::unsigned_int(42);
        assert_eq!(uint_val.as_literal().unwrap(), &42);
    }
}
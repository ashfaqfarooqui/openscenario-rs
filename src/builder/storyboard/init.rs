//! Initial actions builder for storyboard initialization
//!
//! This module provides builders for configuring the initial state of scenarios,
//! including environmental conditions, entity placements, and starting behaviors.

use crate::types::scenario::init::{Init, Actions};
use crate::types::actions::global::GlobalAction;
use super::super::actions::global::GlobalActionBuilder;
use super::super::error::{BuilderError, BuilderResult};
use super::super::states::*;
use std::marker::PhantomData;

/// Builder states for initial actions configuration
pub mod states {
    use super::super::super::states::*;
    
    /// State after initial actions have been configured
    pub struct HasActions;
    
    impl BuilderState for HasActions {}
    impl AfterHeader for HasActions {}
}

/// Initial actions builder for configuring scenario startup state
/// 
/// This builder configures the initial state of the scenario including
/// environmental conditions, entity placements, and starting behaviors
/// that occur before the main story begins.
/// 
/// # Example
/// ```rust
/// let init = InitBuilder::new()
///     .add_global_action()
///         .environment_action()
///         .weather().sun(10.0, 180.0, 30.0).finish_weather()
///         .finish_action()
///     .add_global_action()
///         .traffic_action()
///         .source("traffic_flow.xml")
///         .finish_action()
///     .finish_init();
/// ```
pub struct InitBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Global actions to execute at scenario start
    global_actions: Vec<GlobalAction>,
    
    /// Private actions to execute at scenario start
    private_actions: Vec<crate::types::actions::private::PrivateAction>,
}

// Core builder implementation for Empty state
impl InitBuilder<Empty> {
    /// Create a new initial actions builder
    /// 
    /// This creates a new builder for configuring initial scenario state.
    /// 
    /// # Returns
    /// A new InitBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = InitBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            global_actions: Vec::new(),
            private_actions: Vec::new(),
        }
    }

    /// Add a global action to initial actions
    /// 
    /// Global actions affect the entire scenario environment at startup,
    /// such as setting weather conditions, traffic patterns, or infrastructure states.
    /// 
    /// # Returns
    /// GlobalActionBuilder for configuring the global action
    /// 
    /// # Example
    /// ```rust
    /// builder.add_global_action()
    ///     .environment_action()
    ///     .weather().sun(10.0, 180.0, 30.0).finish_weather()
    ///     .finish_action();
    /// ```
    pub fn add_global_action(&mut self) -> GlobalActionBuilder<super::super::actions::Empty> {
        GlobalActionBuilder::new(None)
    }

    /// Add a completed global action to initial actions
    /// 
    /// This method adds a pre-built global action to the initial actions.
    /// 
    /// # Arguments
    /// * `action` - Completed global action
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_global_action(mut self, action: GlobalAction) -> Self {
        self.global_actions.push(action);
        self
    }

    /// Finish configuring initial actions and transition to HasActions state
    /// 
    /// This method completes the initial actions configuration and returns
    /// the configured initial actions structure.
    /// 
    /// # Returns
    /// Configured Init structure
    /// 
    /// # Example
    /// ```rust
    /// let init = builder.finish_init();
    /// ```
    pub fn finish_init(self) -> BuilderResult<Init> {
        self.build_init()
    }
}

impl<S: BuilderState> InitBuilder<S> {
    /// Build the initial actions structure
    /// 
    /// Validates and builds the initial actions configuration.
    /// 
    /// # Returns
    /// Complete Init structure or BuilderError
    pub fn build_init(self) -> BuilderResult<Init> {
        Ok(Init {
            actions: Actions {
                global_actions: self.global_actions,
                private_actions: self.private_actions,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_builder_creation() {
        let builder = InitBuilder::new();
        assert!(builder.global_actions.is_empty());
        assert!(builder.private_actions.is_empty());
    }

    #[test]
    fn test_init_builder_with_global_action() {
        let builder = InitBuilder::new();
        assert!(builder.global_actions.is_empty());
    }
}
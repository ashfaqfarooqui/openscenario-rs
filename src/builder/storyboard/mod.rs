//! Storyboard builder framework for programmatic storyboard construction
//!
//! This module provides type-safe, ergonomic builders for constructing
//! OpenSCENARIO storyboards programmatically. The builder pattern ensures
//! compile-time safety and guides users through the required construction steps.

pub mod init;
pub mod story;
pub mod events;

// Re-export builders for convenience
pub use init::InitBuilder;
pub use story::StoryBuilder;
pub use events::EventBuilder;

use crate::types::scenario::storyboard::{Storyboard, StopTrigger};
use crate::types::scenario::story::{Story};
use super::error::{BuilderError, BuilderResult};
use super::states::*;
use std::marker::PhantomData;

/// Main storyboard builder for constructing OpenSCENARIO storyboards
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO storyboards. The type parameter S tracks the current
/// construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasInit, HasStories, etc.)
/// 
/// # Example
/// ```rust
/// let storyboard = StoryboardBuilder::new()
///     .with_init()
///         .add_global_action()
///             .environment_action()
///             .weather()
///             .sun(10.0, 180.0, 30.0)
///             .finish_weather()
///             .finish_action()
///         .finish_init()
///     .add_story("main_story")
///         .add_act("setup")
///             .add_event("initial_speed")
///                 .when().simulation_time().greater_than(2.0)
///                 .then().entity("ego").accelerate_to(25.0)
///                 .finish_event()
///             .finish_act()
///         .finish_story()
///     .build()?;
/// ```
pub struct StoryboardBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Initial actions and settings
    init: Option<init::InitBuilder<init::HasActions>>,
    
    /// Stories that define scenario behavior
    stories: Vec<Story>,
    
    /// Optional stop trigger for ending scenario
    stop_trigger: Option<StopTrigger>,
}

// Core builder implementation for Empty state
impl StoryboardBuilder<Empty> {
    /// Create a new storyboard builder
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// initial actions as the first required element.
    /// 
    /// # Returns
    /// A new StoryboardBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = StoryboardBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            init: None,
            stories: Vec::new(),
            stop_trigger: None,
        }
    }

    /// Start configuring initial actions and settings
    /// 
    /// Initial actions define the starting state of the scenario,
    /// including environmental conditions, entity placements, and
    /// initial behaviors that occur before the main story begins.
    /// 
    /// # Returns
    /// InitBuilder for configuring initial actions
    /// 
    /// # Example
    /// ```rust
    /// let builder = StoryboardBuilder::new()
    ///     .with_init()
    ///         .add_global_action()
    ///             .environment_action()
    ///             .weather().sun(10.0, 180.0, 30.0).finish_weather()
    ///             .finish_action()
    ///         .finish_init();
    /// ```
    pub fn with_init(&mut self) -> InitBuilder<init::Empty> {
        InitBuilder::new()
    }
}

// Methods available after initial actions are configured
impl StoryboardBuilder<HasHeader> {
    /// Add a story to the storyboard
    /// 
    /// Stories define the main behavioral sequences of the scenario,
    /// organized into acts and events that trigger based on conditions.
    /// 
    /// # Arguments
    /// * `name` - Story name for identification
    /// 
    /// # Returns
    /// StoryBuilder for configuring the story
    /// 
    /// # Example
    /// ```rust
    /// builder.add_story("main_story")
    ///     .add_act("acceleration")
    ///         .add_event("speed_up")
    ///             .when().simulation_time().greater_than(5.0)
    ///             .then().accelerate_to(30.0)
    ///             .finish_event()
    ///         .finish_act()
    ///     .finish_story();
    /// ```
    pub fn add_story(&mut self, name: &str) -> StoryBuilder<story::Empty> {
        StoryBuilder::new(name)
    }

    /// Set a stop trigger for the scenario
    /// 
    /// The stop trigger defines when the scenario should end,
    /// based on specific conditions being met.
    /// 
    /// # Arguments
    /// * `trigger` - Stop trigger configuration
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.with_stop_trigger()
    ///     .when().simulation_time().greater_than(300.0)  // 5 minutes
    ///     .finish_trigger();
    /// ```
    pub fn with_stop_trigger(&mut self, trigger: StopTrigger) -> &mut Self {
        self.stop_trigger = Some(trigger);
        self
    }
}

impl<S: BuilderState> StoryboardBuilder<S> {
    /// Build the final storyboard
    /// 
    /// Validates the constructed storyboard and builds the final
    /// OpenSCENARIO storyboard structure.
    /// 
    /// # Returns
    /// Complete OpenSCENARIO Storyboard or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<Storyboard> {
        // Validate required elements
        let init = self.init
            .ok_or_else(|| BuilderError::missing_field(
                "init", 
                "Call .with_init() first"
            ))?
            .build_init()?;

        // Validate that storyboard has at least one story
        if self.stories.is_empty() {
            return Err(BuilderError::validation_error(
                "Storyboard must contain at least one story. Add stories using .add_story()"
            ));
        }

        Ok(Storyboard {
            init,
            stories: self.stories,
            stop_trigger: self.stop_trigger,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storyboard_builder_creation() {
        let builder = StoryboardBuilder::new();
        assert!(builder.init.is_none());
        assert!(builder.stories.is_empty());
        assert!(builder.stop_trigger.is_none());
    }

    #[test]
    fn test_storyboard_builder_validation() {
        // Test empty storyboard validation
        let result = StoryboardBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("init"));
    }
}
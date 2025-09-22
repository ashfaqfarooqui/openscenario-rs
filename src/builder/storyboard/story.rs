//! Story builder for scenario behavioral sequences
//!
//! This module provides builders for creating stories, acts, and events
//! that define the main behavioral sequences of scenarios.

use crate::types::scenario::story::{Story, Act, ManeuverGroup, Maneuver};
use crate::types::scenario::triggers::Trigger;
use crate::types::basic::{OSString, Value};
use super::super::conditions::ConditionBuilder;
use super::super::actions::ActionBuilder;
use super::super::error::{BuilderError, BuilderResult};
use super::super::states::*;
use std::marker::PhantomData;

/// Builder states for story configuration
pub mod states {
    use super::super::super::states::*;
    
    /// State after story parameters have been configured
    pub struct HasParameters;
    
    /// State after acts have been added
    pub struct HasActs;
    
    impl BuilderState for HasParameters {}
    impl AfterHeader for HasParameters {}
    impl BuilderState for HasActs {}
    impl AfterHeader for HasActs {}
}

/// Story builder for creating behavioral sequences
/// 
/// This builder creates stories that define the main behavioral sequences
/// of scenarios, organized into acts and events that trigger based on conditions.
/// 
/// # Example
/// ```rust
/// let story = StoryBuilder::new("main_story")
///     .add_act("setup")
///         .add_event("initial_speed")
///             .when().simulation_time().greater_than(2.0)
///             .then().entity("ego").accelerate_to(25.0)
///             .finish_event()
///         .finish_act()
///     .add_act("merge_phase")
///         .add_event("lane_change")
///             .when().entity("ego").speed().greater_than(20.0)
///             .then().entity("ego").change_lane_to("highway", 1)
///             .finish_event()
///         .finish_act()
///     .finish_story();
/// ```
pub struct StoryBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Story name for identification
    name: String,
    
    /// Story owner (optional)
    owner: Option<String>,
    
    /// Current acts being built
    acts: Vec<Act>,
    
    /// Current act being built
    current_act: Option<ActBuilder<act::BuildingAct>>,
}

/// Act builder for creating story acts
pub struct ActBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Act name for identification
    name: String,
    
    /// Trigger that starts the act
    start_trigger: Option<Trigger>,
    
    /// Trigger that stops the act (optional)
    stop_trigger: Option<Trigger>,
    
    /// Maneuver groups in this act
    maneuver_groups: Vec<ManeuverGroup>,
}

/// Event builder for creating act events
pub struct EventBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Event name for identification
    name: String,
    
    /// Priority of the event
    priority: crate::types::enums::Priority,
    
    /// Trigger that starts the event
    trigger: Option<Trigger>,
    
    /// Actions to execute when event triggers
    actions: Vec<crate::types::actions::Action>,
}

// Core builder implementation for Empty state
impl StoryBuilder<Empty> {
    /// Create a new story builder
    /// 
    /// This creates a new builder for creating a story with the given name.
    /// 
    /// # Arguments
    /// * `name` - Story name for identification
    /// 
    /// # Returns
    /// A new StoryBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = StoryBuilder::new("main_story");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            _state: PhantomData,
            name: name.to_string(),
            owner: None,
            acts: Vec::new(),
            current_act: None,
        }
    }

    /// Set the story owner
    /// 
    /// # Arguments
    /// * `owner` - Story owner identifier
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_owner(mut self, owner: &str) -> Self {
        self.owner = Some(owner.to_string());
        self
    }

    /// Add an act to the story
    /// 
    /// Acts are sequential behavioral segments within a story,
    /// each with start/stop triggers and maneuver groups.
    /// 
    /// # Arguments
    /// * `name` - Act name for identification
    /// 
    /// # Returns
    /// ActBuilder for configuring the act
    /// 
    /// # Example
    /// ```rust
    /// builder.add_act("acceleration_phase")
    ///     .add_event("speed_up")
    ///         .when().simulation_time().greater_than(5.0)
    ///         .then().accelerate_to(30.0)
    ///         .finish_event()
    ///     .finish_act();
    /// ```
    pub fn add_act(&mut self, name: &str) -> ActBuilder<act::Empty> {
        ActBuilder::new(name)
    }
}

impl StoryBuilder<HasActs> {
    /// Finish configuring the story and transition to complete state
    /// 
    /// This method completes the story configuration and returns
    /// the configured story structure.
    /// 
    /// # Returns
    /// Configured Story structure
    /// 
    /// # Example
    /// ```rust
    /// let story = builder.finish_story();
    /// ```
    pub fn finish_story(self) -> BuilderResult<Story> {
        self.build_story()
    }
}

impl<S: BuilderState> StoryBuilder<S> {
    /// Build the story structure
    /// 
    /// Validates and builds the story configuration.
    /// 
    /// # Returns
    /// Complete Story structure or BuilderError
    pub fn build_story(self) -> BuilderResult<Story> {
        // Validate that story has at least one act
        if self.acts.is_empty() && self.current_act.is_none() {
            return Err(BuilderError::validation_error(
                "Story must contain at least one act. Add acts using .add_act()"
            ));
        }

        let mut acts = self.acts;
        if let Some(act) = self.current_act {
            acts.push(act.build_act()?);
        }

        Ok(Story {
            name: Value::literal(self.name),
            owner: self.owner.map(|o| Value::literal(o)),
            acts,
        })
    }
}

/// Act builder module
pub mod act {
    use super::*;
    
    /// State for building an act
    pub struct BuildingAct;
    
    impl super::super::super::states::BuilderState for BuildingAct {}
    impl super::super::super::states::AfterHeader for BuildingAct {}
    
    impl ActBuilder<Empty> {
        /// Create a new act builder
        /// 
        /// This creates a new builder for creating an act with the given name.
        /// 
        /// # Arguments
        /// * `name` - Act name for identification
        /// 
        /// # Returns
        /// A new ActBuilder in Empty state
        pub fn new(name: &str) -> Self {
            Self {
                _state: PhantomData,
                name: name.to_string(),
                start_trigger: None,
                stop_trigger: None,
                maneuver_groups: Vec::new(),
            }
        }

        /// Set the act start trigger
        /// 
        /// # Arguments
        /// * `trigger` - Trigger that starts the act
        /// 
        /// # Returns
        /// Self for method chaining
        pub fn with_start_trigger(mut self, trigger: Trigger) -> Self {
            self.start_trigger = Some(trigger);
            self
        }

        /// Set the act stop trigger
        /// 
        /// # Arguments
        /// * `trigger` - Trigger that stops the act
        /// 
        /// # Returns
        /// Self for method chaining
        pub fn with_stop_trigger(mut self, trigger: Trigger) -> Self {
            self.stop_trigger = Some(trigger);
            self
        }

        /// Add an event to the act
        /// 
        /// Events are atomic behavioral units within acts that trigger
        /// based on specific conditions and execute defined actions.
        /// 
        /// # Arguments
        /// * `name` - Event name for identification
        /// 
        /// # Returns
        /// EventBuilder for configuring the event
        pub fn add_event(&mut self, name: &str) -> EventBuilder<event::Empty> {
            EventBuilder::new(name)
        }
    }

    impl ActBuilder<BuildingAct> {
        /// Finish configuring the act and return to story builder
        /// 
        /// # Returns
        /// Parent StoryBuilder
        pub fn finish_act(self) -> BuilderResult<Act> {
            self.build_act()
        }
    }

    impl<S: BuilderState> ActBuilder<S> {
        /// Build the act structure
        /// 
        /// Validates and builds the act configuration.
        /// 
        /// # Returns
        /// Complete Act structure or BuilderError
        pub fn build_act(self) -> BuilderResult<Act> {
            // Validate required elements
            let start_trigger = self.start_trigger
                .ok_or_else(|| BuilderError::missing_field(
                    "start_trigger", 
                    "Set start trigger using .with_start_trigger() or add events"
                ))?;

            Ok(Act {
                name: Value::literal(self.name),
                start_trigger,
                stop_trigger: self.stop_trigger,
                maneuver_groups: self.maneuver_groups,
            })
        }
    }
}

/// Event builder module
pub mod event {
    use super::*;
    
    /// State for empty event builder
    pub struct Empty;
    
    /// State for building event conditions
    pub struct BuildingConditions;
    
    /// State for building event actions
    pub struct BuildingActions;
    
    impl super::super::super::states::BuilderState for Empty {}
    impl super::super::super::states::BuilderState for BuildingConditions {}
    impl super::super::super::states::BuilderState for BuildingActions {}
    
    impl EventBuilder<Empty> {
        /// Create a new event builder
        /// 
        /// This creates a new builder for creating an event with the given name.
        /// 
        /// # Arguments
        /// * `name` - Event name for identification
        /// 
        /// # Returns
        /// A new EventBuilder in Empty state
        pub fn new(name: &str) -> Self {
            Self {
                _state: PhantomData,
                name: name.to_string(),
                priority: crate::types::enums::Priority::Parallel,
                trigger: None,
                actions: Vec::new(),
            }
        }

        /// Set the event priority
        /// 
        /// # Arguments
        /// * `priority` - Event priority (Parallel, Overwrite, Skip)
        /// 
        /// # Returns
        /// Self for method chaining
        pub fn with_priority(mut self, priority: crate::types::enums::Priority) -> Self {
            self.priority = priority;
            self
        }

        /// Start defining the event trigger condition
        /// 
        /// # Returns
        /// ConditionBuilder for configuring the trigger condition
        pub fn when(&mut self) -> ConditionBuilder<super::super::super::conditions::Empty> {
            ConditionBuilder::new()
        }

        /// Start defining the event actions
        /// 
        /// # Returns
        /// ActionBuilder for configuring the event actions
        pub fn then(&mut self) -> ActionBuilder<super::super::super::actions::Empty> {
            ActionBuilder::new()
        }
    }

    impl EventBuilder<Empty> {
        /// Finish configuring the event and return to act builder
        /// 
        /// # Returns
        /// Configured event
        pub fn finish_event(self) -> BuilderResult<crate::types::scenario::story::Event> {
            self.build_event()
        }
    }

    impl<S: BuilderState> EventBuilder<S> {
        /// Build the event structure
        /// 
        /// Validates and builds the event configuration.
        /// 
        /// # Returns
        /// Complete Event structure or BuilderError
        pub fn build_event(self) -> BuilderResult<crate::types::scenario::story::Event> {
            // Validate required elements
            let trigger = self.trigger
                .ok_or_else(|| BuilderError::missing_field(
                    "trigger", 
                    "Set trigger using .when() or provide trigger directly"
                ))?;

            Ok(crate::types::scenario::story::Event {
                name: Value::literal(self.name),
                priority: self.priority,
                trigger,
                actions: self.actions,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_builder_creation() {
        let builder = StoryBuilder::new("test_story");
        assert_eq!(builder.name, "test_story");
        assert!(builder.owner.is_none());
        assert!(builder.acts.is_empty());
    }

    #[test]
    fn test_story_builder_with_owner() {
        let builder = StoryBuilder::new("test_story").with_owner("test_owner");
        assert_eq!(builder.owner.as_ref().unwrap(), "test_owner");
    }

    #[test]
    fn test_act_builder_creation() {
        let builder = ActBuilder::new("test_act");
        assert_eq!(builder.name, "test_act");
        assert!(builder.start_trigger.is_none());
        assert!(builder.stop_trigger.is_none());
        assert!(builder.maneuver_groups.is_empty());
    }

    #[test]
    fn test_event_builder_creation() {
        let builder = EventBuilder::new("test_event");
        assert_eq!(builder.name, "test_event");
        assert_eq!(builder.priority, crate::types::enums::Priority::Parallel);
        assert!(builder.trigger.is_none());
        assert!(builder.actions.is_empty());
    }
}
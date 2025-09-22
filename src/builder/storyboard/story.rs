//! Story builder for programmatic story construction
//!
//! This module provides fluent APIs for creating stories with acts and parameters
//! with comprehensive validation and type safety.

use crate::types::scenario::story::{ScenarioStory, Act};
use crate::types::basic::{OSString, ParameterDeclarations};
use crate::builder::{BuilderError, BuilderResult};
use super::{StoryboardBuilder, ActBuilder};

/// Builder for creating stories within storyboards
pub struct StoryBuilder {
    parent: StoryboardBuilder,
    name: String,
    parameter_declarations: Option<ParameterDeclarations>,
    acts: Vec<Act>,
}

impl StoryBuilder {
    /// Create a new story builder
    pub(super) fn new(parent: StoryboardBuilder, name: String) -> Self {
        Self {
            parent,
            name,
            parameter_declarations: None,
            acts: Vec::new(),
        }
    }

    /// Set parameter declarations for this story
    pub fn with_parameters(mut self, parameters: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(parameters);
        self
    }

    /// Add an act to this story
    pub fn add_act(mut self, name: impl Into<String>) -> ActBuilder {
        ActBuilder::new(self, name.into())
    }

    /// Add a pre-built act
    pub fn with_act(mut self, act: Act) -> Self {
        self.acts.push(act);
        self
    }

    /// Finish building the story and return to storyboard builder
    pub fn finish_story(self) -> BuilderResult<StoryboardBuilder> {
        self.validate()?;

        let story = ScenarioStory {
            name: OSString::literal(self.name),
            parameter_declarations: self.parameter_declarations,
            acts: self.acts,
        };

        Ok(self.parent.add_completed_story(story))
    }

    /// Validate the story configuration
    pub fn validate(&self) -> BuilderResult<()> {
        if self.acts.is_empty() {
            return Err(BuilderError::validation_error(
                &format!("Story '{}' has no acts", self.name),
                "Add at least one act using add_act()"
            ));
        }

        // Validate act names are unique
        let mut act_names = std::collections::HashSet::new();
        for act in &self.acts {
            let act_name = act.name.value();
            if !act_names.insert(act_name.clone()) {
                return Err(BuilderError::validation_error(
                    &format!("Duplicate act name '{}' in story '{}'", act_name, self.name),
                    "Ensure all act names within a story are unique"
                ));
            }
        }

        Ok(())
    }

    /// Internal method to add a completed act
    pub(super) fn add_completed_act(mut self, act: Act) -> Self {
        self.acts.push(act);
        self
    }
}
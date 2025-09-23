//! Entity builders for programmatic entity construction
//!
//! This module provides fluent APIs for creating vehicle and pedestrian entities in OpenSCENARIO
//! scenarios with comprehensive validation and type safety.
//!
//! Note: MiscObject entities are not yet supported as the underlying type system
//! implementation is not complete.

pub mod vehicle;
pub mod pedestrian;
// pub mod misc_object;  // Removed - MiscObject not implemented in type system yet

pub use vehicle::VehicleBuilder;
pub use pedestrian::PedestrianBuilder;
// pub use misc_object::MiscObjectBuilder;  // Removed - MiscObject not implemented in type system yet

use crate::types::entities::{ScenarioObject, Entities};
use crate::builder::{BuilderError, BuilderResult};

/// Trait for entity builders that can be finished and added to a scenario
pub trait EntityBuilder {
    /// The type of entity this builder creates
    type Entity;
    
    /// Finish building the entity and return it
    fn finish(self) -> BuilderResult<Self::Entity>;
    
    /// Get the name of the entity being built
    fn get_name(&self) -> &str;
}

/// Container for building entities within a scenario
pub struct EntitiesBuilder {
    entities: Entities,
}

impl EntitiesBuilder {
    /// Create a new entities builder
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
        }
    }
    
    /// Add a scenario object to the entities
    pub fn add_object(&mut self, object: ScenarioObject) -> BuilderResult<()> {
        // Validate that the object name is unique
        if let Some(name) = object.get_name() {
            if self.entities.find_object(name).is_some() {
                return Err(BuilderError::validation_error(
                    &format!("Entity '{}' already exists", name),
                    "Use a unique name for each entity"
                ));
            }
        }
        
        self.entities.add_object(object);
        Ok(())
    }
    
    /// Get the built entities
    pub fn build(self) -> Entities {
        self.entities
    }
    
    /// Get the current number of entities
    pub fn entity_count(&self) -> usize {
        self.entities.scenario_objects.len()
    }
}

impl Default for EntitiesBuilder {
    fn default() -> Self {
        Self::new()
    }
}
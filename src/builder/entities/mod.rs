//! Entity builders for programmatic entity construction
//!
//! This module provides type-safe builders for constructing OpenSCENARIO entities
//! including vehicles, pedestrians, and miscellaneous objects. The builders ensure
//! proper configuration and validation before adding entities to scenarios.

use crate::types::{
    basic::Value,
    entities::{ScenarioObject, Entities},
    controllers::ObjectController,
};
use super::{
    error::{BuilderError, BuilderResult},
    registry::EntityRegistry,
    states::*,
};
use std::marker::PhantomData;

pub mod vehicle;
pub mod pedestrian;
pub mod misc_object;

// Re-export builders
pub use vehicle::VehicleBuilder;
pub use pedestrian::PedestrianBuilder;
pub use misc_object::MiscObjectBuilder;

/// Entity builder state for tracking entity construction progress
pub trait EntityBuilderState {}

/// Initial state for entity builders
pub struct EntityEmpty;

/// State after entity type is selected
pub struct EntityTypeSelected;

/// State after entity properties are configured
pub struct EntityConfigured;

/// State after entity positioning is set
pub struct EntityPositioned;

/// State when entity is ready to be added to scenario
pub struct EntityReady;

impl EntityBuilderState for EntityEmpty {}
impl EntityBuilderState for EntityTypeSelected {}
impl EntityBuilderState for EntityConfigured {}
impl EntityBuilderState for EntityPositioned {}
impl EntityBuilderState for EntityReady {}

/// Container for managing entity construction within a scenario builder
/// 
/// This builder manages the collection of entities and provides methods for
/// adding different types of entities with proper validation and positioning.
pub struct EntitiesBuilder<S: BuilderState> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// The entities container being built
    entities: Entities,
    
    /// Registry for tracking and validating entity references
    entity_registry: EntityRegistry,
}

impl<S: BuilderState> EntitiesBuilder<S> {
    /// Create a new entities builder
    pub fn new(entity_registry: EntityRegistry) -> Self {
        Self {
            _state: PhantomData,
            entities: Entities::new(),
            entity_registry,
        }
    }

    /// Start building a vehicle entity
    /// 
    /// This method creates a new vehicle builder that guides through the
    /// vehicle configuration process with compile-time safety.
    /// 
    /// # Arguments
    /// * `name` - Unique name for the vehicle entity
    /// 
    /// # Returns
    /// VehicleBuilder for configuring the vehicle
    /// 
    /// # Example
    /// ```rust
    /// entities_builder.add_vehicle("ego")
    ///     .car()
    ///     .with_model("sedan")
    ///     .at_position().lane("1", 1, 100.0)
    ///     .finish_vehicle()
    /// ```
    pub fn add_vehicle(self, name: &str) -> VehicleBuilder<S> {
        VehicleBuilder::new(name.to_string(), self.entities, self.entity_registry)
    }

    /// Start building a pedestrian entity
    /// 
    /// # Arguments
    /// * `name` - Unique name for the pedestrian entity
    /// 
    /// # Returns
    /// PedestrianBuilder for configuring the pedestrian
    pub fn add_pedestrian(self, name: &str) -> PedestrianBuilder<S> {
        PedestrianBuilder::new(name.to_string(), self.entities, self.entity_registry)
    }

    /// Start building a miscellaneous object entity
    /// 
    /// # Arguments
    /// * `name` - Unique name for the misc object entity
    /// 
    /// # Returns
    /// MiscObjectBuilder for configuring the object
    pub fn add_misc_object(self, name: &str) -> MiscObjectBuilder<S> {
        MiscObjectBuilder::new(name.to_string(), self.entities, self.entity_registry)
    }

    /// Add a pre-constructed scenario object
    /// 
    /// This method allows adding entities that have been constructed outside
    /// the builder pattern, useful for integration with existing code.
    /// 
    /// # Arguments
    /// * `object` - Pre-constructed ScenarioObject
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn add_scenario_object(mut self, object: ScenarioObject) -> BuilderResult<Self> {
        // Validate entity name uniqueness
        if let Some(name) = object.get_name() {
            if self.entity_registry.has_entity(name) {
                return Err(BuilderError::duplicate_entity(name));
            }
            
            // Register the entity
            self.entity_registry.add_entity(name.to_string(), object.clone())?;
        }

        self.entities.add_object(object);
        Ok(self)
    }

    /// Get the current entities container
    /// 
    /// This method provides access to the entities being built, useful for
    /// inspection or integration with other systems.
    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    /// Get a mutable reference to the entity registry
    /// 
    /// This allows advanced users to directly manipulate the entity registry
    /// for complex scenarios or integration needs.
    pub fn get_entity_registry_mut(&mut self) -> &mut EntityRegistry {
        &mut self.entity_registry
    }

    /// Finish building entities and return the completed container
    /// 
    /// This method completes the entity building process and returns the
    /// final Entities container along with the updated registry.
    /// 
    /// # Returns
    /// Tuple of (Entities, EntityRegistry) for use in scenario building
    pub fn finish_entities(self) -> (Entities, EntityRegistry) {
        (self.entities, self.entity_registry)
    }
}

/// Trait for entity builders that can be finalized
/// 
/// This trait provides a common interface for all entity builders to
/// complete their construction and add the entity to the scenario.
pub trait FinalizableEntity<S: BuilderState> {
    /// Complete entity construction and add to scenario
    /// 
    /// This method validates the entity configuration and adds it to the
    /// entities container, returning an updated EntitiesBuilder.
    /// 
    /// # Returns
    /// EntitiesBuilder with the new entity added
    fn finalize_entity(self) -> BuilderResult<EntitiesBuilder<S>>;
}

/// Common entity builder functionality
/// 
/// This trait provides shared functionality across all entity builders
/// for consistent behavior and validation.
pub trait EntityBuilder<S: BuilderState> {
    /// Get the entity name
    fn get_name(&self) -> &str;
    
    /// Validate entity configuration
    /// 
    /// This method checks that the entity has been properly configured
    /// and is ready to be added to the scenario.
    fn validate(&self) -> BuilderResult<()>;
    
    /// Set object controller for the entity
    /// 
    /// Controllers define how entities behave during simulation.
    /// 
    /// # Arguments
    /// * `controller` - ObjectController configuration
    /// 
    /// # Returns
    /// Self for method chaining
    fn with_controller(self, controller: ObjectController) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::registry::EntityRegistry;

    #[test]
    fn test_entities_builder_creation() {
        let registry = EntityRegistry::new();
        let builder: EntitiesBuilder<HasRoadNetwork> = EntitiesBuilder::new(registry);
        
        assert_eq!(builder.get_entities().scenario_objects.len(), 0);
    }

    #[test]
    fn test_entities_builder_finish() {
        let registry = EntityRegistry::new();
        let builder: EntitiesBuilder<HasRoadNetwork> = EntitiesBuilder::new(registry);
        
        let (entities, _registry) = builder.finish_entities();
        assert_eq!(entities.scenario_objects.len(), 0);
    }
}
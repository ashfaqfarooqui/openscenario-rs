//! Pedestrian entity builder for programmatic pedestrian construction

use crate::types::{
    basic::{Value, Double},
    entities::{
        pedestrian::{Pedestrian, PedestrianCategory},
        ScenarioObject, Entities,
    },
    controllers::ObjectController,
    positions::Position,
    geometry::{BoundingBox, Center, Dimensions},
    enums::PedestrianCategory as PedestrianCategoryEnum,
};
use super::{
    EntitiesBuilder, EntityBuilder, FinalizableEntity,
    EntityBuilderState, EntityEmpty, EntityTypeSelected, EntityConfigured, EntityPositioned,
};
use crate::builder::{
    error::{BuilderError, BuilderResult},
    registry::EntityRegistry,
    states::BuilderState,
    positions::PositionBuilder,
};
use std::marker::PhantomData;

/// Pedestrian builder for type-safe pedestrian construction
/// 
/// This builder guides users through the pedestrian configuration process,
/// ensuring all required properties are set before the pedestrian can be
/// added to a scenario.
pub struct PedestrianBuilder<S: BuilderState, E: EntityBuilderState = EntityEmpty> {
    /// Type state phantom data for scenario builder
    _scenario_state: PhantomData<S>,
    /// Type state phantom data for entity builder
    _entity_state: PhantomData<E>,
    
    /// Pedestrian name for scenario reference
    name: String,
    
    /// Pedestrian being constructed
    pedestrian: Pedestrian,
    
    /// Position for the pedestrian (optional until positioned)
    position: Option<Position>,
    
    /// Object controller configuration
    controller: Option<ObjectController>,
    
    /// Parent entities container
    entities: Entities,
    
    /// Entity registry for validation
    entity_registry: EntityRegistry,
}

impl<S: BuilderState> PedestrianBuilder<S, EntityEmpty> {
    /// Create a new pedestrian builder
    pub(super) fn new(name: String, entities: Entities, entity_registry: EntityRegistry) -> Self {
        Self {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name,
            pedestrian: Pedestrian::default(),
            position: None,
            controller: None,
            entities,
            entity_registry,
        }
    }

    /// Configure as a standard pedestrian
    /// 
    /// Sets the pedestrian category to pedestrian and provides reasonable defaults.
    /// 
    /// # Returns
    /// PedestrianBuilder in EntityTypeSelected state
    pub fn pedestrian(mut self) -> PedestrianBuilder<S, EntityTypeSelected> {
        self.pedestrian.pedestrian_category = PedestrianCategory::Pedestrian;
        
        // Set reasonable pedestrian defaults
        self.pedestrian.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(0.9),
            },
            dimensions: Dimensions {
                width: Value::literal(0.6),
                length: Value::literal(0.6),
                height: Value::literal(1.8),
            },
        };

        PedestrianBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            pedestrian: self.pedestrian,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as an animal
    /// 
    /// Sets the pedestrian category to animal and provides animal-specific defaults.
    /// 
    /// # Returns
    /// PedestrianBuilder in EntityTypeSelected state
    pub fn animal(mut self) -> PedestrianBuilder<S, EntityTypeSelected> {
        self.pedestrian.pedestrian_category = PedestrianCategory::Animal;
        
        // Set reasonable animal defaults (medium-sized animal like a dog)
        self.pedestrian.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(0.3),
            },
            dimensions: Dimensions {
                width: Value::literal(0.4),
                length: Value::literal(0.8),
                height: Value::literal(0.6),
            },
        };

        PedestrianBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            pedestrian: self.pedestrian,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a wheelchair user
    /// 
    /// Sets the pedestrian category to wheelchair and provides wheelchair-specific defaults.
    /// 
    /// # Returns
    /// PedestrianBuilder in EntityTypeSelected state
    pub fn wheelchair(mut self) -> PedestrianBuilder<S, EntityTypeSelected> {
        self.pedestrian.pedestrian_category = PedestrianCategory::Wheelchair;
        
        // Set reasonable wheelchair defaults
        self.pedestrian.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(0.7),
            },
            dimensions: Dimensions {
                width: Value::literal(0.7),
                length: Value::literal(1.2),
                height: Value::literal(1.4),
            },
        };

        PedestrianBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            pedestrian: self.pedestrian,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState> PedestrianBuilder<S, EntityTypeSelected> {
    /// Set pedestrian model name
    /// 
    /// # Arguments
    /// * `model` - Model name or identifier
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_model(mut self, model: &str) -> Self {
        self.pedestrian.name = Value::literal(model.to_string());
        self
    }

    /// Set pedestrian dimensions
    /// 
    /// # Arguments
    /// * `length` - Pedestrian length in meters
    /// * `width` - Pedestrian width in meters  
    /// * `height` - Pedestrian height in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        self.pedestrian.bounding_box.dimensions = Dimensions {
            length: Value::literal(length),
            width: Value::literal(width),
            height: Value::literal(height),
        };
        self
    }

    /// Set pedestrian mass
    /// 
    /// # Arguments
    /// * `mass` - Pedestrian mass in kg
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_mass(mut self, mass: f64) -> Self {
        // Note: Pedestrian type doesn't have a direct mass field
        // This would need to be added as a property or handled differently
        // For now, we'll skip this implementation
        self
    }

    /// Transition to configured state
    /// 
    /// This method indicates that the basic pedestrian configuration is complete
    /// and the pedestrian is ready for positioning.
    /// 
    /// # Returns
    /// PedestrianBuilder in EntityConfigured state
    pub fn configured(self) -> PedestrianBuilder<S, EntityConfigured> {
        PedestrianBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            pedestrian: self.pedestrian,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState, E: EntityBuilderState> PedestrianBuilder<S, E> 
where 
    E: EntityBuilderState,
{
    /// Start positioning the pedestrian
    /// 
    /// This method creates a position builder for setting the pedestrian's
    /// initial position in the scenario.
    /// 
    /// # Returns
    /// PositionBuilder for configuring pedestrian position
    pub fn at_position(self) -> PositionBuilder<S, Self> {
        PositionBuilder::new(self)
    }

    /// Set pedestrian position directly
    /// 
    /// This method allows setting a pre-constructed position for the pedestrian.
    /// 
    /// # Arguments
    /// * `position` - Position configuration
    /// 
    /// # Returns
    /// PedestrianBuilder in EntityPositioned state
    pub fn with_position(mut self, position: Position) -> PedestrianBuilder<S, EntityPositioned> {
        self.position = Some(position);
        
        PedestrianBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            pedestrian: self.pedestrian,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Set initial speed for the pedestrian
    /// 
    /// # Arguments
    /// * `speed` - Initial speed in m/s
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_initial_speed(self, _speed: f64) -> Self {
        // TODO: Implement initial speed setting in init actions
        // This will be implemented when we add the storyboard builders
        self
    }
}

impl<S: BuilderState> PedestrianBuilder<S, EntityPositioned> {
    /// Complete pedestrian construction and add to scenario
    /// 
    /// This method validates the pedestrian configuration and adds it to the
    /// entities container.
    /// 
    /// # Returns
    /// EntitiesBuilder with the new pedestrian added
    pub fn finish_pedestrian(mut self) -> BuilderResult<EntitiesBuilder<S>> {
        // Validate pedestrian configuration
        self.validate()?;

        // Create scenario object
        let mut scenario_object = ScenarioObject::new_pedestrian(self.name.clone(), self.pedestrian);
        
        // Set controller if provided
        if let Some(controller) = self.controller {
            scenario_object.object_controller = Some(controller);
        }

        // Register entity
        self.entity_registry.add_entity(self.name.clone(), scenario_object.clone())?;

        // Add to entities
        self.entities.add_object(scenario_object);

        Ok(EntitiesBuilder::new(self.entity_registry))
    }
}

impl<S: BuilderState, E: EntityBuilderState> EntityBuilder<S> for PedestrianBuilder<S, E> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self) -> BuilderResult<()> {
        // Check that name is not empty
        if self.name.is_empty() {
            return Err(BuilderError::validation_error("Pedestrian name cannot be empty"));
        }

        // Check that position is set for positioned pedestrians
        if std::any::type_name::<E>().contains("EntityPositioned") && self.position.is_none() {
            return Err(BuilderError::validation_error("Pedestrian position must be set"));
        }

        // Validate pedestrian properties
        let bbox = &self.pedestrian.bounding_box;
        
        // Check dimensions are positive
        if let (Some(length), Some(width), Some(height)) = (
            bbox.dimensions.length.as_literal(),
            bbox.dimensions.width.as_literal(),
            bbox.dimensions.height.as_literal(),
        ) {
            if *length <= 0.0 || *width <= 0.0 || *height <= 0.0 {
                return Err(BuilderError::validation_error("Pedestrian dimensions must be positive"));
            }
        }

        Ok(())
    }

    fn with_controller(mut self, controller: ObjectController) -> Self {
        self.controller = Some(controller);
        self
    }
}

impl<S: BuilderState> FinalizableEntity<S> for PedestrianBuilder<S, EntityPositioned> {
    fn finalize_entity(self) -> BuilderResult<EntitiesBuilder<S>> {
        self.finish_pedestrian()
    }
}

/// Trait for position builder integration
/// 
/// This trait allows the position builder to work with pedestrian builders
/// by providing a way to set the position and transition states.
impl<S: BuilderState, E: EntityBuilderState> super::vehicle::PositionReceiver<S> for PedestrianBuilder<S, E> {
    fn set_position(self, position: Position) -> PedestrianBuilder<S, EntityPositioned> {
        self.with_position(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::registry::EntityRegistry;
    use crate::builder::states::HasRoadNetwork;

    #[test]
    fn test_pedestrian_builder_pedestrian() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = PedestrianBuilder::<HasRoadNetwork>::new("test_ped".to_string(), entities, registry)
            .pedestrian()
            .with_model("adult_male")
            .with_dimensions(0.6, 0.6, 1.8);
            
        assert_eq!(builder.get_name(), "test_ped");
        assert_eq!(builder.pedestrian.pedestrian_category, PedestrianCategory::Pedestrian);
    }

    #[test]
    fn test_pedestrian_builder_animal() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = PedestrianBuilder::<HasRoadNetwork>::new("test_dog".to_string(), entities, registry)
            .animal()
            .with_mass(25.0);
            
        assert_eq!(builder.pedestrian.pedestrian_category, PedestrianCategory::Animal);
    }

    #[test]
    fn test_pedestrian_builder_validation() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = PedestrianBuilder::<HasRoadNetwork>::new("".to_string(), entities, registry);
        
        let result = builder.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name cannot be empty"));
    }

    #[test]
    fn test_pedestrian_builder_wheelchair() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = PedestrianBuilder::<HasRoadNetwork>::new("wheelchair_user".to_string(), entities, registry)
            .wheelchair()
            .with_mass(100.0);
            
        assert_eq!(builder.pedestrian.pedestrian_category, PedestrianCategory::Wheelchair);
        
        let dims = &builder.pedestrian.bounding_box.dimensions;
        assert_eq!(dims.width.as_literal(), Some(&0.7));
        assert_eq!(dims.length.as_literal(), Some(&1.2));
        assert_eq!(dims.height.as_literal(), Some(&1.4));
    }
}
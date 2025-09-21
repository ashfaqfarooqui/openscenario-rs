//! Miscellaneous object entity builder for programmatic misc object construction

use crate::types::{
    basic::{Value, Double},
    entities::{
        ScenarioObject, Entities,
    },
    controllers::ObjectController,
    positions::Position,
    geometry::{BoundingBox, Center, Dimensions},
};

// Placeholder enum for misc object categories until the actual type is implemented
#[derive(Debug, Clone, PartialEq)]
pub enum MiscObjectCategory {
    Barrier,
    Building,
    Crosswalk,
    Gantry,
    Pole,
    Tree,
    Vegetation,
    None,
}

// Placeholder struct for misc object until the actual type is implemented
#[derive(Debug, Clone, PartialEq)]
pub struct MiscObject {
    pub name: Value<String>,
    pub misc_object_category: MiscObjectCategory,
    pub bounding_box: BoundingBox,
}
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

/// Miscellaneous object builder for type-safe misc object construction
/// 
/// This builder guides users through the misc object configuration process,
/// ensuring all required properties are set before the object can be
/// added to a scenario.
pub struct MiscObjectBuilder<S: BuilderState, E: EntityBuilderState = EntityEmpty> {
    /// Type state phantom data for scenario builder
    _scenario_state: PhantomData<S>,
    /// Type state phantom data for entity builder
    _entity_state: PhantomData<E>,
    
    /// Misc object name for scenario reference
    name: String,
    
    /// Misc object being constructed
    misc_object: MiscObject,
    
    /// Position for the misc object (optional until positioned)
    position: Option<Position>,
    
    /// Object controller configuration
    controller: Option<ObjectController>,
    
    /// Parent entities container
    entities: Entities,
    
    /// Entity registry for validation
    entity_registry: EntityRegistry,
}

impl<S: BuilderState> MiscObjectBuilder<S, EntityEmpty> {
    /// Create a new misc object builder
    pub(super) fn new(name: String, entities: Entities, entity_registry: EntityRegistry) -> Self {
        Self {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name,
            misc_object: MiscObject {
                name: Value::literal("DefaultMiscObject".to_string()),
                misc_object_category: MiscObjectCategory::None,
                bounding_box: BoundingBox::default(),
            },
            position: None,
            controller: None,
            entities,
            entity_registry,
        }
    }

    /// Configure as a barrier object
    /// 
    /// Sets the misc object category to barrier and provides barrier-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn barrier(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Barrier;
        
        // Set reasonable barrier defaults (concrete barrier)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.5),
                y: Value::literal(0.0),
                z: Value::literal(0.4),
            },
            dimensions: Dimensions {
                width: Value::literal(0.4),
                length: Value::literal(1.0),
                height: Value::literal(0.8),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a building object
    /// 
    /// Sets the misc object category to building and provides building-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn building(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Building;
        
        // Set reasonable building defaults (small building)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(5.0),
                y: Value::literal(5.0),
                z: Value::literal(3.0),
            },
            dimensions: Dimensions {
                width: Value::literal(10.0),
                length: Value::literal(10.0),
                height: Value::literal(6.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a crosswalk object
    /// 
    /// Sets the misc object category to crosswalk and provides crosswalk-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn crosswalk(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Crosswalk;
        
        // Set reasonable crosswalk defaults
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(2.0),
                y: Value::literal(0.0),
                z: Value::literal(0.0),
            },
            dimensions: Dimensions {
                width: Value::literal(3.0),
                length: Value::literal(4.0),
                height: Value::literal(0.1),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a gantry object
    /// 
    /// Sets the misc object category to gantry and provides gantry-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn gantry(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Gantry;
        
        // Set reasonable gantry defaults (highway sign gantry)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(3.0),
            },
            dimensions: Dimensions {
                width: Value::literal(15.0),
                length: Value::literal(1.0),
                height: Value::literal(6.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a pole object
    /// 
    /// Sets the misc object category to pole and provides pole-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn pole(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Pole;
        
        // Set reasonable pole defaults (street light pole)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(3.0),
            },
            dimensions: Dimensions {
                width: Value::literal(0.3),
                length: Value::literal(0.3),
                height: Value::literal(6.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a tree object
    /// 
    /// Sets the misc object category to tree and provides tree-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn tree(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Tree;
        
        // Set reasonable tree defaults (medium tree)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(2.5),
            },
            dimensions: Dimensions {
                width: Value::literal(3.0),
                length: Value::literal(3.0),
                height: Value::literal(5.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a vegetation object
    /// 
    /// Sets the misc object category to vegetation and provides vegetation-specific defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn vegetation(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::Vegetation;
        
        // Set reasonable vegetation defaults (bush/shrub)
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(0.5),
            },
            dimensions: Dimensions {
                width: Value::literal(1.5),
                length: Value::literal(1.5),
                height: Value::literal(1.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a generic object
    /// 
    /// Sets the misc object category to none and provides minimal defaults.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityTypeSelected state
    pub fn generic(mut self) -> MiscObjectBuilder<S, EntityTypeSelected> {
        self.misc_object.misc_object_category = MiscObjectCategory::None;
        
        // Set minimal generic defaults
        self.misc_object.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(0.5),
            },
            dimensions: Dimensions {
                width: Value::literal(1.0),
                length: Value::literal(1.0),
                height: Value::literal(1.0),
            },
        };

        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState> MiscObjectBuilder<S, EntityTypeSelected> {
    /// Set misc object model name
    /// 
    /// # Arguments
    /// * `model` - Model name or identifier
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_model(mut self, model: &str) -> Self {
        self.misc_object.name = Value::literal(model.to_string());
        self
    }

    /// Set misc object dimensions
    /// 
    /// # Arguments
    /// * `length` - Object length in meters
    /// * `width` - Object width in meters  
    /// * `height` - Object height in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        self.misc_object.bounding_box.dimensions = Dimensions {
            length: Value::literal(length),
            width: Value::literal(width),
            height: Value::literal(height),
        };
        self
    }

    /// Set misc object mass
    /// 
    /// # Arguments
    /// * `mass` - Object mass in kg
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_mass(mut self, mass: f64) -> Self {
        // Note: MiscObject type doesn't have a direct mass field
        // This would need to be added as a property or handled differently
        // For now, we'll skip this implementation
        self
    }

    /// Transition to configured state
    /// 
    /// This method indicates that the basic misc object configuration is complete
    /// and the object is ready for positioning.
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityConfigured state
    pub fn configured(self) -> MiscObjectBuilder<S, EntityConfigured> {
        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState, E: EntityBuilderState> MiscObjectBuilder<S, E> 
where 
    E: EntityBuilderState,
{
    /// Start positioning the misc object
    /// 
    /// This method creates a position builder for setting the object's
    /// initial position in the scenario.
    /// 
    /// # Returns
    /// PositionBuilder for configuring object position
    pub fn at_position(self) -> PositionBuilder<S, Self> {
        PositionBuilder::new(self)
    }

    /// Set misc object position directly
    /// 
    /// This method allows setting a pre-constructed position for the object.
    /// 
    /// # Arguments
    /// * `position` - Position configuration
    /// 
    /// # Returns
    /// MiscObjectBuilder in EntityPositioned state
    pub fn with_position(mut self, position: Position) -> MiscObjectBuilder<S, EntityPositioned> {
        self.position = Some(position);
        
        MiscObjectBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            misc_object: self.misc_object,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState> MiscObjectBuilder<S, EntityPositioned> {
    /// Complete misc object construction and add to scenario
    /// 
    /// This method validates the misc object configuration and adds it to the
    /// entities container.
    /// 
    /// # Returns
    /// EntitiesBuilder with the new misc object added
    pub fn finish_misc_object(mut self) -> BuilderResult<EntitiesBuilder<S>> {
        // Validate misc object configuration
        self.validate()?;

        // Create scenario object
        // Note: We'll need to extend ScenarioObject to support MiscObject
        // For now, we'll create a placeholder implementation
        let scenario_object = ScenarioObject {
            name: Value::literal(self.name.clone()),
            vehicle: None,
            pedestrian: None,
            catalog_reference: None,
            object_controller: self.controller,
        };

        // Register entity
        self.entity_registry.add_entity(self.name.clone(), scenario_object.clone())?;

        // Add to entities
        self.entities.add_object(scenario_object);

        Ok(EntitiesBuilder::new(self.entity_registry))
    }
}

impl<S: BuilderState, E: EntityBuilderState> EntityBuilder<S> for MiscObjectBuilder<S, E> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self) -> BuilderResult<()> {
        // Check that name is not empty
        if self.name.is_empty() {
            return Err(BuilderError::validation_error("Misc object name cannot be empty"));
        }

        // Check that position is set for positioned objects
        if std::any::type_name::<E>().contains("EntityPositioned") && self.position.is_none() {
            return Err(BuilderError::validation_error("Misc object position must be set"));
        }

        // Validate misc object properties
        let bbox = &self.misc_object.bounding_box;
        
        // Check dimensions are positive
        if let (Some(length), Some(width), Some(height)) = (
            bbox.dimensions.length.as_literal(),
            bbox.dimensions.width.as_literal(),
            bbox.dimensions.height.as_literal(),
        ) {
            if *length <= 0.0 || *width <= 0.0 || *height <= 0.0 {
                return Err(BuilderError::validation_error("Misc object dimensions must be positive"));
            }
        }

        Ok(())
    }

    fn with_controller(mut self, controller: ObjectController) -> Self {
        self.controller = Some(controller);
        self
    }
}

impl<S: BuilderState> FinalizableEntity<S> for MiscObjectBuilder<S, EntityPositioned> {
    fn finalize_entity(self) -> BuilderResult<EntitiesBuilder<S>> {
        self.finish_misc_object()
    }
}

/// Trait for position builder integration
/// 
/// This trait allows the position builder to work with misc object builders
/// by providing a way to set the position and transition states.
impl<S: BuilderState, E: EntityBuilderState> super::vehicle::PositionReceiver<S> for MiscObjectBuilder<S, E> {
    fn set_position(self, position: Position) -> MiscObjectBuilder<S, EntityPositioned> {
        self.with_position(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::registry::EntityRegistry;
    use crate::builder::states::HasRoadNetwork;

    #[test]
    fn test_misc_object_builder_barrier() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = MiscObjectBuilder::<HasRoadNetwork>::new("test_barrier".to_string(), entities, registry)
            .barrier()
            .with_model("concrete_barrier")
            .with_dimensions(1.0, 0.4, 0.8);
            
        assert_eq!(builder.get_name(), "test_barrier");
        assert_eq!(builder.misc_object.misc_object_category, MiscObjectCategory::Barrier);
    }

    #[test]
    fn test_misc_object_builder_tree() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = MiscObjectBuilder::<HasRoadNetwork>::new("test_tree".to_string(), entities, registry)
            .tree()
            .with_mass(1000.0);
            
        assert_eq!(builder.misc_object.misc_object_category, MiscObjectCategory::Tree);
    }

    #[test]
    fn test_misc_object_builder_validation() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = MiscObjectBuilder::<HasRoadNetwork>::new("".to_string(), entities, registry);
        
        let result = builder.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name cannot be empty"));
    }

    #[test]
    fn test_misc_object_builder_building() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = MiscObjectBuilder::<HasRoadNetwork>::new("office_building".to_string(), entities, registry)
            .building()
            .with_dimensions(20.0, 15.0, 12.0);
            
        assert_eq!(builder.misc_object.misc_object_category, MiscObjectCategory::Building);
        
        let dims = &builder.misc_object.bounding_box.dimensions;
        assert_eq!(dims.length.as_literal(), Some(&20.0));
        assert_eq!(dims.width.as_literal(), Some(&15.0));
        assert_eq!(dims.height.as_literal(), Some(&12.0));
    }

    #[test]
    fn test_misc_object_builder_pole() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = MiscObjectBuilder::<HasRoadNetwork>::new("street_light".to_string(), entities, registry)
            .pole()
            .with_dimensions(0.3, 0.3, 8.0);
            
        assert_eq!(builder.misc_object.misc_object_category, MiscObjectCategory::Pole);
        
        let dims = &builder.misc_object.bounding_box.dimensions;
        assert_eq!(dims.height.as_literal(), Some(&8.0));
    }
}
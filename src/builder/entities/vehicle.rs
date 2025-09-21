//! Vehicle entity builder for programmatic vehicle construction
//!
//! This module provides a type-safe builder for constructing vehicle entities
//! with proper configuration, positioning, and validation.

use crate::types::{
    basic::{Value, Double},
    entities::{
        vehicle::{Vehicle, VehicleCategory, Performance},
        axles::{Axles, Axle},
        ScenarioObject, Entities,
    },
    controllers::ObjectController,
    positions::Position,
    geometry::{BoundingBox, Center, Dimensions},
    enums::VehicleCategory as VehicleCategoryEnum,
};
use super::{
    EntitiesBuilder, EntityBuilder, FinalizableEntity,
    EntityBuilderState, EntityEmpty, EntityTypeSelected, EntityConfigured, EntityPositioned, EntityReady,
};
use crate::builder::{
    error::{BuilderError, BuilderResult},
    registry::EntityRegistry,
    states::BuilderState,
    positions::PositionBuilder,
};
use std::marker::PhantomData;

/// Vehicle builder for type-safe vehicle construction
/// 
/// This builder guides users through the vehicle configuration process,
/// ensuring all required properties are set before the vehicle can be
/// added to a scenario.
pub struct VehicleBuilder<S: BuilderState, E: EntityBuilderState = EntityEmpty> {
    /// Type state phantom data for scenario builder
    _scenario_state: PhantomData<S>,
    /// Type state phantom data for entity builder
    _entity_state: PhantomData<E>,
    
    /// Vehicle name for scenario reference
    name: String,
    
    /// Vehicle being constructed
    vehicle: Vehicle,
    
    /// Position for the vehicle (optional until positioned)
    position: Option<Position>,
    
    /// Object controller configuration
    controller: Option<ObjectController>,
    
    /// Parent entities container
    entities: Entities,
    
    /// Entity registry for validation
    entity_registry: EntityRegistry,
}

impl<S: BuilderState> VehicleBuilder<S, EntityEmpty> {
    /// Create a new vehicle builder
    pub(super) fn new(name: String, entities: Entities, entity_registry: EntityRegistry) -> Self {
        Self {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name,
            vehicle: Vehicle::default(),
            position: None,
            controller: None,
            entities,
            entity_registry,
        }
    }

    /// Configure as a car vehicle
    /// 
    /// Sets the vehicle category to car and provides car-specific defaults.
    /// 
    /// # Returns
    /// VehicleBuilder in EntityTypeSelected state
    pub fn car(mut self) -> VehicleBuilder<S, EntityTypeSelected> {
        self.vehicle.vehicle_category = VehicleCategory::Car;
        
        // Set reasonable car defaults
        self.vehicle.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(1.4),
                y: Value::literal(0.0),
                z: Value::literal(0.9),
            },
            dimensions: Dimensions {
                width: Value::literal(1.8),
                length: Value::literal(4.5),
                height: Value::literal(1.4),
            },
        };

        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a truck vehicle
    /// 
    /// Sets the vehicle category to truck and provides truck-specific defaults.
    /// 
    /// # Returns
    /// VehicleBuilder in EntityTypeSelected state
    pub fn truck(mut self) -> VehicleBuilder<S, EntityTypeSelected> {
        self.vehicle.vehicle_category = VehicleCategory::Truck;
        
        // Set reasonable truck defaults
        self.vehicle.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(2.5),
                y: Value::literal(0.0),
                z: Value::literal(1.5),
            },
            dimensions: Dimensions {
                width: Value::literal(2.5),
                length: Value::literal(12.0),
                height: Value::literal(3.0),
            },
        };

        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a bus vehicle
    /// 
    /// Sets the vehicle category to bus and provides bus-specific defaults.
    /// 
    /// # Returns
    /// VehicleBuilder in EntityTypeSelected state
    pub fn bus(mut self) -> VehicleBuilder<S, EntityTypeSelected> {
        self.vehicle.vehicle_category = VehicleCategory::Bus;
        
        // Set reasonable bus defaults
        self.vehicle.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(3.0),
                y: Value::literal(0.0),
                z: Value::literal(1.5),
            },
            dimensions: Dimensions {
                width: Value::literal(2.5),
                length: Value::literal(12.0),
                height: Value::literal(3.0),
            },
        };

        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Configure as a motorcycle vehicle
    /// 
    /// Sets the vehicle category to motorbike and provides motorcycle-specific defaults.
    /// 
    /// # Returns
    /// VehicleBuilder in EntityTypeSelected state
    pub fn motorcycle(mut self) -> VehicleBuilder<S, EntityTypeSelected> {
        self.vehicle.vehicle_category = VehicleCategory::Motorbike;
        
        // Set reasonable motorcycle defaults
        self.vehicle.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(1.0),
                y: Value::literal(0.0),
                z: Value::literal(0.6),
            },
            dimensions: Dimensions {
                width: Value::literal(0.8),
                length: Value::literal(2.2),
                height: Value::literal(1.2),
            },
        };

        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState> VehicleBuilder<S, EntityTypeSelected> {
    /// Set vehicle model name
    /// 
    /// # Arguments
    /// * `model` - Model name or identifier
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_model(mut self, model: &str) -> Self {
        self.vehicle.name = Value::literal(model.to_string());
        self
    }

    /// Set vehicle dimensions
    /// 
    /// # Arguments
    /// * `length` - Vehicle length in meters
    /// * `width` - Vehicle width in meters  
    /// * `height` - Vehicle height in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        self.vehicle.bounding_box.dimensions = Dimensions {
            length: Value::literal(length),
            width: Value::literal(width),
            height: Value::literal(height),
        };
        self
    }

    /// Set vehicle performance characteristics
    /// 
    /// # Arguments
    /// * `max_speed` - Maximum speed in m/s
    /// * `max_acceleration` - Maximum acceleration in m/s²
    /// * `max_deceleration` - Maximum deceleration in m/s²
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_performance(mut self, max_speed: f64, max_acceleration: f64, max_deceleration: f64) -> Self {
        self.vehicle.performance = Performance {
            max_speed: Value::literal(max_speed),
            max_acceleration: Value::literal(max_acceleration),
            max_deceleration: Value::literal(max_deceleration),
        };
        self
    }

    /// Set vehicle mass
    /// 
    /// # Arguments
    /// * `mass` - Vehicle mass in kg
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_mass(mut self, mass: f64) -> Self {
        // Note: Vehicle type doesn't have a direct mass field
        // This would need to be added as a property or handled differently
        // For now, we'll skip this implementation
        self
    }

    /// Configure vehicle axles
    /// 
    /// # Arguments
    /// * `front_axle` - Front axle configuration
    /// * `rear_axle` - Rear axle configuration
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_axles(mut self, front_axle: Axle, rear_axle: Axle) -> Self {
        self.vehicle.axles = Axles {
            front_axle,
            rear_axle,
            additional_axles: Vec::new(),
        };
        self
    }

    /// Transition to configured state
    /// 
    /// This method indicates that the basic vehicle configuration is complete
    /// and the vehicle is ready for positioning.
    /// 
    /// # Returns
    /// VehicleBuilder in EntityConfigured state
    pub fn configured(self) -> VehicleBuilder<S, EntityConfigured> {
        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }
}

impl<S: BuilderState, E: EntityBuilderState> VehicleBuilder<S, E> 
where 
    E: EntityBuilderState,
{
    /// Start positioning the vehicle
    /// 
    /// This method creates a position builder for setting the vehicle's
    /// initial position in the scenario.
    /// 
    /// # Returns
    /// PositionBuilder for configuring vehicle position
    pub fn at_position(self) -> PositionBuilder<S, Self> {
        PositionBuilder::new(self)
    }

    /// Set vehicle position directly
    /// 
    /// This method allows setting a pre-constructed position for the vehicle.
    /// 
    /// # Arguments
    /// * `position` - Position configuration
    /// 
    /// # Returns
    /// VehicleBuilder in EntityPositioned state
    pub fn with_position(mut self, position: Position) -> VehicleBuilder<S, EntityPositioned> {
        self.position = Some(position);
        
        VehicleBuilder {
            _scenario_state: PhantomData,
            _entity_state: PhantomData,
            name: self.name,
            vehicle: self.vehicle,
            position: self.position,
            controller: self.controller,
            entities: self.entities,
            entity_registry: self.entity_registry,
        }
    }

    /// Set initial speed for the vehicle
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

impl<S: BuilderState> VehicleBuilder<S, EntityPositioned> {
    /// Complete vehicle construction and add to scenario
    /// 
    /// This method validates the vehicle configuration and adds it to the
    /// entities container.
    /// 
    /// # Returns
    /// EntitiesBuilder with the new vehicle added
    pub fn finish_vehicle(mut self) -> BuilderResult<EntitiesBuilder<S>> {
        // Validate vehicle configuration
        self.validate()?;

        // Create scenario object
        let mut scenario_object = ScenarioObject::new_vehicle(self.name.clone(), self.vehicle);
        
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

impl<S: BuilderState, E: EntityBuilderState> EntityBuilder<S> for VehicleBuilder<S, E> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn validate(&self) -> BuilderResult<()> {
        // Check that name is not empty
        if self.name.is_empty() {
            return Err(BuilderError::validation_error("Vehicle name cannot be empty"));
        }

        // Check that position is set for positioned vehicles
        if std::any::type_name::<E>().contains("EntityPositioned") && self.position.is_none() {
            return Err(BuilderError::validation_error("Vehicle position must be set"));
        }

        // Validate vehicle properties
        let bbox = &self.vehicle.bounding_box;
        
        // Check dimensions are positive
        if let (Some(length), Some(width), Some(height)) = (
            bbox.dimensions.length.as_literal(),
            bbox.dimensions.width.as_literal(),
            bbox.dimensions.height.as_literal(),
        ) {
            if *length <= 0.0 || *width <= 0.0 || *height <= 0.0 {
                return Err(BuilderError::validation_error("Vehicle dimensions must be positive"));
            }
        }

        Ok(())
    }

    fn with_controller(mut self, controller: ObjectController) -> Self {
        self.controller = Some(controller);
        self
    }
}

impl<S: BuilderState> FinalizableEntity<S> for VehicleBuilder<S, EntityPositioned> {
    fn finalize_entity(self) -> BuilderResult<EntitiesBuilder<S>> {
        self.finish_vehicle()
    }
}

/// Trait for position builder integration
/// 
/// This trait allows the position builder to work with vehicle builders
/// by providing a way to set the position and transition states.
pub trait PositionReceiver<S: BuilderState> {
    /// Set position and transition to positioned state
    fn set_position(self, position: Position) -> VehicleBuilder<S, EntityPositioned>;
}

impl<S: BuilderState, E: EntityBuilderState> PositionReceiver<S> for VehicleBuilder<S, E> {
    fn set_position(self, position: Position) -> VehicleBuilder<S, EntityPositioned> {
        self.with_position(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::registry::EntityRegistry;
    use crate::builder::states::HasRoadNetwork;

    #[test]
    fn test_vehicle_builder_car() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = VehicleBuilder::<HasRoadNetwork>::new("test_car".to_string(), entities, registry)
            .car()
            .with_model("sedan")
            .with_dimensions(4.5, 1.8, 1.4);
            
        assert_eq!(builder.get_name(), "test_car");
        assert_eq!(builder.vehicle.vehicle_category, VehicleCategory::Car);
    }

    #[test]
    fn test_vehicle_builder_validation() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = VehicleBuilder::<HasRoadNetwork>::new("".to_string(), entities, registry);
        
        let result = builder.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name cannot be empty"));
    }

    #[test]
    fn test_vehicle_builder_performance() {
        let registry = EntityRegistry::new();
        let entities = Entities::new();
        
        let builder = VehicleBuilder::<HasRoadNetwork>::new("test_car".to_string(), entities, registry)
            .car()
            .with_performance(50.0, 8.0, 10.0);
            
        let performance = &builder.vehicle.performance;
        assert_eq!(performance.max_speed.as_literal(), Some(&50.0));
        assert_eq!(performance.max_acceleration.as_literal(), Some(&8.0));
        assert_eq!(performance.max_deceleration.as_literal(), Some(&10.0));
    }
}
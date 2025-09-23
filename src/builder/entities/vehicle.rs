//! Vehicle builder for programmatic vehicle construction

use crate::types::{
    basic::{Value, Double, OSString},
    entities::{
        Vehicle, Properties, Performance, Axle, // Axles,
        ScenarioObject,
    },
    enums::{VehicleCategory, Role},
    positions::Position,
};
use crate::types::geometry::BoundingBox;
use crate::builder::{
    BuilderError, BuilderResult,
    registry::EntityRegistry,
};
use super::EntityBuilder;

/// Builder for creating vehicle entities with fluent API
pub struct VehicleBuilder {
    name: String,
    vehicle: Vehicle,
    position: Option<Position>,
    entity_registry: Option<EntityRegistry>,
}

impl VehicleBuilder {
    /// Create a new vehicle builder
    pub fn new(name: String) -> Self {
        Self {
            name,
            vehicle: Vehicle::default(),
            position: None,
            entity_registry: None,
        }
    }
    
    /// Set the vehicle category to car
    pub fn car(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Car;
        self
    }
    
    /// Set the vehicle category to truck
    pub fn truck(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Truck;
        self
    }
    
    /// Set the vehicle category to bus
    pub fn bus(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Bus;
        self
    }
    
    /// Set the vehicle category to motorbike
    pub fn motorbike(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Motorbike;
        self
    }
    
    /// Set the vehicle category to bicycle
    pub fn bicycle(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Bicycle;
        self
    }
    
    /// Set the vehicle category to trailer
    pub fn trailer(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Trailer;
        self
    }
    
    /// Set the vehicle category to semitrailer
    pub fn semitrailer(mut self) -> Self {
        self.vehicle.vehicle_category = VehicleCategory::Semitrailer;
        self
    }
    
    /// Set the vehicle model name
    pub fn with_model(mut self, model: &str) -> Self {
        self.vehicle.name = Value::literal(model.to_string());
        self
    }
    
    /// Set vehicle dimensions (length, width, height)
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let bounding_box = BoundingBox {
            center: crate::types::geometry::shapes::Center {
                x: Value::literal(0.0),
                y: Value::literal(0.0),
                z: Value::literal(height / 2.0),
            },
            dimensions: crate::types::geometry::shapes::Dimensions {
                width: Value::literal(width),
                length: Value::literal(length),
                height: Value::literal(height),
            },
        };
        
        self.vehicle.bounding_box = bounding_box;
        self
    }
    
    /// Set vehicle performance characteristics
    pub fn with_performance(mut self, max_speed: f64, max_acceleration: f64, max_deceleration: f64) -> Self {
        let performance = Performance {
            max_speed: Value::literal(max_speed),
            max_acceleration: Value::literal(max_acceleration),
            max_deceleration: Value::literal(max_deceleration),
        };
        
        self.vehicle.performance = performance;
        self
    }
    
    /// Set vehicle mass
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.vehicle.mass = Value::literal(mass);
        self
    }
    
    /// Set vehicle role
    pub fn with_role(mut self, role: Role) -> Self {
        self.vehicle.role = Some(role);
        self
    }
    
    /// Configure front axle
    pub fn with_front_axle(mut self, max_steering: f64, wheel_diameter: f64, track_width: f64, position_x: f64, position_z: f64) -> Self {
        let front_axle = Axle {
            max_steering: Value::literal(max_steering),
            wheel_diameter: Value::literal(wheel_diameter),
            track_width: Value::literal(track_width),
            position_x: Value::literal(position_x),
            position_z: Value::literal(position_z),
        };
        
        self.vehicle.axles.front_axle = front_axle;
        self
    }
    
    /// Configure rear axle
    pub fn with_rear_axle(mut self, max_steering: f64, wheel_diameter: f64, track_width: f64, position_x: f64, position_z: f64) -> Self {
        let rear_axle = Axle {
            max_steering: Value::literal(max_steering),
            wheel_diameter: Value::literal(wheel_diameter),
            track_width: Value::literal(track_width),
            position_x: Value::literal(position_x),
            position_z: Value::literal(position_z),
        };
        
        self.vehicle.axles.rear_axle = rear_axle;
        self
    }
    
    /// Add additional axle
    pub fn with_additional_axle(mut self, max_steering: f64, wheel_diameter: f64, track_width: f64, position_x: f64, position_z: f64) -> Self {
        let additional_axle = Axle {
            max_steering: Value::literal(max_steering),
            wheel_diameter: Value::literal(wheel_diameter),
            track_width: Value::literal(track_width),
            position_x: Value::literal(position_x),
            position_z: Value::literal(position_z),
        };
        
        self.vehicle.axles.additional_axles.push(additional_axle);
        self
    }
    
    /// Set the position for this vehicle
    pub fn at_position(mut self) -> PositionBuilder {
        PositionBuilder::new(self)
    }
    
    /// Set entity registry for validation
    pub fn with_registry(mut self, registry: EntityRegistry) -> Self {
        self.entity_registry = Some(registry);
        self
    }
    
    /// Finish building the vehicle and return a scenario object
    pub fn finish_vehicle(self) -> BuilderResult<ScenarioObject> {
        // Validate the vehicle configuration
        self.validate()?;
        
        // Create the scenario object
        Ok(ScenarioObject::new_vehicle(self.name, self.vehicle))
    }
    
    /// Validate the vehicle configuration
    fn validate(&self) -> BuilderResult<()> {
        // Check that required fields are set
        if self.vehicle.name.as_literal().unwrap_or("").is_empty() {
            return Err(BuilderError::validation_error(
                "Vehicle model name is required",
                "Call with_model() to set the vehicle model"
            ));
        }
        
        // Validate performance values
        if let Some(max_speed) = self.vehicle.performance.max_speed.as_literal() {
            if *max_speed <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Max speed must be positive",
                    "Set max_speed > 0 in with_performance()"
                ));
            }
        }
        
        if let Some(max_acceleration) = self.vehicle.performance.max_acceleration.as_literal() {
            if *max_acceleration <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Max acceleration must be positive",
                    "Set max_acceleration > 0 in with_performance()"
                ));
            }
        }
        
        if let Some(max_deceleration) = self.vehicle.performance.max_deceleration.as_literal() {
            if *max_deceleration <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Max deceleration must be positive",
                    "Set max_deceleration > 0 in with_performance()"
                ));
            }
        }
        
        // Validate mass
        if let Some(mass) = self.vehicle.mass.as_literal() {
            if *mass <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Vehicle mass must be positive",
                    "Set mass > 0 in with_mass()"
                ));
            }
        }
        
        // Validate dimensions
        let dimensions = &self.vehicle.bounding_box.dimensions;
        if let Some(width) = dimensions.width.as_literal() {
            if *width <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Vehicle width must be positive",
                    "Set width > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(length) = dimensions.length.as_literal() {
            if *length <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Vehicle length must be positive",
                    "Set length > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(height) = dimensions.height.as_literal() {
            if *height <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Vehicle height must be positive",
                    "Set height > 0 in with_dimensions()"
                ));
            }
        }
        
        Ok(())
    }
}

impl EntityBuilder for VehicleBuilder {
    type Entity = ScenarioObject;
    
    fn finish(self) -> BuilderResult<Self::Entity> {
        self.finish_vehicle()
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
}

/// Position builder for setting vehicle position
pub struct PositionBuilder {
    vehicle_builder: VehicleBuilder,
}

impl PositionBuilder {
    fn new(vehicle_builder: VehicleBuilder) -> Self {
        Self { vehicle_builder }
    }
    
    /// Set world position
    pub fn world(mut self, x: f64, y: f64, z: f64) -> VehicleBuilder {
        let world_pos = crate::types::positions::WorldPosition {
            x: Value::literal(x),
            y: Value::literal(y),
            z: Value::literal(z),
            h: Value::literal(0.0),
            p: Value::literal(0.0),
            r: Value::literal(0.0),
        };
        
        let position = Position {
            world_position: Some(world_pos),
            ..Position::empty()
        };
        
        self.vehicle_builder.position = Some(position);
        self.vehicle_builder
    }
    
    /// Set lane position
    pub fn lane(mut self, road_id: &str, lane_id: i32, s: f64) -> VehicleBuilder {
        let lane_pos = crate::types::positions::LanePosition {
            road_id: Value::literal(road_id.to_string()),
            lane_id: Value::literal(lane_id),
            s: Value::literal(s),
            offset: Value::literal(0.0),
            orientation: None,
        };
        
        let position = Position {
            lane_position: Some(lane_pos),
            ..Position::empty()
        };
        
        self.vehicle_builder.position = Some(position);
        self.vehicle_builder
    }
    
    /// Set road position
    pub fn road(mut self, road_id: &str, s: f64, t: f64) -> VehicleBuilder {
        let road_pos = crate::types::positions::RoadPosition {
            road_id: Value::literal(road_id.to_string()),
            s: Value::literal(s),
            t: Value::literal(t),
            orientation: None,
        };
        
        let position = Position {
            road_position: Some(road_pos),
            ..Position::empty()
        };
        
        self.vehicle_builder.position = Some(position);
        self.vehicle_builder
    }
    
    /// Set relative position to another entity
    pub fn relative_to(mut self, entity_ref: &str, dx: f64, dy: f64, dz: f64) -> VehicleBuilder {
        let relative_pos = crate::types::positions::RelativeWorldPosition {
            entity_ref: Value::literal(entity_ref.to_string()),
            dx: Value::literal(dx),
            dy: Value::literal(dy),
            dz: Value::literal(dz),
        };
        
        let position = Position {
            relative_world_position: Some(relative_pos),
            ..Position::empty()
        };
        
        self.vehicle_builder.position = Some(position);
        self.vehicle_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vehicle_builder_basic() {
        let vehicle_obj = VehicleBuilder::new("test_vehicle".to_string())
            .car()
            .with_model("TestCar")
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(50.0, 3.0, 8.0)
            .with_mass(1500.0)
            .finish_vehicle()
            .unwrap();
        
        assert_eq!(vehicle_obj.get_name(), Some("test_vehicle"));
        assert!(vehicle_obj.vehicle.is_some());
        
        let vehicle = vehicle_obj.vehicle.unwrap();
        assert_eq!(vehicle.vehicle_category, VehicleCategory::Car);
        assert_eq!(vehicle.name.as_literal().unwrap(), "TestCar");
        assert_eq!(vehicle.mass.as_literal().unwrap(), &1500.0);
    }
    
    #[test]
    fn test_vehicle_builder_with_position() {
        let vehicle_obj = VehicleBuilder::new("positioned_vehicle".to_string())
            .car()
            .with_model("TestCar")
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(50.0, 3.0, 8.0)
            .at_position()
            .world(10.0, 20.0, 0.0)
            .finish_vehicle()
            .unwrap();
        
        assert_eq!(vehicle_obj.get_name(), Some("positioned_vehicle"));
        // Position is stored in the builder but not directly in the scenario object
        // Position would be used when adding to init actions
    }
    
    #[test]
    fn test_vehicle_builder_validation() {
        // Test missing model name
        let result = VehicleBuilder::new("test".to_string())
            .car()
            .finish_vehicle();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("model name"));
        
        // Test invalid performance values
        let result = VehicleBuilder::new("test".to_string())
            .car()
            .with_model("TestCar")
            .with_performance(-10.0, 3.0, 8.0)
            .finish_vehicle();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Max speed"));
    }
    
    #[test]
    fn test_vehicle_builder_axles() {
        let vehicle_obj = VehicleBuilder::new("axle_vehicle".to_string())
            .car()
            .with_model("TestCar")
            .with_dimensions(4.5, 1.8, 1.4)
            .with_performance(50.0, 3.0, 8.0)
            .with_front_axle(0.5, 0.6, 1.6, 2.8, 0.3)
            .with_rear_axle(0.0, 0.6, 1.6, -1.2, 0.3)
            .finish_vehicle()
            .unwrap();
        
        let vehicle = vehicle_obj.vehicle.unwrap();
        assert_eq!(vehicle.axles.front_axle.max_steering.as_literal().unwrap(), &0.5);
        assert_eq!(vehicle.axles.rear_axle.max_steering.as_literal().unwrap(), &0.0);
    }
}
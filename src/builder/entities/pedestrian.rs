//! Pedestrian builder for programmatic pedestrian construction

use crate::types::{
    basic::{Value, Double, OSString},
    entities::{
        Pedestrian,
        ScenarioObject,
    },
    enums::PedestrianCategory,
    positions::Position,
};
use crate::builder::{
    BuilderError, BuilderResult,
    registry::EntityRegistry,
};
use super::EntityBuilder;

/// Builder for creating pedestrian entities with fluent API
pub struct PedestrianBuilder {
    name: String,
    pedestrian: Pedestrian,
    position: Option<Position>,
    entity_registry: Option<EntityRegistry>,
}

impl PedestrianBuilder {
    /// Create a new pedestrian builder
    pub fn new(name: String) -> Self {
        Self {
            name,
            pedestrian: Pedestrian::default(),
            position: None,
            entity_registry: None,
        }
    }
    
    /// Set the pedestrian category to pedestrian
    pub fn pedestrian(mut self) -> Self {
        self.pedestrian.pedestrian_category = PedestrianCategory::Pedestrian;
        self
    }
    
    /// Set the pedestrian category to wheelchair
    pub fn wheelchair(mut self) -> Self {
        self.pedestrian.pedestrian_category = PedestrianCategory::Wheelchair;
        self
    }
    
    /// Set the pedestrian category to animal
    pub fn animal(mut self) -> Self {
        self.pedestrian.pedestrian_category = PedestrianCategory::Animal;
        self
    }
    
    /// Set the pedestrian model name
    pub fn with_model(mut self, model: &str) -> Self {
        self.pedestrian.name = Value::literal(model.to_string());
        self
    }
    
    /// Set pedestrian dimensions (length, width, height)
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let bounding_box = crate::types::entities::vehicle::BoundingBox {
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
        
        self.pedestrian.bounding_box = bounding_box;
        self
    }
    
    /// Set pedestrian mass
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.pedestrian.mass = Value::literal(mass);
        self
    }
    
    /// Set the position for this pedestrian
    pub fn at_position(mut self) -> PedestrianPositionBuilder {
        PedestrianPositionBuilder::new(self)
    }
    
    /// Set entity registry for validation
    pub fn with_registry(mut self, registry: EntityRegistry) -> Self {
        self.entity_registry = Some(registry);
        self
    }
    
    /// Finish building the pedestrian and return a scenario object
    pub fn finish_pedestrian(self) -> BuilderResult<ScenarioObject> {
        // Validate the pedestrian configuration
        self.validate()?;
        
        // Create the scenario object
        Ok(ScenarioObject::new_pedestrian(self.name, self.pedestrian))
    }
    
    /// Validate the pedestrian configuration
    fn validate(&self) -> BuilderResult<()> {
        // Check that required fields are set
        if self.pedestrian.name.as_literal().unwrap_or("").is_empty() {
            return Err(BuilderError::validation_error(
                "Pedestrian model name is required",
                "Call with_model() to set the pedestrian model"
            ));
        }
        
        // Validate mass
        if let Some(mass) = self.pedestrian.mass.as_literal() {
            if *mass <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Pedestrian mass must be positive",
                    "Set mass > 0 in with_mass()"
                ));
            }
        }
        
        // Validate dimensions
        let dimensions = &self.pedestrian.bounding_box.dimensions;
        if let Some(width) = dimensions.width.as_literal() {
            if *width <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Pedestrian width must be positive",
                    "Set width > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(length) = dimensions.length.as_literal() {
            if *length <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Pedestrian length must be positive",
                    "Set length > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(height) = dimensions.height.as_literal() {
            if *height <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Pedestrian height must be positive",
                    "Set height > 0 in with_dimensions()"
                ));
            }
        }
        
        Ok(())
    }
}

impl EntityBuilder for PedestrianBuilder {
    type Entity = ScenarioObject;
    
    fn finish(self) -> BuilderResult<Self::Entity> {
        self.finish_pedestrian()
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
}

/// Position builder for setting pedestrian position
pub struct PedestrianPositionBuilder {
    pedestrian_builder: PedestrianBuilder,
}

impl PedestrianPositionBuilder {
    fn new(pedestrian_builder: PedestrianBuilder) -> Self {
        Self { pedestrian_builder }
    }
    
    /// Set world position
    pub fn world(mut self, x: f64, y: f64, z: f64) -> PedestrianBuilder {
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
        
        self.pedestrian_builder.position = Some(position);
        self.pedestrian_builder
    }
    
    /// Set lane position
    pub fn lane(mut self, road_id: &str, lane_id: i32, s: f64) -> PedestrianBuilder {
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
        
        self.pedestrian_builder.position = Some(position);
        self.pedestrian_builder
    }
    
    /// Set road position
    pub fn road(mut self, road_id: &str, s: f64, t: f64) -> PedestrianBuilder {
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
        
        self.pedestrian_builder.position = Some(position);
        self.pedestrian_builder
    }
    
    /// Set relative position to another entity
    pub fn relative_to(mut self, entity_ref: &str, dx: f64, dy: f64, dz: f64) -> PedestrianBuilder {
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
        
        self.pedestrian_builder.position = Some(position);
        self.pedestrian_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pedestrian_builder_basic() {
        let pedestrian_obj = PedestrianBuilder::new("test_pedestrian".to_string())
            .pedestrian()
            .with_model("TestPedestrian")
            .with_dimensions(0.6, 0.4, 1.8)
            .with_mass(70.0)
            .finish_pedestrian()
            .unwrap();
        
        assert_eq!(pedestrian_obj.get_name(), Some("test_pedestrian"));
        assert!(pedestrian_obj.pedestrian.is_some());
        
        let pedestrian = pedestrian_obj.pedestrian.unwrap();
        assert_eq!(pedestrian.pedestrian_category, PedestrianCategory::Pedestrian);
        assert_eq!(pedestrian.name.as_literal().unwrap(), "TestPedestrian");
        assert_eq!(pedestrian.mass.as_literal().unwrap(), &70.0);
    }
    
    #[test]
    fn test_pedestrian_builder_with_position() {
        let pedestrian_obj = PedestrianBuilder::new("positioned_pedestrian".to_string())
            .pedestrian()
            .with_model("TestPedestrian")
            .with_dimensions(0.6, 0.4, 1.8)
            .with_mass(70.0)
            .at_position()
            .world(5.0, 10.0, 0.0)
            .finish_pedestrian()
            .unwrap();
        
        assert_eq!(pedestrian_obj.get_name(), Some("positioned_pedestrian"));
    }
    
    #[test]
    fn test_pedestrian_builder_validation() {
        // Test missing model name
        let result = PedestrianBuilder::new("test".to_string())
            .pedestrian()
            .finish_pedestrian();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("model name"));
        
        // Test invalid mass
        let result = PedestrianBuilder::new("test".to_string())
            .pedestrian()
            .with_model("TestPedestrian")
            .with_mass(-10.0)
            .finish_pedestrian();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("mass"));
    }
    
    #[test]
    fn test_pedestrian_categories() {
        let wheelchair_obj = PedestrianBuilder::new("wheelchair_user".to_string())
            .wheelchair()
            .with_model("WheelchairUser")
            .with_dimensions(1.2, 0.7, 1.3)
            .with_mass(100.0)
            .finish_pedestrian()
            .unwrap();
        
        let pedestrian = wheelchair_obj.pedestrian.unwrap();
        assert_eq!(pedestrian.pedestrian_category, PedestrianCategory::Wheelchair);
        
        let animal_obj = PedestrianBuilder::new("dog".to_string())
            .animal()
            .with_model("Dog")
            .with_dimensions(1.0, 0.3, 0.6)
            .with_mass(25.0)
            .finish_pedestrian()
            .unwrap();
        
        let animal = animal_obj.pedestrian.unwrap();
        assert_eq!(animal.pedestrian_category, PedestrianCategory::Animal);
    }
}
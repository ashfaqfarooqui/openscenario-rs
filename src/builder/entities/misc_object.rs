//! Miscellaneous object builder for programmatic misc object construction

use crate::types::{
    basic::{Value, Double, OSString},
    entities::{
        misc_object::{MiscObject, MiscObjectCategory},
        ScenarioObject,
    },
    positions::Position,
};
use crate::builder::{
    BuilderError, BuilderResult,
    registry::EntityRegistry,
};
use super::EntityBuilder;

/// Builder for creating miscellaneous object entities with fluent API
pub struct MiscObjectBuilder {
    name: String,
    misc_object: MiscObject,
    position: Option<Position>,
    entity_registry: Option<EntityRegistry>,
}

impl MiscObjectBuilder {
    /// Create a new misc object builder
    pub fn new(name: String) -> Self {
        Self {
            name,
            misc_object: MiscObject::default(),
            position: None,
            entity_registry: None,
        }
    }
    
    /// Set the misc object category to none
    pub fn none(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::None;
        self
    }
    
    /// Set the misc object category to obstacle
    pub fn obstacle(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Obstacle;
        self
    }
    
    /// Set the misc object category to pole
    pub fn pole(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Pole;
        self
    }
    
    /// Set the misc object category to tree
    pub fn tree(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Tree;
        self
    }
    
    /// Set the misc object category to vegetation
    pub fn vegetation(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Vegetation;
        self
    }
    
    /// Set the misc object category to barrier
    pub fn barrier(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Barrier;
        self
    }
    
    /// Set the misc object category to building
    pub fn building(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Building;
        self
    }
    
    /// Set the misc object category to parking space
    pub fn parking_space(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::ParkingSpace;
        self
    }
    
    /// Set the misc object category to patch
    pub fn patch(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Patch;
        self
    }
    
    /// Set the misc object category to railing
    pub fn railing(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Railing;
        self
    }
    
    /// Set the misc object category to traffic island
    pub fn traffic_island(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::TrafficIsland;
        self
    }
    
    /// Set the misc object category to crosswalk
    pub fn crosswalk(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Crosswalk;
        self
    }
    
    /// Set the misc object category to street lamp
    pub fn street_lamp(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::StreetLamp;
        self
    }
    
    /// Set the misc object category to gantry
    pub fn gantry(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Gantry;
        self
    }
    
    /// Set the misc object category to sound barrier
    pub fn sound_barrier(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::SoundBarrier;
        self
    }
    
    /// Set the misc object category to wind
    pub fn wind(mut self) -> Self {
        self.misc_object.misc_object_category = MiscObjectCategory::Wind;
        self
    }
    
    /// Set the misc object model name
    pub fn with_model(mut self, model: &str) -> Self {
        self.misc_object.name = Value::literal(model.to_string());
        self
    }
    
    /// Set misc object dimensions (length, width, height)
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
        
        self.misc_object.bounding_box = bounding_box;
        self
    }
    
    /// Set misc object mass
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.misc_object.mass = Value::literal(mass);
        self
    }
    
    /// Set the position for this misc object
    pub fn at_position(mut self) -> MiscObjectPositionBuilder {
        MiscObjectPositionBuilder::new(self)
    }
    
    /// Set entity registry for validation
    pub fn with_registry(mut self, registry: EntityRegistry) -> Self {
        self.entity_registry = Some(registry);
        self
    }
    
    /// Finish building the misc object and return a scenario object
    pub fn finish_misc_object(self) -> BuilderResult<ScenarioObject> {
        // Validate the misc object configuration
        self.validate()?;
        
        // Create the scenario object
        Ok(ScenarioObject::new_misc_object(self.name, self.misc_object))
    }
    
    /// Validate the misc object configuration
    fn validate(&self) -> BuilderResult<()> {
        // Check that required fields are set
        if self.misc_object.name.as_literal().unwrap_or("").is_empty() {
            return Err(BuilderError::validation_error(
                "Misc object model name is required",
                "Call with_model() to set the misc object model"
            ));
        }
        
        // Validate mass
        if let Some(mass) = self.misc_object.mass.as_literal() {
            if *mass < 0.0 {
                return Err(BuilderError::validation_error(
                    "Misc object mass must be non-negative",
                    "Set mass >= 0 in with_mass()"
                ));
            }
        }
        
        // Validate dimensions
        let dimensions = &self.misc_object.bounding_box.dimensions;
        if let Some(width) = dimensions.width.as_literal() {
            if *width <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Misc object width must be positive",
                    "Set width > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(length) = dimensions.length.as_literal() {
            if *length <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Misc object length must be positive",
                    "Set length > 0 in with_dimensions()"
                ));
            }
        }
        
        if let Some(height) = dimensions.height.as_literal() {
            if *height <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Misc object height must be positive",
                    "Set height > 0 in with_dimensions()"
                ));
            }
        }
        
        Ok(())
    }
}

impl EntityBuilder for MiscObjectBuilder {
    type Entity = ScenarioObject;
    
    fn finish(self) -> BuilderResult<Self::Entity> {
        self.finish_misc_object()
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
}

/// Position builder for setting misc object position
pub struct MiscObjectPositionBuilder {
    misc_object_builder: MiscObjectBuilder,
}

impl MiscObjectPositionBuilder {
    fn new(misc_object_builder: MiscObjectBuilder) -> Self {
        Self { misc_object_builder }
    }
    
    /// Set world position
    pub fn world(mut self, x: f64, y: f64, z: f64) -> MiscObjectBuilder {
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
        
        self.misc_object_builder.position = Some(position);
        self.misc_object_builder
    }
    
    /// Set lane position
    pub fn lane(mut self, road_id: &str, lane_id: i32, s: f64) -> MiscObjectBuilder {
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
        
        self.misc_object_builder.position = Some(position);
        self.misc_object_builder
    }
    
    /// Set road position
    pub fn road(mut self, road_id: &str, s: f64, t: f64) -> MiscObjectBuilder {
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
        
        self.misc_object_builder.position = Some(position);
        self.misc_object_builder
    }
    
    /// Set relative position to another entity
    pub fn relative_to(mut self, entity_ref: &str, dx: f64, dy: f64, dz: f64) -> MiscObjectBuilder {
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
        
        self.misc_object_builder.position = Some(position);
        self.misc_object_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_misc_object_builder_basic() {
        let misc_obj = MiscObjectBuilder::new("test_barrier".to_string())
            .barrier()
            .with_model("ConcreteBarrier")
            .with_dimensions(2.0, 0.5, 1.0)
            .with_mass(500.0)
            .finish_misc_object()
            .unwrap();
        
        assert_eq!(misc_obj.get_name(), Some("test_barrier"));
        assert!(misc_obj.misc_object.is_some());
        
        let misc_object = misc_obj.misc_object.unwrap();
        assert_eq!(misc_object.misc_object_category, MiscObjectCategory::Barrier);
        assert_eq!(misc_object.name.as_literal().unwrap(), "ConcreteBarrier");
        assert_eq!(misc_object.mass.as_literal().unwrap(), &500.0);
    }
    
    #[test]
    fn test_misc_object_builder_with_position() {
        let misc_obj = MiscObjectBuilder::new("positioned_tree".to_string())
            .tree()
            .with_model("Oak")
            .with_dimensions(1.0, 1.0, 8.0)
            .with_mass(1000.0)
            .at_position()
            .world(15.0, 25.0, 0.0)
            .finish_misc_object()
            .unwrap();
        
        assert_eq!(misc_obj.get_name(), Some("positioned_tree"));
    }
    
    #[test]
    fn test_misc_object_builder_validation() {
        // Test missing model name
        let result = MiscObjectBuilder::new("test".to_string())
            .obstacle()
            .finish_misc_object();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("model name"));
        
        // Test invalid mass (negative)
        let result = MiscObjectBuilder::new("test".to_string())
            .obstacle()
            .with_model("TestObstacle")
            .with_mass(-10.0)
            .finish_misc_object();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("mass"));
    }
    
    #[test]
    fn test_misc_object_categories() {
        let pole_obj = MiscObjectBuilder::new("street_pole".to_string())
            .pole()
            .with_model("StreetPole")
            .with_dimensions(0.3, 0.3, 5.0)
            .with_mass(50.0)
            .finish_misc_object()
            .unwrap();
        
        let misc_object = pole_obj.misc_object.unwrap();
        assert_eq!(misc_object.misc_object_category, MiscObjectCategory::Pole);
        
        let building_obj = MiscObjectBuilder::new("office_building".to_string())
            .building()
            .with_model("OfficeBuilding")
            .with_dimensions(20.0, 15.0, 30.0)
            .with_mass(0.0) // Buildings can have zero mass
            .finish_misc_object()
            .unwrap();
        
        let building = building_obj.misc_object.unwrap();
        assert_eq!(building.misc_object_category, MiscObjectCategory::Building);
    }
}
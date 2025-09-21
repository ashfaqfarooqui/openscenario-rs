//! World position builder for absolute coordinate positioning

use crate::types::{
    basic::Value,
    positions::{Position, WorldPosition},
};
use super::{PositionReceiver, validate_coordinates};
use crate::builder::{
    error::BuilderResult,
    states::BuilderState,
};
use std::marker::PhantomData;

/// Builder for world positions using absolute coordinates
/// 
/// World positions define entity placement using absolute X, Y, Z coordinates
/// in the scenario's world coordinate system.
pub struct WorldPositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// X coordinate in meters
    x: f64,
    
    /// Y coordinate in meters
    y: f64,
    
    /// Z coordinate in meters
    z: f64,
    
    /// Heading angle in radians
    h: Option<f64>,
    
    /// Pitch angle in radians
    p: Option<f64>,
    
    /// Roll angle in radians
    r: Option<f64>,
}

impl<S: BuilderState, T> WorldPositionBuilder<S, T> {
    /// Create a new world position builder
    pub fn new(parent: T, x: f64, y: f64, z: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            x,
            y,
            z,
            h: None,
            p: None,
            r: None,
        }
    }

    /// Set the heading (rotation around Z-axis)
    /// 
    /// # Arguments
    /// * `heading` - Heading angle in radians (0 = positive X direction)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_heading(mut self, heading: f64) -> Self {
        self.h = Some(heading);
        self
    }

    /// Set the pitch (rotation around Y-axis)
    /// 
    /// # Arguments
    /// * `pitch` - Pitch angle in radians (positive = nose up)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_pitch(mut self, pitch: f64) -> Self {
        self.orientation.p = Value::literal(pitch);
        self
    }

    /// Set the roll (rotation around X-axis)
    /// 
    /// # Arguments
    /// * `roll` - Roll angle in radians (positive = right side down)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_roll(mut self, roll: f64) -> Self {
        self.orientation.r = Value::literal(roll);
        self
    }

    /// Set complete orientation
    /// 
    /// # Arguments
    /// * `heading` - Heading angle in radians
    /// * `pitch` - Pitch angle in radians
    /// * `roll` - Roll angle in radians
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_orientation(mut self, heading: f64, pitch: f64, roll: f64) -> Self {
        self.orientation = Orientation {
            h: Value::literal(heading),
            p: Value::literal(pitch),
            r: Value::literal(roll),
        };
        self
    }

    /// Complete the world position and return to parent builder
    /// 
    /// This method validates the coordinates and creates the final position.
    /// 
    /// # Returns
    /// Parent builder with world position set
    /// 
    /// # Errors
    /// Returns error if coordinates are invalid (infinite or NaN)
    pub fn finish(self) -> BuilderResult<T>
    where
        T: PositionReceiver<S>,
    {
        // Validate coordinates
        validate_coordinates(self.x, self.y, self.z)?;

        // Create world position
        let world_position = WorldPosition {
            x: Value::literal(self.x),
            y: Value::literal(self.y),
            z: Some(Value::literal(self.z)),
            h: self.h.map(|h| Value::literal(h)),
            p: self.p.map(|p| Value::literal(p)),
            r: self.r.map(|r| Value::literal(r)),
        };

        // Create position container
        let position = Position {
            world_position: Some(world_position),
            ..Position::empty()
        };

        Ok(self.parent.set_position(position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::states::HasRoadNetwork;

    // Mock parent for testing
    struct MockParent {
        position: Option<Position>,
    }

    impl PositionReceiver<HasRoadNetwork> for MockParent {
        fn set_position(mut self, position: Position) -> Self {
            self.position = Some(position);
            self
        }
    }

    #[test]
    fn test_world_position_builder() {
        let parent = MockParent { position: None };
        let builder = WorldPositionBuilder::new(parent, 100.0, 200.0, 0.0);
        
        let result = builder.with_heading(1.57).finish().unwrap();
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.world_position.is_some());
        
        let world_pos = pos.world_position.unwrap();
        assert_eq!(world_pos.x.as_literal(), Some(&100.0));
        assert_eq!(world_pos.y.as_literal(), Some(&200.0));
        assert_eq!(world_pos.z.as_literal(), Some(&0.0));
        assert_eq!(world_pos.h.as_literal(), Some(&1.57));
    }

    #[test]
    fn test_world_position_orientation() {
        let parent = MockParent { position: None };
        let builder = WorldPositionBuilder::new(parent, 0.0, 0.0, 0.0);
        
        let result = builder
            .with_orientation(1.0, 0.5, -0.2)
            .finish()
            .unwrap();
        
        let world_pos = result.position.unwrap().world_position.unwrap();
        assert_eq!(world_pos.h.as_literal(), Some(&1.0));
        assert_eq!(world_pos.p.as_literal(), Some(&0.5));
        assert_eq!(world_pos.r.as_literal(), Some(&-0.2));
    }

    #[test]
    fn test_world_position_validation() {
        let parent = MockParent { position: None };
        let builder = WorldPositionBuilder::new(parent, f64::INFINITY, 0.0, 0.0);
        
        let result = builder.finish();
        assert!(result.is_err());
    }
}
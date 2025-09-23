//! World position builder for programmatic world position construction

use crate::types::{
    basic::Value,
    positions::{Position, WorldPosition, Orientation},
};
use crate::builder::{BuilderError, BuilderResult};
use super::{PositionBuilder, validate_coordinate, validate_angle};

/// Builder for creating world positions with fluent API
#[derive(Debug, Clone)]
pub struct WorldPositionBuilder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    h: Option<f64>, // heading
    p: Option<f64>, // pitch
    r: Option<f64>, // roll
}

impl WorldPositionBuilder {
    /// Create a new world position builder
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            h: Some(0.0), // Default heading
            p: Some(0.0), // Default pitch
            r: Some(0.0), // Default roll
        }
    }
    
    /// Set the x coordinate
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }
    
    /// Set the y coordinate
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }
    
    /// Set the z coordinate
    pub fn z(mut self, z: f64) -> Self {
        self.z = Some(z);
        self
    }
    
    /// Set all coordinates at once
    pub fn coordinates(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self.z = Some(z);
        self
    }
    
    /// Set the heading (rotation around z-axis)
    pub fn heading(mut self, h: f64) -> Self {
        self.h = Some(h);
        self
    }
    
    /// Set the pitch (rotation around y-axis)
    pub fn pitch(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }
    
    /// Set the roll (rotation around x-axis)
    pub fn roll(mut self, r: f64) -> Self {
        self.r = Some(r);
        self
    }
    
    /// Set all orientation angles at once
    pub fn orientation(mut self, h: f64, p: f64, r: f64) -> Self {
        self.h = Some(h);
        self.p = Some(p);
        self.r = Some(r);
        self
    }
    
    /// Set position and orientation from separate values
    pub fn position_and_orientation(mut self, x: f64, y: f64, z: f64, h: f64, p: f64, r: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self.z = Some(z);
        self.h = Some(h);
        self.p = Some(p);
        self.r = Some(r);
        self
    }
}

impl PositionBuilder for WorldPositionBuilder {
    fn validate(&self) -> BuilderResult<()> {
        // Validate that required coordinates are set
        let x = self.x.ok_or_else(|| BuilderError::validation_error(
            "X coordinate is required for world position",
            "Call x() to set the X coordinate"
        ))?;
        
        let y = self.y.ok_or_else(|| BuilderError::validation_error(
            "Y coordinate is required for world position",
            "Call y() to set the Y coordinate"
        ))?;
        
        let z = self.z.ok_or_else(|| BuilderError::validation_error(
            "Z coordinate is required for world position",
            "Call z() to set the Z coordinate"
        ))?;
        
        // Validate coordinate values
        validate_coordinate(x, "X")?;
        validate_coordinate(y, "Y")?;
        validate_coordinate(z, "Z")?;
        
        // Validate orientation angles
        if let Some(h) = self.h {
            validate_angle(h, "heading")?;
        }
        
        if let Some(p) = self.p {
            validate_angle(p, "pitch")?;
        }
        
        if let Some(r) = self.r {
            validate_angle(r, "roll")?;
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;
        
        let world_position = WorldPosition {
            x: Value::literal(self.x.unwrap()),
            y: Value::literal(self.y.unwrap()),
            z: Value::literal(self.z.unwrap()),
            h: Value::literal(self.h.unwrap_or(0.0)),
            p: Value::literal(self.p.unwrap_or(0.0)),
            r: Value::literal(self.r.unwrap_or(0.0)),
        };
        
        Ok(Position {
            world_position: Some(world_position),
            ..Position::empty()
        })
    }
}

impl Default for WorldPositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_world_position_builder_basic() {
        let position = WorldPositionBuilder::new()
            .coordinates(10.0, 20.0, 0.0)
            .finish()
            .unwrap();
        
        let world_pos = position.world_position.unwrap();
        assert_eq!(world_pos.x.as_literal().unwrap(), &10.0);
        assert_eq!(world_pos.y.as_literal().unwrap(), &20.0);
        assert_eq!(world_pos.z.as_literal().unwrap(), &0.0);
        assert_eq!(world_pos.h.as_literal().unwrap(), &0.0);
        assert_eq!(world_pos.p.as_literal().unwrap(), &0.0);
        assert_eq!(world_pos.r.as_literal().unwrap(), &0.0);
    }
    
    #[test]
    fn test_world_position_builder_with_orientation() {
        let position = WorldPositionBuilder::new()
            .position_and_orientation(1.0, 2.0, 3.0, 0.5, 0.1, 0.2)
            .finish()
            .unwrap();
        
        let world_pos = position.world_position.unwrap();
        assert_eq!(world_pos.x.as_literal().unwrap(), &1.0);
        assert_eq!(world_pos.y.as_literal().unwrap(), &2.0);
        assert_eq!(world_pos.z.as_literal().unwrap(), &3.0);
        assert_eq!(world_pos.h.as_literal().unwrap(), &0.5);
        assert_eq!(world_pos.p.as_literal().unwrap(), &0.1);
        assert_eq!(world_pos.r.as_literal().unwrap(), &0.2);
    }
    
    #[test]
    fn test_world_position_builder_fluent_api() {
        let position = WorldPositionBuilder::new()
            .x(5.0)
            .y(10.0)
            .z(1.5)
            .heading(1.57) // 90 degrees
            .finish()
            .unwrap();
        
        let world_pos = position.world_position.unwrap();
        assert_eq!(world_pos.x.as_literal().unwrap(), &5.0);
        assert_eq!(world_pos.y.as_literal().unwrap(), &10.0);
        assert_eq!(world_pos.z.as_literal().unwrap(), &1.5);
        assert_eq!(world_pos.h.as_literal().unwrap(), &1.57);
    }
    
    #[test]
    fn test_world_position_builder_validation_missing_coordinates() {
        let result = WorldPositionBuilder::new()
            .x(1.0)
            .y(2.0)
            // Missing z coordinate
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Z coordinate"));
    }
    
    #[test]
    fn test_world_position_builder_validation_invalid_coordinates() {
        let result = WorldPositionBuilder::new()
            .coordinates(f64::NAN, 2.0, 3.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("X coordinate"));
    }
    
    #[test]
    fn test_world_position_builder_validation_invalid_angles() {
        let result = WorldPositionBuilder::new()
            .coordinates(1.0, 2.0, 3.0)
            .heading(f64::INFINITY)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("heading"));
    }
    
    #[test]
    fn test_world_position_builder_default() {
        let builder = WorldPositionBuilder::default();
        assert!(builder.x.is_none());
        assert!(builder.y.is_none());
        assert!(builder.z.is_none());
        assert_eq!(builder.h, Some(0.0));
        assert_eq!(builder.p, Some(0.0));
        assert_eq!(builder.r, Some(0.0));
    }
}
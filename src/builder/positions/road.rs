//! Road position builder for programmatic road position construction

use crate::types::{
    basic::Value,
    positions::{Position, RoadPosition, Orientation},
};
use crate::builder::{BuilderError, BuilderResult};
use super::{PositionBuilder, validate_coordinate, validate_angle, validate_road_id};

/// Builder for creating road positions with fluent API
#[derive(Debug, Clone)]
pub struct RoadPositionBuilder {
    road_id: Option<String>,
    s: Option<f64>,
    t: Option<f64>,
    orientation: Option<Orientation>,
}

impl RoadPositionBuilder {
    /// Create a new road position builder
    pub fn new() -> Self {
        Self {
            road_id: None,
            s: None,
            t: None,
            orientation: None,
        }
    }
    
    /// Set the road ID
    pub fn road(mut self, road_id: &str) -> Self {
        self.road_id = Some(road_id.to_string());
        self
    }
    
    /// Set the s coordinate (along road)
    pub fn s(mut self, s: f64) -> Self {
        self.s = Some(s);
        self
    }
    
    /// Set the t coordinate (lateral offset)
    pub fn t(mut self, t: f64) -> Self {
        self.t = Some(t);
        self
    }
    
    /// Set both s and t coordinates
    pub fn coordinates(mut self, s: f64, t: f64) -> Self {
        self.s = Some(s);
        self.t = Some(t);
        self
    }
    
    /// Set the road, s, and t coordinates all at once
    pub fn road_coordinates(mut self, road_id: &str, s: f64, t: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.s = Some(s);
        self.t = Some(t);
        self
    }
    
    /// Set relative heading
    pub fn with_heading(mut self, h: f64) -> Self {
        self.orientation = Some(Orientation {
            h: Value::literal(h),
            p: Value::literal(0.0),
            r: Value::literal(0.0),
            type_: None,
        });
        self
    }
    
    /// Set full orientation (heading, pitch, roll)
    pub fn with_orientation(mut self, h: f64, p: f64, r: f64) -> Self {
        self.orientation = Some(Orientation {
            h: Value::literal(h),
            p: Value::literal(p),
            r: Value::literal(r),
            type_: None,
        });
        self
    }
    
    /// Set orientation type (relative or absolute)
    pub fn with_orientation_type(mut self, orientation_type: &str) -> Self {
        if let Some(ref mut orientation) = self.orientation {
            orientation.type_ = Some(Value::literal(orientation_type.to_string()));
        } else {
            self.orientation = Some(Orientation {
                h: Value::literal(0.0),
                p: Value::literal(0.0),
                r: Value::literal(0.0),
                type_: Some(Value::literal(orientation_type.to_string())),
            });
        }
        self
    }
}

impl PositionBuilder for RoadPositionBuilder {
    fn validate(&self) -> BuilderResult<()> {
        // Validate road ID
        let road_id = self.road_id.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Road ID is required for road position",
            "Call road() to set the road identifier"
        ))?;
        
        validate_road_id(road_id)?;
        
        // Validate s coordinate
        let s = self.s.ok_or_else(|| BuilderError::validation_error(
            "S coordinate is required for road position",
            "Call s() to set the s coordinate"
        ))?;
        
        validate_coordinate(s, "S")?;
        
        // S coordinate should be non-negative for most cases
        if s < 0.0 {
            return Err(BuilderError::validation_error(
                "S coordinate should be non-negative",
                "Provide a valid s coordinate >= 0"
            ));
        }
        
        // Validate t coordinate
        let t = self.t.ok_or_else(|| BuilderError::validation_error(
            "T coordinate is required for road position",
            "Call t() to set the t coordinate"
        ))?;
        
        validate_coordinate(t, "T")?;
        
        // Validate orientation if present
        if let Some(ref orientation) = self.orientation {
            if let Some(h) = orientation.h.as_literal() {
                validate_angle(*h, "heading")?;
            }
            if let Some(p) = orientation.p.as_literal() {
                validate_angle(*p, "pitch")?;
            }
            if let Some(r) = orientation.r.as_literal() {
                validate_angle(*r, "roll")?;
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;
        
        let road_position = RoadPosition {
            road_id: Value::literal(self.road_id.unwrap()),
            s: Value::literal(self.s.unwrap()),
            t: Value::literal(self.t.unwrap()),
            orientation: self.orientation,
        };
        
        Ok(Position {
            road_position: Some(road_position),
            ..Position::empty()
        })
    }
}

impl Default for RoadPositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_road_position_builder_basic() {
        let position = RoadPositionBuilder::new()
            .road("highway_1")
            .coordinates(100.0, 2.5)
            .finish()
            .unwrap();
        
        let road_pos = position.road_position.unwrap();
        assert_eq!(road_pos.road_id.as_literal().unwrap(), "highway_1");
        assert_eq!(road_pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(road_pos.t.as_literal().unwrap(), &2.5);
        assert!(road_pos.orientation.is_none());
    }
    
    #[test]
    fn test_road_position_builder_with_heading() {
        let position = RoadPositionBuilder::new()
            .road_coordinates("test_road", 50.0, -1.0)
            .with_heading(0.5)
            .finish()
            .unwrap();
        
        let road_pos = position.road_position.unwrap();
        assert_eq!(road_pos.road_id.as_literal().unwrap(), "test_road");
        assert_eq!(road_pos.s.as_literal().unwrap(), &50.0);
        assert_eq!(road_pos.t.as_literal().unwrap(), &-1.0);
        
        let orientation = road_pos.orientation.unwrap();
        assert_eq!(orientation.h.as_literal().unwrap(), &0.5);
        assert_eq!(orientation.p.as_literal().unwrap(), &0.0);
        assert_eq!(orientation.r.as_literal().unwrap(), &0.0);
    }
    
    #[test]
    fn test_road_position_builder_with_full_orientation() {
        let position = RoadPositionBuilder::new()
            .road("complex_road")
            .s(200.0)
            .t(0.0)
            .with_orientation(1.57, 0.1, 0.05)
            .with_orientation_type("relative")
            .finish()
            .unwrap();
        
        let road_pos = position.road_position.unwrap();
        let orientation = road_pos.orientation.unwrap();
        assert_eq!(orientation.h.as_literal().unwrap(), &1.57);
        assert_eq!(orientation.p.as_literal().unwrap(), &0.1);
        assert_eq!(orientation.r.as_literal().unwrap(), &0.05);
        assert_eq!(orientation.type_.as_ref().unwrap().as_literal().unwrap(), "relative");
    }
    
    #[test]
    fn test_road_position_builder_fluent_api() {
        let position = RoadPositionBuilder::new()
            .road("fluent_road")
            .s(75.0)
            .t(1.5)
            .with_heading(0.2)
            .finish()
            .unwrap();
        
        let road_pos = position.road_position.unwrap();
        assert_eq!(road_pos.road_id.as_literal().unwrap(), "fluent_road");
        assert_eq!(road_pos.s.as_literal().unwrap(), &75.0);
        assert_eq!(road_pos.t.as_literal().unwrap(), &1.5);
        assert!(road_pos.orientation.is_some());
    }
    
    #[test]
    fn test_road_position_builder_validation_missing_road() {
        let result = RoadPositionBuilder::new()
            .coordinates(100.0, 0.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Road ID"));
    }
    
    #[test]
    fn test_road_position_builder_validation_empty_road() {
        let result = RoadPositionBuilder::new()
            .road("")
            .coordinates(100.0, 0.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Road ID"));
    }
    
    #[test]
    fn test_road_position_builder_validation_missing_coordinates() {
        let result = RoadPositionBuilder::new()
            .road("test_road")
            .s(100.0)
            // Missing t coordinate
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("T coordinate"));
    }
    
    #[test]
    fn test_road_position_builder_validation_negative_s() {
        let result = RoadPositionBuilder::new()
            .road("test_road")
            .coordinates(-10.0, 0.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate should be non-negative"));
    }
    
    #[test]
    fn test_road_position_builder_validation_invalid_coordinates() {
        let result = RoadPositionBuilder::new()
            .road("test_road")
            .coordinates(f64::INFINITY, 0.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate"));
    }
    
    #[test]
    fn test_road_position_builder_validation_invalid_angles() {
        let result = RoadPositionBuilder::new()
            .road("test_road")
            .coordinates(100.0, 0.0)
            .with_heading(f64::NAN)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("heading"));
    }
}
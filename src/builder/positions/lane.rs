//! Lane position builder for programmatic lane position construction

use crate::types::{
    basic::Value,
    positions::{Position, LanePosition},
    geometry::shapes::Orientation,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{PositionBuilder, validate_coordinate, validate_angle, validate_road_id};

/// Builder for creating lane positions with fluent API
#[derive(Debug, Clone)]
pub struct LanePositionBuilder {
    road_id: Option<String>,
    lane_id: Option<i32>,
    s: Option<f64>,
    offset: Option<f64>,
    orientation: Option<Orientation>,
}

impl LanePositionBuilder {
    /// Create a new lane position builder
    pub fn new() -> Self {
        Self {
            road_id: None,
            lane_id: None,
            s: None,
            offset: Some(0.0), // Default offset
            orientation: None,
        }
    }
    
    /// Set the road ID
    pub fn road(mut self, road_id: &str) -> Self {
        self.road_id = Some(road_id.to_string());
        self
    }
    
    /// Set the lane ID
    pub fn lane(mut self, lane_id: i32) -> Self {
        self.lane_id = Some(lane_id);
        self
    }
    
    /// Set the s coordinate (along lane)
    pub fn s(mut self, s: f64) -> Self {
        self.s = Some(s);
        self
    }
    
    /// Set the lateral offset from lane center
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// Set road, lane, and s coordinates all at once
    pub fn road_lane_s(mut self, road_id: &str, lane_id: i32, s: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.lane_id = Some(lane_id);
        self.s = Some(s);
        self
    }
    
    /// Set all coordinates including offset
    pub fn coordinates(mut self, road_id: &str, lane_id: i32, s: f64, offset: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.lane_id = Some(lane_id);
        self.s = Some(s);
        self.offset = Some(offset);
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
    
    /// Convenience method for right lane (positive lane ID)
    pub fn right_lane(mut self, road_id: &str, lane_number: u32, s: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.lane_id = Some(lane_number as i32);
        self.s = Some(s);
        self
    }
    
    /// Convenience method for left lane (negative lane ID)
    pub fn left_lane(mut self, road_id: &str, lane_number: u32, s: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.lane_id = Some(-(lane_number as i32));
        self.s = Some(s);
        self
    }
}

impl PositionBuilder for LanePositionBuilder {
    fn validate(&self) -> BuilderResult<()> {
        // Validate road ID
        let road_id = self.road_id.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Road ID is required for lane position",
            "Call road() to set the road identifier"
        ))?;
        
        validate_road_id(road_id)?;
        
        // Validate lane ID
        let lane_id = self.lane_id.ok_or_else(|| BuilderError::validation_error(
            "Lane ID is required for lane position",
            "Call lane() to set the lane identifier"
        ))?;
        
        // Lane ID should not be zero (OpenDRIVE convention)
        if lane_id == 0 {
            return Err(BuilderError::validation_error(
                "Lane ID cannot be zero",
                "Use positive IDs for right lanes, negative for left lanes"
            ));
        }
        
        // Validate s coordinate
        let s = self.s.ok_or_else(|| BuilderError::validation_error(
            "S coordinate is required for lane position",
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
        
        // Validate offset
        if let Some(offset) = self.offset {
            validate_coordinate(offset, "offset")?;
        }
        
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
        
        let lane_position = LanePosition {
            road_id: Value::literal(self.road_id.unwrap()),
            lane_id: Value::literal(self.lane_id.unwrap()),
            s: Value::literal(self.s.unwrap()),
            offset: Value::literal(self.offset.unwrap_or(0.0)),
            orientation: self.orientation,
        };
        
        Ok(Position {
            lane_position: Some(lane_position),
            ..Position::empty()
        })
    }
}

impl Default for LanePositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lane_position_builder_basic() {
        let position = LanePositionBuilder::new()
            .road("highway_1")
            .lane(1)
            .s(100.0)
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal().unwrap(), "highway_1");
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), &1);
        assert_eq!(lane_pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(lane_pos.offset.as_literal().unwrap(), &0.0);
        assert!(lane_pos.orientation.is_none());
    }
    
    #[test]
    fn test_lane_position_builder_with_offset() {
        let position = LanePositionBuilder::new()
            .coordinates("test_road", -1, 50.0, 1.5)
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal().unwrap(), "test_road");
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), &-1);
        assert_eq!(lane_pos.s.as_literal().unwrap(), &50.0);
        assert_eq!(lane_pos.offset.as_literal().unwrap(), &1.5);
    }
    
    #[test]
    fn test_lane_position_builder_with_heading() {
        let position = LanePositionBuilder::new()
            .road_lane_s("curved_road", 2, 200.0)
            .with_heading(0.3)
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal().unwrap(), "curved_road");
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), &2);
        assert_eq!(lane_pos.s.as_literal().unwrap(), &200.0);
        
        let orientation = lane_pos.orientation.unwrap();
        assert_eq!(orientation.h.as_literal().unwrap(), &0.3);
        assert_eq!(orientation.p.as_literal().unwrap(), &0.0);
        assert_eq!(orientation.r.as_literal().unwrap(), &0.0);
    }
    
    #[test]
    fn test_lane_position_builder_convenience_methods() {
        // Test right lane
        let position = LanePositionBuilder::new()
            .right_lane("highway", 1, 150.0)
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), &1);
        
        // Test left lane
        let position = LanePositionBuilder::new()
            .left_lane("highway", 2, 150.0)
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), &-2);
    }
    
    #[test]
    fn test_lane_position_builder_with_full_orientation() {
        let position = LanePositionBuilder::new()
            .road("complex_road")
            .lane(-1)
            .s(75.0)
            .offset(-0.5)
            .with_orientation(1.57, 0.1, 0.05)
            .with_orientation_type("relative")
            .finish()
            .unwrap();
        
        let lane_pos = position.lane_position.unwrap();
        let orientation = lane_pos.orientation.unwrap();
        assert_eq!(orientation.h.as_literal().unwrap(), &1.57);
        assert_eq!(orientation.p.as_literal().unwrap(), &0.1);
        assert_eq!(orientation.r.as_literal().unwrap(), &0.05);
        assert_eq!(orientation.type_.as_ref().unwrap().as_literal().unwrap(), "relative");
    }
    
    #[test]
    fn test_lane_position_builder_validation_missing_road() {
        let result = LanePositionBuilder::new()
            .lane(1)
            .s(100.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Road ID"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_missing_lane() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .s(100.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Lane ID"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_zero_lane() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .lane(0)
            .s(100.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Lane ID cannot be zero"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_missing_s() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .lane(1)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_negative_s() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .lane(1)
            .s(-10.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate should be non-negative"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_invalid_coordinates() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .lane(1)
            .s(f64::NAN)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate"));
    }
    
    #[test]
    fn test_lane_position_builder_validation_invalid_offset() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .lane(1)
            .s(100.0)
            .offset(f64::INFINITY)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("offset"));
    }
}
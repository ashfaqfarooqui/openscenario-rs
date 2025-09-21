//! Road and lane position builders for road-relative positioning

use crate::types::{
    basic::{Value, OSString},
    positions::{Position, LanePosition, RoadPosition, Orientation},
};
use super::{PositionReceiver, default_orientation, validate_lane_id, validate_road_id};
use crate::builder::{
    error::BuilderResult,
    states::BuilderState,
};
use std::marker::PhantomData;

/// Builder for lane positions using road-relative coordinates
/// 
/// Lane positions define entity placement relative to a specific lane
/// on a road, using the lane's coordinate system.
pub struct LanePositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Road ID
    road_id: String,
    
    /// Lane ID (negative for right lanes, positive for left)
    lane_id: i32,
    
    /// Distance along the lane in meters
    s: f64,
    
    /// Lateral offset from lane center in meters
    offset: f64,
    
    /// Orientation (heading, pitch, roll)
    orientation: Orientation,
}

impl<S: BuilderState, T> LanePositionBuilder<S, T> {
    /// Create a new lane position builder
    pub fn new(parent: T, road_id: &str, lane_id: i32, s: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            road_id: road_id.to_string(),
            lane_id,
            s,
            offset: 0.0,
            orientation: default_orientation(),
        }
    }

    /// Set lateral offset from lane center
    /// 
    /// # Arguments
    /// * `offset` - Lateral offset in meters (positive = left, negative = right)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_offset(mut self, offset: f64) -> Self {
        self.offset = offset;
        self
    }

    /// Set the heading relative to lane direction
    /// 
    /// # Arguments
    /// * `heading` - Heading angle in radians relative to lane direction
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_heading(mut self, heading: f64) -> Self {
        self.orientation.h = Value::literal(heading);
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

    /// Complete the lane position and return to parent builder
    /// 
    /// This method validates the lane parameters and creates the final position.
    /// 
    /// # Returns
    /// Parent builder with lane position set
    /// 
    /// # Errors
    /// Returns error if road ID is empty or lane ID is zero
    pub fn finish(self) -> BuilderResult<T>
    where
        T: PositionReceiver<S>,
    {
        // Validate parameters
        validate_road_id(&self.road_id)?;
        validate_lane_id(self.lane_id)?;

        // Create lane position
        let lane_position = LanePosition {
            road_id: Value::literal(self.road_id),
            lane_id: Value::literal(self.lane_id),
            s: Value::literal(self.s),
            offset: Value::literal(self.offset),
            orientation: Some(self.orientation),
        };

        // Create position container
        let position = Position {
            lane_position: Some(lane_position),
            ..Position::empty()
        };

        Ok(self.parent.set_position(position))
    }
}

/// Builder for road positions using road-relative coordinates
/// 
/// Road positions define entity placement relative to a road's reference line
/// using the road coordinate system.
pub struct RoadPositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Road ID
    road_id: String,
    
    /// Distance along the road in meters
    s: f64,
    
    /// Lateral offset from road center in meters
    t: f64,
    
    /// Orientation (heading, pitch, roll)
    orientation: Orientation,
}

impl<S: BuilderState, T> RoadPositionBuilder<S, T> {
    /// Create a new road position builder
    pub fn new(parent: T, road_id: &str, s: f64, t: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            road_id: road_id.to_string(),
            s,
            t,
            orientation: default_orientation(),
        }
    }

    /// Set the heading relative to road direction
    /// 
    /// # Arguments
    /// * `heading` - Heading angle in radians relative to road direction
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_heading(mut self, heading: f64) -> Self {
        self.orientation.h = Value::literal(heading);
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

    /// Complete the road position and return to parent builder
    /// 
    /// This method validates the road parameters and creates the final position.
    /// 
    /// # Returns
    /// Parent builder with road position set
    /// 
    /// # Errors
    /// Returns error if road ID is empty
    pub fn finish(self) -> BuilderResult<T>
    where
        T: PositionReceiver<S>,
    {
        // Validate parameters
        validate_road_id(&self.road_id)?;

        // Create road position
        let road_position = RoadPosition {
            road_id: Value::literal(self.road_id),
            s: Value::literal(self.s),
            t: Value::literal(self.t),
            orientation: Some(self.orientation),
        };

        // Create position container
        let position = Position {
            road_position: Some(road_position),
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
    fn test_lane_position_builder() {
        let parent = MockParent { position: None };
        let builder = LanePositionBuilder::new(parent, "road1", -1, 100.0);
        
        let result = builder
            .with_offset(1.5)
            .with_heading(0.1)
            .finish()
            .unwrap();
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.lane_position.is_some());
        
        let lane_pos = pos.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal(), Some(&"road1".to_string()));
        assert_eq!(lane_pos.lane_id.as_literal(), Some(&-1));
        assert_eq!(lane_pos.s.as_literal(), Some(&100.0));
        assert_eq!(lane_pos.offset.as_literal(), Some(&1.5));
    }

    #[test]
    fn test_road_position_builder() {
        let parent = MockParent { position: None };
        let builder = RoadPositionBuilder::new(parent, "road1", 50.0, 2.0);
        
        let result = builder.with_heading(1.57).finish().unwrap();
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.road_position.is_some());
        
        let road_pos = pos.road_position.unwrap();
        assert_eq!(road_pos.road_id.as_literal(), Some(&"road1".to_string()));
        assert_eq!(road_pos.s.as_literal(), Some(&50.0));
        assert_eq!(road_pos.t.as_literal(), Some(&2.0));
    }

    #[test]
    fn test_lane_position_validation() {
        let parent = MockParent { position: None };
        
        // Test invalid lane ID (zero)
        let builder = LanePositionBuilder::new(parent, "road1", 0, 100.0);
        let result = builder.finish();
        assert!(result.is_err());
    }

    #[test]
    fn test_road_position_validation() {
        let parent = MockParent { position: None };
        
        // Test invalid road ID (empty)
        let builder = RoadPositionBuilder::new(parent, "", 50.0, 2.0);
        let result = builder.finish();
        assert!(result.is_err());
    }
}
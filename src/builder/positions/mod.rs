//! Position builders for type-safe position construction
//!
//! This module provides builders for constructing OpenSCENARIO positions
//! with compile-time safety and validation. Supports all position types
//! including world, road, lane, and relative positions.

use crate::types::{
    basic::{Value, Double, OSString},
    positions::{
        Position, WorldPosition, LanePosition, RoadPosition,
        RelativeWorldPosition, RelativeLanePosition, RelativeRoadPosition,
        RelativeObjectPosition, GeographicPosition, TrajectoryPosition,
        Orientation,
    },
};
use crate::builder::{
    error::{BuilderError, BuilderResult},
    states::BuilderState,
};
use std::marker::PhantomData;

pub mod world;
pub mod road;
pub mod relative;

pub use world::WorldPositionBuilder;
pub use road::{LanePositionBuilder, RoadPositionBuilder};
pub use relative::{RelativePositionBuilder, RelativeObjectPositionBuilder};

/// Position builder state tracking
pub trait PositionBuilderState {}

/// Initial position builder state
pub struct PositionEmpty;

/// State after position type is selected
pub struct PositionTypeSelected;

/// State when position is configured and ready
pub struct PositionReady;

impl PositionBuilderState for PositionEmpty {}
impl PositionBuilderState for PositionTypeSelected {}
impl PositionBuilderState for PositionReady {}

/// Main position builder for constructing positions
/// 
/// This builder provides a fluent interface for constructing different types
/// of positions with proper validation and type safety.
pub struct PositionBuilder<S: BuilderState, T> {
    /// Type state phantom data for scenario builder
    _scenario_state: PhantomData<S>,
    
    /// The parent builder that will receive the position
    parent: T,
}

impl<S: BuilderState, T> PositionBuilder<S, T> {
    /// Create a new position builder
    pub fn new(parent: T) -> Self {
        Self {
            _scenario_state: PhantomData,
            parent,
        }
    }

    /// Create a world position (absolute coordinates)
    /// 
    /// World positions use absolute X, Y, Z coordinates in the scenario
    /// coordinate system.
    /// 
    /// # Arguments
    /// * `x` - X coordinate in meters
    /// * `y` - Y coordinate in meters
    /// * `z` - Z coordinate in meters (optional, defaults to 0.0)
    /// 
    /// # Returns
    /// WorldPositionBuilder for further configuration
    pub fn world(self, x: f64, y: f64, z: Option<f64>) -> WorldPositionBuilder<S, T> {
        WorldPositionBuilder::new(self.parent, x, y, z.unwrap_or(0.0))
    }

    /// Create a lane position (road-relative coordinates)
    /// 
    /// Lane positions are relative to a specific lane on a road, using
    /// the lane's coordinate system.
    /// 
    /// # Arguments
    /// * `road_id` - ID of the road
    /// * `lane_id` - ID of the lane (negative for right lanes, positive for left)
    /// * `s` - Distance along the lane in meters
    /// 
    /// # Returns
    /// LanePositionBuilder for further configuration
    pub fn lane(self, road_id: &str, lane_id: i32, s: f64) -> LanePositionBuilder<S, T> {
        LanePositionBuilder::new(self.parent, road_id, lane_id, s)
    }

    /// Create a road position (road-relative coordinates)
    /// 
    /// Road positions are relative to a road's reference line using
    /// the road coordinate system.
    /// 
    /// # Arguments
    /// * `road_id` - ID of the road
    /// * `s` - Distance along the road in meters
    /// * `t` - Lateral offset from road center in meters
    /// 
    /// # Returns
    /// RoadPositionBuilder for further configuration
    pub fn road(self, road_id: &str, s: f64, t: f64) -> RoadPositionBuilder<S, T> {
        RoadPositionBuilder::new(self.parent, road_id, s, t)
    }

    /// Create a relative position to another entity
    /// 
    /// Relative positions are defined relative to another entity's position
    /// and orientation.
    /// 
    /// # Arguments
    /// * `entity_ref` - Name of the reference entity
    /// 
    /// # Returns
    /// RelativePositionBuilder for further configuration
    pub fn relative_to(self, entity_ref: &str) -> RelativePositionBuilder<S, T> {
        RelativePositionBuilder::new(self.parent, entity_ref)
    }

    /// Create a geographic position (GPS coordinates)
    /// 
    /// Geographic positions use latitude, longitude, and altitude
    /// in the WGS84 coordinate system.
    /// 
    /// # Arguments
    /// * `latitude` - Latitude in degrees
    /// * `longitude` - Longitude in degrees
    /// * `height` - Height above sea level in meters (optional)
    /// 
    /// # Returns
    /// Parent builder with geographic position set
    pub fn geographic(self, latitude: f64, longitude: f64, height: Option<f64>) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            geographic_position: Some(GeographicPosition {
                latitude: Value::literal(latitude),
                longitude: Value::literal(longitude),
                height: height.map(|h| Value::literal(h)),
                orientation: None,
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }
}

/// Trait for builders that can receive positions
/// 
/// This trait allows position builders to work with different types of
/// parent builders (vehicle, pedestrian, etc.) by providing a common
/// interface for setting positions.
pub trait PositionReceiver<S: BuilderState> {
    /// Set the position and return the updated builder
    fn set_position(self, position: Position) -> Self;
}

/// Helper function to create default orientation
pub fn default_orientation() -> Orientation {
    Orientation {
        h: Some(Value::literal(0.0)),
        p: Some(Value::literal(0.0)),
        r: Some(Value::literal(0.0)),
    }
}

/// Helper function to validate position coordinates
pub fn validate_coordinates(x: f64, y: f64, z: f64) -> BuilderResult<()> {
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err(BuilderError::validation_error("Position coordinates must be finite"));
    }
    Ok(())
}

/// Helper function to validate lane ID
pub fn validate_lane_id(lane_id: i32) -> BuilderResult<()> {
    if lane_id == 0 {
        return Err(BuilderError::validation_error("Lane ID cannot be zero"));
    }
    Ok(())
}

/// Helper function to validate road ID
pub fn validate_road_id(road_id: &str) -> BuilderResult<()> {
    if road_id.is_empty() {
        return Err(BuilderError::validation_error("Road ID cannot be empty"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::states::HasRoadNetwork;

    // Mock parent builder for testing
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
    fn test_position_builder_creation() {
        let parent = MockParent { position: None };
        let _builder: PositionBuilder<HasRoadNetwork, MockParent> = PositionBuilder::new(parent);
    }

    #[test]
    fn test_geographic_position() {
        let parent = MockParent { position: None };
        let builder = PositionBuilder::new(parent);
        
        let result = builder.geographic(52.5, 13.4, Some(100.0));
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.geographic_position.is_some());
        
        let geo_pos = pos.geographic_position.unwrap();
        assert_eq!(geo_pos.latitude.as_literal(), Some(&52.5));
        assert_eq!(geo_pos.longitude.as_literal(), Some(&13.4));
        assert_eq!(geo_pos.height.as_ref().and_then(|h| h.as_literal()), Some(&100.0));
    }

    #[test]
    fn test_coordinate_validation() {
        assert!(validate_coordinates(1.0, 2.0, 3.0).is_ok());
        assert!(validate_coordinates(f64::INFINITY, 2.0, 3.0).is_err());
        assert!(validate_coordinates(1.0, f64::NAN, 3.0).is_err());
    }

    #[test]
    fn test_lane_id_validation() {
        assert!(validate_lane_id(1).is_ok());
        assert!(validate_lane_id(-1).is_ok());
        assert!(validate_lane_id(0).is_err());
    }

    #[test]
    fn test_road_id_validation() {
        assert!(validate_road_id("road1").is_ok());
        assert!(validate_road_id("").is_err());
    }
}
//! Relative position builders for entity-relative positioning

use crate::types::{
    basic::{Value, OSString},
    positions::{Position, RelativeWorldPosition, RelativeLanePosition, RelativeRoadPosition, RelativeObjectPosition, Orientation},
    enums::ReferenceContext,
};
use super::{PositionReceiver, default_orientation};
use crate::builder::{
    error::{BuilderError, BuilderResult},
    states::BuilderState,
};
use std::marker::PhantomData;

/// Builder for relative positions to other entities
/// 
/// Relative positions define entity placement relative to another entity's
/// position and orientation, useful for convoy scenarios and formations.
pub struct RelativePositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Reference entity name
    entity_ref: String,
}

impl<S: BuilderState, T> RelativePositionBuilder<S, T> {
    /// Create a new relative position builder
    pub fn new(parent: T, entity_ref: &str) -> Self {
        Self {
            _state: PhantomData,
            parent,
            entity_ref: entity_ref.to_string(),
        }
    }

    /// Position ahead of the reference entity
    /// 
    /// # Arguments
    /// * `distance` - Distance ahead in meters (positive = ahead)
    /// 
    /// # Returns
    /// Parent builder with relative position set
    pub fn ahead(self, distance: f64) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_world_position: Some(RelativeWorldPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(distance),
                dy: Value::literal(0.0),
                dz: Value::literal(0.0),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }

    /// Position behind the reference entity
    /// 
    /// # Arguments
    /// * `distance` - Distance behind in meters (positive = behind)
    /// 
    /// # Returns
    /// Parent builder with relative position set
    pub fn behind(self, distance: f64) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_world_position: Some(RelativeWorldPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(-distance),
                dy: Value::literal(0.0),
                dz: Value::literal(0.0),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }

    /// Position to the left of the reference entity
    /// 
    /// # Arguments
    /// * `distance` - Distance to the left in meters (positive = left)
    /// 
    /// # Returns
    /// Parent builder with relative position set
    pub fn left_of(self, distance: f64) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_world_position: Some(RelativeWorldPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(0.0),
                dy: Value::literal(distance),
                dz: Value::literal(0.0),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }

    /// Position to the right of the reference entity
    /// 
    /// # Arguments
    /// * `distance` - Distance to the right in meters (positive = right)
    /// 
    /// # Returns
    /// Parent builder with relative position set
    pub fn right_of(self, distance: f64) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_world_position: Some(RelativeWorldPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(0.0),
                dy: Value::literal(-distance),
                dz: Value::literal(0.0),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }

    /// Position with custom offset from the reference entity
    /// 
    /// # Arguments
    /// * `dx` - Longitudinal offset in meters (positive = ahead)
    /// * `dy` - Lateral offset in meters (positive = left)
    /// * `dz` - Vertical offset in meters (positive = up)
    /// 
    /// # Returns
    /// Parent builder with relative position set
    pub fn with_offset(self, dx: f64, dy: f64, dz: f64) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_world_position: Some(RelativeWorldPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(dx),
                dy: Value::literal(dy),
                dz: Value::literal(dz),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }

    /// Create a relative lane position builder
    /// 
    /// This allows positioning relative to the reference entity's lane position.
    /// 
    /// # Arguments
    /// * `ds` - Longitudinal offset along the lane in meters
    /// * `dt` - Lateral offset from the lane in meters
    /// 
    /// # Returns
    /// RelativeLanePositionBuilder for further configuration
    pub fn in_lane_with_offset(self, ds: f64, dt: f64) -> RelativeLanePositionBuilder<S, T> {
        RelativeLanePositionBuilder::new(self.parent, &self.entity_ref, ds, dt)
    }

    /// Create a relative road position builder
    /// 
    /// This allows positioning relative to the reference entity's road position.
    /// 
    /// # Arguments
    /// * `ds` - Longitudinal offset along the road in meters
    /// * `dt` - Lateral offset from the road in meters
    /// 
    /// # Returns
    /// RelativeRoadPositionBuilder for further configuration
    pub fn on_road_with_offset(self, ds: f64, dt: f64) -> RelativeRoadPositionBuilder<S, T> {
        RelativeRoadPositionBuilder::new(self.parent, &self.entity_ref, ds, dt)
    }
}

/// Builder for relative lane positions
pub struct RelativeLanePositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Reference entity name
    entity_ref: String,
    
    /// Longitudinal offset in meters
    ds: f64,
    
    /// Lateral offset in meters
    dt: f64,
    
    /// Lane offset (number of lanes)
    dlane_id: i32,
    
    /// Orientation
    orientation: Orientation,
}

impl<S: BuilderState, T> RelativeLanePositionBuilder<S, T> {
    /// Create a new relative lane position builder
    pub fn new(parent: T, entity_ref: &str, ds: f64, dt: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            entity_ref: entity_ref.to_string(),
            ds,
            dt,
            dlane_id: 0,
            orientation: default_orientation(),
        }
    }

    /// Set lane offset (number of lanes to offset)
    /// 
    /// # Arguments
    /// * `dlane_id` - Lane offset (positive = left, negative = right)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_lane_offset(mut self, dlane_id: i32) -> Self {
        self.dlane_id = dlane_id;
        self
    }

    /// Set orientation
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

    /// Complete the relative lane position
    /// 
    /// # Returns
    /// Parent builder with relative lane position set
    pub fn finish(self) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_lane_position: Some(RelativeLanePosition {
                entity_ref: Value::literal(self.entity_ref),
                ds: Value::literal(self.ds),
                d_lane: Value::literal(self.dlane_id),
                offset: Value::literal(self.dt),
                orientation: Some(self.orientation),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }
}

/// Builder for relative road positions
pub struct RelativeRoadPositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Reference entity name
    entity_ref: String,
    
    /// Longitudinal offset in meters
    ds: f64,
    
    /// Lateral offset in meters
    dt: f64,
    
    /// Orientation
    orientation: Orientation,
}

impl<S: BuilderState, T> RelativeRoadPositionBuilder<S, T> {
    /// Create a new relative road position builder
    pub fn new(parent: T, entity_ref: &str, ds: f64, dt: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            entity_ref: entity_ref.to_string(),
            ds,
            dt,
            orientation: default_orientation(),
        }
    }

    /// Set orientation
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

    /// Complete the relative road position
    /// 
    /// # Returns
    /// Parent builder with relative road position set
    pub fn finish(self) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_road_position: Some(RelativeRoadPosition {
                entity_ref: Value::literal(self.entity_ref),
                ds: Value::literal(self.ds),
                dt: Value::literal(self.dt),
                orientation: Some(self.orientation),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
    }
}

/// Builder for relative object positions with advanced features
pub struct RelativeObjectPositionBuilder<S: BuilderState, T> {
    /// Type state phantom data
    _state: PhantomData<S>,
    
    /// Parent builder that will receive the position
    parent: T,
    
    /// Reference entity name
    entity_ref: String,
    
    /// Longitudinal offset in meters
    dx: f64,
    
    /// Lateral offset in meters
    dy: f64,
    
    /// Vertical offset in meters
    dz: f64,
    
    /// Orientation
    orientation: Orientation,
}

impl<S: BuilderState, T> RelativeObjectPositionBuilder<S, T> {
    /// Create a new relative object position builder
    pub fn new(parent: T, entity_ref: &str, dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            _state: PhantomData,
            parent,
            entity_ref: entity_ref.to_string(),
            dx,
            dy,
            dz,
            orientation: default_orientation(),
        }
    }

    /// Set orientation
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

    /// Complete the relative object position
    /// 
    /// # Returns
    /// Parent builder with relative object position set
    pub fn finish(self) -> T
    where
        T: PositionReceiver<S>,
    {
        let position = Position {
            relative_object_position: Some(RelativeObjectPosition {
                entity_ref: Value::literal(self.entity_ref),
                dx: Value::literal(self.dx),
                dy: Value::literal(self.dy),
                dz: Value::literal(self.dz),
                orientation: Some(self.orientation),
            }),
            ..Position::empty()
        };

        self.parent.set_position(position)
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
    fn test_relative_position_ahead() {
        let parent = MockParent { position: None };
        let builder = RelativePositionBuilder::new(parent, "ego");
        
        let result = builder.ahead(50.0);
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.relative_world_position.is_some());
        
        let rel_pos = pos.relative_world_position.unwrap();
        assert_eq!(rel_pos.entity_ref.as_literal(), Some(&"ego".to_string()));
        assert_eq!(rel_pos.dx.as_literal(), Some(&50.0));
        assert_eq!(rel_pos.dy.as_literal(), Some(&0.0));
    }

    #[test]
    fn test_relative_position_behind() {
        let parent = MockParent { position: None };
        let builder = RelativePositionBuilder::new(parent, "ego");
        
        let result = builder.behind(30.0);
        
        let rel_pos = result.position.unwrap().relative_world_position.unwrap();
        assert_eq!(rel_pos.dx.as_literal(), Some(&-30.0));
    }

    #[test]
    fn test_relative_position_left() {
        let parent = MockParent { position: None };
        let builder = RelativePositionBuilder::new(parent, "ego");
        
        let result = builder.left_of(3.5);
        
        let rel_pos = result.position.unwrap().relative_world_position.unwrap();
        assert_eq!(rel_pos.dy.as_literal(), Some(&3.5));
    }

    #[test]
    fn test_relative_position_custom_offset() {
        let parent = MockParent { position: None };
        let builder = RelativePositionBuilder::new(parent, "ego");
        
        let result = builder.with_offset(10.0, -2.0, 1.0);
        
        let rel_pos = result.position.unwrap().relative_world_position.unwrap();
        assert_eq!(rel_pos.dx.as_literal(), Some(&10.0));
        assert_eq!(rel_pos.dy.as_literal(), Some(&-2.0));
        assert_eq!(rel_pos.dz.as_literal(), Some(&1.0));
    }

    #[test]
    fn test_relative_lane_position() {
        let parent = MockParent { position: None };
        let builder = RelativePositionBuilder::new(parent, "ego");
        
        let result = builder
            .in_lane_with_offset(20.0, 1.5)
            .with_lane_offset(-1)
            .finish();
        
        assert!(result.position.is_some());
        let pos = result.position.unwrap();
        assert!(pos.relative_lane_position.is_some());
        
        let rel_lane_pos = pos.relative_lane_position.unwrap();
        assert_eq!(rel_lane_pos.ds.as_literal(), Some(&20.0));
        assert_eq!(rel_lane_pos.dt.as_literal(), Some(&1.5));
        assert_eq!(rel_lane_pos.dlane_id.as_literal(), Some(&-1));
    }
}
//! Relative position builder for programmatic relative position construction

use crate::types::{
    basic::Value,
    positions::{Position, RelativeWorldPosition, RelativeLanePosition, RelativeRoadPosition, Orientation},
};
use crate::builder::{BuilderError, BuilderResult};
use super::{PositionBuilder, validate_coordinate, validate_angle, validate_entity_ref};

/// Builder for creating relative positions with fluent API
#[derive(Debug, Clone)]
pub struct RelativePositionBuilder {
    entity_ref: Option<String>,
    position_type: RelativePositionType,
}

/// Types of relative positions
#[derive(Debug, Clone)]
enum RelativePositionType {
    World {
        dx: Option<f64>,
        dy: Option<f64>,
        dz: Option<f64>,
        orientation: Option<Orientation>,
    },
    Lane {
        ds: Option<f64>,
        dt: Option<f64>,
        orientation: Option<Orientation>,
    },
    Road {
        ds: Option<f64>,
        dt: Option<f64>,
        orientation: Option<Orientation>,
    },
}

impl RelativePositionBuilder {
    /// Create a new relative position builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            position_type: RelativePositionType::World {
                dx: None,
                dy: None,
                dz: None,
                orientation: None,
            },
        }
    }
    
    /// Set the entity reference
    pub fn to_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Configure as relative world position
    pub fn world_offset(mut self, dx: f64, dy: f64, dz: f64) -> Self {
        self.position_type = RelativePositionType::World {
            dx: Some(dx),
            dy: Some(dy),
            dz: Some(dz),
            orientation: None,
        };
        self
    }
    
    /// Configure as relative lane position
    pub fn lane_offset(mut self, ds: f64, dt: f64) -> Self {
        self.position_type = RelativePositionType::Lane {
            ds: Some(ds),
            dt: Some(dt),
            orientation: None,
        };
        self
    }
    
    /// Configure as relative road position
    pub fn road_offset(mut self, ds: f64, dt: f64) -> Self {
        self.position_type = RelativePositionType::Road {
            ds: Some(ds),
            dt: Some(dt),
            orientation: None,
        };
        self
    }
    
    /// Set relative orientation for world position
    pub fn with_world_orientation(mut self, dh: f64, dp: f64, dr: f64) -> Self {
        if let RelativePositionType::World { ref mut orientation, .. } = self.position_type {
            *orientation = Some(Orientation {
                h: Value::literal(dh),
                p: Value::literal(dp),
                r: Value::literal(dr),
                type_: None,
            });
        }
        self
    }
    
    /// Set relative orientation for lane position
    pub fn with_lane_orientation(mut self, dh: f64) -> Self {
        if let RelativePositionType::Lane { ref mut orientation, .. } = self.position_type {
            *orientation = Some(Orientation {
                h: Value::literal(dh),
                p: Value::literal(0.0),
                r: Value::literal(0.0),
                type_: None,
            });
        }
        self
    }
    
    /// Set relative orientation for road position
    pub fn with_road_orientation(mut self, dh: f64) -> Self {
        if let RelativePositionType::Road { ref mut orientation, .. } = self.position_type {
            *orientation = Some(Orientation {
                h: Value::literal(dh),
                p: Value::literal(0.0),
                r: Value::literal(0.0),
                type_: None,
            });
        }
        self
    }
}

impl PositionBuilder for RelativePositionBuilder {
    fn validate(&self) -> BuilderResult<()> {
        // Validate entity reference
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for relative position",
            "Call to_entity() to set the reference entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        // Validate position type specific fields
        match &self.position_type {
            RelativePositionType::World { dx, dy, dz, orientation } => {
                let dx_val = dx.ok_or_else(|| BuilderError::validation_error(
                    "DX offset is required for relative world position",
                    "Call world_offset() to set the offsets"
                ))?;
                
                let dy_val = dy.ok_or_else(|| BuilderError::validation_error(
                    "DY offset is required for relative world position",
                    "Call world_offset() to set the offsets"
                ))?;
                
                let dz_val = dz.ok_or_else(|| BuilderError::validation_error(
                    "DZ offset is required for relative world position",
                    "Call world_offset() to set the offsets"
                ))?;
                
                validate_coordinate(dx_val, "DX")?;
                validate_coordinate(dy_val, "DY")?;
                validate_coordinate(dz_val, "DZ")?;
                
                if let Some(orient) = orientation {
                    if let Some(h) = orient.h.as_literal() {
                        validate_angle(*h, "relative heading")?;
                    }
                    if let Some(p) = orient.p.as_literal() {
                        validate_angle(*p, "relative pitch")?;
                    }
                    if let Some(r) = orient.r.as_literal() {
                        validate_angle(*r, "relative roll")?;
                    }
                }
            },
            RelativePositionType::Lane { ds, dt, orientation } => {
                let ds_val = ds.ok_or_else(|| BuilderError::validation_error(
                    "DS offset is required for relative lane position",
                    "Call lane_offset() to set the offsets"
                ))?;
                
                let dt_val = dt.ok_or_else(|| BuilderError::validation_error(
                    "DT offset is required for relative lane position",
                    "Call lane_offset() to set the offsets"
                ))?;
                
                validate_coordinate(ds_val, "DS")?;
                validate_coordinate(dt_val, "DT")?;
                
                if let Some(orient) = orientation {
                    if let Some(h) = orient.h.as_literal() {
                        validate_angle(*h, "relative heading")?;
                    }
                }
            },
            RelativePositionType::Road { ds, dt, orientation } => {
                let ds_val = ds.ok_or_else(|| BuilderError::validation_error(
                    "DS offset is required for relative road position",
                    "Call road_offset() to set the offsets"
                ))?;
                
                let dt_val = dt.ok_or_else(|| BuilderError::validation_error(
                    "DT offset is required for relative road position",
                    "Call road_offset() to set the offsets"
                ))?;
                
                validate_coordinate(ds_val, "DS")?;
                validate_coordinate(dt_val, "DT")?;
                
                if let Some(orient) = orientation {
                    if let Some(h) = orient.h.as_literal() {
                        validate_angle(*h, "relative heading")?;
                    }
                }
            },
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;
        
        let entity_ref = self.entity_ref.unwrap();
        
        match self.position_type {
            RelativePositionType::World { dx, dy, dz, orientation } => {
                let relative_world_position = RelativeWorldPosition {
                    entity_ref: Value::literal(entity_ref),
                    dx: Value::literal(dx.unwrap()),
                    dy: Value::literal(dy.unwrap()),
                    dz: Value::literal(dz.unwrap()),
                    orientation,
                };
                
                Ok(Position {
                    relative_world_position: Some(relative_world_position),
                    ..Position::empty()
                })
            },
            RelativePositionType::Lane { ds, dt, orientation } => {
                let relative_lane_position = RelativeLanePosition {
                    entity_ref: Value::literal(entity_ref),
                    ds: Value::literal(ds.unwrap()),
                    dt: Value::literal(dt.unwrap()),
                    orientation,
                };
                
                Ok(Position {
                    relative_lane_position: Some(relative_lane_position),
                    ..Position::empty()
                })
            },
            RelativePositionType::Road { ds, dt, orientation } => {
                let relative_road_position = RelativeRoadPosition {
                    entity_ref: Value::literal(entity_ref),
                    ds: Value::literal(ds.unwrap()),
                    dt: Value::literal(dt.unwrap()),
                    orientation,
                };
                
                Ok(Position {
                    relative_road_position: Some(relative_road_position),
                    ..Position::empty()
                })
            },
        }
    }
}

impl Default for RelativePositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_relative_world_position_builder() {
        let position = RelativePositionBuilder::new()
            .to_entity("target_vehicle")
            .world_offset(5.0, -2.0, 0.0)
            .finish()
            .unwrap();
        
        let rel_world_pos = position.relative_world_position.unwrap();
        assert_eq!(rel_world_pos.entity_ref.as_literal().unwrap(), "target_vehicle");
        assert_eq!(rel_world_pos.dx.as_literal().unwrap(), &5.0);
        assert_eq!(rel_world_pos.dy.as_literal().unwrap(), &-2.0);
        assert_eq!(rel_world_pos.dz.as_literal().unwrap(), &0.0);
    }
    
    #[test]
    fn test_relative_lane_position_builder() {
        let position = RelativePositionBuilder::new()
            .to_entity("lead_vehicle")
            .lane_offset(10.0, 0.0)
            .with_lane_orientation(0.1)
            .finish()
            .unwrap();
        
        let rel_lane_pos = position.relative_lane_position.unwrap();
        assert_eq!(rel_lane_pos.entity_ref.as_literal().unwrap(), "lead_vehicle");
        assert_eq!(rel_lane_pos.ds.as_literal().unwrap(), &10.0);
        assert_eq!(rel_lane_pos.dt.as_literal().unwrap(), &0.0);
        assert!(rel_lane_pos.orientation.is_some());
    }
    
    #[test]
    fn test_relative_road_position_builder() {
        let position = RelativePositionBuilder::new()
            .to_entity("reference_vehicle")
            .road_offset(15.0, 3.5)
            .finish()
            .unwrap();
        
        let rel_road_pos = position.relative_road_position.unwrap();
        assert_eq!(rel_road_pos.entity_ref.as_literal().unwrap(), "reference_vehicle");
        assert_eq!(rel_road_pos.ds.as_literal().unwrap(), &15.0);
        assert_eq!(rel_road_pos.dt.as_literal().unwrap(), &3.5);
    }
    
    #[test]
    fn test_relative_position_builder_validation_missing_entity() {
        let result = RelativePositionBuilder::new()
            .world_offset(1.0, 2.0, 3.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference"));
    }
    
    #[test]
    fn test_relative_position_builder_validation_empty_entity() {
        let result = RelativePositionBuilder::new()
            .to_entity("")
            .world_offset(1.0, 2.0, 3.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference"));
    }
    
    #[test]
    fn test_relative_position_builder_validation_missing_offsets() {
        let result = RelativePositionBuilder::new()
            .to_entity("test_entity")
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DX offset"));
    }
    
    #[test]
    fn test_relative_position_builder_validation_invalid_coordinates() {
        let result = RelativePositionBuilder::new()
            .to_entity("test_entity")
            .world_offset(f64::NAN, 2.0, 3.0)
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DX coordinate"));
    }
}
//! Traffic action builders for programmatic traffic action construction

use crate::types::{
    basic::{Double, Boolean, OSString, UnsignedInt},
    actions::{
        TrafficSignalAction, TrafficSwarmAction, TrafficSourceAction,
        TrafficSinkAction, TrafficAreaAction, TrafficStopAction,
        TrafficSignalStateAction, TrafficSignalControllerAction,
        VehicleCategory, VehicleCategoryDistribution, CentralSwarmObject,
        TrafficDefinition, TrafficArea, TrafficAreaVertex,
    },
    enums::{TrafficSignalState, TrafficSignalGroupState},
    positions::Position,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ActionBuilder, validate_entity_ref, validate_timing};

/// Builder for creating traffic signal actions
#[derive(Debug, Clone)]
pub struct TrafficSignalActionBuilder {
    entity_ref: Option<String>,
    traffic_signal_controller_ref: Option<String>,
    state: Option<TrafficSignalState>,
    group_state: Option<TrafficSignalGroupState>,
}

impl TrafficSignalActionBuilder {
    /// Create a new traffic signal action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            traffic_signal_controller_ref: None,
            state: None,
            group_state: None,
        }
    }
    
    /// Set the entity reference
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the traffic signal controller reference
    pub fn controller(mut self, controller_ref: impl Into<String>) -> Self {
        self.traffic_signal_controller_ref = Some(controller_ref.into());
        self
    }
    
    /// Set the traffic signal state
    pub fn state(mut self, state: TrafficSignalState) -> Self {
        self.state = Some(state);
        self
    }
    
    /// Set the traffic signal group state
    pub fn group_state(mut self, group_state: TrafficSignalGroupState) -> Self {
        self.group_state = Some(group_state);
        self
    }
}

impl ActionBuilder for TrafficSignalActionBuilder {
    type ActionType = TrafficSignalAction;
    
    fn validate(&self) -> BuilderResult<()> {
        if self.entity_ref.is_none() && self.traffic_signal_controller_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Either entity reference or traffic signal controller reference is required",
                "Call entity() or controller() to set a reference"
            ));
        }
        
        if let Some(entity_ref) = &self.entity_ref {
            validate_entity_ref(entity_ref)?;
        }
        
        if let Some(controller_ref) = &self.traffic_signal_controller_ref {
            validate_entity_ref(controller_ref)?;
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(TrafficSignalAction {
            traffic_signal_controller_ref: self.traffic_signal_controller_ref.map(|r| OSString::literal(r)),
            state: self.state,
            group_state: self.group_state,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating traffic swarm actions
#[derive(Debug, Clone)]
pub struct TrafficSwarmActionBuilder {
    entity_ref: Option<String>,
    central_object: Option<CentralSwarmObject>,
    number_of_vehicles: Option<u32>,
    inner_radius: Option<f64>,
    outer_radius: Option<f64>,
    velocity: Option<f64>,
    semi_major_axis: Option<f64>,
    semi_minor_axis: Option<f64>,
}

impl TrafficSwarmActionBuilder {
    /// Create a new traffic swarm action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            central_object: None,
            number_of_vehicles: None,
            inner_radius: None,
            outer_radius: None,
            velocity: None,
            semi_major_axis: None,
            semi_minor_axis: None,
        }
    }
    
    /// Set the entity reference
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the central object
    pub fn central_object(mut self, central_object: CentralSwarmObject) -> Self {
        self.central_object = Some(central_object);
        self
    }
    
    /// Set the number of vehicles in the swarm
    pub fn number_of_vehicles(mut self, count: u32) -> Self {
        self.number_of_vehicles = Some(count);
        self
    }
    
    /// Set the inner radius of the swarm
    pub fn inner_radius(mut self, radius: f64) -> Self {
        self.inner_radius = Some(radius);
        self
    }
    
    /// Set the outer radius of the swarm
    pub fn outer_radius(mut self, radius: f64) -> Self {
        self.outer_radius = Some(radius);
        self
    }
    
    /// Set the velocity of the swarm
    pub fn velocity(mut self, velocity: f64) -> Self {
        self.velocity = Some(velocity);
        self
    }
    
    /// Set elliptical swarm parameters
    pub fn elliptical(mut self, semi_major_axis: f64, semi_minor_axis: f64) -> Self {
        self.semi_major_axis = Some(semi_major_axis);
        self.semi_minor_axis = Some(semi_minor_axis);
        self
    }
}

impl ActionBuilder for TrafficSwarmActionBuilder {
    type ActionType = TrafficSwarmAction;
    
    fn validate(&self) -> BuilderResult<()> {
        if let Some(entity_ref) = &self.entity_ref {
            validate_entity_ref(entity_ref)?;
        }
        
        if self.central_object.is_none() {
            return Err(BuilderError::validation_error(
                "Central object is required for traffic swarm action",
                "Call central_object() to set the central object"
            ));
        }
        
        if let Some(count) = self.number_of_vehicles {
            if count == 0 {
                return Err(BuilderError::validation_error(
                    "Number of vehicles must be greater than 0",
                    "Provide a positive number of vehicles"
                ));
            }
        }
        
        // Validate radius values
        if let Some(inner_radius) = self.inner_radius {
            if inner_radius < 0.0 {
                return Err(BuilderError::validation_error(
                    "Inner radius cannot be negative",
                    "Provide a non-negative radius value"
                ));
            }
        }
        
        if let Some(outer_radius) = self.outer_radius {
            if outer_radius < 0.0 {
                return Err(BuilderError::validation_error(
                    "Outer radius cannot be negative",
                    "Provide a non-negative radius value"
                ));
            }
        }
        
        // Validate that outer radius is greater than inner radius
        if let (Some(inner), Some(outer)) = (self.inner_radius, self.outer_radius) {
            if outer <= inner {
                return Err(BuilderError::validation_error(
                    "Outer radius must be greater than inner radius",
                    "Ensure outer radius > inner radius"
                ));
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(TrafficSwarmAction {
            central_object: self.central_object.unwrap(),
            number_of_vehicles: self.number_of_vehicles.map(|v| UnsignedInt::literal(v)),
            inner_radius: self.inner_radius.map(|v| Double::literal(v)),
            outer_radius: self.outer_radius.map(|v| Double::literal(v)),
            velocity: self.velocity.map(|v| Double::literal(v)),
            semi_major_axis: self.semi_major_axis.map(|v| Double::literal(v)),
            semi_minor_axis: self.semi_minor_axis.map(|v| Double::literal(v)),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating traffic source actions
#[derive(Debug, Clone)]
pub struct TrafficSourceActionBuilder {
    entity_ref: Option<String>,
    rate: Option<f64>,
    radius: Option<f64>,
    velocity: Option<f64>,
    position: Option<Position>,
    traffic_definition: Option<TrafficDefinition>,
}

impl TrafficSourceActionBuilder {
    /// Create a new traffic source action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            rate: None,
            radius: None,
            velocity: None,
            position: None,
            traffic_definition: None,
        }
    }
    
    /// Set the entity reference
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the traffic generation rate (vehicles per second)
    pub fn rate(mut self, rate: f64) -> Self {
        self.rate = Some(rate);
        self
    }
    
    /// Set the radius of the traffic source
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }
    
    /// Set the initial velocity of generated vehicles
    pub fn velocity(mut self, velocity: f64) -> Self {
        self.velocity = Some(velocity);
        self
    }
    
    /// Set the position of the traffic source
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    /// Set the traffic definition
    pub fn traffic_definition(mut self, traffic_definition: TrafficDefinition) -> Self {
        self.traffic_definition = Some(traffic_definition);
        self
    }
}

impl ActionBuilder for TrafficSourceActionBuilder {
    type ActionType = TrafficSourceAction;
    
    fn validate(&self) -> BuilderResult<()> {
        if let Some(entity_ref) = &self.entity_ref {
            validate_entity_ref(entity_ref)?;
        }
        
        if let Some(rate) = self.rate {
            if rate <= 0.0 {
                return Err(BuilderError::validation_error(
                    "Traffic generation rate must be positive",
                    "Provide a positive rate value"
                ));
            }
        }
        
        if let Some(radius) = self.radius {
            if radius < 0.0 {
                return Err(BuilderError::validation_error(
                    "Radius cannot be negative",
                    "Provide a non-negative radius value"
                ));
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(TrafficSourceAction {
            rate: self.rate.map(|v| Double::literal(v)),
            radius: self.radius.map(|v| Double::literal(v)),
            velocity: self.velocity.map(|v| Double::literal(v)),
            position: self.position,
            traffic_definition: self.traffic_definition,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for TrafficSignalActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TrafficSwarmActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TrafficSourceActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_traffic_signal_action_builder() {
        let action = TrafficSignalActionBuilder::new()
            .controller("signal_controller_1")
            .state(TrafficSignalState::Red)
            .finish()
            .unwrap();
        
        assert_eq!(action.traffic_signal_controller_ref.unwrap().as_literal().unwrap(), &"signal_controller_1".to_string());
        assert_eq!(action.state.unwrap(), TrafficSignalState::Red);
    }
    
    #[test]
    fn test_traffic_signal_action_validation() {
        let result = TrafficSignalActionBuilder::new()
            // Missing both entity and controller reference
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Either entity reference or traffic signal controller reference"));
    }
    
    #[test]
    fn test_traffic_source_action_builder() {
        let action = TrafficSourceActionBuilder::new()
            .entity("traffic_source")
            .rate(0.5)
            .radius(10.0)
            .velocity(15.0)
            .finish()
            .unwrap();
        
        assert_eq!(action.rate.unwrap().as_literal().unwrap(), &0.5);
        assert_eq!(action.radius.unwrap().as_literal().unwrap(), &10.0);
        assert_eq!(action.velocity.unwrap().as_literal().unwrap(), &15.0);
    }
    
    #[test]
    fn test_traffic_source_action_validation() {
        let result = TrafficSourceActionBuilder::new()
            .entity("traffic_source")
            .rate(-1.0) // Invalid negative rate
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Traffic generation rate"));
    }
}
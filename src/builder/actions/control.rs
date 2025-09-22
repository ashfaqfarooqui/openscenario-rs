//! Control action builders for programmatic control action construction

use crate::types::{
    basic::{Double, Boolean, OSString},
    actions::{
        ActivateControllerAction, AssignControllerAction, OverrideControllerValueAction,
        OverrideControllerValueActionBrake, OverrideControllerValueActionThrottle,
        OverrideControllerValueActionSteeringWheel, OverrideControllerValueActionGear,
        OverrideControllerValueActionParkingBrake, OverrideControllerValueActionClutch,
    },
    controllers::Controller,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ActionBuilder, validate_entity_ref};

/// Builder for creating activate controller actions
#[derive(Debug, Clone)]
pub struct ActivateControllerActionBuilder {
    entity_ref: Option<String>,
    lateral: Option<bool>,
    longitudinal: Option<bool>,
}

impl ActivateControllerActionBuilder {
    /// Create a new activate controller action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            lateral: None,
            longitudinal: None,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Enable lateral control
    pub fn lateral(mut self, lateral: bool) -> Self {
        self.lateral = Some(lateral);
        self
    }
    
    /// Enable longitudinal control
    pub fn longitudinal(mut self, longitudinal: bool) -> Self {
        self.longitudinal = Some(longitudinal);
        self
    }
    
    /// Enable both lateral and longitudinal control
    pub fn full_control(mut self) -> Self {
        self.lateral = Some(true);
        self.longitudinal = Some(true);
        self
    }
}

impl ActionBuilder for ActivateControllerActionBuilder {
    type ActionType = ActivateControllerAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for activate controller action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(ActivateControllerAction {
            lateral: self.lateral.map(|v| Boolean::literal(v)),
            longitudinal: self.longitudinal.map(|v| Boolean::literal(v)),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating assign controller actions
#[derive(Debug, Clone)]
pub struct ControllerActionBuilder {
    entity_ref: Option<String>,
    controller: Option<Controller>,
    catalog_reference: Option<String>,
}

impl ControllerActionBuilder {
    /// Create a new controller action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            controller: None,
            catalog_reference: None,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the controller directly
    pub fn controller(mut self, controller: Controller) -> Self {
        self.controller = Some(controller);
        self.catalog_reference = None;
        self
    }
    
    /// Set the controller from catalog reference
    pub fn catalog_controller(mut self, catalog_ref: impl Into<String>) -> Self {
        self.catalog_reference = Some(catalog_ref.into());
        self.controller = None;
        self
    }
}

impl ActionBuilder for ControllerActionBuilder {
    type ActionType = AssignControllerAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for controller action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.controller.is_none() && self.catalog_reference.is_none() {
            return Err(BuilderError::validation_error(
                "Either controller or catalog reference is required",
                "Call controller() or catalog_controller() to set the controller"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(AssignControllerAction {
            controller: self.controller,
            catalog_reference: self.catalog_reference.map(|name| {
                // This is a simplified implementation - in reality we'd need proper catalog reference structure
                crate::types::catalogs::references::CatalogReference::new("Controllers".to_string(), name)
            }),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating override controller actions
#[derive(Debug, Clone)]
pub struct OverrideControllerActionBuilder {
    entity_ref: Option<String>,
    throttle: Option<f64>,
    brake: Option<f64>,
    clutch: Option<f64>,
    parking_brake: Option<bool>,
    steering_wheel: Option<f64>,
    gear: Option<i32>,
    active: bool,
}

impl OverrideControllerActionBuilder {
    /// Create a new override controller action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            throttle: None,
            brake: None,
            clutch: None,
            parking_brake: None,
            steering_wheel: None,
            gear: None,
            active: true,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set throttle override value (0.0 to 1.0)
    pub fn throttle(mut self, value: f64) -> Self {
        self.throttle = Some(value);
        self
    }
    
    /// Set brake override value (0.0 to 1.0)
    pub fn brake(mut self, value: f64) -> Self {
        self.brake = Some(value);
        self
    }
    
    /// Set clutch override value (0.0 to 1.0)
    pub fn clutch(mut self, value: f64) -> Self {
        self.clutch = Some(value);
        self
    }
    
    /// Set parking brake override
    pub fn parking_brake(mut self, engaged: bool) -> Self {
        self.parking_brake = Some(engaged);
        self
    }
    
    /// Set steering wheel override value (in radians)
    pub fn steering_wheel(mut self, angle: f64) -> Self {
        self.steering_wheel = Some(angle);
        self
    }
    
    /// Set gear override value
    pub fn gear(mut self, gear: i32) -> Self {
        self.gear = Some(gear);
        self
    }
    
    /// Set whether the override is active
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl ActionBuilder for OverrideControllerActionBuilder {
    type ActionType = OverrideControllerValueAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for override controller action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        // Validate throttle range
        if let Some(throttle) = self.throttle {
            if throttle < 0.0 || throttle > 1.0 {
                return Err(BuilderError::validation_error(
                    "Throttle value must be between 0.0 and 1.0",
                    "Provide a valid throttle value"
                ));
            }
        }
        
        // Validate brake range
        if let Some(brake) = self.brake {
            if brake < 0.0 || brake > 1.0 {
                return Err(BuilderError::validation_error(
                    "Brake value must be between 0.0 and 1.0",
                    "Provide a valid brake value"
                ));
            }
        }
        
        // Validate clutch range
        if let Some(clutch) = self.clutch {
            if clutch < 0.0 || clutch > 1.0 {
                return Err(BuilderError::validation_error(
                    "Clutch value must be between 0.0 and 1.0",
                    "Provide a valid clutch value"
                ));
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(OverrideControllerValueAction {
            throttle: self.throttle.map(|v| OverrideControllerValueActionThrottle {
                active: Boolean::literal(self.active),
                value: Double::literal(v),
            }),
            brake: self.brake.map(|v| OverrideControllerValueActionBrake {
                active: Boolean::literal(self.active),
                value: Double::literal(v),
            }),
            clutch: self.clutch.map(|v| OverrideControllerValueActionClutch {
                active: Boolean::literal(self.active),
                value: Double::literal(v),
            }),
            parking_brake: self.parking_brake.map(|v| OverrideControllerValueActionParkingBrake {
                active: Boolean::literal(self.active),
                value: Boolean::literal(v),
            }),
            steering_wheel: self.steering_wheel.map(|v| OverrideControllerValueActionSteeringWheel {
                active: Boolean::literal(self.active),
                value: Double::literal(v),
            }),
            gear: self.gear.map(|v| OverrideControllerValueActionGear {
                active: Boolean::literal(self.active),
                number: Double::literal(v as f64),
            }),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for ActivateControllerActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ControllerActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OverrideControllerActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_activate_controller_action_builder() {
        let action = ActivateControllerActionBuilder::new()
            .entity("test_vehicle")
            .full_control()
            .finish()
            .unwrap();
        
        assert_eq!(action.lateral.unwrap().as_literal().unwrap(), &true);
        assert_eq!(action.longitudinal.unwrap().as_literal().unwrap(), &true);
    }
    
    #[test]
    fn test_activate_controller_action_validation() {
        let result = ActivateControllerActionBuilder::new()
            // Missing entity reference
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference"));
    }
    
    #[test]
    fn test_override_controller_action_builder() {
        let action = OverrideControllerActionBuilder::new()
            .entity("test_vehicle")
            .throttle(0.8)
            .brake(0.2)
            .steering_wheel(0.1)
            .finish()
            .unwrap();
        
        assert_eq!(action.throttle.unwrap().value.as_literal().unwrap(), &0.8);
        assert_eq!(action.brake.unwrap().value.as_literal().unwrap(), &0.2);
        assert_eq!(action.steering_wheel.unwrap().value.as_literal().unwrap(), &0.1);
    }
    
    #[test]
    fn test_override_controller_action_validation() {
        let result = OverrideControllerActionBuilder::new()
            .entity("test_vehicle")
            .throttle(1.5) // Invalid value > 1.0
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Throttle value"));
    }
}
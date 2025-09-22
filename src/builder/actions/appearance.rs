//! Appearance action builders for programmatic appearance action construction

use crate::types::{
    basic::{Double, Boolean, OSString},
    actions::{
        AppearanceAction, LightStateAction, AnimationAction, VisibilityAction,
    },
    enums::LightType,
};
use crate::builder::{BuilderError, BuilderResult};
use super::{ActionBuilder, validate_entity_ref};

/// Builder for creating appearance actions
#[derive(Debug, Clone)]
pub struct AppearanceActionBuilder {
    entity_ref: Option<String>,
    light_state: Option<LightStateAction>,
    animation: Option<AnimationAction>,
    visibility: Option<VisibilityAction>,
}

impl AppearanceActionBuilder {
    /// Create a new appearance action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            light_state: None,
            animation: None,
            visibility: None,
        }
    }
    
    /// Set the entity to modify
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set light state action
    pub fn light_state(mut self, light_state: LightStateAction) -> Self {
        self.light_state = Some(light_state);
        self
    }
    
    /// Set animation action
    pub fn animation(mut self, animation: AnimationAction) -> Self {
        self.animation = Some(animation);
        self
    }
    
    /// Set visibility action
    pub fn visibility(mut self, visibility: VisibilityAction) -> Self {
        self.visibility = Some(visibility);
        self
    }
}

impl ActionBuilder for AppearanceActionBuilder {
    type ActionType = AppearanceAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for appearance action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        let action_count = [
            self.light_state.is_some(),
            self.animation.is_some(),
            self.visibility.is_some(),
        ].iter().filter(|&&x| x).count();
        
        if action_count == 0 {
            return Err(BuilderError::validation_error(
                "At least one appearance action type is required",
                "Call light_state(), animation(), or visibility()"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(AppearanceAction {
            light_state_action: self.light_state,
            animation_action: self.animation,
            visibility_action: self.visibility,
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating light actions
#[derive(Debug, Clone)]
pub struct LightActionBuilder {
    entity_ref: Option<String>,
    light_type: Option<LightType>,
    state: Option<bool>,
    flashing_on_duration: Option<f64>,
    flashing_off_duration: Option<f64>,
    luminous_intensity: Option<f64>,
}

impl LightActionBuilder {
    /// Create a new light action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            light_type: None,
            state: None,
            flashing_on_duration: None,
            flashing_off_duration: None,
            luminous_intensity: None,
        }
    }
    
    /// Set the entity to control
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the light type
    pub fn light_type(mut self, light_type: LightType) -> Self {
        self.light_type = Some(light_type);
        self
    }
    
    /// Turn the light on
    pub fn on(mut self) -> Self {
        self.state = Some(true);
        self
    }
    
    /// Turn the light off
    pub fn off(mut self) -> Self {
        self.state = Some(false);
        self
    }
    
    /// Set the light state
    pub fn state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    
    /// Set flashing parameters
    pub fn flashing(mut self, on_duration: f64, off_duration: f64) -> Self {
        self.flashing_on_duration = Some(on_duration);
        self.flashing_off_duration = Some(off_duration);
        self
    }
    
    /// Set luminous intensity
    pub fn luminous_intensity(mut self, intensity: f64) -> Self {
        self.luminous_intensity = Some(intensity);
        self
    }
}

impl ActionBuilder for LightActionBuilder {
    type ActionType = LightStateAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for light action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.light_type.is_none() {
            return Err(BuilderError::validation_error(
                "Light type is required for light action",
                "Call light_type() to set the light type"
            ));
        }
        
        if self.state.is_none() {
            return Err(BuilderError::validation_error(
                "Light state is required for light action",
                "Call on(), off(), or state() to set the light state"
            ));
        }
        
        // Validate flashing durations
        if let Some(on_duration) = self.flashing_on_duration {
            if on_duration < 0.0 {
                return Err(BuilderError::validation_error(
                    "Flashing on duration cannot be negative",
                    "Provide a non-negative duration"
                ));
            }
        }
        
        if let Some(off_duration) = self.flashing_off_duration {
            if off_duration < 0.0 {
                return Err(BuilderError::validation_error(
                    "Flashing off duration cannot be negative",
                    "Provide a non-negative duration"
                ));
            }
        }
        
        // Validate luminous intensity
        if let Some(intensity) = self.luminous_intensity {
            if intensity < 0.0 {
                return Err(BuilderError::validation_error(
                    "Luminous intensity cannot be negative",
                    "Provide a non-negative intensity value"
                ));
            }
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(LightStateAction {
            light_type: self.light_type.unwrap(),
            state: Boolean::literal(self.state.unwrap()),
            flashing_on_duration: self.flashing_on_duration.map(|v| Double::literal(v)),
            flashing_off_duration: self.flashing_off_duration.map(|v| Double::literal(v)),
            luminous_intensity: self.luminous_intensity.map(|v| Double::literal(v)),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for creating animation actions
#[derive(Debug, Clone)]
pub struct AnimationActionBuilder {
    entity_ref: Option<String>,
    animation_type: Option<String>,
    state: Option<String>,
    loop_count: Option<i32>,
}

impl AnimationActionBuilder {
    /// Create a new animation action builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            animation_type: None,
            state: None,
            loop_count: None,
        }
    }
    
    /// Set the entity to animate
    pub fn entity(mut self, entity_ref: impl Into<String>) -> Self {
        self.entity_ref = Some(entity_ref.into());
        self
    }
    
    /// Set the animation type
    pub fn animation_type(mut self, animation_type: impl Into<String>) -> Self {
        self.animation_type = Some(animation_type.into());
        self
    }
    
    /// Set the animation state
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }
    
    /// Set the loop count
    pub fn loop_count(mut self, count: i32) -> Self {
        self.loop_count = Some(count);
        self
    }
    
    /// Set infinite looping
    pub fn loop_infinite(mut self) -> Self {
        self.loop_count = Some(-1);
        self
    }
}

impl ActionBuilder for AnimationActionBuilder {
    type ActionType = AnimationAction;
    
    fn validate(&self) -> BuilderResult<()> {
        let entity_ref = self.entity_ref.as_ref().ok_or_else(|| BuilderError::validation_error(
            "Entity reference is required for animation action",
            "Call entity() to set the target entity"
        ))?;
        
        validate_entity_ref(entity_ref)?;
        
        if self.animation_type.is_none() {
            return Err(BuilderError::validation_error(
                "Animation type is required for animation action",
                "Call animation_type() to set the animation type"
            ));
        }
        
        Ok(())
    }
    
    fn finish(self) -> BuilderResult<Self::ActionType> {
        self.validate()?;
        
        Ok(AnimationAction {
            animation_type: OSString::literal(self.animation_type.unwrap()),
            state: self.state.map(|s| OSString::literal(s)),
            loop_count: self.loop_count.map(|c| crate::types::basic::Int::literal(c)),
        })
    }
    
    fn get_entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

// Default implementations
impl Default for AppearanceActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LightActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AnimationActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_light_action_builder() {
        let action = LightActionBuilder::new()
            .entity("test_vehicle")
            .light_type(LightType::Headlight)
            .on()
            .luminous_intensity(1000.0)
            .finish()
            .unwrap();
        
        assert_eq!(action.light_type, LightType::Headlight);
        assert_eq!(action.state.as_literal().unwrap(), &true);
        assert_eq!(action.luminous_intensity.unwrap().as_literal().unwrap(), &1000.0);
    }
    
    #[test]
    fn test_light_action_builder_validation() {
        let result = LightActionBuilder::new()
            .entity("test_vehicle")
            .light_type(LightType::Headlight)
            // Missing state
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Light state"));
    }
    
    #[test]
    fn test_animation_action_builder() {
        let action = AnimationActionBuilder::new()
            .entity("test_vehicle")
            .animation_type("door_open")
            .state("opening")
            .loop_count(1)
            .finish()
            .unwrap();
        
        assert_eq!(action.animation_type.as_literal().unwrap(), &"door_open".to_string());
        assert_eq!(action.state.unwrap().as_literal().unwrap(), &"opening".to_string());
        assert_eq!(action.loop_count.unwrap().as_literal().unwrap(), &1);
    }
    
    #[test]
    fn test_animation_action_builder_validation() {
        let result = AnimationActionBuilder::new()
            .entity("test_vehicle")
            // Missing animation type
            .finish();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Animation type"));
    }
}
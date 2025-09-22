//! Private action builders for entity-specific actions
//!
//! This module provides builders for private actions that are specific to
//! individual entities, including teleport, visibility, controller, and
//! appearance actions.

use crate::types::actions::{
    Action, TeleportAction, VisibilityAction, ControllerAction, AppearanceAction,
    ActivateControllerAction, AssignControllerAction, OverrideControllerValueAction,
    TrailerAction, ConnectTrailerAction, DisconnectTrailerAction
};
use crate::types::basic::{OSString, Value, Boolean};
use crate::types::positions::Position;
use crate::types::catalogs::references::ControllerCatalogLocation;
use super::{ActionBuilderTrait, ActionUtils};
use crate::builder::error::{BuilderError, BuilderResult};
use crate::builder::states::*;
use std::marker::PhantomData;

/// Builder for private actions (entity-specific actions)
/// 
/// This builder provides a type-safe way to construct private actions
/// that are specific to individual entities.
/// 
/// # Type Parameters
/// * `S` - Current builder state
/// 
/// # Example
/// ```rust
/// let action = PrivateActionBuilder::new(Some("teleport".to_string()))
///     .teleport_action()
///     .to_position(position)
///     .build()?;
/// ```
pub struct PrivateActionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Action name for identification
    name: Option<String>,
    
    /// Partially constructed action data
    action_data: PartialPrivateData,
}

/// Internal data structure for building private actions
#[derive(Debug, Default)]
struct PartialPrivateData {
    /// Action type being built
    action_type: Option<PrivateActionType>,
    
    /// Teleport action data
    teleport_data: Option<TeleportActionData>,
    
    /// Visibility action data
    visibility_data: Option<VisibilityActionData>,
    
    /// Controller action data
    controller_data: Option<ControllerActionData>,
    
    /// Appearance action data
    appearance_data: Option<AppearanceActionData>,
    
    /// Trailer action data
    trailer_data: Option<TrailerActionData>,
}

/// Types of private actions
#[derive(Debug, Clone)]
enum PrivateActionType {
    Teleport,
    Visibility,
    Controller,
    Appearance,
    Trailer,
}

/// Teleport action configuration data
#[derive(Debug, Default)]
struct TeleportActionData {
    position: Option<Position>,
}

/// Visibility action configuration data
#[derive(Debug, Default)]
struct VisibilityActionData {
    graphics: Option<bool>,
    traffic: Option<bool>,
    sensors: Option<bool>,
}

/// Controller action configuration data
#[derive(Debug, Default)]
struct ControllerActionData {
    controller_type: Option<ControllerActionType>,
    activate_data: Option<ActivateControllerData>,
    assign_data: Option<AssignControllerData>,
    override_data: Option<OverrideControllerData>,
}

/// Types of controller actions
#[derive(Debug, Clone)]
enum ControllerActionType {
    Activate,
    Assign,
    Override,
}

/// Activate controller action data
#[derive(Debug, Default)]
struct ActivateControllerData {
    lateral: Option<bool>,
    longitudinal: Option<bool>,
}

/// Assign controller action data
#[derive(Debug, Default)]
struct AssignControllerData {
    catalog_reference: Option<ControllerCatalogLocation>,
    properties: Option<Vec<(String, String)>>,
}

/// Override controller action data
#[derive(Debug, Default)]
struct OverrideControllerData {
    throttle: Option<f64>,
    brake: Option<f64>,
    clutch: Option<f64>,
    parking_brake: Option<f64>,
    steering_wheel: Option<f64>,
    gear: Option<String>,
}

/// Appearance action configuration data
#[derive(Debug, Default)]
struct AppearanceActionData {
    // Placeholder for appearance action properties
    animation_type: Option<String>,
    animation_state: Option<String>,
}

/// Trailer action configuration data
#[derive(Debug, Default)]
struct TrailerActionData {
    trailer_type: Option<TrailerActionType>,
    entity_ref: Option<String>,
}

/// Types of trailer actions
#[derive(Debug, Clone)]
enum TrailerActionType {
    Connect,
    Disconnect,
}

// Core builder implementation for Empty state
impl PrivateActionBuilder<Empty> {
    /// Create a new private action builder
    /// 
    /// # Arguments
    /// * `name` - Optional action name
    /// 
    /// # Returns
    /// A new PrivateActionBuilder in Empty state
    pub fn new(name: Option<String>) -> Self {
        Self {
            _state: PhantomData,
            name,
            action_data: PartialPrivateData::default(),
        }
    }

    /// Start building a teleport action
    /// 
    /// Teleport actions instantly move an entity to a new position.
    /// 
    /// # Returns
    /// PrivateActionBuilder in HasType state for teleport configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = PrivateActionBuilder::new(None)
    ///     .teleport_action()
    ///     .to_position(position);
    /// ```
    pub fn teleport_action(mut self) -> PrivateActionBuilder<HasType> {
        self.action_data.action_type = Some(PrivateActionType::Teleport);
        self.action_data.teleport_data = Some(TeleportActionData::default());
        
        PrivateActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a visibility action
    /// 
    /// Visibility actions control the visibility of an entity in different contexts.
    /// 
    /// # Returns
    /// PrivateActionBuilder in HasType state for visibility configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = PrivateActionBuilder::new(None)
    ///     .visibility_action()
    ///     .graphics(true)
    ///     .traffic(false);
    /// ```
    pub fn visibility_action(mut self) -> PrivateActionBuilder<HasType> {
        self.action_data.action_type = Some(PrivateActionType::Visibility);
        self.action_data.visibility_data = Some(VisibilityActionData::default());
        
        PrivateActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a controller action
    /// 
    /// Controller actions manage entity controllers (activate, assign, override).
    /// 
    /// # Returns
    /// PrivateActionBuilder in HasType state for controller configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = PrivateActionBuilder::new(None)
    ///     .controller_action()
    ///     .activate_controller()
    ///     .lateral(true)
    ///     .longitudinal(true);
    /// ```
    pub fn controller_action(mut self) -> PrivateActionBuilder<HasType> {
        self.action_data.action_type = Some(PrivateActionType::Controller);
        self.action_data.controller_data = Some(ControllerActionData::default());
        
        PrivateActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building an appearance action
    /// 
    /// Appearance actions control the visual appearance and animations of entities.
    /// 
    /// # Returns
    /// PrivateActionBuilder in HasType state for appearance configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = PrivateActionBuilder::new(None)
    ///     .appearance_action()
    ///     .animation_type("wave")
    ///     .animation_state("start");
    /// ```
    pub fn appearance_action(mut self) -> PrivateActionBuilder<HasType> {
        self.action_data.action_type = Some(PrivateActionType::Appearance);
        self.action_data.appearance_data = Some(AppearanceActionData::default());
        
        PrivateActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a trailer action
    /// 
    /// Trailer actions control trailer connections and disconnections.
    /// 
    /// # Returns
    /// PrivateActionBuilder in HasType state for trailer configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = PrivateActionBuilder::new(None)
    ///     .trailer_action()
    ///     .connect_trailer("trailer_entity");
    /// ```
    pub fn trailer_action(mut self) -> PrivateActionBuilder<HasType> {
        self.action_data.action_type = Some(PrivateActionType::Trailer);
        self.action_data.trailer_data = Some(TrailerActionData::default());
        
        PrivateActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }
}

// Methods for configuring private actions
impl PrivateActionBuilder<HasType> {
    /// Set the position for teleport actions
    /// 
    /// # Arguments
    /// * `position` - Target position to teleport to
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.to_position(position)
    /// ```
    pub fn to_position(mut self, position: Position) -> Self {
        if let Some(ref mut teleport_data) = self.action_data.teleport_data {
            teleport_data.position = Some(position);
        }
        self
    }

    /// Set graphics visibility
    /// 
    /// # Arguments
    /// * `visible` - Whether the entity should be visible in graphics
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.graphics(true)
    /// ```
    pub fn graphics(mut self, visible: bool) -> Self {
        if let Some(ref mut visibility_data) = self.action_data.visibility_data {
            visibility_data.graphics = Some(visible);
        }
        self
    }

    /// Set traffic visibility
    /// 
    /// # Arguments
    /// * `visible` - Whether the entity should be visible to traffic
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.traffic(false)
    /// ```
    pub fn traffic(mut self, visible: bool) -> Self {
        if let Some(ref mut visibility_data) = self.action_data.visibility_data {
            visibility_data.traffic = Some(visible);
        }
        self
    }

    /// Set sensor visibility
    /// 
    /// # Arguments
    /// * `visible` - Whether the entity should be visible to sensors
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.sensors(true)
    /// ```
    pub fn sensors(mut self, visible: bool) -> Self {
        if let Some(ref mut visibility_data) = self.action_data.visibility_data {
            visibility_data.sensors = Some(visible);
        }
        self
    }

    /// Configure activate controller action
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.activate_controller()
    /// ```
    pub fn activate_controller(mut self) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            controller_data.controller_type = Some(ControllerActionType::Activate);
            controller_data.activate_data = Some(ActivateControllerData::default());
        }
        self
    }

    /// Configure assign controller action
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.assign_controller()
    /// ```
    pub fn assign_controller(mut self) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            controller_data.controller_type = Some(ControllerActionType::Assign);
            controller_data.assign_data = Some(AssignControllerData::default());
        }
        self
    }

    /// Configure override controller action
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.override_controller()
    /// ```
    pub fn override_controller(mut self) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            controller_data.controller_type = Some(ControllerActionType::Override);
            controller_data.override_data = Some(OverrideControllerData::default());
        }
        self
    }

    /// Set lateral controller activation
    /// 
    /// # Arguments
    /// * `active` - Whether lateral control should be active
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.lateral(true)
    /// ```
    pub fn lateral(mut self, active: bool) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            if let Some(ref mut activate_data) = controller_data.activate_data {
                activate_data.lateral = Some(active);
            }
        }
        self
    }

    /// Set longitudinal controller activation
    /// 
    /// # Arguments
    /// * `active` - Whether longitudinal control should be active
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.longitudinal(true)
    /// ```
    pub fn longitudinal(mut self, active: bool) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            if let Some(ref mut activate_data) = controller_data.activate_data {
                activate_data.longitudinal = Some(active);
            }
        }
        self
    }

    /// Set throttle override value
    /// 
    /// # Arguments
    /// * `value` - Throttle value (0.0 to 1.0)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.throttle(0.8)
    /// ```
    pub fn throttle(mut self, value: f64) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            if let Some(ref mut override_data) = controller_data.override_data {
                override_data.throttle = Some(value);
            }
        }
        self
    }

    /// Set brake override value
    /// 
    /// # Arguments
    /// * `value` - Brake value (0.0 to 1.0)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.brake(0.5)
    /// ```
    pub fn brake(mut self, value: f64) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            if let Some(ref mut override_data) = controller_data.override_data {
                override_data.brake = Some(value);
            }
        }
        self
    }

    /// Set steering wheel override value
    /// 
    /// # Arguments
    /// * `value` - Steering wheel angle in radians
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.steering_wheel(0.1)
    /// ```
    pub fn steering_wheel(mut self, value: f64) -> Self {
        if let Some(ref mut controller_data) = self.action_data.controller_data {
            if let Some(ref mut override_data) = controller_data.override_data {
                override_data.steering_wheel = Some(value);
            }
        }
        self
    }

    /// Set animation type for appearance actions
    /// 
    /// # Arguments
    /// * `animation_type` - Type of animation
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.animation_type("wave")
    /// ```
    pub fn animation_type(mut self, animation_type: &str) -> Self {
        if let Some(ref mut appearance_data) = self.action_data.appearance_data {
            appearance_data.animation_type = Some(animation_type.to_string());
        }
        self
    }

    /// Set animation state for appearance actions
    /// 
    /// # Arguments
    /// * `state` - Animation state
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.animation_state("start")
    /// ```
    pub fn animation_state(mut self, state: &str) -> Self {
        if let Some(ref mut appearance_data) = self.action_data.appearance_data {
            appearance_data.animation_state = Some(state.to_string());
        }
        self
    }

    /// Connect a trailer
    /// 
    /// # Arguments
    /// * `entity_ref` - Reference to the trailer entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.connect_trailer("trailer_entity")
    /// ```
    pub fn connect_trailer(mut self, entity_ref: &str) -> Self {
        if let Some(ref mut trailer_data) = self.action_data.trailer_data {
            trailer_data.trailer_type = Some(TrailerActionType::Connect);
            trailer_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }

    /// Disconnect a trailer
    /// 
    /// # Arguments
    /// * `entity_ref` - Reference to the trailer entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.disconnect_trailer("trailer_entity")
    /// ```
    pub fn disconnect_trailer(mut self, entity_ref: &str) -> Self {
        if let Some(ref mut trailer_data) = self.action_data.trailer_data {
            trailer_data.trailer_type = Some(TrailerActionType::Disconnect);
            trailer_data.entity_ref = Some(entity_ref.to_string());
        }
        self
    }
}

// Build implementation
impl<S: BuilderState> PrivateActionBuilder<S> {
    /// Build the final private action
    /// 
    /// # Returns
    /// Complete Action or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<Action> {
        match self.action_data.action_type {
            Some(PrivateActionType::Teleport) => self.build_teleport_action(),
            Some(PrivateActionType::Visibility) => self.build_visibility_action(),
            Some(PrivateActionType::Controller) => self.build_controller_action(),
            Some(PrivateActionType::Appearance) => self.build_appearance_action(),
            Some(PrivateActionType::Trailer) => self.build_trailer_action(),
            None => Err(BuilderError::missing_field(
                "action_type",
                "Call one of the action type methods first (teleport_action, etc.)"
            )),
        }
    }

    fn build_teleport_action(self) -> BuilderResult<Action> {
        let teleport_data = self.action_data.teleport_data
            .ok_or_else(|| BuilderError::missing_field("teleport_data", "Internal error"))?;

        let position = teleport_data.position
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .to_position() to set the target position"
            ))?;

        let teleport_action = TeleportAction {
            position,
        };

        Ok(Action::Teleport(teleport_action))
    }

    fn build_visibility_action(self) -> BuilderResult<Action> {
        let visibility_data = self.action_data.visibility_data
            .ok_or_else(|| BuilderError::missing_field("visibility_data", "Internal error"))?;

        let visibility_action = VisibilityAction {
            graphics: Value::literal(visibility_data.graphics.unwrap_or(true)),
            traffic: Value::literal(visibility_data.traffic.unwrap_or(true)),
            sensors: Value::literal(visibility_data.sensors.unwrap_or(true)),
        };

        Ok(Action::Visibility(visibility_action))
    }

    fn build_controller_action(self) -> BuilderResult<Action> {
        let controller_data = self.action_data.controller_data
            .ok_or_else(|| BuilderError::missing_field("controller_data", "Internal error"))?;

        let controller_type = controller_data.controller_type
            .ok_or_else(|| BuilderError::missing_field(
                "controller_type",
                "Call one of the controller methods (activate_controller, etc.)"
            ))?;

        match controller_type {
            ControllerActionType::Activate => {
                let activate_data = controller_data.activate_data
                    .ok_or_else(|| BuilderError::missing_field("activate_data", "Internal error"))?;

                let activate_action = ActivateControllerAction {
                    lateral: Value::literal(activate_data.lateral.unwrap_or(true)),
                    longitudinal: Value::literal(activate_data.longitudinal.unwrap_or(true)),
                };

                Ok(Action::ActivateController(activate_action))
            }
            ControllerActionType::Assign => {
                // For now, create a basic assign controller action
                let assign_action = AssignControllerAction {
                    catalog_reference: None,
                    properties: None,
                };

                Ok(Action::AssignController(assign_action))
            }
            ControllerActionType::Override => {
                // For now, create a basic override controller action
                let override_action = OverrideControllerValueAction {
                    throttle: None,
                    brake: None,
                    clutch: None,
                    parking_brake: None,
                    steering_wheel: None,
                    gear: None,
                };

                Ok(Action::OverrideControllerValue(override_action))
            }
        }
    }

    fn build_appearance_action(self) -> BuilderResult<Action> {
        let _appearance_data = self.action_data.appearance_data
            .ok_or_else(|| BuilderError::missing_field("appearance_data", "Internal error"))?;

        // For now, create a basic appearance action
        let appearance_action = AppearanceAction {
            animation_action: None,
            light_state_action: None,
        };

        Ok(Action::Appearance(appearance_action))
    }

    fn build_trailer_action(self) -> BuilderResult<Action> {
        let trailer_data = self.action_data.trailer_data
            .ok_or_else(|| BuilderError::missing_field("trailer_data", "Internal error"))?;

        let trailer_type = trailer_data.trailer_type
            .ok_or_else(|| BuilderError::missing_field(
                "trailer_type",
                "Call connect_trailer() or disconnect_trailer()"
            ))?;

        let entity_ref = trailer_data.entity_ref
            .ok_or_else(|| BuilderError::missing_field(
                "entity_ref",
                "Specify the trailer entity reference"
            ))?;

        match trailer_type {
            TrailerActionType::Connect => {
                let connect_action = ConnectTrailerAction {
                    trailer_ref: ActionUtils::os_string(&entity_ref),
                };
                Ok(Action::ConnectTrailer(connect_action))
            }
            TrailerActionType::Disconnect => {
                let disconnect_action = DisconnectTrailerAction {
                    trailer_ref: ActionUtils::os_string(&entity_ref),
                };
                Ok(Action::DisconnectTrailer(disconnect_action))
            }
        }
    }
}

impl<S: BuilderState> ActionBuilderTrait for PrivateActionBuilder<S> {
    fn build_action(self) -> BuilderResult<Action> {
        self.build()
    }

    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::{Position, WorldPosition};
    use crate::types::basic::Value;

    #[test]
    fn test_teleport_action_builder() {
        let position = Position::WorldPosition(WorldPosition {
            x: Value::literal(100.0),
            y: Value::literal(50.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let action = PrivateActionBuilder::new(Some("teleport".to_string()))
            .teleport_action()
            .to_position(position)
            .build()
            .unwrap();

        match action {
            Action::Teleport(teleport_action) => {
                match teleport_action.position {
                    Position::WorldPosition(world_pos) => {
                        assert_eq!(world_pos.x.as_literal().unwrap(), &100.0);
                        assert_eq!(world_pos.y.as_literal().unwrap(), &50.0);
                    }
                    _ => panic!("Expected WorldPosition"),
                }
            }
            _ => panic!("Expected Teleport action"),
        }
    }

    #[test]
    fn test_visibility_action_builder() {
        let action = PrivateActionBuilder::new(None)
            .visibility_action()
            .graphics(true)
            .traffic(false)
            .sensors(true)
            .build()
            .unwrap();

        match action {
            Action::Visibility(visibility_action) => {
                assert_eq!(visibility_action.graphics.as_literal().unwrap(), &true);
                assert_eq!(visibility_action.traffic.as_literal().unwrap(), &false);
                assert_eq!(visibility_action.sensors.as_literal().unwrap(), &true);
            }
            _ => panic!("Expected Visibility action"),
        }
    }

    #[test]
    fn test_activate_controller_action_builder() {
        let action = PrivateActionBuilder::new(None)
            .controller_action()
            .activate_controller()
            .lateral(true)
            .longitudinal(false)
            .build()
            .unwrap();

        match action {
            Action::ActivateController(activate_action) => {
                assert_eq!(activate_action.lateral.as_literal().unwrap(), &true);
                assert_eq!(activate_action.longitudinal.as_literal().unwrap(), &false);
            }
            _ => panic!("Expected ActivateController action"),
        }
    }

    #[test]
    fn test_connect_trailer_action_builder() {
        let action = PrivateActionBuilder::new(None)
            .trailer_action()
            .connect_trailer("trailer_1")
            .build()
            .unwrap();

        match action {
            Action::ConnectTrailer(connect_action) => {
                assert_eq!(connect_action.trailer_ref.as_literal().unwrap(), "trailer_1");
            }
            _ => panic!("Expected ConnectTrailer action"),
        }
    }

    #[test]
    fn test_teleport_action_missing_position() {
        let result = PrivateActionBuilder::new(None)
            .teleport_action()
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("position"));
    }

    #[test]
    fn test_controller_action_missing_type() {
        let result = PrivateActionBuilder::new(None)
            .controller_action()
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("controller_type"));
    }

    #[test]
    fn test_builder_with_name() {
        let builder = PrivateActionBuilder::new(Some("test_private".to_string()));
        assert_eq!(builder.get_name().unwrap(), "test_private");
    }
}
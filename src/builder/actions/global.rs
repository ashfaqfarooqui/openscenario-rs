//! Global action builders for environment and infrastructure actions
//!
//! This module provides builders for global actions that affect the entire
//! scenario environment, including traffic signals, weather, and infrastructure.

use crate::types::actions::{
    Action, TrafficSignalAction, TrafficSignalControllerAction, TrafficSignalStateAction,
    TrafficSourceAction, TrafficSinkAction, TrafficSwarmAction, TrafficAreaAction,
    TrafficStopAction, TrafficSignalController, TrafficSignalState, TrafficSignalGroupState,
    TrafficDefinition, VehicleCategory, VehicleCategoryDistribution, CentralSwarmObject,
    ControllerDistribution, Phase, TrafficArea, TrafficAreaVertex
};
use crate::types::basic::{OSString, Value, Double, UnsignedInt};
use crate::types::enums::{TrafficSignalStateType, VehicleCategoryType};
use crate::types::positions::Position;
use super::{ActionBuilderTrait, ActionUtils};
use crate::builder::error::{BuilderError, BuilderResult};
use crate::builder::states::*;
use std::marker::PhantomData;

/// Builder for global actions (environment and infrastructure actions)
/// 
/// This builder provides a type-safe way to construct global actions
/// that affect the entire scenario environment.
/// 
/// # Type Parameters
/// * `S` - Current builder state
/// 
/// # Example
/// ```rust
/// let action = GlobalActionBuilder::new(Some("traffic_signal".to_string()))
///     .traffic_signal_action()
///     .controller_id("signal_controller_1")
///     .phase_id("green_phase")
///     .build()?;
/// ```
pub struct GlobalActionBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Action name for identification
    name: Option<String>,
    
    /// Partially constructed action data
    action_data: PartialGlobalData,
}

/// Internal data structure for building global actions
#[derive(Debug, Default)]
struct PartialGlobalData {
    /// Action type being built
    action_type: Option<GlobalActionType>,
    
    /// Traffic signal action data
    traffic_signal_data: Option<TrafficSignalActionData>,
    
    /// Traffic source action data
    traffic_source_data: Option<TrafficSourceActionData>,
    
    /// Traffic sink action data
    traffic_sink_data: Option<TrafficSinkActionData>,
    
    /// Traffic swarm action data
    traffic_swarm_data: Option<TrafficSwarmActionData>,
    
    /// Traffic area action data
    traffic_area_data: Option<TrafficAreaActionData>,
    
    /// Traffic stop action data
    traffic_stop_data: Option<TrafficStopActionData>,
}

/// Types of global actions
#[derive(Debug, Clone)]
enum GlobalActionType {
    TrafficSignal,
    TrafficSignalController,
    TrafficSignalState,
    TrafficSource,
    TrafficSink,
    TrafficSwarm,
    TrafficArea,
    TrafficStop,
}

/// Traffic signal action configuration data
#[derive(Debug, Default)]
struct TrafficSignalActionData {
    traffic_signal_controller_action: Option<TrafficSignalControllerActionData>,
    traffic_signal_state_action: Option<TrafficSignalStateActionData>,
}

/// Traffic signal controller action data
#[derive(Debug, Default)]
struct TrafficSignalControllerActionData {
    traffic_signal_controller_ref: Option<String>,
    phase_ref: Option<String>,
}

/// Traffic signal state action data
#[derive(Debug, Default)]
struct TrafficSignalStateActionData {
    name: Option<String>,
    state: Option<TrafficSignalStateType>,
}

/// Traffic source action configuration data
#[derive(Debug, Default)]
struct TrafficSourceActionData {
    rate: Option<f64>,
    radius: Option<f64>,
    velocity: Option<f64>,
    position: Option<Position>,
    traffic_definition: Option<TrafficDefinitionData>,
}

/// Traffic definition data
#[derive(Debug, Default)]
struct TrafficDefinitionData {
    vehicle_category_distributions: Vec<VehicleCategoryDistributionData>,
}

/// Vehicle category distribution data
#[derive(Debug, Default)]
struct VehicleCategoryDistributionData {
    category: Option<VehicleCategoryType>,
    weight: Option<f64>,
}

/// Traffic sink action configuration data
#[derive(Debug, Default)]
struct TrafficSinkActionData {
    radius: Option<f64>,
    rate: Option<f64>,
    position: Option<Position>,
}

/// Traffic swarm action configuration data
#[derive(Debug, Default)]
struct TrafficSwarmActionData {
    central_object: Option<String>,
    number_of_vehicles: Option<u32>,
    inner_radius: Option<f64>,
    outer_radius: Option<f64>,
    velocity: Option<f64>,
}

/// Traffic area action configuration data
#[derive(Debug, Default)]
struct TrafficAreaActionData {
    vertices: Vec<TrafficAreaVertexData>,
}

/// Traffic area vertex data
#[derive(Debug, Default)]
struct TrafficAreaVertexData {
    position: Option<Position>,
}

/// Traffic stop action configuration data
#[derive(Debug, Default)]
struct TrafficStopActionData {
    // Placeholder for traffic stop action properties
}

// Core builder implementation for Empty state
impl GlobalActionBuilder<Empty> {
    /// Create a new global action builder
    /// 
    /// # Arguments
    /// * `name` - Optional action name
    /// 
    /// # Returns
    /// A new GlobalActionBuilder in Empty state
    pub fn new(name: Option<String>) -> Self {
        Self {
            _state: PhantomData,
            name,
            action_data: PartialGlobalData::default(),
        }
    }

    /// Start building a traffic signal action
    /// 
    /// Traffic signal actions control traffic light states and phases.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic signal configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_signal_action()
    ///     .controller_id("signal_1");
    /// ```
    pub fn traffic_signal_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficSignal);
        self.action_data.traffic_signal_data = Some(TrafficSignalActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a traffic source action
    /// 
    /// Traffic source actions generate traffic at specified locations.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic source configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_source_action()
    ///     .rate(10.0)
    ///     .position(position);
    /// ```
    pub fn traffic_source_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficSource);
        self.action_data.traffic_source_data = Some(TrafficSourceActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a traffic sink action
    /// 
    /// Traffic sink actions remove traffic at specified locations.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic sink configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_sink_action()
    ///     .radius(50.0)
    ///     .position(position);
    /// ```
    pub fn traffic_sink_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficSink);
        self.action_data.traffic_sink_data = Some(TrafficSinkActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a traffic swarm action
    /// 
    /// Traffic swarm actions create groups of vehicles around a central object.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic swarm configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_swarm_action()
    ///     .central_object("ego_vehicle")
    ///     .number_of_vehicles(5);
    /// ```
    pub fn traffic_swarm_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficSwarm);
        self.action_data.traffic_swarm_data = Some(TrafficSwarmActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a traffic area action
    /// 
    /// Traffic area actions define traffic behavior within specific areas.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic area configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_area_action()
    ///     .add_vertex(position1)
    ///     .add_vertex(position2);
    /// ```
    pub fn traffic_area_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficArea);
        self.action_data.traffic_area_data = Some(TrafficAreaActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }

    /// Start building a traffic stop action
    /// 
    /// Traffic stop actions halt all traffic in the scenario.
    /// 
    /// # Returns
    /// GlobalActionBuilder in HasType state for traffic stop configuration
    /// 
    /// # Example
    /// ```rust
    /// let builder = GlobalActionBuilder::new(None)
    ///     .traffic_stop_action();
    /// ```
    pub fn traffic_stop_action(mut self) -> GlobalActionBuilder<HasType> {
        self.action_data.action_type = Some(GlobalActionType::TrafficStop);
        self.action_data.traffic_stop_data = Some(TrafficStopActionData::default());
        
        GlobalActionBuilder {
            _state: PhantomData,
            name: self.name,
            action_data: self.action_data,
        }
    }
}

// Methods for configuring global actions
impl GlobalActionBuilder<HasType> {
    /// Set the traffic signal controller ID
    /// 
    /// # Arguments
    /// * `controller_id` - ID of the traffic signal controller
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.controller_id("signal_controller_1")
    /// ```
    pub fn controller_id(mut self, controller_id: &str) -> Self {
        if let Some(ref mut traffic_signal_data) = self.action_data.traffic_signal_data {
            if traffic_signal_data.traffic_signal_controller_action.is_none() {
                traffic_signal_data.traffic_signal_controller_action = Some(TrafficSignalControllerActionData::default());
            }
            if let Some(ref mut controller_data) = traffic_signal_data.traffic_signal_controller_action {
                controller_data.traffic_signal_controller_ref = Some(controller_id.to_string());
            }
        }
        self
    }

    /// Set the traffic signal phase ID
    /// 
    /// # Arguments
    /// * `phase_id` - ID of the traffic signal phase
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.phase_id("green_phase")
    /// ```
    pub fn phase_id(mut self, phase_id: &str) -> Self {
        if let Some(ref mut traffic_signal_data) = self.action_data.traffic_signal_data {
            if traffic_signal_data.traffic_signal_controller_action.is_none() {
                traffic_signal_data.traffic_signal_controller_action = Some(TrafficSignalControllerActionData::default());
            }
            if let Some(ref mut controller_data) = traffic_signal_data.traffic_signal_controller_action {
                controller_data.phase_ref = Some(phase_id.to_string());
            }
        }
        self
    }

    /// Set the traffic signal state
    /// 
    /// # Arguments
    /// * `name` - Name of the traffic signal
    /// * `state` - State of the traffic signal
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.signal_state("main_signal", TrafficSignalStateType::Green)
    /// ```
    pub fn signal_state(mut self, name: &str, state: TrafficSignalStateType) -> Self {
        if let Some(ref mut traffic_signal_data) = self.action_data.traffic_signal_data {
            if traffic_signal_data.traffic_signal_state_action.is_none() {
                traffic_signal_data.traffic_signal_state_action = Some(TrafficSignalStateActionData::default());
            }
            if let Some(ref mut state_data) = traffic_signal_data.traffic_signal_state_action {
                state_data.name = Some(name.to_string());
                state_data.state = Some(state);
            }
        }
        self
    }

    /// Set the traffic generation rate
    /// 
    /// # Arguments
    /// * `rate` - Traffic generation rate (vehicles per second)
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.rate(10.0)
    /// ```
    pub fn rate(mut self, rate: f64) -> Self {
        if let Some(ref mut traffic_source_data) = self.action_data.traffic_source_data {
            traffic_source_data.rate = Some(rate);
        }
        if let Some(ref mut traffic_sink_data) = self.action_data.traffic_sink_data {
            traffic_sink_data.rate = Some(rate);
        }
        self
    }

    /// Set the radius for traffic actions
    /// 
    /// # Arguments
    /// * `radius` - Radius in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.radius(50.0)
    /// ```
    pub fn radius(mut self, radius: f64) -> Self {
        if let Some(ref mut traffic_source_data) = self.action_data.traffic_source_data {
            traffic_source_data.radius = Some(radius);
        }
        if let Some(ref mut traffic_sink_data) = self.action_data.traffic_sink_data {
            traffic_sink_data.radius = Some(radius);
        }
        self
    }

    /// Set the velocity for traffic generation
    /// 
    /// # Arguments
    /// * `velocity` - Initial velocity in m/s
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.velocity(25.0)
    /// ```
    pub fn velocity(mut self, velocity: f64) -> Self {
        if let Some(ref mut traffic_source_data) = self.action_data.traffic_source_data {
            traffic_source_data.velocity = Some(velocity);
        }
        if let Some(ref mut traffic_swarm_data) = self.action_data.traffic_swarm_data {
            traffic_swarm_data.velocity = Some(velocity);
        }
        self
    }

    /// Set the position for traffic actions
    /// 
    /// # Arguments
    /// * `position` - Position for the traffic action
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.position(position)
    /// ```
    pub fn position(mut self, position: Position) -> Self {
        if let Some(ref mut traffic_source_data) = self.action_data.traffic_source_data {
            traffic_source_data.position = Some(position.clone());
        }
        if let Some(ref mut traffic_sink_data) = self.action_data.traffic_sink_data {
            traffic_sink_data.position = Some(position);
        }
        self
    }

    /// Set the central object for traffic swarm
    /// 
    /// # Arguments
    /// * `entity_ref` - Reference to the central entity
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.central_object("ego_vehicle")
    /// ```
    pub fn central_object(mut self, entity_ref: &str) -> Self {
        if let Some(ref mut traffic_swarm_data) = self.action_data.traffic_swarm_data {
            traffic_swarm_data.central_object = Some(entity_ref.to_string());
        }
        self
    }

    /// Set the number of vehicles for traffic swarm
    /// 
    /// # Arguments
    /// * `count` - Number of vehicles to generate
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.number_of_vehicles(5)
    /// ```
    pub fn number_of_vehicles(mut self, count: u32) -> Self {
        if let Some(ref mut traffic_swarm_data) = self.action_data.traffic_swarm_data {
            traffic_swarm_data.number_of_vehicles = Some(count);
        }
        self
    }

    /// Set the inner radius for traffic swarm
    /// 
    /// # Arguments
    /// * `radius` - Inner radius in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.inner_radius(10.0)
    /// ```
    pub fn inner_radius(mut self, radius: f64) -> Self {
        if let Some(ref mut traffic_swarm_data) = self.action_data.traffic_swarm_data {
            traffic_swarm_data.inner_radius = Some(radius);
        }
        self
    }

    /// Set the outer radius for traffic swarm
    /// 
    /// # Arguments
    /// * `radius` - Outer radius in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.outer_radius(50.0)
    /// ```
    pub fn outer_radius(mut self, radius: f64) -> Self {
        if let Some(ref mut traffic_swarm_data) = self.action_data.traffic_swarm_data {
            traffic_swarm_data.outer_radius = Some(radius);
        }
        self
    }

    /// Add a vertex to traffic area
    /// 
    /// # Arguments
    /// * `position` - Position of the vertex
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_vertex(position)
    /// ```
    pub fn add_vertex(mut self, position: Position) -> Self {
        if let Some(ref mut traffic_area_data) = self.action_data.traffic_area_data {
            traffic_area_data.vertices.push(TrafficAreaVertexData {
                position: Some(position),
            });
        }
        self
    }

    /// Add a vehicle category distribution for traffic generation
    /// 
    /// # Arguments
    /// * `category` - Vehicle category type
    /// * `weight` - Distribution weight
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_vehicle_category(VehicleCategoryType::Car, 0.8)
    /// ```
    pub fn add_vehicle_category(mut self, category: VehicleCategoryType, weight: f64) -> Self {
        if let Some(ref mut traffic_source_data) = self.action_data.traffic_source_data {
            if traffic_source_data.traffic_definition.is_none() {
                traffic_source_data.traffic_definition = Some(TrafficDefinitionData::default());
            }
            if let Some(ref mut traffic_def) = traffic_source_data.traffic_definition {
                traffic_def.vehicle_category_distributions.push(VehicleCategoryDistributionData {
                    category: Some(category),
                    weight: Some(weight),
                });
            }
        }
        self
    }
}

// Build implementation
impl<S: BuilderState> GlobalActionBuilder<S> {
    /// Build the final global action
    /// 
    /// # Returns
    /// Complete Action or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    pub fn build(self) -> BuilderResult<Action> {
        match self.action_data.action_type {
            Some(GlobalActionType::TrafficSignal) => self.build_traffic_signal_action(),
            Some(GlobalActionType::TrafficSource) => self.build_traffic_source_action(),
            Some(GlobalActionType::TrafficSink) => self.build_traffic_sink_action(),
            Some(GlobalActionType::TrafficSwarm) => self.build_traffic_swarm_action(),
            Some(GlobalActionType::TrafficArea) => self.build_traffic_area_action(),
            Some(GlobalActionType::TrafficStop) => self.build_traffic_stop_action(),
            _ => Err(BuilderError::missing_field(
                "action_type",
                "Call one of the action type methods first (traffic_signal_action, etc.)"
            )),
        }
    }

    fn build_traffic_signal_action(self) -> BuilderResult<Action> {
        let traffic_signal_data = self.action_data.traffic_signal_data
            .ok_or_else(|| BuilderError::missing_field("traffic_signal_data", "Internal error"))?;

        // For now, build a basic traffic signal controller action
        if let Some(controller_data) = traffic_signal_data.traffic_signal_controller_action {
            let controller_ref = controller_data.traffic_signal_controller_ref
                .ok_or_else(|| BuilderError::missing_field(
                    "traffic_signal_controller_ref",
                    "Call .controller_id() to set the controller reference"
                ))?;

            let traffic_signal_controller_action = TrafficSignalControllerAction {
                traffic_signal_controller_ref: ActionUtils::os_string(&controller_ref),
                phase_ref: controller_data.phase_ref.map(|p| ActionUtils::os_string(&p)),
            };

            return Ok(Action::TrafficSignalController(traffic_signal_controller_action));
        }

        // Build traffic signal state action if configured
        if let Some(state_data) = traffic_signal_data.traffic_signal_state_action {
            let name = state_data.name
                .ok_or_else(|| BuilderError::missing_field(
                    "name",
                    "Call .signal_state() to set the signal name and state"
                ))?;

            let state = state_data.state
                .ok_or_else(|| BuilderError::missing_field(
                    "state",
                    "Call .signal_state() to set the signal state"
                ))?;

            let traffic_signal_state_action = TrafficSignalStateAction {
                name: ActionUtils::os_string(&name),
                state: TrafficSignalState {
                    traffic_signal_id: ActionUtils::os_string(&name),
                    state: ActionUtils::os_string(&format!("{:?}", state)),
                },
            };

            return Ok(Action::TrafficSignalState(traffic_signal_state_action));
        }

        Err(BuilderError::missing_field(
            "traffic_signal_configuration",
            "Configure either controller or state action"
        ))
    }

    fn build_traffic_source_action(self) -> BuilderResult<Action> {
        let traffic_source_data = self.action_data.traffic_source_data
            .ok_or_else(|| BuilderError::missing_field("traffic_source_data", "Internal error"))?;

        let rate = traffic_source_data.rate
            .ok_or_else(|| BuilderError::missing_field(
                "rate",
                "Call .rate() to set the traffic generation rate"
            ))?;

        let position = traffic_source_data.position
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .position() to set the traffic source position"
            ))?;

        let traffic_source_action = TrafficSourceAction {
            rate: ActionUtils::double(rate),
            radius: traffic_source_data.radius.map(|r| ActionUtils::double(r)),
            velocity: traffic_source_data.velocity.map(|v| ActionUtils::double(v)),
            position,
            traffic_definition: None, // Simplified for now
        };

        Ok(Action::TrafficSource(traffic_source_action))
    }

    fn build_traffic_sink_action(self) -> BuilderResult<Action> {
        let traffic_sink_data = self.action_data.traffic_sink_data
            .ok_or_else(|| BuilderError::missing_field("traffic_sink_data", "Internal error"))?;

        let position = traffic_sink_data.position
            .ok_or_else(|| BuilderError::missing_field(
                "position",
                "Call .position() to set the traffic sink position"
            ))?;

        let traffic_sink_action = TrafficSinkAction {
            radius: traffic_sink_data.radius.map(|r| ActionUtils::double(r)),
            rate: traffic_sink_data.rate.map(|r| ActionUtils::double(r)),
            position,
        };

        Ok(Action::TrafficSink(traffic_sink_action))
    }

    fn build_traffic_swarm_action(self) -> BuilderResult<Action> {
        let traffic_swarm_data = self.action_data.traffic_swarm_data
            .ok_or_else(|| BuilderError::missing_field("traffic_swarm_data", "Internal error"))?;

        let central_object = traffic_swarm_data.central_object
            .ok_or_else(|| BuilderError::missing_field(
                "central_object",
                "Call .central_object() to set the central entity"
            ))?;

        let number_of_vehicles = traffic_swarm_data.number_of_vehicles
            .ok_or_else(|| BuilderError::missing_field(
                "number_of_vehicles",
                "Call .number_of_vehicles() to set the vehicle count"
            ))?;

        let traffic_swarm_action = TrafficSwarmAction {
            central_swarm_object: CentralSwarmObject {
                entity_ref: ActionUtils::os_string(&central_object),
            },
            number_of_vehicles: ActionUtils::unsigned_int(number_of_vehicles),
            inner_radius: traffic_swarm_data.inner_radius.map(|r| ActionUtils::double(r)),
            outer_radius: traffic_swarm_data.outer_radius.map(|r| ActionUtils::double(r)),
            velocity: traffic_swarm_data.velocity.map(|v| ActionUtils::double(v)),
            semi_major_axis: None,
            semi_minor_axis: None,
        };

        Ok(Action::TrafficSwarm(traffic_swarm_action))
    }

    fn build_traffic_area_action(self) -> BuilderResult<Action> {
        let traffic_area_data = self.action_data.traffic_area_data
            .ok_or_else(|| BuilderError::missing_field("traffic_area_data", "Internal error"))?;

        if traffic_area_data.vertices.is_empty() {
            return Err(BuilderError::missing_field(
                "vertices",
                "Call .add_vertex() to add at least one vertex"
            ));
        }

        let vertices: Result<Vec<_>, _> = traffic_area_data.vertices
            .into_iter()
            .map(|v| {
                v.position.ok_or_else(|| BuilderError::missing_field(
                    "vertex_position",
                    "All vertices must have positions"
                ))
            })
            .collect();

        let vertices = vertices?;
        let traffic_area_vertices: Vec<TrafficAreaVertex> = vertices
            .into_iter()
            .map(|pos| TrafficAreaVertex { position: pos })
            .collect();

        let traffic_area_action = TrafficAreaAction {
            traffic_area: TrafficArea {
                vertices: traffic_area_vertices,
            },
        };

        Ok(Action::TrafficArea(traffic_area_action))
    }

    fn build_traffic_stop_action(self) -> BuilderResult<Action> {
        let _traffic_stop_data = self.action_data.traffic_stop_data
            .ok_or_else(|| BuilderError::missing_field("traffic_stop_data", "Internal error"))?;

        let traffic_stop_action = TrafficStopAction {
            // Basic traffic stop action
        };

        Ok(Action::TrafficStop(traffic_stop_action))
    }
}

impl<S: BuilderState> ActionBuilderTrait for GlobalActionBuilder<S> {
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
    use crate::types::enums::{TrafficSignalStateType, VehicleCategoryType};

    #[test]
    fn test_traffic_signal_controller_action_builder() {
        let action = GlobalActionBuilder::new(Some("traffic_signal".to_string()))
            .traffic_signal_action()
            .controller_id("signal_controller_1")
            .phase_id("green_phase")
            .build()
            .unwrap();

        match action {
            Action::TrafficSignalController(controller_action) => {
                assert_eq!(controller_action.traffic_signal_controller_ref.as_literal().unwrap(), "signal_controller_1");
                assert_eq!(controller_action.phase_ref.as_ref().unwrap().as_literal().unwrap(), "green_phase");
            }
            _ => panic!("Expected TrafficSignalController action"),
        }
    }

    #[test]
    fn test_traffic_source_action_builder() {
        let position = Position::WorldPosition(WorldPosition {
            x: Value::literal(100.0),
            y: Value::literal(50.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let action = GlobalActionBuilder::new(None)
            .traffic_source_action()
            .rate(10.0)
            .radius(25.0)
            .velocity(30.0)
            .position(position)
            .build()
            .unwrap();

        match action {
            Action::TrafficSource(source_action) => {
                assert_eq!(source_action.rate.as_literal().unwrap(), &10.0);
                assert_eq!(source_action.radius.as_ref().unwrap().as_literal().unwrap(), &25.0);
                assert_eq!(source_action.velocity.as_ref().unwrap().as_literal().unwrap(), &30.0);
            }
            _ => panic!("Expected TrafficSource action"),
        }
    }

    #[test]
    fn test_traffic_swarm_action_builder() {
        let action = GlobalActionBuilder::new(None)
            .traffic_swarm_action()
            .central_object("ego_vehicle")
            .number_of_vehicles(5)
            .inner_radius(10.0)
            .outer_radius(50.0)
            .velocity(25.0)
            .build()
            .unwrap();

        match action {
            Action::TrafficSwarm(swarm_action) => {
                assert_eq!(swarm_action.central_swarm_object.entity_ref.as_literal().unwrap(), "ego_vehicle");
                assert_eq!(swarm_action.number_of_vehicles.as_literal().unwrap(), &5);
                assert_eq!(swarm_action.inner_radius.as_ref().unwrap().as_literal().unwrap(), &10.0);
                assert_eq!(swarm_action.outer_radius.as_ref().unwrap().as_literal().unwrap(), &50.0);
            }
            _ => panic!("Expected TrafficSwarm action"),
        }
    }

    #[test]
    fn test_traffic_area_action_builder() {
        let position1 = Position::WorldPosition(WorldPosition {
            x: Value::literal(0.0),
            y: Value::literal(0.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let position2 = Position::WorldPosition(WorldPosition {
            x: Value::literal(100.0),
            y: Value::literal(100.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let action = GlobalActionBuilder::new(None)
            .traffic_area_action()
            .add_vertex(position1)
            .add_vertex(position2)
            .build()
            .unwrap();

        match action {
            Action::TrafficArea(area_action) => {
                assert_eq!(area_action.traffic_area.vertices.len(), 2);
            }
            _ => panic!("Expected TrafficArea action"),
        }
    }

    #[test]
    fn test_traffic_source_missing_rate() {
        let position = Position::WorldPosition(WorldPosition {
            x: Value::literal(0.0),
            y: Value::literal(0.0),
            z: None,
            h: Value::literal(0.0),
            p: None,
            r: None,
        });

        let result = GlobalActionBuilder::new(None)
            .traffic_source_action()
            .position(position)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rate"));
    }

    #[test]
    fn test_traffic_swarm_missing_central_object() {
        let result = GlobalActionBuilder::new(None)
            .traffic_swarm_action()
            .number_of_vehicles(5)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("central_object"));
    }

    #[test]
    fn test_builder_with_name() {
        let builder = GlobalActionBuilder::new(Some("test_global".to_string()));
        assert_eq!(builder.get_name().unwrap(), "test_global");
    }
}
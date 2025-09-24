//! Core scenario builder for programmatic scenario construction
//!
//! This module provides the main ScenarioBuilder with type-safe state transitions,
//! comprehensive validation, and integration with the existing type system.
//!
//! # Type Safety
//!
//! - Compile-time prevention of invalid state transitions
//! - Rich error handling with helpful suggestions
//! - Comprehensive validation with registry tracking
//! - Integration with existing OpenSCENARIO type system

use super::{
    error::{BuilderError, BuilderResult},
    registry::{CatalogRegistry, EntityRegistry, ParameterRegistry},
    states::*,
};
use crate::types::{
    basic::{ParameterDeclarations, Value},
    catalogs::locations::CatalogLocations,
    entities::Entities,
    road::{LogicFile, RoadNetwork, SceneGraphFile},
    scenario::{
        init::Init,
        monitors::MonitorDeclarations,
        storyboard::{FileHeader, OpenScenario, Storyboard},
        variables::VariableDeclarations,
    },
};
use std::marker::PhantomData;

/// Main scenario builder with type-safe state transitions
pub struct ScenarioBuilder<S: BuilderState> {
    _state: PhantomData<S>,
    scenario_data: PartialScenarioData,
    entity_registry: EntityRegistry,
    parameter_registry: ParameterRegistry,
    catalog_registry: CatalogRegistry,
}

/// Internal data structure for building scenarios
///
/// Optimized for cache efficiency with frequently accessed fields first
#[derive(Debug, Default)]
struct PartialScenarioData {
    // Most frequently accessed fields first for cache efficiency
    file_header: Option<FileHeader>,
    entities: Option<Entities>,
    storyboard: Option<Storyboard>,
    road_network: Option<RoadNetwork>,

    // Less frequently accessed fields
    catalog_locations: Option<CatalogLocations>,
    parameter_declarations: Option<ParameterDeclarations>,
    variable_declarations: Option<VariableDeclarations>,
    monitor_declarations: Option<MonitorDeclarations>,
}

// Forward declarations for entity builders (to be implemented in future phases)
/// Vehicle builder for adding vehicles to scenarios
pub struct VehicleBuilder {
    // Implementation will be added in future phases
    _placeholder: PhantomData<()>,
}

/// Pedestrian builder for adding pedestrians to scenarios
pub struct PedestrianBuilder {
    // Implementation will be added in future phases
    _placeholder: PhantomData<()>,
}

impl ScenarioBuilder<Empty> {
    /// Create a new scenario builder
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            scenario_data: PartialScenarioData::default(),
            entity_registry: EntityRegistry::new(),
            parameter_registry: ParameterRegistry::new(),
            catalog_registry: CatalogRegistry::new(),
        }
    }

    /// Set the file header and transition to HasHeader state
    pub fn with_header(
        mut self,
        author: String,
        date: String,
        description: String,
        rev_major: u16,
        rev_minor: u16,
    ) -> ScenarioBuilder<HasHeader> {
        let header = FileHeader {
            author: Value::literal(author),
            date: Value::literal(date),
            description: Value::literal(description),
            rev_major: Value::literal(rev_major),
            rev_minor: Value::literal(rev_minor),
        };

        self.scenario_data.file_header = Some(header);

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Set a simple header with default values
    pub fn with_simple_header(self, description: &str, author: &str) -> ScenarioBuilder<HasHeader> {
        self.with_header(
            author.to_string(),
            "2024-01-01T00:00:00".to_string(),
            description.to_string(),
            1,
            0,
        )
    }
}

impl<S: AfterHeader> ScenarioBuilder<S> {
    /// Add parameter declarations
    pub fn with_parameters(mut self, parameters: ParameterDeclarations) -> BuilderResult<Self> {
        // Register parameters for validation
        self.parameter_registry
            .add_parameter_declarations(&parameters)?;
        self.scenario_data.parameter_declarations = Some(parameters);
        Ok(self)
    }

    /// Add variable declarations
    pub fn with_variables(mut self, variables: VariableDeclarations) -> Self {
        self.scenario_data.variable_declarations = Some(variables);
        self
    }

    /// Add monitor declarations
    pub fn with_monitors(mut self, monitors: MonitorDeclarations) -> Self {
        self.scenario_data.monitor_declarations = Some(monitors);
        self
    }
}

impl ScenarioBuilder<HasHeader> {
    /// Set catalog locations and transition to HasCatalogLocations state
    pub fn with_catalog_locations(
        mut self,
        locations: CatalogLocations,
    ) -> BuilderResult<ScenarioBuilder<HasCatalogLocations>> {
        // Register catalog locations for validation
        self.catalog_registry
            .set_catalog_locations(locations.clone())?;
        self.scenario_data.catalog_locations = Some(locations);

        Ok(ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        })
    }

    /// Set default catalog locations (empty)
    pub fn with_default_catalogs(self) -> BuilderResult<ScenarioBuilder<HasCatalogLocations>> {
        self.with_catalog_locations(CatalogLocations::default())
    }
}

impl ScenarioBuilder<HasCatalogLocations> {
    /// Set road network and transition to HasRoadNetwork state
    pub fn with_road_network_from_logic_file(
        mut self,
        logic_file_path: String,
    ) -> ScenarioBuilder<HasRoadNetwork> {
        let road_network = RoadNetwork::new(LogicFile::new(logic_file_path));
        self.scenario_data.road_network = Some(road_network);

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Set road network with both logic file and scene graph file
    pub fn with_road_network_full(
        mut self,
        logic_file_path: String,
        scene_graph_path: Option<String>,
    ) -> ScenarioBuilder<HasRoadNetwork> {
        let mut road_network = RoadNetwork::new(LogicFile::new(logic_file_path));

        if let Some(scene_path) = scene_graph_path {
            road_network.scene_graph_file = Some(SceneGraphFile::new(scene_path));
        }

        self.scenario_data.road_network = Some(road_network);

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Set road network from a string path (convenience method)
    pub fn with_road_network(self, logic_file_path: &str) -> ScenarioBuilder<HasRoadNetwork> {
        self.with_road_network_from_logic_file(logic_file_path.to_string())
    }
}

impl ScenarioBuilder<HasRoadNetwork> {
    /// Initialize entities section and transition to HasEntities state
    pub fn with_entities(mut self) -> ScenarioBuilder<HasEntities> {
        self.scenario_data.entities = Some(Entities::new());

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }
}
impl<S: CanAddEntities> ScenarioBuilder<S> {
    /// Add a vehicle entity (placeholder - returns VehicleBuilder for future implementation)
    pub fn add_vehicle(&mut self, _name: &str) -> VehicleBuilder {
        // In future phases, this will:
        // 1. Create a VehicleBuilder
        // 2. Allow configuration of the vehicle
        // 3. Add the vehicle to the entity registry when complete

        VehicleBuilder {
            _placeholder: PhantomData,
        }
    }

    /// Add a pedestrian entity (placeholder - returns PedestrianBuilder for future implementation)
    pub fn add_pedestrian(&mut self, _name: &str) -> PedestrianBuilder {
        // In future phases, this will:
        // 1. Create a PedestrianBuilder
        // 2. Allow configuration of the pedestrian
        // 3. Add the pedestrian to the entity registry when complete

        PedestrianBuilder {
            _placeholder: PhantomData,
        }
    }

    /// Add a storyboard to the scenario
    pub fn with_storyboard(mut self) -> ScenarioStoryboardBuilder<S> {
        ScenarioStoryboardBuilder::new(self)
    }
}

impl<S: CanBuild> ScenarioBuilder<S> {
    /// Build the complete scenario document
    pub fn build(self) -> BuilderResult<OpenScenario> {
        self.build_internal(false)
    }

    /// Build the scenario with strict validation
    pub fn build_validated(self) -> BuilderResult<OpenScenario> {
        self.build_internal(true)
    }

    /// Internal build method with validation control
    fn build_internal(self, strict_validation: bool) -> BuilderResult<OpenScenario> {
        // Validate required fields
        let file_header = self.scenario_data.file_header.ok_or_else(|| {
            BuilderError::missing_field("file_header", "Call with_header() first")
        })?;

        let catalog_locations = self.scenario_data.catalog_locations.ok_or_else(|| {
            BuilderError::missing_field("catalog_locations", "Call with_catalog_locations() first")
        })?;

        let road_network = self.scenario_data.road_network.ok_or_else(|| {
            BuilderError::missing_field("road_network", "Call with_road_network() first")
        })?;

        let entities = self
            .scenario_data
            .entities
            .ok_or_else(|| BuilderError::missing_field("entities", "Call with_entities() first"))?;

        // In strict mode, validate entity count
        if strict_validation && entities.scenario_objects.is_empty() {
            return Err(BuilderError::validation_error(
                "No entities defined in scenario",
                "Add at least one entity using add_vehicle() or add_pedestrian()",
            ));
        }

        // Create default storyboard if not provided
        let storyboard = self.scenario_data.storyboard.unwrap_or_else(|| Storyboard {
            init: Init::default(),
            stories: Vec::new(),
            stop_trigger: None,
        });

        // Build the complete OpenScenario document
        Ok(OpenScenario {
            file_header,
            parameter_declarations: self.scenario_data.parameter_declarations,
            variable_declarations: self.scenario_data.variable_declarations,
            monitor_declarations: self.scenario_data.monitor_declarations,
            catalog_locations: Some(catalog_locations),
            road_network: Some(road_network),
            entities: Some(entities),
            storyboard: Some(storyboard),
            parameter_value_distribution: None,
            catalog: None,
        })
    }
}

impl<S: BuilderState> ScenarioBuilder<S> {
    /// Get the current entity registry (for inspection)
    pub fn entity_registry(&self) -> &EntityRegistry {
        &self.entity_registry
    }

    /// Get the current parameter registry (for inspection)
    pub fn parameter_registry(&self) -> &ParameterRegistry {
        &self.parameter_registry
    }

    /// Get the current catalog registry (for inspection)
    pub fn catalog_registry(&self) -> &CatalogRegistry {
        &self.catalog_registry
    }
}

// Implement Default for Empty state
impl Default for ScenarioBuilder<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Clone for all states (needed for some use cases)
impl<S: BuilderState> Clone for ScenarioBuilder<S> {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone that doesn't preserve registry state
        // In a full implementation, we'd need to clone the registries as well
        Self {
            _state: PhantomData,
            scenario_data: PartialScenarioData::default(),
            entity_registry: EntityRegistry::new(),
            parameter_registry: ParameterRegistry::new(),
            catalog_registry: CatalogRegistry::new(),
        }
    }
}

/// Builder for integrating storyboards with scenarios
pub struct ScenarioStoryboardBuilder<S: BuilderState> {
    scenario_builder: ScenarioBuilder<S>,
    storyboard_builder: crate::builder::storyboard::StoryboardBuilder,
}

impl<S: CanAddEntities> ScenarioStoryboardBuilder<S> {
    /// Create a new scenario storyboard builder
    fn new(scenario_builder: ScenarioBuilder<S>) -> Self {
        Self {
            scenario_builder,
            storyboard_builder: crate::builder::storyboard::StoryboardBuilder::new(),
        }
    }

    /// Add an init action to the storyboard
    pub fn add_init_action(mut self) -> ScenarioInitActionBuilder<S> {
        ScenarioInitActionBuilder::new(self)
    }

    /// Add a story to the storyboard
    pub fn add_story(mut self, name: impl Into<String>) -> ScenarioStoryBuilder<S> {
        ScenarioStoryBuilder::new(self, name.into())
    }

    /// Add a stop trigger to the storyboard
    pub fn with_stop_trigger(mut self) -> ScenarioStoryboardTriggerBuilder<S> {
        ScenarioStoryboardTriggerBuilder::new(self)
    }

    /// Finish building the storyboard and return to scenario builder
    pub fn finish_storyboard(mut self) -> BuilderResult<ScenarioBuilder<S>> {
        let storyboard = self.storyboard_builder.finish_storyboard()?;
        self.scenario_builder.scenario_data.storyboard = Some(storyboard);
        Ok(self.scenario_builder)
    }

    /// Internal method to update the storyboard builder
    fn set_storyboard_builder(
        mut self,
        storyboard_builder: crate::builder::storyboard::StoryboardBuilder,
    ) -> Self {
        self.storyboard_builder = storyboard_builder;
        self
    }
}

/// Builder for init actions within scenario storyboards
pub struct ScenarioInitActionBuilder<S: BuilderState> {
    parent: ScenarioStoryboardBuilder<S>,
}

impl<S: CanAddEntities> ScenarioInitActionBuilder<S> {
    fn new(parent: ScenarioStoryboardBuilder<S>) -> Self {
        Self { parent }
    }

    /// Add a teleport action
    pub fn teleport(
        mut self,
        entity_ref: impl Into<String>,
    ) -> ScenarioInitTeleportActionBuilder<S> {
        ScenarioInitTeleportActionBuilder::new(self, entity_ref.into())
    }

    /// Add a speed action
    pub fn speed(mut self, entity_ref: impl Into<String>) -> ScenarioInitSpeedActionBuilder<S> {
        ScenarioInitSpeedActionBuilder::new(self, entity_ref.into())
    }

    /// Finish building init actions
    pub fn finish_init(self) -> ScenarioStoryboardBuilder<S> {
        // For now, just return the parent
        // In a full implementation, we'd collect the init actions
        self.parent
    }
}

/// Builder for teleport actions in scenario init
pub struct ScenarioInitTeleportActionBuilder<S: BuilderState> {
    parent: ScenarioInitActionBuilder<S>,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl<S: CanAddEntities> ScenarioInitTeleportActionBuilder<S> {
    fn new(parent: ScenarioInitActionBuilder<S>, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            position: None,
        }
    }

    /// Set the target position using world coordinates
    pub fn to_world_position(mut self, x: f64, y: f64, z: f64) -> Self {
        use crate::types::basic::Value;
        use crate::types::positions::{Position, WorldPosition};

        let world_pos = WorldPosition {
            x: Value::literal(x),
            y: Value::literal(y),
            z: Some(Value::literal(z)),
            h: Some(Value::literal(0.0)),
            p: Some(Value::literal(0.0)),
            r: Some(Value::literal(0.0)),
        };

        self.position = Some(Position {
            world_position: Some(world_pos),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        });
        self
    }

    /// Finish building the teleport action
    pub fn finish_action(self) -> BuilderResult<ScenarioInitActionBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the teleport action to the init
        Ok(self.parent)
    }
}

// Position setting methods for ScenarioInitTeleportActionBuilder
impl<S: CanAddEntities> ScenarioInitTeleportActionBuilder<S> {
    /// Set position directly
    pub fn with_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// Builder for speed actions in scenario init
pub struct ScenarioInitSpeedActionBuilder<S: BuilderState> {
    parent: ScenarioInitActionBuilder<S>,
    entity_ref: String,
    speed: Option<f64>,
}

impl<S: CanAddEntities> ScenarioInitSpeedActionBuilder<S> {
    fn new(parent: ScenarioInitActionBuilder<S>, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            speed: None,
        }
    }

    /// Set the initial speed
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    /// Finish building the speed action
    pub fn finish_action(self) -> BuilderResult<ScenarioInitActionBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the speed action to the init
        Ok(self.parent)
    }
}

/// Builder for stories within scenario storyboards
pub struct ScenarioStoryBuilder<S: BuilderState> {
    parent: ScenarioStoryboardBuilder<S>,
    name: String,
}

impl<S: CanAddEntities> ScenarioStoryBuilder<S> {
    fn new(parent: ScenarioStoryboardBuilder<S>, name: String) -> Self {
        Self { parent, name }
    }

    /// Add an act to this story
    pub fn add_act(mut self, name: impl Into<String>) -> ScenarioActBuilder<S> {
        ScenarioActBuilder::new(self, name.into())
    }

    /// Finish building the story
    pub fn finish_story(self) -> BuilderResult<ScenarioStoryboardBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the story to the storyboard
        Ok(self.parent)
    }
}

/// Builder for acts within scenario stories
pub struct ScenarioActBuilder<S: BuilderState> {
    parent: ScenarioStoryBuilder<S>,
    name: String,
}

impl<S: CanAddEntities> ScenarioActBuilder<S> {
    fn new(parent: ScenarioStoryBuilder<S>, name: String) -> Self {
        Self { parent, name }
    }

    /// Add a maneuver group to this act
    pub fn add_maneuver_group(
        mut self,
        name: impl Into<String>,
    ) -> ScenarioManeuverGroupBuilder<S> {
        ScenarioManeuverGroupBuilder::new(self, name.into())
    }

    /// Finish building the act
    pub fn finish_act(self) -> BuilderResult<ScenarioStoryBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the act to the story
        Ok(self.parent)
    }
}

/// Builder for maneuver groups within scenario acts
pub struct ScenarioManeuverGroupBuilder<S: BuilderState> {
    parent: ScenarioActBuilder<S>,
    name: String,
}

impl<S: CanAddEntities> ScenarioManeuverGroupBuilder<S> {
    fn new(parent: ScenarioActBuilder<S>, name: String) -> Self {
        Self { parent, name }
    }

    /// Add a maneuver to this group
    pub fn add_maneuver(mut self, name: impl Into<String>) -> ScenarioManeuverBuilder<S> {
        ScenarioManeuverBuilder::new(self, name.into())
    }

    /// Finish building the maneuver group
    pub fn finish_maneuver_group(self) -> BuilderResult<ScenarioActBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the maneuver group to the act
        Ok(self.parent)
    }
}

/// Builder for maneuvers within scenario maneuver groups
pub struct ScenarioManeuverBuilder<S: BuilderState> {
    parent: ScenarioManeuverGroupBuilder<S>,
    name: String,
}

impl<S: CanAddEntities> ScenarioManeuverBuilder<S> {
    fn new(parent: ScenarioManeuverGroupBuilder<S>, name: String) -> Self {
        Self { parent, name }
    }

    /// Add an event to this maneuver
    pub fn add_event(mut self, name: impl Into<String>) -> ScenarioEventBuilder<S> {
        ScenarioEventBuilder::new(self, name.into())
    }

    /// Finish building the maneuver
    pub fn finish_maneuver(self) -> BuilderResult<ScenarioManeuverGroupBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the maneuver to the group
        Ok(self.parent)
    }
}

/// Builder for events within scenario maneuvers
pub struct ScenarioEventBuilder<S: BuilderState> {
    parent: ScenarioManeuverBuilder<S>,
    name: String,
}

impl<S: CanAddEntities> ScenarioEventBuilder<S> {
    fn new(parent: ScenarioManeuverBuilder<S>, name: String) -> Self {
        Self { parent, name }
    }

    /// Add an action to this event
    pub fn add_action(mut self) -> ScenarioEventActionBuilder<S> {
        ScenarioEventActionBuilder::new(self)
    }

    /// Add a start trigger to this event
    pub fn with_start_trigger(mut self) -> ScenarioEventTriggerBuilder<S> {
        ScenarioEventTriggerBuilder::new(self)
    }

    /// Finish building the event
    pub fn finish_event(self) -> BuilderResult<ScenarioManeuverBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the event to the maneuver
        Ok(self.parent)
    }
}

/// Builder for actions within scenario events
pub struct ScenarioEventActionBuilder<S: BuilderState> {
    parent: ScenarioEventBuilder<S>,
}

impl<S: CanAddEntities> ScenarioEventActionBuilder<S> {
    fn new(parent: ScenarioEventBuilder<S>) -> Self {
        Self { parent }
    }

    /// Add a longitudinal action
    pub fn longitudinal(
        mut self,
        entity_ref: impl Into<String>,
    ) -> ScenarioLongitudinalActionBuilder<S> {
        ScenarioLongitudinalActionBuilder::new(self, entity_ref.into())
    }

    /// Add a teleport action
    pub fn teleport(mut self, entity_ref: impl Into<String>) -> ScenarioTeleportActionBuilder<S> {
        ScenarioTeleportActionBuilder::new(self, entity_ref.into())
    }

    /// Finish building the action
    pub fn finish_action(self) -> ScenarioEventBuilder<S> {
        // For now, just return the parent
        // In a full implementation, we'd add the action to the event
        self.parent
    }
}

/// Builder for longitudinal actions within scenario events
pub struct ScenarioLongitudinalActionBuilder<S: BuilderState> {
    parent: ScenarioEventActionBuilder<S>,
    entity_ref: String,
}

impl<S: CanAddEntities> ScenarioLongitudinalActionBuilder<S> {
    fn new(parent: ScenarioEventActionBuilder<S>, entity_ref: String) -> Self {
        Self { parent, entity_ref }
    }

    /// Create a speed action
    pub fn speed_action(mut self) -> ScenarioSpeedActionBuilder<S> {
        ScenarioSpeedActionBuilder::new(self)
    }

    /// Finish building the longitudinal action
    pub fn finish_action(self) -> BuilderResult<ScenarioEventActionBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the longitudinal action to the event
        Ok(self.parent)
    }
}

/// Builder for speed actions within scenario longitudinal actions
pub struct ScenarioSpeedActionBuilder<S: BuilderState> {
    parent: ScenarioLongitudinalActionBuilder<S>,
    target: Option<f64>,
}

impl<S: CanAddEntities> ScenarioSpeedActionBuilder<S> {
    fn new(parent: ScenarioLongitudinalActionBuilder<S>) -> Self {
        Self {
            parent,
            target: None,
        }
    }

    /// Set absolute target speed
    pub fn absolute_target(mut self, speed: f64) -> Self {
        self.target = Some(speed);
        self
    }

    /// Finish building the speed action
    pub fn finish_action(self) -> BuilderResult<ScenarioLongitudinalActionBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd configure the speed action
        Ok(self.parent)
    }
}

/// Builder for teleport actions within scenario events
pub struct ScenarioTeleportActionBuilder<S: BuilderState> {
    parent: ScenarioEventActionBuilder<S>,
    entity_ref: String,
    position: Option<crate::types::positions::Position>,
}

impl<S: CanAddEntities> ScenarioTeleportActionBuilder<S> {
    fn new(parent: ScenarioEventActionBuilder<S>, entity_ref: String) -> Self {
        Self {
            parent,
            entity_ref,
            position: None,
        }
    }

    /// Set the target position using world coordinates
    pub fn to_world_position(mut self, x: f64, y: f64, z: f64) -> Self {
        use crate::types::basic::Value;
        use crate::types::positions::{Position, WorldPosition};

        let world_pos = WorldPosition {
            x: Value::literal(x),
            y: Value::literal(y),
            z: Some(Value::literal(z)),
            h: Some(Value::literal(0.0)),
            p: Some(Value::literal(0.0)),
            r: Some(Value::literal(0.0)),
        };

        self.position = Some(Position {
            world_position: Some(world_pos),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        });
        self
    }

    /// Finish building the teleport action
    pub fn finish_action(self) -> BuilderResult<ScenarioEventActionBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the teleport action to the event
        Ok(self.parent)
    }
}

// Position setting methods for ScenarioTeleportActionBuilder
impl<S: CanAddEntities> ScenarioTeleportActionBuilder<S> {
    /// Set position directly
    pub fn with_position(mut self, position: crate::types::positions::Position) -> Self {
        self.position = Some(position);
        self
    }
}

/// Builder for triggers within scenario events
pub struct ScenarioEventTriggerBuilder<S: BuilderState> {
    parent: ScenarioEventBuilder<S>,
}

impl<S: CanAddEntities> ScenarioEventTriggerBuilder<S> {
    fn new(parent: ScenarioEventBuilder<S>) -> Self {
        Self { parent }
    }

    /// Add a simulation time condition
    pub fn simulation_time(mut self) -> ScenarioSimulationTimeConditionBuilder<S> {
        ScenarioSimulationTimeConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<ScenarioEventBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the trigger to the event
        Ok(self.parent)
    }
}

/// Builder for simulation time conditions in scenario event triggers
pub struct ScenarioSimulationTimeConditionBuilder<S: BuilderState> {
    parent: ScenarioEventTriggerBuilder<S>,
    value: Option<f64>,
}

impl<S: CanAddEntities> ScenarioSimulationTimeConditionBuilder<S> {
    fn new(parent: ScenarioEventTriggerBuilder<S>) -> Self {
        Self {
            parent,
            value: None,
        }
    }

    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Finish building the condition
    pub fn finish_condition(self) -> BuilderResult<ScenarioEventTriggerBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the condition to the trigger
        Ok(self.parent)
    }
}

/// Builder for stop triggers within scenario storyboards
pub struct ScenarioStoryboardTriggerBuilder<S: BuilderState> {
    parent: ScenarioStoryboardBuilder<S>,
}

impl<S: CanAddEntities> ScenarioStoryboardTriggerBuilder<S> {
    fn new(parent: ScenarioStoryboardBuilder<S>) -> Self {
        Self { parent }
    }

    /// Add a simulation time condition
    pub fn simulation_time(mut self) -> ScenarioStoryboardSimulationTimeConditionBuilder<S> {
        ScenarioStoryboardSimulationTimeConditionBuilder::new(self)
    }

    /// Finish building the trigger
    pub fn finish_trigger(self) -> BuilderResult<ScenarioStoryboardBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the trigger to the storyboard
        Ok(self.parent)
    }
}

/// Builder for simulation time conditions in scenario storyboard triggers
pub struct ScenarioStoryboardSimulationTimeConditionBuilder<S: BuilderState> {
    parent: ScenarioStoryboardTriggerBuilder<S>,
    value: Option<f64>,
}

impl<S: CanAddEntities> ScenarioStoryboardSimulationTimeConditionBuilder<S> {
    fn new(parent: ScenarioStoryboardTriggerBuilder<S>) -> Self {
        Self {
            parent,
            value: None,
        }
    }

    /// Set greater than rule
    pub fn greater_than(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Finish building the condition
    pub fn finish_condition(self) -> BuilderResult<ScenarioStoryboardTriggerBuilder<S>> {
        // For now, just return the parent
        // In a full implementation, we'd add the condition to the trigger
        Ok(self.parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_builder_state_transitions() {
        // Test Empty -> HasHeader transition
        let builder = ScenarioBuilder::new();
        let builder = builder.with_simple_header("Test Scenario", "Test Author");

        // Test HasHeader -> HasCatalogLocations transition
        let builder = builder.with_default_catalogs().unwrap();

        // Test HasCatalogLocations -> HasRoadNetwork transition
        let builder = builder.with_road_network("test.xodr");

        // Test HasRoadNetwork -> HasEntities transition
        let builder = builder.with_entities();

        // Test that we can build from HasEntities state
        let scenario = builder.build().unwrap();

        // Verify the built scenario has the expected structure
        assert!(scenario
            .file_header
            .description
            .as_literal()
            .unwrap()
            .contains("Test"));
        assert!(scenario.catalog_locations.is_some());
        assert!(scenario.road_network.is_some());
        assert!(scenario.entities.is_some());
    }

    #[test]
    fn test_scenario_builder_with_parameters() {
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap()
            .with_road_network("test.xodr")
            .with_entities();

        let scenario = builder.build().unwrap();
        assert!(scenario.is_scenario());
    }

    #[test]
    fn test_scenario_builder_missing_required_fields() {
        // Test building without header
        let builder = ScenarioBuilder::new();
        // Can't call build() on Empty state - this is prevented at compile time

        // Test building without catalog locations
        let builder = ScenarioBuilder::new().with_simple_header("Test", "Author");
        // Can't call build() on HasHeader state - this is prevented at compile time

        // Test building without road network
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap();
        // Can't call build() on HasCatalogLocations state - this is prevented at compile time
    }

    #[test]
    fn test_scenario_builder_strict_validation() {
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap()
            .with_road_network("test.xodr")
            .with_entities();

        // Normal build should succeed even with no entities
        assert!(builder.clone().build().is_ok());

        // Strict validation should fail with no entities
        let result = builder.build_validated();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BuilderError::ValidationError { .. }
        ));
    }

    #[test]
    fn test_scenario_builder_road_network_variants() {
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap();

        // Test with logic file only
        let builder1 = builder
            .clone()
            .with_road_network_from_logic_file("test.xodr".to_string());
        let scenario1 = builder1.with_entities().build().unwrap();
        assert!(scenario1
            .road_network
            .as_ref()
            .unwrap()
            .logic_file
            .is_some());
        assert!(scenario1
            .road_network
            .as_ref()
            .unwrap()
            .scene_graph_file
            .is_none());

        // Test with both logic file and scene graph
        let builder2 =
            builder.with_road_network_full("test.xodr".to_string(), Some("test.osgb".to_string()));
        let scenario2 = builder2.with_entities().build().unwrap();
        assert!(scenario2
            .road_network
            .as_ref()
            .unwrap()
            .logic_file
            .is_some());
        assert!(scenario2
            .road_network
            .as_ref()
            .unwrap()
            .scene_graph_file
            .is_some());
    }

    #[test]
    fn test_scenario_builder_registry_access() {
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap()
            .with_road_network("test.xodr")
            .with_entities();

        // Test that we can access registries
        assert_eq!(builder.entity_registry().entity_count(), 0);
        assert_eq!(builder.parameter_registry().parameter_count(), 0);
        assert_eq!(builder.catalog_registry().catalog_count(), 0);
    }

    #[test]
    fn test_entity_builder_placeholders() {
        let mut builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .unwrap()
            .with_road_network("test.xodr")
            .with_entities();

        // Test that entity builder methods exist and return the expected types
        let _vehicle_builder = builder.add_vehicle("test_vehicle");
        let _pedestrian_builder = builder.add_pedestrian("test_pedestrian");

        // These are placeholders for now, but they demonstrate the API
    }

    #[test]
    fn test_scenario_builder_default() {
        let builder = ScenarioBuilder::default();
        let builder = builder.with_simple_header("Default Test", "Default Author");

        let scenario = builder
            .with_default_catalogs()
            .unwrap()
            .with_road_network("default.xodr")
            .with_entities()
            .build()
            .unwrap();

        assert!(scenario
            .file_header
            .description
            .as_literal()
            .unwrap()
            .contains("Default Test"));
    }
}

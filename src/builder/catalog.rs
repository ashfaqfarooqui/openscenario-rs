//! Catalog builder for programmatic catalog document construction
//!
//! This module provides the CatalogBuilder for creating OpenSCENARIO catalog documents
//! with type-safe state transitions and comprehensive validation.

use crate::types::{
    basic::{Value, OSString, Double},
    catalogs::files::CatalogContent,
    scenario::storyboard::{FileHeader, OpenScenario, CatalogDefinition},
    catalogs::entities::{
        CatalogVehicle, CatalogController, CatalogPedestrian, CatalogEnvironment,
        CatalogManeuver, CatalogTrajectory, CatalogRoute, CatalogPerformance, CatalogAxles,
        CatalogFrontAxle, CatalogRearAxle, ParameterDefinition,
    },
    geometry::{BoundingBox, Center, Dimensions},
};
use super::{
    error::{BuilderError, BuilderResult},
    states::*,
};
use std::marker::PhantomData;

/// Catalog builder with type-safe state transitions
pub struct CatalogBuilder<S: BuilderState> {
    _state: PhantomData<S>,
    catalog_data: PartialCatalogData,
}

/// Internal data structure for building catalogs
#[derive(Debug, Default)]
struct PartialCatalogData {
    file_header: Option<FileHeader>,
    catalog_name: Option<String>,
    vehicles: Vec<CatalogVehicle>,
    controllers: Vec<CatalogController>,
    pedestrians: Vec<CatalogPedestrian>,
    environments: Vec<CatalogEnvironment>,
    maneuvers: Vec<CatalogManeuver>,
    trajectories: Vec<CatalogTrajectory>,
    routes: Vec<CatalogRoute>,
}

impl CatalogBuilder<Empty> {
    /// Create a new catalog builder with the specified catalog name
    pub fn new(catalog_name: &str) -> CatalogBuilder<HasHeader> {
        let mut catalog_data = PartialCatalogData::default();
        catalog_data.catalog_name = Some(catalog_name.to_string());
        
        CatalogBuilder {
            _state: PhantomData,
            catalog_data,
        }
    }
}

impl CatalogBuilder<HasHeader> {
    /// Set the file header and keep in HasHeader state
    pub fn with_header(
        mut self,
        author: String,
        date: String,
        description: String,
        rev_major: u16,
        rev_minor: u16,
    ) -> Self {
        let header = FileHeader {
            author: Value::literal(author),
            date: Value::literal(date),
            description: Value::literal(description),
            rev_major: Value::literal(rev_major),
            rev_minor: Value::literal(rev_minor),
        };

        self.catalog_data.file_header = Some(header);
        self
    }

    /// Set a simple header with default values
    pub fn with_simple_header(self, description: &str, author: &str) -> Self {
        self.with_header(
            author.to_string(),
            "2024-01-01T00:00:00".to_string(),
            description.to_string(),
            1,
            3,
        )
    }

    /// Add a vehicle entity and return a vehicle builder
    pub fn add_vehicle(self, name: &str) -> CatalogVehicleBuilder {
        CatalogVehicleBuilder::new(self, name.to_string())
    }

    /// Add a controller entity and return a controller builder
    pub fn add_controller(self, name: &str) -> CatalogControllerBuilder {
        CatalogControllerBuilder::new(self, name.to_string())
    }

    /// Add a pedestrian entity and return a pedestrian builder
    pub fn add_pedestrian(self, name: &str) -> CatalogPedestrianBuilder {
        CatalogPedestrianBuilder::new(self, name.to_string())
    }

    /// Add an environment entity and return an environment builder
    pub fn add_environment(self, name: &str) -> CatalogEnvironmentBuilder {
        CatalogEnvironmentBuilder::new(self, name.to_string())
    }

    /// Add a maneuver entity and return a maneuver builder
    pub fn add_maneuver(self, name: &str) -> CatalogManeuverBuilder {
        CatalogManeuverBuilder::new(self, name.to_string())
    }

    /// Add a trajectory entity and return a trajectory builder
    pub fn add_trajectory(self, name: &str) -> CatalogTrajectoryBuilder {
        CatalogTrajectoryBuilder::new(self, name.to_string())
    }

    /// Add a route entity and return a route builder
    pub fn add_route(self, name: &str) -> CatalogRouteBuilder {
        CatalogRouteBuilder::new(self, name.to_string())
    }

    /// Build the complete catalog document
    pub fn build(self) -> BuilderResult<OpenScenario> {
        // Validate required fields
        let file_header = self.catalog_data.file_header
            .ok_or_else(|| BuilderError::missing_field("file_header", "Call with_header() or with_simple_header() first"))?;

        let catalog_name = self.catalog_data.catalog_name
            .ok_or_else(|| BuilderError::missing_field("catalog_name", "Catalog name should be set during construction"))?;

        // Validate that at least one entity is defined
        let total_entities = self.catalog_data.vehicles.len() +
            self.catalog_data.controllers.len() +
            self.catalog_data.pedestrians.len() +
            self.catalog_data.environments.len() +
            self.catalog_data.maneuvers.len() +
            self.catalog_data.trajectories.len() +
            self.catalog_data.routes.len();

        if total_entities == 0 {
            return Err(BuilderError::validation_error(
                "No entities defined in catalog",
                "Add at least one entity using add_vehicle(), add_controller(), etc."
            ));
        }

        // Build the catalog content
        let catalog_content = CatalogContent {
            name: Value::literal(catalog_name),
            vehicles: self.catalog_data.vehicles,
            controllers: self.catalog_data.controllers,
            pedestrians: self.catalog_data.pedestrians,
            misc_objects: Vec::new(), // Not implemented in this phase
            environments: self.catalog_data.environments,
            maneuvers: self.catalog_data.maneuvers,
            trajectories: self.catalog_data.trajectories,
            routes: self.catalog_data.routes,
        };

        // Build the complete OpenScenario document as a catalog
        Ok(OpenScenario {
            file_header,
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: None,
            road_network: None,
            entities: None,
            storyboard: None,
            parameter_value_distribution: None,
            catalog: Some(CatalogDefinition {
                catalog: catalog_content,
            }),
        })
    }
}

// Entity builders for catalog entities

/// Builder for catalog vehicle entities
pub struct CatalogVehicleBuilder {
    parent: CatalogBuilder<HasHeader>,
    vehicle_data: PartialVehicleData,
}

#[derive(Debug, Default)]
struct PartialVehicleData {
    name: String,
    vehicle_category: Option<String>,
    length: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    max_speed: Option<f64>,
    max_acceleration: Option<f64>,
    max_deceleration: Option<f64>,
}

impl CatalogVehicleBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            vehicle_data: PartialVehicleData {
                name,
                ..Default::default()
            },
        }
    }

    /// Set the vehicle model/category
    pub fn with_model(mut self, model: &str) -> Self {
        self.vehicle_data.vehicle_category = Some(model.to_string());
        self
    }

    /// Set vehicle dimensions (length, width, height)
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        self.vehicle_data.length = Some(length);
        self.vehicle_data.width = Some(width);
        self.vehicle_data.height = Some(height);
        self
    }

    /// Set vehicle performance characteristics
    pub fn with_performance(mut self, max_speed: f64, max_acceleration: f64, max_deceleration: f64) -> Self {
        self.vehicle_data.max_speed = Some(max_speed);
        self.vehicle_data.max_acceleration = Some(max_acceleration);
        self.vehicle_data.max_deceleration = Some(max_deceleration);
        self
    }

    /// Finish building the vehicle and return to the catalog builder
    pub fn finish_vehicle(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog vehicle with the configured data
        let vehicle = CatalogVehicle {
            name: self.vehicle_data.name.clone(),
            vehicle_category: OSString::literal(self.vehicle_data.vehicle_category.unwrap_or_else(|| "car".to_string())),
            bounding_box: BoundingBox {
                center: Center {
                    x: Double::literal(0.0),
                    y: Double::literal(0.0),
                    z: Double::literal(0.0),
                },
                dimensions: Dimensions {
                    width: Double::literal(self.vehicle_data.width.unwrap_or(1.8)),
                    length: Double::literal(self.vehicle_data.length.unwrap_or(4.5)),
                    height: Double::literal(self.vehicle_data.height.unwrap_or(1.4)),
                },
            },
            performance: CatalogPerformance {
                max_speed: Double::literal(self.vehicle_data.max_speed.unwrap_or(200.0)),
                max_acceleration: Double::literal(self.vehicle_data.max_acceleration.unwrap_or(8.0)),
                max_deceleration: Double::literal(self.vehicle_data.max_deceleration.unwrap_or(10.0)),
            },
            axles: CatalogAxles {
                front_axle: CatalogFrontAxle {
                    max_steering: Double::literal(0.5),
                    wheel_diameter: Double::literal(0.6),
                    track_width: Double::literal(1.5),
                    position_x: Double::literal(2.8),
                    position_z: Double::literal(0.3),
                },
                rear_axle: CatalogRearAxle {
                    max_steering: Double::literal(0.0),
                    wheel_diameter: Double::literal(0.6),
                    track_width: Double::literal(1.5),
                    position_x: Double::literal(0.0),
                    position_z: Double::literal(0.3),
                },
            },
            properties: None,
            parameter_declarations: None,
        };

        // Add the vehicle to the parent catalog builder
        self.parent.catalog_data.vehicles.push(vehicle);
        self.parent
    }
}

/// Builder for catalog controller entities
pub struct CatalogControllerBuilder {
    parent: CatalogBuilder<HasHeader>,
    controller_data: PartialControllerData,
}

#[derive(Debug, Default)]
struct PartialControllerData {
    name: String,
    controller_type: Option<String>,
}

impl CatalogControllerBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            controller_data: PartialControllerData {
                name,
                ..Default::default()
            },
        }
    }

    /// Set the controller type
    pub fn with_type(mut self, controller_type: &str) -> Self {
        self.controller_data.controller_type = Some(controller_type.to_string());
        self
    }

    /// Finish building the controller and return to the catalog builder
    pub fn finish_controller(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog controller with the configured data
        let controller = CatalogController {
            name: self.controller_data.name.clone(),
            controller_type: self.controller_data.controller_type.map(|t| OSString::literal(t)),
            parameter_declarations: None,
            properties: None,
        };

        // Add the controller to the parent catalog builder
        self.parent.catalog_data.controllers.push(controller);
        self.parent
    }
}

/// Builder for catalog pedestrian entities
pub struct CatalogPedestrianBuilder {
    parent: CatalogBuilder<HasHeader>,
    pedestrian_data: PartialPedestrianData,
}

#[derive(Debug, Default)]
struct PartialPedestrianData {
    name: String,
    category: Option<String>,
}

impl CatalogPedestrianBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            pedestrian_data: PartialPedestrianData {
                name,
                ..Default::default()
            },
        }
    }

    /// Set the pedestrian model/category
    pub fn with_model(mut self, model: &str) -> Self {
        self.pedestrian_data.category = Some(model.to_string());
        self
    }

    /// Finish building the pedestrian and return to the catalog builder
    pub fn finish_pedestrian(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog pedestrian with the configured data
        let pedestrian = CatalogPedestrian {
            name: self.pedestrian_data.name.clone(),
            pedestrian_category: OSString::literal(self.pedestrian_data.category.unwrap_or_else(|| "pedestrian".to_string())),
            bounding_box: BoundingBox {
                center: Center {
                    x: Double::literal(0.0),
                    y: Double::literal(0.0),
                    z: Double::literal(0.0),
                },
                dimensions: Dimensions {
                    width: Double::literal(0.6),
                    length: Double::literal(0.6),
                    height: Double::literal(1.8),
                },
            },
            parameter_declarations: None,
        };

        // Add the pedestrian to the parent catalog builder
        self.parent.catalog_data.pedestrians.push(pedestrian);
        self.parent
    }
}

/// Builder for catalog environment entities
pub struct CatalogEnvironmentBuilder {
    parent: CatalogBuilder<HasHeader>,
    environment_data: PartialEnvironmentData,
}

#[derive(Debug, Default)]
struct PartialEnvironmentData {
    name: String,
}

impl CatalogEnvironmentBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            environment_data: PartialEnvironmentData { name },
        }
    }

    /// Finish building the environment and return to the catalog builder
    pub fn finish_environment(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog environment with the configured data
        let environment = CatalogEnvironment {
            name: self.environment_data.name.clone(),
            parameter_declarations: None,
        };

        // Add the environment to the parent catalog builder
        self.parent.catalog_data.environments.push(environment);
        self.parent
    }
}

/// Builder for catalog maneuver entities
pub struct CatalogManeuverBuilder {
    parent: CatalogBuilder<HasHeader>,
    maneuver_data: PartialManeuverData,
}

#[derive(Debug, Default)]
struct PartialManeuverData {
    name: String,
}

impl CatalogManeuverBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            maneuver_data: PartialManeuverData { name },
        }
    }

    /// Finish building the maneuver and return to the catalog builder
    pub fn finish_maneuver(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog maneuver with the configured data
        let maneuver = CatalogManeuver {
            name: self.maneuver_data.name.clone(),
            parameter_declarations: None,
        };

        // Add the maneuver to the parent catalog builder
        self.parent.catalog_data.maneuvers.push(maneuver);
        self.parent
    }
}

/// Builder for catalog trajectory entities
pub struct CatalogTrajectoryBuilder {
    parent: CatalogBuilder<HasHeader>,
    trajectory_data: PartialTrajectoryData,
}

#[derive(Debug, Default)]
struct PartialTrajectoryData {
    name: String,
}

impl CatalogTrajectoryBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            trajectory_data: PartialTrajectoryData { name },
        }
    }

    /// Finish building the trajectory and return to the catalog builder
    pub fn finish_trajectory(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog trajectory with the configured data
        let trajectory = CatalogTrajectory {
            name: self.trajectory_data.name.clone(),
            parameter_declarations: None,
        };

        // Add the trajectory to the parent catalog builder
        self.parent.catalog_data.trajectories.push(trajectory);
        self.parent
    }
}

/// Builder for catalog route entities
pub struct CatalogRouteBuilder {
    parent: CatalogBuilder<HasHeader>,
    route_data: PartialRouteData,
}

#[derive(Debug, Default)]
struct PartialRouteData {
    name: String,
}

impl CatalogRouteBuilder {
    fn new(parent: CatalogBuilder<HasHeader>, name: String) -> Self {
        Self {
            parent,
            route_data: PartialRouteData { name },
        }
    }

    /// Finish building the route and return to the catalog builder
    pub fn finish_route(mut self) -> CatalogBuilder<HasHeader> {
        // Create a basic catalog route with the configured data
        let route = CatalogRoute {
            name: self.route_data.name.clone(),
            parameter_declarations: None,
        };

        // Add the route to the parent catalog builder
        self.parent.catalog_data.routes.push(route);
        self.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_builder_basic_construction() {
        let catalog = CatalogBuilder::new("TestCatalog")
            .with_simple_header("Test Catalog", "Test Author")
            .add_vehicle("test_vehicle")
                .with_model("TestModel")
                .with_dimensions(4.5, 1.8, 1.4)
                .with_performance(200.0, 8.0, 10.0)
                .finish_vehicle()
            .build()
            .expect("Should build valid catalog");

        assert!(catalog.is_catalog());
        assert!(catalog.catalog.is_some());
        
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.vehicles[0].name, "test_vehicle");
    }

    #[test]
    fn test_catalog_builder_multiple_entities() {
        let catalog = CatalogBuilder::new("MultiEntityCatalog")
            .with_simple_header("Multi Entity Catalog", "Test Author")
            .add_vehicle("vehicle1")
                .with_model("Model1")
                .finish_vehicle()
            .add_vehicle("vehicle2")
                .with_model("Model2")
                .finish_vehicle()
            .add_pedestrian("pedestrian1")
                .with_model("PedModel1")
                .finish_pedestrian()
            .build()
            .expect("Should build valid catalog");

        assert!(catalog.is_catalog());
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 2);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
    }

    #[test]
    fn test_catalog_builder_empty_catalog_fails() {
        let result = CatalogBuilder::new("EmptyCatalog")
            .with_simple_header("Empty Catalog", "Test Author")
            .build();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::ValidationError { .. }));
    }

    #[test]
    fn test_catalog_builder_all_entity_types() {
        let catalog = CatalogBuilder::new("AllEntitiesCatalog")
            .with_simple_header("All Entities Catalog", "Test Author")
            .add_vehicle("vehicle1")
                .finish_vehicle()
            .add_controller("controller1")
                .finish_controller()
            .add_pedestrian("pedestrian1")
                .finish_pedestrian()
            .add_environment("environment1")
                .finish_environment()
            .add_maneuver("maneuver1")
                .finish_maneuver()
            .add_trajectory("trajectory1")
                .finish_trajectory()
            .add_route("route1")
                .finish_route()
            .build()
            .expect("Should build valid catalog");

        assert!(catalog.is_catalog());
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.controllers.len(), 1);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
        assert_eq!(catalog_def.catalog.environments.len(), 1);
        assert_eq!(catalog_def.catalog.maneuvers.len(), 1);
        assert_eq!(catalog_def.catalog.trajectories.len(), 1);
        assert_eq!(catalog_def.catalog.routes.len(), 1);
    }

    #[test]
    fn test_vehicle_builder_configuration() {
        let catalog = CatalogBuilder::new("VehicleCatalog")
            .with_simple_header("Vehicle Test", "Test Author")
            .add_vehicle("configured_vehicle")
                .with_model("SportsCar")
                .with_dimensions(4.2, 1.9, 1.3)
                .with_performance(250.0, 10.0, 12.0)
                .finish_vehicle()
            .build()
            .expect("Should build valid catalog");

        let catalog_def = catalog.catalog.unwrap();
        let vehicle = &catalog_def.catalog.vehicles[0];
        
        assert_eq!(vehicle.name, "configured_vehicle");
        assert_eq!(vehicle.bounding_box.dimensions.length.as_literal().unwrap(), &4.2);
        assert_eq!(vehicle.bounding_box.dimensions.width.as_literal().unwrap(), &1.9);
        assert_eq!(vehicle.bounding_box.dimensions.height.as_literal().unwrap(), &1.3);
        assert_eq!(vehicle.performance.max_speed.as_literal().unwrap(), &250.0);
        assert_eq!(vehicle.performance.max_acceleration.as_literal().unwrap(), &10.0);
        assert_eq!(vehicle.performance.max_deceleration.as_literal().unwrap(), &12.0);
    }

    #[test]
    fn test_pedestrian_builder_configuration() {
        let catalog = CatalogBuilder::new("PedestrianCatalog")
            .with_simple_header("Pedestrian Test", "Test Author")
            .add_pedestrian("configured_pedestrian")
                .with_model("AdultMale")
                .finish_pedestrian()
            .build()
            .expect("Should build valid catalog");

        let catalog_def = catalog.catalog.unwrap();
        let pedestrian = &catalog_def.catalog.pedestrians[0];
        
        assert_eq!(pedestrian.name, "configured_pedestrian");
        assert_eq!(pedestrian.pedestrian_category.as_literal().unwrap(), "AdultMale");
    }
}
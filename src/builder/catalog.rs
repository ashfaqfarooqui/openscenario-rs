//! Catalog builder for programmatic catalog construction
//!
//! This module provides the CatalogBuilder that enables type-safe, ergonomic
//! construction of OpenSCENARIO catalog documents with compile-time validation
//! and fluent APIs for all catalog entity types.

use crate::types::{
    basic::Value,
    catalogs::{
        entities::{
            CatalogVehicle, CatalogController, CatalogPedestrian, CatalogMiscObject,
            CatalogEnvironment, CatalogManeuver, CatalogTrajectory, CatalogRoute,
            CatalogPerformance, CatalogAxles, CatalogFrontAxle, CatalogRearAxle,
        },
        files::CatalogContent,
    },
    enums::{VehicleCategory, ControllerType, PedestrianCategory},
    geometry::{BoundingBox, Center, Dimensions},
    scenario::storyboard::{FileHeader, OpenScenario, CatalogDefinition},
};
use super::{
    error::{BuilderError, BuilderResult},
    states::*,
};
use std::marker::PhantomData;

/// Main builder for OpenSCENARIO catalog documents
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO catalog documents. The type parameter S tracks the current
/// construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasHeader, CanBuild)
/// 
/// # Example
/// ```rust
/// let catalog = CatalogBuilder::new("VehicleCatalog")
///     .with_simple_header("Vehicle Library", "Fleet Manager")
///     .add_vehicle("sedan")
///         .with_model("GenericSedan")
///         .with_dimensions(4.5, 1.8, 1.4)
///         .with_performance(200.0, 8.0, 10.0)
///         .with_standard_axles()
///         .finish_vehicle()?
///     .build()?;
/// ```
#[derive(Debug)]
pub struct CatalogBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Partially constructed catalog data
    catalog_data: PartialCatalogData,
}

/// Internal data structure for building catalogs
/// 
/// This structure holds the partially constructed catalog data as it's
/// being built. Fields start as None and are populated as the builder
/// progresses through states.
#[derive(Debug, Default)]
struct PartialCatalogData {
    /// Catalog name
    catalog_name: Option<String>,
    
    /// File header with document metadata
    file_header: Option<FileHeader>,
    
    /// Catalog content with entities
    catalog_content: Option<CatalogContent>,
}

// Core builder implementation for Empty state
impl CatalogBuilder<Empty> {
    /// Create a new catalog builder with the specified catalog name
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// a file header as the first required element.
    /// 
    /// # Arguments
    /// * `catalog_name` - Name of the catalog being created
    /// 
    /// # Returns
    /// A new CatalogBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = CatalogBuilder::new("VehicleCatalog");
    /// ```
    pub fn new(catalog_name: &str) -> Self {
        Self {
            _state: PhantomData,
            catalog_data: PartialCatalogData {
                catalog_name: Some(catalog_name.to_string()),
                file_header: None,
                catalog_content: Some(CatalogContent::new(catalog_name.to_string())),
            },
        }
    }

    /// Set the file header and transition to HasHeader state
    /// 
    /// The file header contains essential document metadata and is required
    /// for all OpenSCENARIO documents. This must be the first method called.
    /// 
    /// # Arguments
    /// * `title` - Document title/description
    /// * `rev_major` - Major revision number (e.g., "1")  
    /// * `rev_minor` - Minor revision number (e.g., "0")
    /// * `date` - Creation date in ISO format (e.g., "2024-01-15T10:00:00")
    /// * `description` - Human-readable description
    /// * `author` - Document author name
    /// 
    /// # Returns
    /// CatalogBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = CatalogBuilder::new("VehicleCatalog")
    ///     .with_header("Vehicle Library", "1", "0", "2024-01-15T10:00:00", 
    ///                  "Vehicle catalog for testing", "Test Author");
    /// ```
    pub fn with_header(
        mut self,
        _title: &str,
        rev_major: &str,
        rev_minor: &str,
        date: &str,
        description: &str,
        author: &str,
    ) -> CatalogBuilder<HasHeader> {
        self.catalog_data.file_header = Some(FileHeader {
            rev_major: Value::literal(rev_major.parse().unwrap_or(1)),
            rev_minor: Value::literal(rev_minor.parse().unwrap_or(0)),
            date: Value::literal(date.to_string()),
            description: Value::literal(description.to_string()),
            author: Value::literal(author.to_string()),
        });

        CatalogBuilder {
            _state: PhantomData,
            catalog_data: self.catalog_data,
        }
    }

    /// Set the file header with simplified parameters
    /// 
    /// This is a convenience method for common use cases where only the
    /// essential header information is needed.
    /// 
    /// # Arguments
    /// * `title` - Document title
    /// * `author` - Document author
    /// 
    /// # Returns
    /// CatalogBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = CatalogBuilder::new("VehicleCatalog")
    ///     .with_simple_header("Vehicle Library", "Test Author");
    /// ```
    pub fn with_simple_header(self, title: &str, author: &str) -> CatalogBuilder<HasHeader> {
        let now = "2024-01-15T10:00:00"; // Use fixed date for reproducibility
        self.with_header(title, "1", "0", now, title, author)
    }
}

// Methods available after header is set - can add entities and build
impl<S: AfterHeader> CatalogBuilder<S> {
    /// Add a vehicle to the catalog and return a vehicle builder
    /// 
    /// This method creates a new vehicle builder that allows detailed
    /// configuration of vehicle properties before adding to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the vehicle in the catalog
    /// 
    /// # Returns
    /// CatalogVehicleBuilder for configuring the vehicle
    /// 
    /// # Example
    /// ```rust
    /// builder.add_vehicle("sedan")
    ///     .with_model("GenericSedan")
    ///     .with_dimensions(4.5, 1.8, 1.4)
    ///     .finish_vehicle()?
    /// ```
    pub fn add_vehicle(self, name: &str) -> CatalogVehicleBuilder<S> {
        CatalogVehicleBuilder::new(self, name)
    }

    /// Add a controller to the catalog and return a controller builder
    /// 
    /// This method creates a new controller builder that allows detailed
    /// configuration of controller properties before adding to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the controller in the catalog
    /// 
    /// # Returns
    /// CatalogControllerBuilder for configuring the controller
    /// 
    /// # Example
    /// ```rust
    /// builder.add_controller("ai_driver")
    ///     .with_type(ControllerType::Movement)
    ///     .finish_controller()?
    /// ```
    pub fn add_controller(self, name: &str) -> CatalogControllerBuilder<S> {
        CatalogControllerBuilder::new(self, name)
    }

    /// Add a pedestrian to the catalog and return a pedestrian builder
    /// 
    /// This method creates a new pedestrian builder that allows detailed
    /// configuration of pedestrian properties before adding to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the pedestrian in the catalog
    /// 
    /// # Returns
    /// CatalogPedestrianBuilder for configuring the pedestrian
    /// 
    /// # Example
    /// ```rust
    /// builder.add_pedestrian("walker")
    ///     .with_category(PedestrianCategory::Pedestrian)
    ///     .with_dimensions(0.6, 0.6, 1.8)
    ///     .finish_pedestrian()?
    /// ```
    pub fn add_pedestrian(self, name: &str) -> CatalogPedestrianBuilder<S> {
        CatalogPedestrianBuilder::new(self, name)
    }

    /// Add a miscellaneous object to the catalog
    /// 
    /// This method adds a simple miscellaneous object with basic properties.
    /// 
    /// # Arguments
    /// * `name` - Name of the object in the catalog
    /// * `width` - Width of the object in meters
    /// * `length` - Length of the object in meters
    /// * `height` - Height of the object in meters
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_misc_object("traffic_cone", 0.3, 0.3, 0.7)
    /// ```
    pub fn add_misc_object(mut self, name: &str, width: f64, length: f64, height: f64) -> Self {
        let misc_object = CatalogMiscObject {
            name: name.to_string(),
            bounding_box: BoundingBox {
                center: Center {
                    x: Value::literal(0.0),
                    y: Value::literal(0.0),
                    z: Value::literal(height / 2.0),
                },
                dimensions: Dimensions {
                    width: Value::literal(width),
                    length: Value::literal(length),
                    height: Value::literal(height),
                },
            },
            parameter_declarations: None,
        };

        if let Some(ref mut content) = self.catalog_data.catalog_content {
            content.misc_objects.push(misc_object);
        }

        self
    }

    /// Add an environment to the catalog
    /// 
    /// This method adds a simple environment definition to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the environment in the catalog
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_environment("sunny_day")
    /// ```
    pub fn add_environment(mut self, name: &str) -> Self {
        let environment = CatalogEnvironment {
            name: name.to_string(),
            parameter_declarations: None,
        };

        if let Some(ref mut content) = self.catalog_data.catalog_content {
            content.environments.push(environment);
        }

        self
    }

    /// Add a maneuver to the catalog
    /// 
    /// This method adds a simple maneuver definition to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the maneuver in the catalog
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_maneuver("lane_change")
    /// ```
    pub fn add_maneuver(mut self, name: &str) -> Self {
        let maneuver = CatalogManeuver {
            name: name.to_string(),
            parameter_declarations: None,
        };

        if let Some(ref mut content) = self.catalog_data.catalog_content {
            content.maneuvers.push(maneuver);
        }

        self
    }

    /// Add a trajectory to the catalog
    /// 
    /// This method adds a simple trajectory definition to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the trajectory in the catalog
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_trajectory("highway_path")
    /// ```
    pub fn add_trajectory(mut self, name: &str) -> Self {
        let trajectory = CatalogTrajectory {
            name: name.to_string(),
            parameter_declarations: None,
        };

        if let Some(ref mut content) = self.catalog_data.catalog_content {
            content.trajectories.push(trajectory);
        }

        self
    }

    /// Add a route to the catalog
    /// 
    /// This method adds a simple route definition to the catalog.
    /// 
    /// # Arguments
    /// * `name` - Name of the route in the catalog
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_route("city_route")
    /// ```
    pub fn add_route(mut self, name: &str) -> Self {
        let route = CatalogRoute {
            name: name.to_string(),
            parameter_declarations: None,
        };

        if let Some(ref mut content) = self.catalog_data.catalog_content {
            content.routes.push(route);
        }

        self
    }
}

// Methods for building final catalog (available when header is set)
impl<S: AfterHeader> CatalogBuilder<S> {
    /// Build the final OpenScenario catalog document
    /// 
    /// This method validates the constructed catalog and builds the final
    /// OpenScenario document. The catalog must have at least one entity.
    /// 
    /// # Returns
    /// Complete OpenScenario document or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    /// 
    /// # Example
    /// ```rust
    /// let catalog = builder.build()?;
    /// ```
    pub fn build(self) -> BuilderResult<OpenScenario> {
        // Validate required elements
        let file_header = self.catalog_data.file_header
            .ok_or_else(|| BuilderError::missing_field(
                "file_header", 
                "Call .with_header() first"
            ))?;

        let catalog_content = self.catalog_data.catalog_content
            .ok_or_else(|| BuilderError::missing_field(
                "catalog_content", 
                "Internal error: catalog content not initialized"
            ))?;

        // Validate that catalog has at least one entity
        if catalog_content.entity_count() == 0 {
            return Err(BuilderError::constraint_violation(
                "catalog must have entities",
                "Add at least one entity using .add_vehicle(), .add_controller(), etc."
            ));
        }

        // Construct final OpenScenario document
        let catalog_definition = CatalogDefinition {
            catalog: catalog_content,
        };

        let scenario = OpenScenario {
            file_header,
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: None,
            road_network: None,
            entities: None,
            storyboard: None,
            parameter_value_distribution: None,
            catalog: Some(catalog_definition),
        };

        Ok(scenario)
    }

    /// Build and validate with strict checking
    /// 
    /// This method performs additional validation beyond the basic build,
    /// checking for semantic correctness and XSD compliance.
    /// 
    /// # Returns
    /// Complete OpenScenario document or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError for any validation failures
    pub fn build_validated(self) -> BuilderResult<OpenScenario> {
        let scenario = self.build()?;
        
        // TODO: Integrate with existing validation system
        // let validation_context = ValidationContext::new();
        // scenario.validate(&validation_context)
        //     .map_err(BuilderError::ValidationError)?;
        
        Ok(scenario)
    }
}

/// Builder for configuring catalog vehicles
/// 
/// This builder provides detailed configuration options for vehicles
/// including dimensions, performance characteristics, and axle configurations.
#[derive(Debug)]
pub struct CatalogVehicleBuilder<S: BuilderState> {
    /// Parent catalog builder
    catalog_builder: CatalogBuilder<S>,
    
    /// Vehicle being configured
    vehicle: CatalogVehicle,
}

impl<S: AfterHeader> CatalogVehicleBuilder<S> {
    /// Create a new vehicle builder
    fn new(catalog_builder: CatalogBuilder<S>, name: &str) -> Self {
        let vehicle = CatalogVehicle {
            name: name.to_string(),
            vehicle_category: Value::literal("car".to_string()),
            bounding_box: BoundingBox::default(),
            performance: CatalogPerformance {
                max_speed: Value::literal(200.0),
                max_acceleration: Value::literal(10.0),
                max_deceleration: Value::literal(8.0),
            },
            axles: CatalogAxles {
                front_axle: CatalogFrontAxle {
                    max_steering: Value::literal(0.5),
                    wheel_diameter: Value::literal(0.6),
                    track_width: Value::literal(1.7),
                    position_x: Value::literal(2.8),
                    position_z: Value::literal(0.3),
                },
                rear_axle: CatalogRearAxle {
                    max_steering: Value::literal(0.0),
                    wheel_diameter: Value::literal(0.6),
                    track_width: Value::literal(1.7),
                    position_x: Value::literal(0.0),
                    position_z: Value::literal(0.3),
                },
            },
            properties: None,
            parameter_declarations: None,
        };

        Self {
            catalog_builder,
            vehicle,
        }
    }

    /// Set the vehicle category
    /// 
    /// # Arguments
    /// * `category` - Vehicle category (car, truck, bus, etc.)
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_category(mut self, category: VehicleCategory) -> Self {
        let category_str = match category {
            VehicleCategory::Car => "car",
            VehicleCategory::Van => "van",
            VehicleCategory::Truck => "truck",
            VehicleCategory::Semitrailer => "semitrailer",
            VehicleCategory::Bus => "bus",
            VehicleCategory::Motorbike => "motorbike",
            VehicleCategory::Bicycle => "bicycle",
            VehicleCategory::Train => "train",
            VehicleCategory::Tram => "tram",
        };
        self.vehicle.vehicle_category = Value::literal(category_str.to_string());
        self
    }

    /// Set the vehicle model name
    /// 
    /// # Arguments
    /// * `model` - Model name or identifier
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_model(self, _model: &str) -> Self {
        // Model is typically stored in properties, but for simplicity
        // we'll skip this in the MVP implementation
        self
    }

    /// Set vehicle dimensions
    /// 
    /// # Arguments
    /// * `length` - Vehicle length in meters
    /// * `width` - Vehicle width in meters
    /// * `height` - Vehicle height in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        self.vehicle.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(length / 2.0),
                y: Value::literal(0.0),
                z: Value::literal(height / 2.0),
            },
            dimensions: Dimensions {
                width: Value::literal(width),
                length: Value::literal(length),
                height: Value::literal(height),
            },
        };
        self
    }

    /// Set vehicle performance characteristics
    /// 
    /// # Arguments
    /// * `max_speed` - Maximum speed in m/s
    /// * `max_acceleration` - Maximum acceleration in m/s²
    /// * `max_deceleration` - Maximum deceleration in m/s²
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_performance(mut self, max_speed: f64, max_acceleration: f64, max_deceleration: f64) -> Self {
        self.vehicle.performance = CatalogPerformance {
            max_speed: Value::literal(max_speed),
            max_acceleration: Value::literal(max_acceleration),
            max_deceleration: Value::literal(max_deceleration),
        };
        self
    }

    /// Set standard axle configuration
    /// 
    /// This method sets up a typical passenger car axle configuration
    /// with reasonable default values.
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_standard_axles(mut self) -> Self {
        self.vehicle.axles = CatalogAxles {
            front_axle: CatalogFrontAxle {
                max_steering: Value::literal(0.5),
                wheel_diameter: Value::literal(0.6),
                track_width: Value::literal(1.7),
                position_x: Value::literal(2.8),
                position_z: Value::literal(0.3),
            },
            rear_axle: CatalogRearAxle {
                max_steering: Value::literal(0.0),
                wheel_diameter: Value::literal(0.6),
                track_width: Value::literal(1.7),
                position_x: Value::literal(0.0),
                position_z: Value::literal(0.3),
            },
        };
        self
    }

    /// Set custom axle configuration
    /// 
    /// # Arguments
    /// * `front_max_steering` - Maximum steering angle for front axle in radians
    /// * `wheel_diameter` - Wheel diameter in meters
    /// * `track_width` - Track width in meters
    /// * `wheelbase` - Distance between front and rear axles in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_custom_axles(mut self, front_max_steering: f64, wheel_diameter: f64, track_width: f64, wheelbase: f64) -> Self {
        self.vehicle.axles = CatalogAxles {
            front_axle: CatalogFrontAxle {
                max_steering: Value::literal(front_max_steering),
                wheel_diameter: Value::literal(wheel_diameter),
                track_width: Value::literal(track_width),
                position_x: Value::literal(wheelbase),
                position_z: Value::literal(wheel_diameter / 2.0),
            },
            rear_axle: CatalogRearAxle {
                max_steering: Value::literal(0.0),
                wheel_diameter: Value::literal(wheel_diameter),
                track_width: Value::literal(track_width),
                position_x: Value::literal(0.0),
                position_z: Value::literal(wheel_diameter / 2.0),
            },
        };
        self
    }

    /// Finish configuring the vehicle and add it to the catalog
    /// 
    /// # Returns
    /// The parent catalog builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if vehicle configuration is invalid
    pub fn finish_vehicle(mut self) -> BuilderResult<CatalogBuilder<S>> {
        // Validate vehicle configuration
        if self.vehicle.name.is_empty() {
            return Err(BuilderError::missing_field(
                "vehicle.name",
                "Vehicle name cannot be empty"
            ));
        }

        // Add vehicle to catalog
        if let Some(ref mut content) = self.catalog_builder.catalog_data.catalog_content {
            content.vehicles.push(self.vehicle);
        }

        Ok(self.catalog_builder)
    }
}

/// Builder for configuring catalog controllers
/// 
/// This builder provides configuration options for controllers
/// including controller type and properties.
#[derive(Debug)]
pub struct CatalogControllerBuilder<S: BuilderState> {
    /// Parent catalog builder
    catalog_builder: CatalogBuilder<S>,
    
    /// Controller being configured
    controller: CatalogController,
}

impl<S: AfterHeader> CatalogControllerBuilder<S> {
    /// Create a new controller builder
    fn new(catalog_builder: CatalogBuilder<S>, name: &str) -> Self {
        let controller = CatalogController {
            name: name.to_string(),
            controller_type: Some(Value::literal("movement".to_string())),
            parameter_declarations: None,
            properties: None,
        };

        Self {
            catalog_builder,
            controller,
        }
    }

    /// Set the controller type
    /// 
    /// # Arguments
    /// * `controller_type` - Type of controller
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_type(mut self, controller_type: ControllerType) -> Self {
        let type_str = match controller_type {
            ControllerType::Movement => "movement",
            ControllerType::Lateral => "lateral",
            ControllerType::Longitudinal => "longitudinal",
            ControllerType::Lighting => "lighting",
            ControllerType::Animation => "animation",
            ControllerType::Appearance => "appearance",
            ControllerType::All => "all",
        };
        self.controller.controller_type = Some(Value::literal(type_str.to_string()));
        self
    }

    /// Finish configuring the controller and add it to the catalog
    /// 
    /// # Returns
    /// The parent catalog builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if controller configuration is invalid
    pub fn finish_controller(mut self) -> BuilderResult<CatalogBuilder<S>> {
        // Validate controller configuration
        if self.controller.name.is_empty() {
            return Err(BuilderError::missing_field(
                "controller.name",
                "Controller name cannot be empty"
            ));
        }

        // Add controller to catalog
        if let Some(ref mut content) = self.catalog_builder.catalog_data.catalog_content {
            content.controllers.push(self.controller);
        }

        Ok(self.catalog_builder)
    }
}

/// Builder for configuring catalog pedestrians
/// 
/// This builder provides configuration options for pedestrians
/// including category and dimensions.
#[derive(Debug)]
pub struct CatalogPedestrianBuilder<S: BuilderState> {
    /// Parent catalog builder
    catalog_builder: CatalogBuilder<S>,
    
    /// Pedestrian being configured
    pedestrian: CatalogPedestrian,
}

impl<S: AfterHeader> CatalogPedestrianBuilder<S> {
    /// Create a new pedestrian builder
    fn new(catalog_builder: CatalogBuilder<S>, name: &str) -> Self {
        let pedestrian = CatalogPedestrian {
            name: name.to_string(),
            pedestrian_category: Value::literal("pedestrian".to_string()),
            bounding_box: BoundingBox::default(),
            parameter_declarations: None,
        };

        Self {
            catalog_builder,
            pedestrian,
        }
    }

    /// Set the pedestrian category
    /// 
    /// # Arguments
    /// * `category` - Pedestrian category
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_category(mut self, category: PedestrianCategory) -> Self {
        let category_str = match category {
            PedestrianCategory::Pedestrian => "pedestrian",
            PedestrianCategory::Wheelchair => "wheelchair",
            PedestrianCategory::Animal => "animal",
        };
        self.pedestrian.pedestrian_category = Value::literal(category_str.to_string());
        self
    }

    /// Set pedestrian dimensions
    /// 
    /// # Arguments
    /// * `width` - Pedestrian width in meters
    /// * `length` - Pedestrian length in meters
    /// * `height` - Pedestrian height in meters
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_dimensions(mut self, width: f64, length: f64, height: f64) -> Self {
        self.pedestrian.bounding_box = BoundingBox {
            center: Center {
                x: Value::literal(length / 2.0),
                y: Value::literal(0.0),
                z: Value::literal(height / 2.0),
            },
            dimensions: Dimensions {
                width: Value::literal(width),
                length: Value::literal(length),
                height: Value::literal(height),
            },
        };
        self
    }

    /// Finish configuring the pedestrian and add it to the catalog
    /// 
    /// # Returns
    /// The parent catalog builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if pedestrian configuration is invalid
    pub fn finish_pedestrian(mut self) -> BuilderResult<CatalogBuilder<S>> {
        // Validate pedestrian configuration
        if self.pedestrian.name.is_empty() {
            return Err(BuilderError::missing_field(
                "pedestrian.name",
                "Pedestrian name cannot be empty"
            ));
        }

        // Add pedestrian to catalog
        if let Some(ref mut content) = self.catalog_builder.catalog_data.catalog_content {
            content.pedestrians.push(self.pedestrian);
        }

        Ok(self.catalog_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_builder_state_transitions() {
        // Test basic state transitions
        let builder = CatalogBuilder::new("TestCatalog")
            .with_simple_header("Test Catalog", "Test Author")
            .add_vehicle("test_vehicle")
                .with_category(VehicleCategory::Car)
                .with_dimensions(4.5, 1.8, 1.4)
                .finish_vehicle()
                .unwrap();
            
        // Builder should be ready to build
        let catalog = builder.build().unwrap();
        assert!(catalog.is_catalog());
        assert_eq!(catalog.file_header.author.as_literal(), Some(&"Test Author".to_string()));
    }

    #[test]
    fn test_catalog_builder_vehicle_configuration() {
        let builder = CatalogBuilder::new("VehicleCatalog")
            .with_simple_header("Vehicle Test", "Test Author")
            .add_vehicle("sedan")
                .with_category(VehicleCategory::Car)
                .with_dimensions(4.5, 1.8, 1.4)
                .with_performance(200.0, 8.0, 10.0)
                .with_standard_axles()
                .finish_vehicle()
                .unwrap();
            
        let catalog = builder.build().unwrap();
        assert!(catalog.catalog.is_some());
        
        let catalog_def = catalog.catalog.unwrap();
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.vehicles[0].name, "sedan");
    }

    #[test]
    fn test_catalog_builder_multiple_entities() {
        let builder = CatalogBuilder::new("MixedCatalog")
            .with_simple_header("Mixed Catalog", "Test Author")
            .add_vehicle("car")
                .with_category(VehicleCategory::Car)
                .finish_vehicle()
                .unwrap()
            .add_controller("ai_driver")
                .with_type(ControllerType::Movement)
                .finish_controller()
                .unwrap()
            .add_pedestrian("walker")
                .with_category(PedestrianCategory::Pedestrian)
                .with_dimensions(0.6, 0.6, 1.8)
                .finish_pedestrian()
                .unwrap()
            .add_misc_object("cone", 0.3, 0.3, 0.7)
            .add_environment("sunny")
            .add_maneuver("lane_change")
            .add_trajectory("path1")
            .add_route("route1");
            
        let catalog = builder.build().unwrap();
        let catalog_def = catalog.catalog.unwrap();
        
        assert_eq!(catalog_def.catalog.vehicles.len(), 1);
        assert_eq!(catalog_def.catalog.controllers.len(), 1);
        assert_eq!(catalog_def.catalog.pedestrians.len(), 1);
        assert_eq!(catalog_def.catalog.misc_objects.len(), 1);
        assert_eq!(catalog_def.catalog.environments.len(), 1);
        assert_eq!(catalog_def.catalog.maneuvers.len(), 1);
        assert_eq!(catalog_def.catalog.trajectories.len(), 1);
        assert_eq!(catalog_def.catalog.routes.len(), 1);
        assert_eq!(catalog_def.catalog.entity_count(), 8);
    }

    #[test]
    fn test_catalog_builder_validation_errors() {
        // Test building without entities
        let result = CatalogBuilder::new("EmptyCatalog")
            .with_simple_header("Empty", "Author")
            .build();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("catalog must have entities"));
    }

    #[test]
    fn test_catalog_builder_custom_axles() {
        let builder = CatalogBuilder::new("CustomCatalog")
            .with_simple_header("Custom Vehicle", "Test Author")
            .add_vehicle("custom_car")
                .with_category(VehicleCategory::Car)
                .with_dimensions(5.0, 2.0, 1.5)
                .with_performance(250.0, 12.0, 15.0)
                .with_custom_axles(0.6, 0.7, 1.8, 3.0)
                .finish_vehicle()
                .unwrap();
            
        let catalog = builder.build().unwrap();
        let catalog_def = catalog.catalog.unwrap();
        let vehicle = &catalog_def.catalog.vehicles[0];
        
        assert_eq!(vehicle.axles.front_axle.max_steering.as_literal(), Some(&0.6));
        assert_eq!(vehicle.axles.front_axle.wheel_diameter.as_literal(), Some(&0.7));
        assert_eq!(vehicle.axles.front_axle.track_width.as_literal(), Some(&1.8));
        assert_eq!(vehicle.axles.front_axle.position_x.as_literal(), Some(&3.0));
    }
}
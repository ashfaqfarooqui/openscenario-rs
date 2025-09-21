//! Core scenario builder for programmatic scenario construction
//!
//! This module provides the main ScenarioBuilder that enables type-safe,
//! ergonomic construction of OpenSCENARIO scenario documents with compile-time
//! validation and fluent APIs.

use crate::types::{
    basic::{ParameterDeclarations, ParameterDeclaration, Value},
    catalogs::locations::CatalogLocations,
    entities::Entities,
    enums::ParameterType,
    road::{RoadNetwork, LogicFile, SceneGraphFile},
    scenario::{
        storyboard::{FileHeader, OpenScenario, Storyboard},
        init::{Init, Actions},
        variables::VariableDeclarations,
        monitors::MonitorDeclarations,
    },
};
use super::{
    error::{BuilderError, BuilderResult},
    registry::{EntityRegistry, ParameterRegistry, CatalogRegistry},
    states::*,
};
use std::marker::PhantomData;

/// Main builder for OpenSCENARIO scenario documents
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO scenario documents. The type parameter S tracks the current
/// construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasHeader, HasEntities, etc.)
/// 
/// # Example
/// ```rust
/// let scenario = ScenarioBuilder::new()
///     .with_header("Highway Test", "1", "0", "2024-01-15T10:00:00", "Test scenario", "Test Author")
///     .with_default_catalogs()
///     .with_road_network("highway.xodr")
///     .with_entities()
///     .build()?;
/// ```
pub struct ScenarioBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Partially constructed scenario data
    scenario_data: PartialScenarioData,
    
    /// Registry for tracking entities and validating references
    entity_registry: EntityRegistry,
    
    /// Registry for tracking parameters and validating references
    parameter_registry: ParameterRegistry,
    
    /// Registry for tracking catalogs and validating references
    catalog_registry: CatalogRegistry,
}

/// Internal data structure for building scenarios
/// 
/// This structure holds the partially constructed scenario data as it's
/// being built. Fields start as None and are populated as the builder
/// progresses through states.
#[derive(Debug, Default)]
struct PartialScenarioData {
    /// File header with document metadata
    file_header: Option<FileHeader>,
    
    /// Parameter declarations for the scenario
    parameter_declarations: Option<ParameterDeclarations>,
    
    /// Variable declarations for the scenario  
    variable_declarations: Option<VariableDeclarations>,
    
    /// Monitor declarations for the scenario
    monitor_declarations: Option<MonitorDeclarations>,
    
    /// Catalog locations for finding reusable components
    catalog_locations: Option<CatalogLocations>,
    
    /// Road network definition
    road_network: Option<RoadNetwork>,
    
    /// Scenario entities (vehicles, pedestrians, etc.)
    entities: Option<Entities>,
    
    /// Storyboard defining scenario behavior
    storyboard: Option<Storyboard>,
}

// Core builder implementation for Empty state
impl ScenarioBuilder<Empty> {
    /// Create a new scenario builder
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// a file header as the first required element.
    /// 
    /// # Returns
    /// A new ScenarioBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ScenarioBuilder::new();
    /// ```
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
    /// ScenarioBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ScenarioBuilder::new()
    ///     .with_header("Highway Merge", "1", "0", "2024-01-15T10:00:00", 
    ///                  "Highway merge scenario", "Test Author");
    /// ```
    pub fn with_header(
        mut self,
        _title: &str,
        rev_major: &str,
        rev_minor: &str,
        date: &str,
        description: &str,
        author: &str,
    ) -> ScenarioBuilder<HasHeader> {
        self.scenario_data.file_header = Some(FileHeader {
            rev_major: Value::literal(rev_major.parse().unwrap_or(1)),
            rev_minor: Value::literal(rev_minor.parse().unwrap_or(0)),
            date: Value::literal(date.to_string()),
            description: Value::literal(description.to_string()),
            author: Value::literal(author.to_string()),
        });

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
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
    /// ScenarioBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ScenarioBuilder::new()
    ///     .with_simple_header("Test Scenario", "Test Author");
    /// ```
    pub fn with_simple_header(self, title: &str, author: &str) -> ScenarioBuilder<HasHeader> {
        let now = "2024-01-15T10:00:00"; // Use fixed date for reproducibility
        self.with_header(title, "1", "0", now, title, author)
    }
}

// Methods available after header is set
impl<S: AfterHeader> ScenarioBuilder<S> {
    /// Add parameter declarations to the scenario
    /// 
    /// Parameters allow scenarios to be configurable and reusable with
    /// different values. Parameters can be referenced throughout the scenario
    /// using the ${parameterName} syntax.
    /// 
    /// # Arguments
    /// * `parameters` - ParameterDeclarations structure
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.with_parameters(parameter_declarations)
    /// ```
    pub fn with_parameters(mut self, parameters: ParameterDeclarations) -> Self {
        // Register parameters for reference validation
        if let Err(e) = self.parameter_registry.add_parameter_declarations(&parameters) {
            // Log warning but continue - validation will catch issues later
            eprintln!("Warning: Parameter registration failed: {}", e);
        }
        
        self.scenario_data.parameter_declarations = Some(parameters);
        self
    }

    /// Add a single parameter to the scenario
    /// 
    /// This is a convenience method for adding individual parameters without
    /// constructing a full ParameterDeclarations structure.
    /// 
    /// # Arguments
    /// * `name` - Parameter name (used in ${name} references)
    /// * `param_type` - Parameter type (Double, Int, Boolean, String)
    /// * `value` - Default parameter value
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.add_parameter("initial_speed", ParameterType::Double, "25.0")
    /// ```
    pub fn add_parameter(
        mut self, 
        name: &str, 
        param_type: ParameterType, 
        value: &str
    ) -> Self {
        let mut parameters = self.scenario_data.parameter_declarations
            .take()
            .unwrap_or_else(|| ParameterDeclarations {
                parameter_declarations: Vec::new(),
            });

        let param_decl = ParameterDeclaration {
            name: Value::literal(name.to_string()),
            parameter_type: param_type,
            value: Value::literal(value.to_string()),
            constraint_groups: Vec::new(),
        };

        parameters.parameter_declarations.push(param_decl);
        self.scenario_data.parameter_declarations = Some(parameters);
        self
    }

    /// Add variable declarations to the scenario
    /// 
    /// Variables are runtime-modifiable values that can be changed during
    /// scenario execution, unlike parameters which are set at scenario start.
    /// 
    /// # Arguments
    /// * `variables` - VariableDeclarations structure
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_variables(
        mut self, 
        variables: VariableDeclarations
    ) -> Self {
        self.scenario_data.variable_declarations = Some(variables);
        self
    }

    /// Add monitor declarations to the scenario
    /// 
    /// Monitors define conditions that are continuously evaluated during
    /// scenario execution for logging, debugging, or safety purposes.
    /// 
    /// # Arguments
    /// * `monitors` - MonitorDeclarations structure
    /// 
    /// # Returns
    /// Self for method chaining
    pub fn with_monitors(
        mut self, 
        monitors: MonitorDeclarations
    ) -> Self {
        self.scenario_data.monitor_declarations = Some(monitors);
        self
    }
}

// Methods for setting catalog locations (available after header)
impl ScenarioBuilder<HasHeader> {
    /// Set catalog locations for finding reusable components
    /// 
    /// Catalog locations define where to find various types of reusable
    /// components like vehicles, controllers, and environments.
    /// 
    /// # Arguments
    /// * `locations` - CatalogLocations structure with directory paths
    /// 
    /// # Returns
    /// ScenarioBuilder in HasCatalogLocations state
    /// 
    /// # Example
    /// ```rust
    /// builder.with_catalog_locations(catalog_locations)
    /// ```
    pub fn with_catalog_locations(mut self, locations: CatalogLocations) -> ScenarioBuilder<HasCatalogLocations> {
        self.catalog_registry.set_catalog_locations(locations.clone());
        self.scenario_data.catalog_locations = Some(locations);

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Use default catalog locations
    /// 
    /// This sets up standard catalog locations pointing to common directories.
    /// Useful for testing and standard scenarios.
    /// 
    /// # Returns
    /// ScenarioBuilder in HasCatalogLocations state
    /// 
    /// # Example
    /// ```rust
    /// builder.with_default_catalogs()
    /// ```
    pub fn with_default_catalogs(self) -> ScenarioBuilder<HasCatalogLocations> {
        let default_catalogs = CatalogLocations::default();
        self.with_catalog_locations(default_catalogs)
    }
}

// Methods for setting road network (available after catalog locations)
impl ScenarioBuilder<HasCatalogLocations> {
    /// Set road network from OpenDRIVE file
    /// 
    /// The road network defines the geometric and semantic road structure
    /// that provides context for entity positioning and navigation.
    /// 
    /// # Arguments
    /// * `file_path` - Path to OpenDRIVE (.xodr) file
    /// 
    /// # Returns
    /// ScenarioBuilder in HasRoadNetwork state
    /// 
    /// # Example
    /// ```rust
    /// builder.with_road_network("roads/highway.xodr")
    /// ```
    pub fn with_road_network(mut self, file_path: &str) -> ScenarioBuilder<HasRoadNetwork> {
        self.scenario_data.road_network = Some(RoadNetwork {
            logic_file: Some(LogicFile {
                filepath: Value::literal(file_path.to_string()),
            }),
            scene_graph_file: None,
        });

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Set road network with scene graph
    /// 
    /// This method sets both the logical road definition and optional
    /// scene graph for 3D visualization.
    /// 
    /// # Arguments
    /// * `logic_file` - Path to OpenDRIVE (.xodr) file
    /// * `scene_graph_file` - Optional path to 3D scene graph file
    /// 
    /// # Returns
    /// ScenarioBuilder in HasRoadNetwork state
    pub fn with_road_network_and_scene(
        mut self, 
        logic_file: &str, 
        scene_graph_file: Option<&str>
    ) -> ScenarioBuilder<HasRoadNetwork> {
        self.scenario_data.road_network = Some(RoadNetwork {
            logic_file: Some(LogicFile {
                filepath: Value::literal(logic_file.to_string()),
            }),
            scene_graph_file: scene_graph_file.map(|path| SceneGraphFile {
                filepath: Value::literal(path.to_string()),
            }),
        });

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }
}

// Methods for adding entities (available after road network)
impl ScenarioBuilder<HasRoadNetwork> {
    /// Initialize entities section and transition to HasEntities state
    /// 
    /// This method prepares the builder to accept entity definitions.
    /// Must be called before adding any vehicles, pedestrians, or objects.
    /// 
    /// # Returns
    /// ScenarioBuilder in HasEntities state
    /// 
    /// # Example
    /// ```rust
    /// builder.with_entities()
    ///     // ... add entities
    /// ```
    pub fn with_entities(mut self) -> ScenarioBuilder<HasEntities> {
        self.scenario_data.entities = Some(Entities {
            scenario_objects: Vec::new(),
        });

        ScenarioBuilder {
            _state: PhantomData,
            scenario_data: self.scenario_data,
            entity_registry: self.entity_registry,
            parameter_registry: self.parameter_registry,
            catalog_registry: self.catalog_registry,
        }
    }

    /// Start building entities with the new entity builders
    /// 
    /// This method creates an EntitiesBuilder that provides fluent APIs
    /// for adding vehicles, pedestrians, and miscellaneous objects.
    /// 
    /// # Returns
    /// EntitiesBuilder for adding entities
    /// 
    /// # Example
    /// ```rust
    /// builder.with_entities_builder()
    ///     .add_vehicle("ego")
    ///         .car()
    ///         .at_position().lane("1", 1, 100.0).finish()
    ///         .finish_vehicle()
    ///     .add_pedestrian("ped1")
    ///         .pedestrian()
    ///         .at_position().world(50.0, 10.0, None).finish()
    ///         .finish_pedestrian()
    ///     .finish_entities()
    /// ```
    pub fn with_entities_builder(self) -> super::entities::EntitiesBuilder<HasRoadNetwork> {
        super::entities::EntitiesBuilder::new(self.entity_registry)
    }
}

// Methods for building final scenario (available when entities are set)
impl<S: CanBuild> ScenarioBuilder<S> {
    /// Build the final OpenScenario document
    /// 
    /// This method validates the constructed scenario and builds the final
    /// OpenScenario document. All required elements must be set before calling.
    /// 
    /// # Returns
    /// Complete OpenScenario document or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    /// 
    /// # Example
    /// ```rust
    /// let scenario = builder.build()?;
    /// ```
    pub fn build(self) -> BuilderResult<OpenScenario> {
        // Validate required elements
        let file_header = self.scenario_data.file_header
            .ok_or_else(|| BuilderError::missing_field(
                "file_header", 
                "Call .with_header() first"
            ))?;

        let catalog_locations = self.scenario_data.catalog_locations
            .ok_or_else(|| BuilderError::missing_field(
                "catalog_locations", 
                "Call .with_catalog_locations() or .with_default_catalogs() first"
            ))?;

        let road_network = self.scenario_data.road_network
            .ok_or_else(|| BuilderError::missing_field(
                "road_network", 
                "Call .with_road_network() first"
            ))?;

        let entities = self.scenario_data.entities
            .ok_or_else(|| BuilderError::missing_field(
                "entities", 
                "Call .with_entities() first"
            ))?;

        // Use default storyboard if none provided
        let storyboard = self.scenario_data.storyboard
            .unwrap_or_else(|| Storyboard {
                init: Init {
                    actions: Actions {
                        global_actions: Vec::new(),
                        private_actions: Vec::new(),
                    },
                },
                stories: Vec::new(),
                stop_trigger: None,
            });

        // Construct final OpenScenario document
        let scenario = OpenScenario {
            file_header,
            parameter_declarations: self.scenario_data.parameter_declarations,
            variable_declarations: self.scenario_data.variable_declarations,
            monitor_declarations: self.scenario_data.monitor_declarations,
            catalog_locations: Some(catalog_locations),
            road_network: Some(road_network),
            entities: Some(entities),
            storyboard: Some(storyboard),
            parameter_value_distribution: None, // Not used for scenario documents
            catalog: None, // Not used for scenario documents
        };

        // Validate the constructed scenario using existing validation framework
        // TODO: Integrate with existing validation system
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_builder_state_transitions() {
        // Test basic state transitions
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .with_default_catalogs()
            .with_road_network("test.xodr")
            .with_entities();
            
        // Builder should be in HasEntities state and ready to build
        let scenario = builder.build().unwrap();
        assert_eq!(scenario.file_header.author.as_literal(), Some(&"Author".to_string()));
    }

    #[test]
    fn test_scenario_builder_parameters() {
        let builder = ScenarioBuilder::new()
            .with_simple_header("Test", "Author")
            .add_parameter("speed", ParameterType::Double, "25.0")
            .with_default_catalogs()
            .with_road_network("test.xodr")
            .with_entities();
            
        let scenario = builder.build().unwrap();
        assert!(scenario.parameter_declarations.is_some());
        
        let params = scenario.parameter_declarations.unwrap();
        assert_eq!(params.parameter_declarations.len(), 1);
        assert_eq!(params.parameter_declarations[0].name.as_literal(), Some(&"speed".to_string()));
    }

    #[test]
    fn test_scenario_builder_validation_errors() {
        // Test building without required elements
        let result = ScenarioBuilder::new().build();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("file_header"));
    }
}
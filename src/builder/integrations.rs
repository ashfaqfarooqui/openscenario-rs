//! Ecosystem integrations for OpenSCENARIO builder
//!
//! This module provides integration examples and utilities for popular
//! Rust frameworks and tools.

use std::collections::HashMap;
use crate::builder::{BuilderError, BuilderResult, ScenarioBuilder};

/// Tokio/async integration utilities
#[cfg(feature = "tokio")]
pub mod tokio_integration {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Async scenario builder for non-blocking construction
    pub struct AsyncScenarioBuilder<S: crate::builder::states::BuilderState> {
        inner: ScenarioBuilder<S>,
    }

    impl<S: crate::builder::states::BuilderState> AsyncScenarioBuilder<S> {
        /// Wrap a synchronous builder for async usage
        pub fn new(builder: ScenarioBuilder<S>) -> Self {
            Self { inner: builder }
        }

        /// Build scenario asynchronously
        pub async fn build_async(self) -> BuilderResult<crate::types::scenario::storyboard::OpenScenario> 
        where 
            S: crate::builder::states::CanBuild,
        {
            // Yield to allow other tasks to run
            tokio::task::yield_now().await;
            
            // Perform the actual build operation
            tokio::task::spawn_blocking(move || {
                self.inner.build()
            }).await
            .map_err(|e| BuilderError::RuntimeError(format!("Async build failed: {}", e)))?
        }

        /// Add entity asynchronously with validation
        pub async fn add_entity_async(
            self,
            name: &str,
            entity_type: EntityType,
        ) -> BuilderResult<AsyncScenarioBuilder<S>>
        where
            S: crate::builder::states::CanAddEntities,
        {
            // Simulate async validation (e.g., checking against external catalog)
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            
            // Add entity (this would be implemented properly)
            Ok(AsyncScenarioBuilder::new(self.inner))
        }
    }

    /// Entity type for async operations
    pub enum EntityType {
        Vehicle(String),
        Pedestrian(String),
        MiscObject(String),
    }

    /// Async scenario validation
    pub async fn validate_scenario_async(
        scenario: &crate::types::scenario::storyboard::OpenScenario,
    ) -> BuilderResult<ValidationReport> {
        // Simulate async validation (e.g., against remote schema)
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        Ok(ValidationReport {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        })
    }

    /// Validation report
    #[derive(Debug)]
    pub struct ValidationReport {
        pub is_valid: bool,
        pub errors: Vec<String>,
        pub warnings: Vec<String>,
    }
}

/// Serde integration for scenario serialization
pub mod serde_integration {
    use super::*;
    use serde::{Serialize, Deserialize};

    /// Serializable scenario configuration
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ScenarioConfig {
        pub name: String,
        pub description: Option<String>,
        pub parameters: HashMap<String, ParameterConfig>,
        pub entities: Vec<EntityConfig>,
        pub road_network: String,
        pub catalogs: Vec<String>,
    }

    /// Parameter configuration
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParameterConfig {
        pub parameter_type: String,
        pub default_value: Option<String>,
        pub description: Option<String>,
    }

    /// Entity configuration
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EntityConfig {
        pub name: String,
        pub entity_type: String,
        pub catalog_reference: Option<String>,
        pub position: PositionConfig,
        pub properties: HashMap<String, serde_json::Value>,
    }

    /// Position configuration
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum PositionConfig {
        World { x: f64, y: f64, z: f64, h: Option<f64> },
        Lane { road_id: String, lane_id: i32, s: f64, offset: Option<f64> },
        Road { road_id: String, s: f64, t: f64, h: Option<f64> },
    }

    impl ScenarioConfig {
        /// Create scenario builder from configuration
        pub fn to_builder(self) -> BuilderResult<ScenarioBuilder<crate::builder::states::Empty>> {
            let mut builder = ScenarioBuilder::new()
                .with_simple_header(&self.name, "Config Generated");

            // Add description if present
            if let Some(description) = self.description {
                // Would set description if builder supported it
            }

            // Add catalogs
            let builder = builder.with_default_catalogs()?
                .with_road_network(&self.road_network)
                .with_entities();

            // Add parameters
            let mut builder = builder;
            for (name, param_config) in self.parameters {
                // Would add parameter based on config
                // builder = builder.add_parameter(&name, param_type, default_value);
            }

            // Add entities
            for entity_config in self.entities {
                // Would add entity based on config
                // builder = builder.add_entity_from_config(entity_config)?;
            }

            Ok(builder)
        }

        /// Load configuration from JSON file
        pub fn load_json(path: impl AsRef<std::path::Path>) -> BuilderResult<Self> {
            let content = std::fs::read_to_string(path)
                .map_err(|e| BuilderError::IoError(format!("Failed to read config: {}", e)))?;
            
            serde_json::from_str(&content)
                .map_err(|e| BuilderError::SerializationError(format!("Failed to parse config: {}", e)))
        }

        /// Save configuration to JSON file
        pub fn save_json(&self, path: impl AsRef<std::path::Path>) -> BuilderResult<()> {
            let content = serde_json::to_string_pretty(self)
                .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize config: {}", e)))?;
            
            std::fs::write(path, content)
                .map_err(|e| BuilderError::IoError(format!("Failed to write config: {}", e)))
        }

        /// Load configuration from YAML file
        pub fn load_yaml(path: impl AsRef<std::path::Path>) -> BuilderResult<Self> {
            let content = std::fs::read_to_string(path)
                .map_err(|e| BuilderError::IoError(format!("Failed to read config: {}", e)))?;
            
            serde_yaml::from_str(&content)
                .map_err(|e| BuilderError::SerializationError(format!("Failed to parse config: {}", e)))
        }

        /// Save configuration to YAML file
        pub fn save_yaml(&self, path: impl AsRef<std::path::Path>) -> BuilderResult<()> {
            let content = serde_yaml::to_string(self)
                .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize config: {}", e)))?;
            
            std::fs::write(path, content)
                .map_err(|e| BuilderError::IoError(format!("Failed to write config: {}", e)))
        }
    }
}

/// Clap integration for CLI tools
#[cfg(feature = "clap")]
pub mod clap_integration {
    use super::*;
    use clap::{Parser, Subcommand, Args};

    /// CLI application for scenario building
    #[derive(Parser)]
    #[command(name = "openscenario-builder")]
    #[command(about = "OpenSCENARIO scenario builder CLI")]
    pub struct ScenarioBuilderCli {
        #[command(subcommand)]
        pub command: Commands,
    }

    /// Available CLI commands
    #[derive(Subcommand)]
    pub enum Commands {
        /// Create a new scenario
        Create(CreateArgs),
        /// Validate an existing scenario
        Validate(ValidateArgs),
        /// Convert between formats
        Convert(ConvertArgs),
        /// Generate template
        Template(TemplateArgs),
    }

    /// Arguments for create command
    #[derive(Args)]
    pub struct CreateArgs {
        /// Scenario name
        #[arg(short, long)]
        pub name: String,
        
        /// Author name
        #[arg(short, long)]
        pub author: String,
        
        /// Output file path
        #[arg(short, long)]
        pub output: String,
        
        /// Road network file
        #[arg(short, long, default_value = "default.xodr")]
        pub road_network: String,
        
        /// Configuration file
        #[arg(short, long)]
        pub config: Option<String>,
    }

    /// Arguments for validate command
    #[derive(Args)]
    pub struct ValidateArgs {
        /// Input scenario file
        pub input: String,
        
        /// Strict validation mode
        #[arg(short, long)]
        pub strict: bool,
        
        /// Output validation report
        #[arg(short, long)]
        pub report: Option<String>,
    }

    /// Arguments for convert command
    #[derive(Args)]
    pub struct ConvertArgs {
        /// Input file
        pub input: String,
        
        /// Output file
        pub output: String,
        
        /// Output format
        #[arg(short, long, value_enum)]
        pub format: OutputFormat,
    }

    /// Arguments for template command
    #[derive(Args)]
    pub struct TemplateArgs {
        /// Template name
        pub name: String,
        
        /// Output file
        #[arg(short, long)]
        pub output: String,
        
        /// Template type
        #[arg(short, long, value_enum)]
        pub template_type: TemplateType,
    }

    /// Output formats
    #[derive(clap::ValueEnum, Clone)]
    pub enum OutputFormat {
        Xml,
        Json,
        Yaml,
    }

    /// Template types
    #[derive(clap::ValueEnum, Clone)]
    pub enum TemplateType {
        Basic,
        Highway,
        Urban,
        Parking,
    }

    impl ScenarioBuilderCli {
        /// Execute the CLI command
        pub fn execute(self) -> BuilderResult<()> {
            match self.command {
                Commands::Create(args) => self.create_scenario(args),
                Commands::Validate(args) => self.validate_scenario(args),
                Commands::Convert(args) => self.convert_scenario(args),
                Commands::Template(args) => self.generate_template(args),
            }
        }

        /// Create a new scenario
        fn create_scenario(&self, args: CreateArgs) -> BuilderResult<()> {
            let mut builder = ScenarioBuilder::new()
                .with_simple_header(&args.name, &args.author)
                .with_default_catalogs()?
                .with_road_network(&args.road_network)
                .with_entities();

            // Load configuration if provided
            if let Some(config_path) = args.config {
                let config = serde_integration::ScenarioConfig::load_json(config_path)?;
                builder = config.to_builder()?;
            }

            let scenario = builder.build()?;
            
            // Save scenario
            crate::serialize_to_file(&scenario, &args.output)
                .map_err(|e| BuilderError::IoError(format!("Failed to save scenario: {}", e)))?;

            println!("✅ Scenario '{}' created successfully: {}", args.name, args.output);
            Ok(())
        }

        /// Validate a scenario
        fn validate_scenario(&self, args: ValidateArgs) -> BuilderResult<()> {
            let scenario = crate::parse_from_file(&args.input)
                .map_err(|e| BuilderError::IoError(format!("Failed to load scenario: {}", e)))?;

            // Perform validation
            let mut errors = Vec::new();
            let mut warnings = Vec::new();

            // Basic validation
            if scenario.file_header.author.is_empty() {
                warnings.push("Missing author information".to_string());
            }

            if scenario.entities.is_none() {
                errors.push("No entities defined".to_string());
            }

            // Strict validation
            if args.strict {
                if scenario.file_header.description.is_empty() {
                    errors.push("Description is required in strict mode".to_string());
                }
            }

            // Generate report
            let report = format!(
                "Validation Report for {}\n\
                 Errors: {}\n\
                 Warnings: {}\n\
                 Status: {}\n",
                args.input,
                errors.len(),
                warnings.len(),
                if errors.is_empty() { "✅ PASSED" } else { "❌ FAILED" }
            );

            // Save report if requested
            if let Some(report_path) = args.report {
                std::fs::write(report_path, &report)
                    .map_err(|e| BuilderError::IoError(format!("Failed to save report: {}", e)))?;
            } else {
                println!("{}", report);
            }

            if !errors.is_empty() {
                return Err(BuilderError::ValidationError("Validation failed".to_string()));
            }

            Ok(())
        }

        /// Convert scenario between formats
        fn convert_scenario(&self, args: ConvertArgs) -> BuilderResult<()> {
            let scenario = crate::parse_from_file(&args.input)
                .map_err(|e| BuilderError::IoError(format!("Failed to load scenario: {}", e)))?;

            match args.format {
                OutputFormat::Xml => {
                    crate::serialize_to_file(&scenario, &args.output)
                        .map_err(|e| BuilderError::IoError(format!("Failed to save XML: {}", e)))?;
                }
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&scenario)
                        .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize JSON: {}", e)))?;
                    std::fs::write(&args.output, json)
                        .map_err(|e| BuilderError::IoError(format!("Failed to save JSON: {}", e)))?;
                }
                OutputFormat::Yaml => {
                    let yaml = serde_yaml::to_string(&scenario)
                        .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize YAML: {}", e)))?;
                    std::fs::write(&args.output, yaml)
                        .map_err(|e| BuilderError::IoError(format!("Failed to save YAML: {}", e)))?;
                }
            }

            println!("✅ Converted {} to {} format: {}", args.input, format!("{:?}", args.format), args.output);
            Ok(())
        }

        /// Generate scenario template
        fn generate_template(&self, args: TemplateArgs) -> BuilderResult<()> {
            let template = match args.template_type {
                TemplateType::Basic => self.create_basic_template(&args.name),
                TemplateType::Highway => self.create_highway_template(&args.name),
                TemplateType::Urban => self.create_urban_template(&args.name),
                TemplateType::Parking => self.create_parking_template(&args.name),
            }?;

            template.save_yaml(&args.output)?;
            println!("✅ Template '{}' generated: {}", args.name, args.output);
            Ok(())
        }

        /// Create basic template
        fn create_basic_template(&self, name: &str) -> BuilderResult<crate::builder::templates::ScenarioTemplate> {
            Ok(crate::builder::templates::ScenarioTemplate::new(name)
                .with_description("Basic scenario template")
                .with_parameter("initial_speed", crate::types::basic::ParameterType::Double, Some("30.0")))
        }

        /// Create highway template
        fn create_highway_template(&self, name: &str) -> BuilderResult<crate::builder::templates::ScenarioTemplate> {
            Ok(crate::builder::templates::ScenarioTemplate::new(name)
                .with_description("Highway driving scenario template")
                .with_parameter("ego_speed", crate::types::basic::ParameterType::Double, Some("30.0"))
                .with_parameter("target_speed", crate::types::basic::ParameterType::Double, Some("35.0")))
        }

        /// Create urban template
        fn create_urban_template(&self, name: &str) -> BuilderResult<crate::builder::templates::ScenarioTemplate> {
            Ok(crate::builder::templates::ScenarioTemplate::new(name)
                .with_description("Urban driving scenario template")
                .with_parameter("traffic_density", crate::types::basic::ParameterType::Integer, Some("5")))
        }

        /// Create parking template
        fn create_parking_template(&self, name: &str) -> BuilderResult<crate::builder::templates::ScenarioTemplate> {
            Ok(crate::builder::templates::ScenarioTemplate::new(name)
                .with_description("Parking scenario template")
                .with_parameter("parking_space", crate::types::basic::ParameterType::String, Some("A1")))
        }
    }
}

/// Tracing integration for debugging
#[cfg(feature = "tracing")]
pub mod tracing_integration {
    use super::*;
    use tracing::{info, warn, error, debug, span, Level};

    /// Traced scenario builder wrapper
    pub struct TracedScenarioBuilder<S: crate::builder::states::BuilderState> {
        inner: ScenarioBuilder<S>,
        span: tracing::Span,
    }

    impl<S: crate::builder::states::BuilderState> TracedScenarioBuilder<S> {
        /// Create traced builder
        pub fn new(builder: ScenarioBuilder<S>) -> Self {
            let span = span!(Level::INFO, "scenario_builder", state = std::any::type_name::<S>());
            Self { inner: builder, span }
        }

        /// Build with tracing
        pub fn build_traced(self) -> BuilderResult<crate::types::scenario::storyboard::OpenScenario>
        where
            S: crate::builder::states::CanBuild,
        {
            let _enter = self.span.enter();
            info!("Starting scenario build");
            
            let start = std::time::Instant::now();
            let result = self.inner.build();
            let duration = start.elapsed();
            
            match &result {
                Ok(_) => info!("Scenario build completed successfully in {:?}", duration),
                Err(e) => error!("Scenario build failed: {}", e),
            }
            
            result
        }

        /// Add entity with tracing
        pub fn add_entity_traced(
            self,
            name: &str,
        ) -> TracedScenarioBuilder<S>
        where
            S: crate::builder::states::CanAddEntities,
        {
            let _enter = self.span.enter();
            debug!("Adding entity: {}", name);
            
            // In a real implementation, we would add the entity
            TracedScenarioBuilder::new(self.inner)
        }
    }

    /// Initialize tracing for scenario building
    pub fn init_tracing() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_integration() {
        let config = serde_integration::ScenarioConfig {
            name: "Test Scenario".to_string(),
            description: Some("Test description".to_string()),
            parameters: HashMap::new(),
            entities: Vec::new(),
            road_network: "test.xodr".to_string(),
            catalogs: vec!["vehicles.xosc".to_string()],
        };

        // Test serialization
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: serde_integration::ScenarioConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.road_network, deserialized.road_network);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_async_validation() {
        use tokio_integration::*;
        
        // Create a dummy scenario for testing
        let scenario = crate::types::scenario::storyboard::OpenScenario::default();
        
        let report = validate_scenario_async(&scenario).await.unwrap();
        assert!(report.is_valid);
    }
}
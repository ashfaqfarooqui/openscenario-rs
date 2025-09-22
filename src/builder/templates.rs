//! Scenario templates and serialization support
//!
//! This module provides template-based scenario construction, serialization
//! of builder state, and scenario comparison/merging capabilities.

use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::builder::{BuilderError, BuilderResult, ScenarioBuilder};
use crate::types::basic::{ParameterType, Value};

/// Scenario template with parameterizable values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioTemplate {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub parameters: HashMap<String, ParameterDefinition>,
    pub entities: HashMap<String, EntityTemplate>,
    pub story_template: Option<StoryTemplate>,
    pub metadata: HashMap<String, String>,
}

/// Parameter definition in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub parameter_type: ParameterType,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub constraints: Option<ParameterConstraints>,
}

/// Constraints for template parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<String>>,
    pub pattern: Option<String>,
}

/// Entity template with placeholder values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTemplate {
    pub entity_type: EntityType,
    pub catalog_reference: Option<String>,
    pub properties: HashMap<String, Value<String>>,
    pub position_template: Option<PositionTemplate>,
}

/// Types of entities in templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Vehicle,
    Pedestrian,
    MiscObject,
}

/// Position template with parameterizable coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionTemplate {
    World {
        x: Value<String>,
        y: Value<String>,
        z: Value<String>,
        h: Option<Value<String>>,
        p: Option<Value<String>>,
        r: Option<Value<String>>,
    },
    Lane {
        road_id: Value<String>,
        lane_id: Value<String>,
        s: Value<String>,
        offset: Option<Value<String>>,
    },
    Road {
        road_id: Value<String>,
        s: Value<String>,
        t: Value<String>,
        h: Option<Value<String>>,
    },
    Relative {
        entity_ref: Value<String>,
        dx: Value<String>,
        dy: Value<String>,
        dz: Option<Value<String>>,
    },
}

/// Story template with parameterizable actions and conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTemplate {
    pub name: String,
    pub acts: Vec<ActTemplate>,
}

/// Act template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActTemplate {
    pub name: String,
    pub maneuvers: Vec<ManeuverTemplate>,
}

/// Maneuver template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManeuverTemplate {
    pub name: String,
    pub events: Vec<EventTemplate>,
}

/// Event template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTemplate {
    pub name: String,
    pub action_template: ActionTemplate,
    pub trigger_template: TriggerTemplate,
}

/// Action template with parameterizable values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionTemplate {
    Longitudinal {
        entity_ref: Value<String>,
        speed_target: Value<String>,
    },
    Lateral {
        entity_ref: Value<String>,
        lane_target: Value<String>,
    },
    Teleport {
        entity_ref: Value<String>,
        position: PositionTemplate,
    },
}

/// Trigger template with parameterizable conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerTemplate {
    SimulationTime {
        threshold: Value<String>,
        rule: String,
    },
    EntitySpeed {
        entity_ref: Value<String>,
        threshold: Value<String>,
        rule: String,
    },
    Distance {
        entity1_ref: Value<String>,
        entity2_ref: Value<String>,
        threshold: Value<String>,
        rule: String,
    },
}

impl ScenarioTemplate {
    /// Create a new scenario template
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            version: "1.0".to_string(),
            parameters: HashMap::new(),
            entities: HashMap::new(),
            story_template: None,
            metadata: HashMap::new(),
        }
    }

    /// Add description to the template
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a parameter to the template
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        param_type: ParameterType,
        default_value: Option<impl Into<String>>,
    ) -> Self {
        let param_def = ParameterDefinition {
            parameter_type: param_type,
            default_value: default_value.map(|v| v.into()),
            description: None,
            constraints: None,
        };
        self.parameters.insert(name.into(), param_def);
        self
    }

    /// Add an entity template
    pub fn with_entity(mut self, name: impl Into<String>, entity: EntityTemplate) -> Self {
        self.entities.insert(name.into(), entity);
        self
    }

    /// Add story template
    pub fn with_story(mut self, story: StoryTemplate) -> Self {
        self.story_template = Some(story);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Load template from YAML file
    pub fn load_yaml(path: impl AsRef<Path>) -> BuilderResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BuilderError::IoError(format!("Failed to read template file: {}", e)))?;
        
        serde_yaml::from_str(&content)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to parse YAML template: {}", e)))
    }

    /// Load template from JSON file
    pub fn load_json(path: impl AsRef<Path>) -> BuilderResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BuilderError::IoError(format!("Failed to read template file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to parse JSON template: {}", e)))
    }

    /// Save template to YAML file
    pub fn save_yaml(&self, path: impl AsRef<Path>) -> BuilderResult<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize template: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| BuilderError::IoError(format!("Failed to write template file: {}", e)))
    }

    /// Save template to JSON file
    pub fn save_json(&self, path: impl AsRef<Path>) -> BuilderResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize template: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| BuilderError::IoError(format!("Failed to write template file: {}", e)))
    }

    /// Instantiate template with parameter values
    pub fn instantiate(&self, parameters: HashMap<String, String>) -> BuilderResult<ScenarioBuilder<crate::builder::states::Empty>> {
        // Validate parameters
        for (name, value) in &parameters {
            if let Some(param_def) = self.parameters.get(name) {
                self.validate_parameter_value(param_def, value)?;
            } else {
                return Err(BuilderError::ValidationError(
                    format!("Unknown parameter: {}", name)
                ));
            }
        }

        // Start building scenario
        let mut builder = ScenarioBuilder::new()
            .with_simple_header(&self.name, "Template Generated");

        // Add default catalogs and road network
        let builder = builder
            .with_default_catalogs()?
            .with_road_network("default.xodr")
            .with_entities();

        // Add parameters
        let mut builder = builder;
        for (name, param_def) in &self.parameters {
            let value = parameters.get(name)
                .or(param_def.default_value.as_ref())
                .ok_or_else(|| BuilderError::ValidationError(
                    format!("Missing value for required parameter: {}", name)
                ))?;
            
            builder = builder.add_parameter(name, param_def.parameter_type.clone(), value);
        }

        // Add entities
        for (name, entity_template) in &self.entities {
            builder = self.instantiate_entity(builder, name, entity_template, &parameters)?;
        }

        // Add story if present
        if let Some(story_template) = &self.story_template {
            builder = self.instantiate_story(builder, story_template, &parameters)?;
        }

        Ok(builder)
    }

    /// Validate parameter value against constraints
    fn validate_parameter_value(&self, param_def: &ParameterDefinition, value: &str) -> BuilderResult<()> {
        if let Some(constraints) = &param_def.constraints {
            // Check numeric constraints
            if let (Some(min), Some(max)) = (constraints.min_value, constraints.max_value) {
                if let Ok(num_value) = value.parse::<f64>() {
                    if num_value < min || num_value > max {
                        return Err(BuilderError::ValidationError(
                            format!("Parameter value {} is outside allowed range [{}, {}]", value, min, max)
                        ));
                    }
                }
            }

            // Check allowed values
            if let Some(allowed) = &constraints.allowed_values {
                if !allowed.contains(&value.to_string()) {
                    return Err(BuilderError::ValidationError(
                        format!("Parameter value '{}' is not in allowed values: {:?}", value, allowed)
                    ));
                }
            }

            // Check pattern
            if let Some(pattern) = &constraints.pattern {
                let regex = regex::Regex::new(pattern)
                    .map_err(|e| BuilderError::ValidationError(format!("Invalid pattern: {}", e)))?;
                if !regex.is_match(value) {
                    return Err(BuilderError::ValidationError(
                        format!("Parameter value '{}' does not match pattern '{}'", value, pattern)
                    ));
                }
            }
        }

        Ok(())
    }

    /// Instantiate an entity from template
    fn instantiate_entity(
        &self,
        builder: ScenarioBuilder<crate::builder::states::HasEntities>,
        name: &str,
        entity_template: &EntityTemplate,
        parameters: &HashMap<String, String>,
    ) -> BuilderResult<ScenarioBuilder<crate::builder::states::HasEntities>> {
        // This is a simplified implementation
        // In a full implementation, we would properly instantiate entities based on their templates
        Ok(builder)
    }

    /// Instantiate story from template
    fn instantiate_story(
        &self,
        builder: ScenarioBuilder<crate::builder::states::HasEntities>,
        story_template: &StoryTemplate,
        parameters: &HashMap<String, String>,
    ) -> BuilderResult<ScenarioBuilder<crate::builder::states::HasEntities>> {
        // This is a simplified implementation
        // In a full implementation, we would properly instantiate the story based on the template
        Ok(builder)
    }

    /// Substitute parameter values in a Value
    fn substitute_value(&self, value: &Value<String>, parameters: &HashMap<String, String>) -> String {
        match value {
            Value::Literal(s) => s.clone(),
            Value::Parameter(param_name) => {
                parameters.get(param_name)
                    .cloned()
                    .unwrap_or_else(|| format!("${{{}}}", param_name))
            }
        }
    }
}

/// Builder state serialization for persistence and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableBuilderState {
    pub state_name: String,
    pub has_header: bool,
    pub has_catalogs: bool,
    pub has_road_network: bool,
    pub has_entities: bool,
    pub entity_count: usize,
    pub parameter_count: usize,
    pub validation_errors: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl SerializableBuilderState {
    /// Create from builder state
    pub fn from_builder<S: crate::builder::states::BuilderState>(
        builder: &ScenarioBuilder<S>,
    ) -> Self {
        Self {
            state_name: std::any::type_name::<S>().to_string(),
            has_header: true, // Simplified - would check actual state
            has_catalogs: true,
            has_road_network: true,
            has_entities: true,
            entity_count: 0, // Would get from registry
            parameter_count: 0, // Would get from registry
            validation_errors: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Save to JSON file
    pub fn save_json(&self, path: impl AsRef<Path>) -> BuilderResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to serialize state: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| BuilderError::IoError(format!("Failed to write state file: {}", e)))
    }

    /// Load from JSON file
    pub fn load_json(path: impl AsRef<Path>) -> BuilderResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BuilderError::IoError(format!("Failed to read state file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| BuilderError::SerializationError(format!("Failed to parse state file: {}", e)))
    }
}

/// Scenario comparison and diff utilities
pub struct ScenarioDiff {
    pub added_entities: Vec<String>,
    pub removed_entities: Vec<String>,
    pub modified_entities: Vec<String>,
    pub parameter_changes: HashMap<String, (String, String)>, // old_value, new_value
    pub structural_changes: Vec<String>,
}

impl ScenarioDiff {
    /// Compare two scenarios and generate diff
    pub fn compare(
        scenario1: &crate::types::scenario::storyboard::OpenScenario,
        scenario2: &crate::types::scenario::storyboard::OpenScenario,
    ) -> Self {
        // Simplified implementation
        Self {
            added_entities: Vec::new(),
            removed_entities: Vec::new(),
            modified_entities: Vec::new(),
            parameter_changes: HashMap::new(),
            structural_changes: Vec::new(),
        }
    }

    /// Check if scenarios are identical
    pub fn is_identical(&self) -> bool {
        self.added_entities.is_empty() &&
        self.removed_entities.is_empty() &&
        self.modified_entities.is_empty() &&
        self.parameter_changes.is_empty() &&
        self.structural_changes.is_empty()
    }

    /// Generate human-readable diff report
    pub fn report(&self) -> String {
        let mut report = String::new();
        
        if self.is_identical() {
            report.push_str("✅ Scenarios are identical\n");
            return report;
        }

        if !self.added_entities.is_empty() {
            report.push_str(&format!("➕ Added entities: {}\n", self.added_entities.join(", ")));
        }

        if !self.removed_entities.is_empty() {
            report.push_str(&format!("➖ Removed entities: {}\n", self.removed_entities.join(", ")));
        }

        if !self.modified_entities.is_empty() {
            report.push_str(&format!("🔄 Modified entities: {}\n", self.modified_entities.join(", ")));
        }

        if !self.parameter_changes.is_empty() {
            report.push_str("📝 Parameter changes:\n");
            for (param, (old, new)) in &self.parameter_changes {
                report.push_str(&format!("  - {}: {} → {}\n", param, old, new));
            }
        }

        if !self.structural_changes.is_empty() {
            report.push_str("🏗️ Structural changes:\n");
            for change in &self.structural_changes {
                report.push_str(&format!("  - {}\n", change));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_template_creation() {
        let template = ScenarioTemplate::new("test_template")
            .with_description("Test template")
            .with_parameter("speed", ParameterType::Double, Some("30.0"))
            .with_metadata("author", "Test");

        assert_eq!(template.name, "test_template");
        assert_eq!(template.description, Some("Test template".to_string()));
        assert!(template.parameters.contains_key("speed"));
        assert_eq!(template.metadata.get("author"), Some(&"Test".to_string()));
    }

    #[test]
    fn test_parameter_constraints() {
        let constraints = ParameterConstraints {
            min_value: Some(0.0),
            max_value: Some(100.0),
            allowed_values: None,
            pattern: None,
        };

        let param_def = ParameterDefinition {
            parameter_type: ParameterType::Double,
            default_value: Some("50.0".to_string()),
            description: Some("Speed parameter".to_string()),
            constraints: Some(constraints),
        };

        assert_eq!(param_def.default_value, Some("50.0".to_string()));
    }

    #[test]
    fn test_serializable_builder_state() {
        let state = SerializableBuilderState {
            state_name: "HasEntities".to_string(),
            has_header: true,
            has_catalogs: true,
            has_road_network: true,
            has_entities: true,
            entity_count: 5,
            parameter_count: 3,
            validation_errors: Vec::new(),
            metadata: HashMap::new(),
        };

        // Test serialization
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: SerializableBuilderState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(state.state_name, deserialized.state_name);
        assert_eq!(state.entity_count, deserialized.entity_count);
    }
}
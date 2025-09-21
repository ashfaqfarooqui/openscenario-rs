//! Registry system for tracking and validating builder references
//!
//! This module provides registry systems that track entities, parameters,
//! and catalog references during builder operations, enabling compile-time
//! and build-time validation of cross-references.

use crate::types::{
    basic::{ParameterDeclarations, ParameterDeclaration, Value},
    catalogs::locations::CatalogLocations,
    entities::{ScenarioObject, Vehicle},
    enums::{ObjectType, ParameterType},
    EntityRef, ParameterContext,
};
use super::error::{BuilderError, BuilderResult};
use std::collections::{HashMap, HashSet};

/// Registry for tracking scenario entities and validating references
/// 
/// This registry maintains a list of all entities added to a scenario and
/// provides validation for entity references used in actions and conditions.
/// It ensures that all entity references point to existing entities.
#[derive(Debug, Default)]
pub struct EntityRegistry {
    /// Map of entity names to their definitions
    entities: HashMap<String, ScenarioObject>,
    /// Map of entity names to their object types for quick lookup
    entity_types: HashMap<String, ObjectType>,
    /// Set of entity names for fast existence checking
    entity_names: HashSet<String>,
}

impl EntityRegistry {
    /// Create a new empty entity registry
    /// 
    /// This creates an empty registry that can be populated as entities
    /// are added to the scenario during construction.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entity to the registry
    /// 
    /// This method registers a new entity, making it available for reference
    /// in actions and conditions. Returns an error if an entity with the
    /// same name already exists.
    /// 
    /// # Arguments
    /// * `name` - Unique name for the entity
    /// * `entity` - The scenario object definition
    /// 
    /// # Errors
    /// Returns `BuilderError::ConstraintViolation` if entity name already exists
    pub fn add_entity(&mut self, name: String, entity: ScenarioObject) -> BuilderResult<()> {
        if self.entity_names.contains(&name) {
            return Err(BuilderError::constraint_violation(
                "unique entity names",
                &format!("Entity '{}' already exists", name),
            ));
        }

        // Determine object type from entity structure
        let object_type = self.determine_object_type(&entity);
        
        self.entity_names.insert(name.clone());
        self.entity_types.insert(name.clone(), object_type);
        self.entities.insert(name, entity);
        
        Ok(())
    }

    /// Check if an entity exists in the registry
    /// 
    /// This is a fast lookup to check entity existence without retrieving
    /// the full entity definition.
    pub fn has_entity(&self, name: &str) -> bool {
        self.entity_names.contains(name)
    }

    /// Get entity object type
    /// 
    /// Returns the ObjectType for the specified entity, used for type-specific
    /// validation in actions and conditions.
    pub fn get_entity_type(&self, name: &str) -> Option<&ObjectType> {
        self.entity_types.get(name)
    }

    /// Get entity definition
    /// 
    /// Returns the full ScenarioObject definition for the specified entity,
    /// useful for detailed validation and property access.
    pub fn get_entity(&self, name: &str) -> Option<&ScenarioObject> {
        self.entities.get(name)
    }

    /// Validate an entity reference
    /// 
    /// This method checks that an entity reference points to an existing
    /// entity and returns a helpful error if not.
    /// 
    /// # Arguments
    /// * `entity_ref` - The entity name to validate
    /// 
    /// # Errors
    /// Returns `BuilderError::InvalidEntityReference` if entity doesn't exist
    pub fn validate_reference(&self, entity_ref: &str) -> BuilderResult<()> {
        if !self.has_entity(entity_ref) {
            let available: Vec<String> = self.entity_names.iter().cloned().collect();
            return Err(BuilderError::invalid_entity(entity_ref, &available));
        }
        Ok(())
    }

    /// Get all entity names
    /// 
    /// Returns a vector of all registered entity names, useful for
    /// error messages and autocomplete functionality.
    pub fn get_entity_names(&self) -> Vec<String> {
        self.entity_names.iter().cloned().collect()
    }

    /// Get entities by type
    /// 
    /// Returns all entities of a specific type, useful for type-specific
    /// operations and validation.
    pub fn get_entities_by_type(&self, object_type: &ObjectType) -> Vec<String> {
        self.entity_types
            .iter()
            .filter(|(_, ot)| *ot == object_type)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Determine object type from scenario object structure
    /// 
    /// This private method examines a ScenarioObject to determine its type
    /// based on which variant is present in the entity object union.
    fn determine_object_type(&self, entity: &ScenarioObject) -> ObjectType {
        // Check which object type is present in the scenario object
        if entity.catalog_reference.is_some() {
            // For catalog references, we'd need to look up the catalog to determine type
            // For now, default to Vehicle - this should be improved with actual catalog loading
            ObjectType::Vehicle
        } else if entity.vehicle.is_some() {
            ObjectType::Vehicle
        } else if entity.pedestrian.is_some() {
            ObjectType::Pedestrian
        } else {
            // Default fallback
            ObjectType::Vehicle
        }
    }
}

/// Registry for tracking scenario parameters and validating references
/// 
/// This registry maintains parameter declarations and provides validation
/// for parameter references used throughout the scenario.
#[derive(Debug, Default)]
pub struct ParameterRegistry {
    /// Map of parameter names to their declarations
    parameters: HashMap<String, ParameterDeclaration>,
    /// Set of parameter names for fast existence checking
    parameter_names: HashSet<String>,
}

impl ParameterRegistry {
    /// Create a new empty parameter registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Add parameter declarations from ParameterDeclarations
    /// 
    /// This method processes a ParameterDeclarations structure and adds
    /// all contained parameters to the registry for reference validation.
    pub fn add_parameter_declarations(&mut self, declarations: &ParameterDeclarations) -> BuilderResult<()> {
        for declaration in &declarations.parameter_declarations {
            self.add_parameter(declaration.clone())?;
        }
        Ok(())
    }

    /// Add a single parameter declaration
    /// 
    /// This method adds a parameter to the registry, making it available
    /// for reference in expressions and actions.
    pub fn add_parameter(&mut self, declaration: ParameterDeclaration) -> BuilderResult<()> {
        let name = declaration.name.as_literal()
            .ok_or_else(|| BuilderError::constraint_violation(
                "parameter name", 
                "Parameter name must be a literal value"
            ))?;

        if self.parameter_names.contains(name) {
            return Err(BuilderError::constraint_violation(
                "unique parameter names",
                &format!("Parameter '{}' already exists", name),
            ));
        }

        self.parameter_names.insert(name.to_string());
        self.parameters.insert(name.to_string(), declaration);
        Ok(())
    }

    /// Validate a parameter reference
    /// 
    /// This method checks that a parameter reference points to an existing
    /// parameter declaration.
    pub fn validate_reference(&self, parameter: &str) -> BuilderResult<()> {
        if !self.parameter_names.contains(parameter) {
            let available: Vec<String> = self.parameter_names.iter().cloned().collect();
            return Err(BuilderError::invalid_parameter(parameter, &available));
        }
        Ok(())
    }

    /// Get parameter declaration
    /// 
    /// Returns the ParameterDeclaration for the specified parameter name.
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterDeclaration> {
        self.parameters.get(name)
    }

    /// Get all parameter names
    /// 
    /// Returns a vector of all registered parameter names.
    pub fn get_parameter_names(&self) -> Vec<String> {
        self.parameter_names.iter().cloned().collect()
    }

    /// Create parameter context for resolution
    /// 
    /// Creates a ParameterContext that can be used with the existing
    /// parameter resolution system for resolving parameter references.
    pub fn create_context(&self) -> ParameterContext {
        let mut context = ParameterContext::new();
        for (name, declaration) in &self.parameters {
            if let Some(value) = declaration.value.as_literal() {
                context = context.with_parameter(name.clone(), value.to_string());
            }
        }
        context
    }
}

/// Registry for tracking catalog locations and validating catalog references
/// 
/// This registry manages catalog locations and provides validation for
/// catalog references used in entities and other scenario components.
#[derive(Debug, Default)]
pub struct CatalogRegistry {
    /// Configured catalog locations
    catalog_locations: Option<CatalogLocations>,
    /// Map of catalog type to directory path
    catalog_paths: HashMap<String, String>,
}

impl CatalogRegistry {
    /// Create a new empty catalog registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Set catalog locations
    /// 
    /// This method configures the catalog locations that define where
    /// to find different types of catalogs.
    pub fn set_catalog_locations(&mut self, locations: CatalogLocations) {
        // Extract directory paths from catalog locations
        if let Some(ref vehicle_catalog) = locations.vehicle_catalog {
            if let Some(dir) = vehicle_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("vehicle".to_string(), dir.to_string());
            }
        }
        if let Some(ref controller_catalog) = locations.controller_catalog {
            if let Some(dir) = controller_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("controller".to_string(), dir.to_string());
            }
        }
        if let Some(ref pedestrian_catalog) = locations.pedestrian_catalog {
            if let Some(dir) = pedestrian_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("pedestrian".to_string(), dir.to_string());
            }
        }
        if let Some(ref misc_object_catalog) = locations.misc_object_catalog {
            if let Some(dir) = misc_object_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("misc_object".to_string(), dir.to_string());
            }
        }
        if let Some(ref environment_catalog) = locations.environment_catalog {
            if let Some(dir) = environment_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("environment".to_string(), dir.to_string());
            }
        }
        if let Some(ref maneuver_catalog) = locations.maneuver_catalog {
            if let Some(dir) = maneuver_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("maneuver".to_string(), dir.to_string());
            }
        }
        if let Some(ref trajectory_catalog) = locations.trajectory_catalog {
            if let Some(dir) = trajectory_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("trajectory".to_string(), dir.to_string());
            }
        }
        if let Some(ref route_catalog) = locations.route_catalog {
            if let Some(dir) = route_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("route".to_string(), dir.to_string());
            }
        }

        self.catalog_locations = Some(locations);
    }

    /// Validate a catalog reference (basic validation)
    /// 
    /// This method checks that catalog locations are configured.
    /// In a full implementation, this would load and validate the actual catalog file.
    pub fn validate_basic_setup(&self) -> BuilderResult<()> {
        if self.catalog_locations.is_none() {
            return Err(BuilderError::catalog_reference(
                "unknown", "unknown",
                "No catalog locations configured. Call .with_catalog_locations() first."
            ));
        }
        Ok(())
    }

    /// Load catalog content (placeholder for full implementation)
    /// 
    /// This method would load catalog content from disk and cache it
    /// for validation. Current implementation is a placeholder.
    pub fn load_catalog(&mut self, catalog_type: &str) -> BuilderResult<()> {
        if self.catalog_paths.contains_key(catalog_type) {
            // TODO: Implement actual catalog loading from filesystem
            // For now, just record that we attempted to load it
            Ok(())
        } else {
            Err(BuilderError::catalog_reference(
                catalog_type, "unknown",
                &format!("No catalog location configured for type '{}'", catalog_type)
            ))
        }
    }

    /// Get configured catalog paths
    /// 
    /// Returns a map of catalog types to their configured directory paths.
    pub fn get_catalog_paths(&self) -> &HashMap<String, String> {
        &self.catalog_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_registry_basic_operations() {
        let mut registry = EntityRegistry::new();
        
        // Test empty registry
        assert!(!registry.has_entity("test"));
        assert!(registry.get_entity_names().is_empty());
        
        // Test adding entity
        let entity = ScenarioObject {
            name: Value::literal("test_vehicle".to_string()),
            catalog_reference: None,
            vehicle: Some(Vehicle::default()),
            pedestrian: None,
            object_controller: None,
        };
        
        registry.add_entity("test_vehicle".to_string(), entity).unwrap();
        
        // Test entity exists
        assert!(registry.has_entity("test_vehicle"));
        assert_eq!(registry.get_entity_names().len(), 1);
        
        // Test validation
        assert!(registry.validate_reference("test_vehicle").is_ok());
        assert!(registry.validate_reference("nonexistent").is_err());
    }

    #[test]
    fn test_entity_registry_duplicate_names() {
        let mut registry = EntityRegistry::new();
        
        let entity1 = ScenarioObject {
            name: Value::literal("duplicate".to_string()),
            catalog_reference: None,
            vehicle: Some(Vehicle::default()),
            pedestrian: None,
            object_controller: None,
        };
        let entity2 = ScenarioObject {
            name: Value::literal("duplicate".to_string()),
            catalog_reference: None,
            vehicle: Some(Vehicle::default()),
            pedestrian: None,
            object_controller: None,
        };
        
        // First addition should succeed
        assert!(registry.add_entity("duplicate".to_string(), entity1).is_ok());
        
        // Second addition should fail
        assert!(registry.add_entity("duplicate".to_string(), entity2).is_err());
    }

    #[test]
    fn test_parameter_registry_basic_operations() {
        let mut registry = ParameterRegistry::new();
        
        let param_decl = ParameterDeclaration {
            name: Value::Literal("test_param".to_string()),
            parameter_type: ParameterType::Double,
            value: Value::Literal("10.0".to_string()),
            constraint_groups: Vec::new(),
        };
        
        registry.add_parameter(param_decl).unwrap();
        
        assert!(registry.validate_reference("test_param").is_ok());
        assert!(registry.validate_reference("nonexistent").is_err());
        assert_eq!(registry.get_parameter_names().len(), 1);
    }

    #[test] 
    fn test_catalog_registry_basic_operations() {
        let mut registry = CatalogRegistry::new();
        
        // Test empty registry
        assert!(registry.get_catalog_paths().is_empty());
        
        // Test basic validation without catalog locations
        assert!(registry.validate_basic_setup().is_err());
    }
}
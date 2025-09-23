//! Registry system for tracking and validating builder references
//!
//! This module provides registries for tracking entities, parameters, and catalogs
//! during scenario construction, enabling validation and reference resolution.

use crate::types::{
    basic::{ParameterDeclarations, ParameterDeclaration},
    catalogs::locations::CatalogLocations,
    entities::ScenarioObject,
    enums::ObjectType,
    ParameterContext,
};
use super::error::{BuilderError, BuilderResult};
use std::collections::{HashMap, HashSet};

/// Registry for tracking scenario entities and their types
#[derive(Debug, Default)]
pub struct EntityRegistry {
    /// Map of entity name to scenario object
    entities: HashMap<String, ScenarioObject>,
    /// Map of entity name to object type for quick lookup
    entity_types: HashMap<String, ObjectType>,
    /// Set of entity names for duplicate checking
    entity_names: HashSet<String>,
}

impl EntityRegistry {
    /// Create a new empty entity registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entity to the registry
    pub fn add_entity(&mut self, name: String, entity: ScenarioObject) -> BuilderResult<()> {
        // Check for duplicate names
        if self.entity_names.contains(&name) {
            return Err(BuilderError::duplicate_entity(&name));
        }

        // Determine object type from the entity
        let object_type = self.detect_object_type(&entity)?;

        // Add to all tracking structures
        self.entity_names.insert(name.clone());
        self.entity_types.insert(name.clone(), object_type);
        self.entities.insert(name, entity);

        Ok(())
    }

    /// Get an entity by name
    pub fn get_entity(&self, name: &str) -> Option<&ScenarioObject> {
        self.entities.get(name)
    }

    /// Get the object type for an entity
    pub fn get_entity_type(&self, name: &str) -> Option<&ObjectType> {
        self.entity_types.get(name)
    }

    /// Check if an entity exists
    pub fn has_entity(&self, name: &str) -> bool {
        self.entity_names.contains(name)
    }

    /// Get all entity names
    pub fn get_entity_names(&self) -> Vec<String> {
        self.entity_names.iter().cloned().collect()
    }

    /// Get count of entities
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Validate an entity reference
    pub fn validate_entity_reference(&self, name: &str) -> BuilderResult<()> {
        if self.has_entity(name) {
            Ok(())
        } else {
            Err(BuilderError::entity_not_found(name, &self.get_entity_names()))
        }
    }

    /// Detect object type from scenario object
    fn detect_object_type(&self, entity: &ScenarioObject) -> BuilderResult<ObjectType> {
        if entity.vehicle.is_some() {
            Ok(ObjectType::Vehicle)
        } else if entity.pedestrian.is_some() {
            Ok(ObjectType::Pedestrian)
        } else if entity.catalog_reference.is_some() {
            // For catalog references, we'll assume vehicle for now
            // In a full implementation, we'd need to resolve the catalog reference
            Ok(ObjectType::Vehicle)
        } else {
            Err(BuilderError::validation_error(
                "Entity has no vehicle, pedestrian, or catalog reference",
                "Ensure entity has at least one object type defined"
            ))
        }
    }

    /// Clear all entities
    pub fn clear(&mut self) {
        self.entities.clear();
        self.entity_types.clear();
        self.entity_names.clear();
    }
}

/// Registry for tracking scenario parameters
#[derive(Debug, Default)]
pub struct ParameterRegistry {
    /// Map of parameter name to parameter declaration
    parameters: HashMap<String, ParameterDeclaration>,
    /// Set of parameter names for quick lookup
    parameter_names: HashSet<String>,
}

impl ParameterRegistry {
    /// Create a new empty parameter registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Add parameters from parameter declarations
    pub fn add_parameter_declarations(&mut self, declarations: &ParameterDeclarations) -> BuilderResult<()> {
        for declaration in &declarations.parameter_declarations {
            let name = declaration.name.as_literal()
                .ok_or_else(|| BuilderError::validation_error(
                    "Parameter name must be a literal value",
                    "Use literal strings for parameter names"
                ))?;

            if self.parameter_names.contains(name) {
                return Err(BuilderError::validation_error(
                    &format!("Duplicate parameter name: {}", name),
                    "Parameter names must be unique"
                ));
            }

            self.parameter_names.insert(name.clone());
            self.parameters.insert(name.clone(), declaration.clone());
        }

        Ok(())
    }

    /// Add a single parameter
    pub fn add_parameter(&mut self, name: String, declaration: ParameterDeclaration) -> BuilderResult<()> {
        if self.parameter_names.contains(&name) {
            return Err(BuilderError::validation_error(
                &format!("Duplicate parameter name: {}", name),
                "Parameter names must be unique"
            ));
        }

        self.parameter_names.insert(name.clone());
        self.parameters.insert(name, declaration);
        Ok(())
    }

    /// Get a parameter declaration by name
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterDeclaration> {
        self.parameters.get(name)
    }

    /// Check if a parameter exists
    pub fn has_parameter(&self, name: &str) -> bool {
        self.parameter_names.contains(name)
    }

    /// Get all parameter names
    pub fn get_parameter_names(&self) -> Vec<String> {
        self.parameter_names.iter().cloned().collect()
    }

    /// Get count of parameters
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Validate a parameter reference
    pub fn validate_parameter_reference(&self, name: &str) -> BuilderResult<()> {
        if self.has_parameter(name) {
            Ok(())
        } else {
            Err(BuilderError::parameter_not_found(name, &self.get_parameter_names()))
        }
    }

    /// Create a parameter context from current parameters
    pub fn create_parameter_context(&self) -> ParameterContext {
        let mut context = ParameterContext::new();
        
        for (name, declaration) in &self.parameters {
            if let Some(value) = declaration.value.as_literal() {
                context = context.with_parameter(name.clone(), value.clone());
            }
        }

        context
    }

    /// Clear all parameters
    pub fn clear(&mut self) {
        self.parameters.clear();
        self.parameter_names.clear();
    }
}

/// Registry for tracking catalog locations and references
#[derive(Debug, Default)]
pub struct CatalogRegistry {
    /// Catalog locations configuration
    catalog_locations: Option<CatalogLocations>,
    /// Map of catalog type to directory path
    catalog_paths: HashMap<String, String>,
    /// Set of available catalog types
    available_catalogs: HashSet<String>,
}

impl CatalogRegistry {
    /// Create a new empty catalog registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Set catalog locations and extract directory paths
    pub fn set_catalog_locations(&mut self, locations: CatalogLocations) -> BuilderResult<()> {
        self.catalog_paths.clear();
        self.available_catalogs.clear();

        // Extract directory paths from each catalog location
        if let Some(vehicle_catalog) = &locations.vehicle_catalog {
            if let Some(path) = vehicle_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("vehicle".to_string(), path.clone());
                self.available_catalogs.insert("vehicle".to_string());
            }
        }

        if let Some(controller_catalog) = &locations.controller_catalog {
            if let Some(path) = controller_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("controller".to_string(), path.clone());
                self.available_catalogs.insert("controller".to_string());
            }
        }

        if let Some(pedestrian_catalog) = &locations.pedestrian_catalog {
            if let Some(path) = pedestrian_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("pedestrian".to_string(), path.clone());
                self.available_catalogs.insert("pedestrian".to_string());
            }
        }

        // misc_object catalog support removed - MiscObject not implemented in type system yet
        // if let Some(misc_object_catalog) = &locations.misc_object_catalog {
        //     if let Some(path) = misc_object_catalog.directory.path.as_literal() {
        //         self.catalog_paths.insert("misc_object".to_string(), path.clone());
        //         self.available_catalogs.insert("misc_object".to_string());
        //     }
        // }

        if let Some(environment_catalog) = &locations.environment_catalog {
            if let Some(path) = environment_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("environment".to_string(), path.clone());
                self.available_catalogs.insert("environment".to_string());
            }
        }

        if let Some(maneuver_catalog) = &locations.maneuver_catalog {
            if let Some(path) = maneuver_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("maneuver".to_string(), path.clone());
                self.available_catalogs.insert("maneuver".to_string());
            }
        }

        if let Some(trajectory_catalog) = &locations.trajectory_catalog {
            if let Some(path) = trajectory_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("trajectory".to_string(), path.clone());
                self.available_catalogs.insert("trajectory".to_string());
            }
        }

        if let Some(route_catalog) = &locations.route_catalog {
            if let Some(path) = route_catalog.directory.path.as_literal() {
                self.catalog_paths.insert("route".to_string(), path.clone());
                self.available_catalogs.insert("route".to_string());
            }
        }

        self.catalog_locations = Some(locations);
        Ok(())
    }

    /// Get catalog locations
    pub fn get_catalog_locations(&self) -> Option<&CatalogLocations> {
        self.catalog_locations.as_ref()
    }

    /// Get directory path for a catalog type
    pub fn get_catalog_path(&self, catalog_type: &str) -> Option<&String> {
        self.catalog_paths.get(catalog_type)
    }

    /// Check if a catalog type is available
    pub fn has_catalog(&self, catalog_type: &str) -> bool {
        self.available_catalogs.contains(catalog_type)
    }

    /// Get all available catalog types
    pub fn get_available_catalogs(&self) -> Vec<String> {
        self.available_catalogs.iter().cloned().collect()
    }

    /// Validate a catalog reference (placeholder implementation)
    pub fn validate_catalog_reference(&self, catalog_type: &str, entry_name: &str) -> BuilderResult<()> {
        if !self.has_catalog(catalog_type) {
            return Err(BuilderError::catalog_entry_not_found(
                catalog_type,
                entry_name,
                &format!("Available catalogs: {}", self.get_available_catalogs().join(", "))
            ));
        }

        // In a full implementation, we would:
        // 1. Load the catalog file from the directory
        // 2. Parse the catalog content
        // 3. Check if the entry exists
        // For now, we'll just validate that the catalog type exists

        Ok(())
    }

    /// Get count of available catalogs
    pub fn catalog_count(&self) -> usize {
        self.available_catalogs.len()
    }

    /// Clear all catalog information
    pub fn clear(&mut self) {
        self.catalog_locations = None;
        self.catalog_paths.clear();
        self.available_catalogs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        basic::Value,
        entities::Vehicle,
        catalogs::locations::VehicleCatalogLocation,
        enums::ParameterType,
    };

    #[test]
    fn test_entity_registry_basic_operations() {
        let mut registry = EntityRegistry::new();
        
        // Test empty registry
        assert_eq!(registry.entity_count(), 0);
        assert!(!registry.has_entity("test"));
        assert!(registry.get_entity_names().is_empty());

        // Create a test entity
        let vehicle = Vehicle::default();
        let entity = ScenarioObject::new_vehicle("test_vehicle".to_string(), vehicle);

        // Add entity
        registry.add_entity("test_vehicle".to_string(), entity).unwrap();

        // Test entity was added
        assert_eq!(registry.entity_count(), 1);
        assert!(registry.has_entity("test_vehicle"));
        assert_eq!(registry.get_entity_names(), vec!["test_vehicle"]);
        assert_eq!(registry.get_entity_type("test_vehicle"), Some(&ObjectType::Vehicle));
    }

    #[test]
    fn test_entity_registry_duplicate_names() {
        let mut registry = EntityRegistry::new();
        
        let vehicle1 = Vehicle::default();
        let entity1 = ScenarioObject::new_vehicle("test".to_string(), vehicle1);
        
        let vehicle2 = Vehicle::default();
        let entity2 = ScenarioObject::new_vehicle("test".to_string(), vehicle2);

        // First addition should succeed
        registry.add_entity("test".to_string(), entity1).unwrap();

        // Second addition with same name should fail
        let result = registry.add_entity("test".to_string(), entity2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::DuplicateEntityName { .. }));
    }

    #[test]
    fn test_entity_registry_validation() {
        let mut registry = EntityRegistry::new();
        
        let vehicle = Vehicle::default();
        let entity = ScenarioObject::new_vehicle("test".to_string(), vehicle);
        registry.add_entity("test".to_string(), entity).unwrap();

        // Valid reference should succeed
        assert!(registry.validate_entity_reference("test").is_ok());

        // Invalid reference should fail
        let result = registry.validate_entity_reference("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::EntityNotFound { .. }));
    }

    #[test]
    fn test_parameter_registry_basic_operations() {
        let mut registry = ParameterRegistry::new();
        
        // Test empty registry
        assert_eq!(registry.parameter_count(), 0);
        assert!(!registry.has_parameter("test"));

        // Create a test parameter
        let param = ParameterDeclaration {
            name: Value::literal("test_param".to_string()),
            parameter_type: ParameterType::Double,
            value: Value::literal("10.0".to_string()),
            constraint_groups: Vec::new(),
        };

        // Add parameter
        registry.add_parameter("test_param".to_string(), param).unwrap();

        // Test parameter was added
        assert_eq!(registry.parameter_count(), 1);
        assert!(registry.has_parameter("test_param"));
        assert_eq!(registry.get_parameter_names(), vec!["test_param"]);
    }

    #[test]
    fn test_parameter_registry_duplicate_names() {
        let mut registry = ParameterRegistry::new();
        
        let param1 = ParameterDeclaration {
            name: Value::literal("test".to_string()),
            parameter_type: ParameterType::Double,
            value: Value::literal("10.0".to_string()),
            constraint_groups: Vec::new(),
        };
        
        let param2 = ParameterDeclaration {
            name: Value::literal("test".to_string()),
            parameter_type: ParameterType::String,
            value: Value::literal("hello".to_string()),
            constraint_groups: Vec::new(),
        };

        // First addition should succeed
        registry.add_parameter("test".to_string(), param1).unwrap();

        // Second addition with same name should fail
        let result = registry.add_parameter("test".to_string(), param2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::ValidationError { .. }));
    }

    #[test]
    fn test_parameter_registry_context_creation() {
        let mut registry = ParameterRegistry::new();
        
        let param = ParameterDeclaration {
            name: Value::literal("speed".to_string()),
            parameter_type: ParameterType::Double,
            value: Value::literal("50.0".to_string()),
            constraint_groups: Vec::new(),
        };

        registry.add_parameter("speed".to_string(), param).unwrap();

        let context = registry.create_parameter_context();
        assert_eq!(context.get("speed"), Some("50.0"));
    }

    #[test]
    fn test_catalog_registry_basic_operations() {
        let mut registry = CatalogRegistry::new();
        
        // Test empty registry
        assert_eq!(registry.catalog_count(), 0);
        assert!(!registry.has_catalog("vehicle"));

        // Create catalog locations
        let mut locations = CatalogLocations::new();
        locations.vehicle_catalog = Some(VehicleCatalogLocation::new(
            crate::types::basic::Directory::new("./catalogs/vehicles".to_string())
        ));

        // Set catalog locations
        registry.set_catalog_locations(locations).unwrap();

        // Test catalog was added
        assert_eq!(registry.catalog_count(), 1);
        assert!(registry.has_catalog("vehicle"));
        assert_eq!(registry.get_catalog_path("vehicle"), Some(&"./catalogs/vehicles".to_string()));
    }

    #[test]
    fn test_catalog_registry_validation() {
        let mut registry = CatalogRegistry::new();
        
        let mut locations = CatalogLocations::new();
        locations.vehicle_catalog = Some(VehicleCatalogLocation::new(
            crate::types::basic::Directory::new("./catalogs/vehicles".to_string())
        ));
        registry.set_catalog_locations(locations).unwrap();

        // Valid catalog type should succeed
        assert!(registry.validate_catalog_reference("vehicle", "sports_car").is_ok());

        // Invalid catalog type should fail
        let result = registry.validate_catalog_reference("nonexistent", "entry");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::CatalogEntryNotFound { .. }));
    }

    #[test]
    fn test_registry_clear_operations() {
        let mut entity_registry = EntityRegistry::new();
        let mut parameter_registry = ParameterRegistry::new();
        let mut catalog_registry = CatalogRegistry::new();

        // Add some data
        let vehicle = Vehicle::default();
        let entity = ScenarioObject::new_vehicle("test".to_string(), vehicle);
        entity_registry.add_entity("test".to_string(), entity).unwrap();

        let param = ParameterDeclaration {
            name: Value::literal("test".to_string()),
            parameter_type: ParameterType::Double,
            value: Value::literal("10.0".to_string()),
            constraint_groups: Vec::new(),
        };
        parameter_registry.add_parameter("test".to_string(), param).unwrap();

        let locations = CatalogLocations::new();
        catalog_registry.set_catalog_locations(locations).unwrap();

        // Clear all registries
        entity_registry.clear();
        parameter_registry.clear();
        catalog_registry.clear();

        // Verify all are empty
        assert_eq!(entity_registry.entity_count(), 0);
        assert_eq!(parameter_registry.parameter_count(), 0);
        assert_eq!(catalog_registry.catalog_count(), 0);
    }
}
//! Builder-specific error types and handling
//!
//! This module provides comprehensive error handling for the builder system,
//! with detailed error messages and helpful suggestions for fixing common
//! construction issues.

use thiserror::Error;

/// Builder-specific error types with detailed context and suggestions
#[derive(Debug, Error)]
pub enum BuilderError {
    /// Required field is missing during build
    #[error("Required field missing: {field}. {suggestion}")]
    MissingRequiredField {
        field: String,
        suggestion: String,
    },

    /// Invalid state transition attempted
    #[error("Invalid state transition: {from} -> {to}. {suggestion}")]
    InvalidStateTransition {
        from: String,
        to: String,
        suggestion: String,
    },

    /// Entity reference not found
    #[error("Entity '{entity}' not found. Available entities: {available}")]
    EntityNotFound {
        entity: String,
        available: String,
    },

    /// Duplicate entity name
    #[error("Entity name '{name}' already exists. Entity names must be unique.")]
    DuplicateEntityName { name: String },

    /// Parameter reference not found
    #[error("Parameter '{parameter}' not found. Available parameters: {available}")]
    ParameterNotFound {
        parameter: String,
        available: String,
    },

    /// Invalid parameter value
    #[error("Invalid parameter value for '{parameter}': {value}. {suggestion}")]
    InvalidParameterValue {
        parameter: String,
        value: String,
        suggestion: String,
    },

    /// Catalog reference not found
    #[error("Catalog entry '{entry}' not found in catalog '{catalog}'. {suggestion}")]
    CatalogEntryNotFound {
        catalog: String,
        entry: String,
        suggestion: String,
    },

    /// Invalid catalog location
    #[error("Invalid catalog location: {path}. {suggestion}")]
    InvalidCatalogLocation {
        path: String,
        suggestion: String,
    },

    /// Validation error during build
    #[error("Validation failed: {message}. {suggestion}")]
    ValidationError {
        message: String,
        suggestion: String,
    },

    /// Integration error with existing error system
    #[error("Integration error: {0}")]
    IntegrationError(#[from] crate::error::Error),

    /// Generic builder error with context
    #[error("Builder error: {message}")]
    Generic { message: String },
}

/// Result type alias for builder operations
pub type BuilderResult<T> = Result<T, BuilderError>;

/// Trait for converting results into builder results
pub trait IntoBuilderResult<T> {
    /// Convert into a builder result with context
    fn into_builder_result(self) -> BuilderResult<T>;
    
    /// Convert into a builder result with additional context
    fn with_builder_context(self, context: &str) -> BuilderResult<T>;
}

impl<T> IntoBuilderResult<T> for Result<T, crate::error::Error> {
    fn into_builder_result(self) -> BuilderResult<T> {
        self.map_err(BuilderError::IntegrationError)
    }

    fn with_builder_context(self, context: &str) -> BuilderResult<T> {
        self.map_err(|e| BuilderError::ValidationError {
            message: e.to_string(),
            suggestion: format!("Check {}", context),
        })
    }
}

impl BuilderError {
    /// Create a missing required field error with suggestion
    pub fn missing_field(field: &str, suggestion: &str) -> Self {
        Self::MissingRequiredField {
            field: field.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create an invalid state transition error
    pub fn invalid_transition(from: &str, to: &str, suggestion: &str) -> Self {
        Self::InvalidStateTransition {
            from: from.to_string(),
            to: to.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create an entity not found error with available options
    pub fn entity_not_found(entity: &str, available: &[String]) -> Self {
        Self::EntityNotFound {
            entity: entity.to_string(),
            available: if available.is_empty() {
                "none".to_string()
            } else {
                available.join(", ")
            },
        }
    }

    /// Create a duplicate entity name error
    pub fn duplicate_entity(name: &str) -> Self {
        Self::DuplicateEntityName {
            name: name.to_string(),
        }
    }

    /// Create a parameter not found error with available options
    pub fn parameter_not_found(parameter: &str, available: &[String]) -> Self {
        Self::ParameterNotFound {
            parameter: parameter.to_string(),
            available: if available.is_empty() {
                "none".to_string()
            } else {
                available.join(", ")
            },
        }
    }

    /// Create an invalid parameter value error
    pub fn invalid_parameter_value(parameter: &str, value: &str, suggestion: &str) -> Self {
        Self::InvalidParameterValue {
            parameter: parameter.to_string(),
            value: value.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a catalog entry not found error
    pub fn catalog_entry_not_found(catalog: &str, entry: &str, suggestion: &str) -> Self {
        Self::CatalogEntryNotFound {
            catalog: catalog.to_string(),
            entry: entry.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create an invalid catalog location error
    pub fn invalid_catalog_location(path: &str, suggestion: &str) -> Self {
        Self::InvalidCatalogLocation {
            path: path.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a validation error with suggestion
    pub fn validation_error(message: &str, suggestion: &str) -> Self {
        Self::ValidationError {
            message: message.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a generic builder error
    pub fn generic(message: &str) -> Self {
        Self::Generic {
            message: message.to_string(),
        }
    }

    /// Add context to an existing error
    pub fn with_context(self, context: &str) -> Self {
        match self {
            Self::MissingRequiredField { field, suggestion } => Self::MissingRequiredField {
                field,
                suggestion: format!("{} ({})", suggestion, context),
            },
            Self::ValidationError { message, suggestion } => Self::ValidationError {
                message: format!("{} ({})", message, context),
                suggestion,
            },
            other => other,
        }
    }

    /// Get helpful suggestions for common error patterns
    pub fn get_suggestion(&self) -> Option<&str> {
        match self {
            Self::MissingRequiredField { suggestion, .. } => Some(suggestion),
            Self::InvalidStateTransition { suggestion, .. } => Some(suggestion),
            Self::InvalidParameterValue { suggestion, .. } => Some(suggestion),
            Self::CatalogEntryNotFound { suggestion, .. } => Some(suggestion),
            Self::InvalidCatalogLocation { suggestion, .. } => Some(suggestion),
            Self::ValidationError { suggestion, .. } => Some(suggestion),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_field_error() {
        let error = BuilderError::missing_field("file_header", "Call with_header() first");
        
        assert!(error.to_string().contains("Required field missing: file_header"));
        assert!(error.to_string().contains("Call with_header() first"));
        assert_eq!(error.get_suggestion(), Some("Call with_header() first"));
    }

    #[test]
    fn test_entity_not_found_error() {
        let available = vec!["vehicle1".to_string(), "vehicle2".to_string()];
        let error = BuilderError::entity_not_found("vehicle3", &available);
        
        assert!(error.to_string().contains("Entity 'vehicle3' not found"));
        assert!(error.to_string().contains("vehicle1, vehicle2"));
    }

    #[test]
    fn test_entity_not_found_empty_available() {
        let error = BuilderError::entity_not_found("vehicle1", &[]);
        
        assert!(error.to_string().contains("Available entities: none"));
    }

    #[test]
    fn test_duplicate_entity_error() {
        let error = BuilderError::duplicate_entity("vehicle1");
        
        assert!(error.to_string().contains("Entity name 'vehicle1' already exists"));
        assert!(error.to_string().contains("must be unique"));
    }

    #[test]
    fn test_parameter_not_found_error() {
        let available = vec!["speed".to_string(), "distance".to_string()];
        let error = BuilderError::parameter_not_found("acceleration", &available);
        
        assert!(error.to_string().contains("Parameter 'acceleration' not found"));
        assert!(error.to_string().contains("speed, distance"));
    }

    #[test]
    fn test_invalid_parameter_value_error() {
        let error = BuilderError::invalid_parameter_value(
            "speed",
            "invalid",
            "Must be a positive number"
        );
        
        assert!(error.to_string().contains("Invalid parameter value for 'speed': invalid"));
        assert!(error.to_string().contains("Must be a positive number"));
    }

    #[test]
    fn test_catalog_entry_not_found_error() {
        let error = BuilderError::catalog_entry_not_found(
            "vehicles",
            "sports_car",
            "Check catalog file exists and entry name is correct"
        );
        
        assert!(error.to_string().contains("Catalog entry 'sports_car' not found in catalog 'vehicles'"));
        assert!(error.to_string().contains("Check catalog file exists"));
    }

    #[test]
    fn test_validation_error() {
        let error = BuilderError::validation_error(
            "Invalid road network",
            "Ensure LogicFile path is valid"
        );
        
        assert!(error.to_string().contains("Validation failed: Invalid road network"));
        assert!(error.to_string().contains("Ensure LogicFile path is valid"));
    }

    #[test]
    fn test_error_with_context() {
        let error = BuilderError::missing_field("entities", "Add at least one entity")
            .with_context("during scenario build");
        
        let error_str = error.to_string();
        assert!(error_str.contains("Add at least one entity (during scenario build)"));
    }

    #[test]
    fn test_integration_error() {
        let base_error = crate::error::Error::validation_error("test", "test message");
        let builder_error = BuilderError::IntegrationError(base_error);
        
        assert!(builder_error.to_string().contains("Integration error"));
    }

    #[test]
    fn test_into_builder_result() {
        let result: Result<(), crate::error::Error> = Err(crate::error::Error::validation_error("test", "message"));
        let builder_result = result.into_builder_result();
        
        assert!(builder_result.is_err());
        assert!(matches!(builder_result.unwrap_err(), BuilderError::IntegrationError(_)));
    }

    #[test]
    fn test_with_builder_context() {
        let result: Result<(), crate::error::Error> = Err(crate::error::Error::validation_error("test", "message"));
        let builder_result = result.with_builder_context("entity validation");
        
        assert!(builder_result.is_err());
        if let Err(BuilderError::ValidationError { suggestion, .. }) = builder_result {
            assert!(suggestion.contains("Check entity validation"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_error_display_formatting() {
        let error = BuilderError::missing_field("header", "Use with_header()");
        let display = format!("{}", error);
        
        assert!(display.contains("Required field missing: header"));
        assert!(display.contains("Use with_header()"));
    }

    #[test]
    fn test_generic_error() {
        let error = BuilderError::generic("Something went wrong");
        
        assert!(error.to_string().contains("Builder error: Something went wrong"));
        assert!(error.get_suggestion().is_none());
    }
}
//! Builder-specific error types and handling
//!
//! This module provides comprehensive error types for builder operations,
//! offering detailed context about what went wrong during scenario construction.

use thiserror::Error;

/// Comprehensive error type for builder operations
/// 
/// These errors provide specific context about builder failures, making it
/// easy for users to understand and fix issues during scenario construction.
#[derive(Debug, Error)]
pub enum BuilderError {
    /// A required field was not set before attempting to build
    /// 
    /// This error occurs when trying to build a document without setting
    /// all required fields. The field name helps identify what's missing.
    #[error("Required field missing: {field}. {suggestion}")]
    MissingRequiredField {
        field: String,
        suggestion: String,
    },

    /// Reference to an entity that doesn't exist
    /// 
    /// This error occurs when actions or conditions reference entities that
    /// haven't been added to the scenario. Entity names must match exactly.
    #[error("Invalid entity reference: '{entity_ref}'. Available entities: {available}")]
    InvalidEntityReference {
        entity_ref: String,
        available: String,
    },

    /// Reference to a parameter that doesn't exist
    /// 
    /// This error occurs when trying to use a parameter that wasn't declared
    /// in the parameter declarations section.
    #[error("Invalid parameter reference: '{parameter}'. Available parameters: {available}")]
    InvalidParameterReference {
        parameter: String,
        available: String,
    },

    /// A constraint was violated during construction
    /// 
    /// This error occurs when values or combinations violate OpenSCENARIO
    /// constraints, such as invalid ranges or incompatible settings.
    #[error("Constraint violation: {constraint}. {details}")]
    ConstraintViolation {
        constraint: String,
        details: String,
    },

    /// XSD schema validation failed
    /// 
    /// This error occurs when the constructed document doesn't conform to
    /// the OpenSCENARIO XSD schema requirements.
    #[error("XSD validation failed: {details}")]
    XsdValidationFailed { details: String },

    /// Builder is in wrong state for the requested operation
    /// 
    /// This error should rarely occur due to type system protection, but
    /// provides helpful context when it does happen.
    #[error("Builder state error: {message}. Current state: {current_state}")]
    StateError {
        message: String,
        current_state: String,
    },

    /// Reference to a catalog entry that doesn't exist
    /// 
    /// This error occurs when referencing catalog entries that can't be
    /// found in the specified catalog locations.
    #[error("Catalog reference error: {catalog}.{entry}. {details}")]
    CatalogReferenceError {
        catalog: String,
        entry: String,
        details: String,
    },

    /// Invalid position specification
    /// 
    /// This error occurs when position data is invalid or incomplete,
    /// such as missing required coordinates or invalid reference contexts.
    #[error("Invalid position: {reason}. {suggestion}")]
    InvalidPosition {
        reason: String,
        suggestion: String,
    },

    /// Circular dependency detected
    /// 
    /// This error occurs when dependencies form a cycle, such as entities
    /// referencing each other in positions or catalog circular references.
    #[error("Circular dependency detected: {cycle_description}")]
    CircularDependency { cycle_description: String },

    /// Value out of valid range
    /// 
    /// This error occurs when numeric values are outside allowed ranges
    /// specified by the OpenSCENARIO standard.
    #[error("Value out of range: {value} not in [{min}, {max}] for {field}")]
    ValueOutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    /// Integration error with existing validation system
    /// 
    /// This error wraps errors from the existing validation framework
    /// to provide consistent error handling across the library.
    #[error("Validation error: {0}")]
    ValidationError(#[from] crate::error::Error),

    /// Serialization/deserialization errors
    /// 
    /// This error occurs when converting between builder representations
    /// and final OpenSCENARIO structures fails.
    #[error("Serialization error: {details}")]
    SerializationError { details: String },
}

/// Result type for builder operations
/// 
/// This type alias provides a convenient way to return either a successful
/// result or a builder-specific error with rich context.
pub type BuilderResult<T> = Result<T, BuilderError>;

impl BuilderError {
    /// Create a missing field error with helpful suggestion
    /// 
    /// This helper method creates user-friendly error messages for missing
    /// required fields, including suggestions for how to fix the issue.
    pub fn missing_field(field: &str, suggestion: &str) -> Self {
        Self::MissingRequiredField {
            field: field.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create an entity reference error with available entities list
    /// 
    /// This helper method creates informative error messages when entity
    /// references are invalid, showing what entities are available.
    pub fn invalid_entity(entity_ref: &str, available_entities: &[String]) -> Self {
        Self::InvalidEntityReference {
            entity_ref: entity_ref.to_string(),
            available: available_entities.join(", "),
        }
    }

    /// Create a parameter reference error with available parameters
    /// 
    /// This helper method creates informative error messages when parameter
    /// references are invalid, showing what parameters are available.
    pub fn invalid_parameter(parameter: &str, available_parameters: &[String]) -> Self {
        Self::InvalidParameterReference {
            parameter: parameter.to_string(),
            available: available_parameters.join(", "),
        }
    }

    /// Create a constraint violation error with details
    /// 
    /// This helper method creates detailed error messages for constraint
    /// violations, explaining what went wrong and how to fix it.
    pub fn constraint_violation(constraint: &str, details: &str) -> Self {
        Self::ConstraintViolation {
            constraint: constraint.to_string(),
            details: details.to_string(),
        }
    }

    /// Create a catalog reference error with context
    /// 
    /// This helper method creates informative error messages when catalog
    /// references fail, providing context about what went wrong.
    pub fn catalog_reference(catalog: &str, entry: &str, details: &str) -> Self {
        Self::CatalogReferenceError {
            catalog: catalog.to_string(),
            entry: entry.to_string(),
            details: details.to_string(),
        }
    }

    /// Create an invalid position error with suggestion
    /// 
    /// This helper method creates helpful error messages for position
    /// errors, explaining what's wrong and how to fix it.
    pub fn invalid_position(reason: &str, suggestion: &str) -> Self {
        Self::InvalidPosition {
            reason: reason.to_string(),
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a value out of range error
    /// 
    /// This helper method creates clear error messages for range violations,
    /// showing the invalid value and valid range.
    pub fn value_out_of_range(field: &str, value: &str, min: &str, max: &str) -> Self {
        Self::ValueOutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        }
    }
}

/// Extension trait for converting Results to BuilderResult
/// 
/// This trait provides convenient methods for converting standard errors
/// and validation errors into builder-specific errors with context.
pub trait IntoBuilderResult<T> {
    fn into_builder_result(self) -> BuilderResult<T>;
}

impl<T> IntoBuilderResult<T> for Result<T, crate::error::Error> {
    fn into_builder_result(self) -> BuilderResult<T> {
        self.map_err(BuilderError::ValidationError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_construction() {
        let error = BuilderError::missing_field("entities", "Call .with_entities() first");
        assert!(error.to_string().contains("entities"));
        assert!(error.to_string().contains("with_entities"));
    }

    #[test]
    fn test_entity_reference_error() {
        let error = BuilderError::invalid_entity("nonexistent", &["ego".to_string(), "target".to_string()]);
        assert!(error.to_string().contains("nonexistent"));
        assert!(error.to_string().contains("ego, target"));
    }

    #[test]
    fn test_constraint_violation() {
        let error = BuilderError::constraint_violation("speed range", "Speed must be positive");
        assert!(error.to_string().contains("speed range"));
        assert!(error.to_string().contains("positive"));
    }

    #[test]
    fn test_catalog_reference_error() {
        let error = BuilderError::catalog_reference("VehicleCatalog", "sedan", "Entry not found");
        assert!(error.to_string().contains("VehicleCatalog"));
        assert!(error.to_string().contains("sedan"));
        assert!(error.to_string().contains("Entry not found"));
    }

    #[test]
    fn test_value_out_of_range() {
        let error = BuilderError::value_out_of_range("speed", "150", "0", "100");
        assert!(error.to_string().contains("speed"));
        assert!(error.to_string().contains("150"));
        assert!(error.to_string().contains("[0, 100]"));
    }
}
//! Builder state system for compile-time safety
//!
//! This module implements a type-state pattern that ensures builder methods
//! can only be called in valid states, preventing invalid scenario construction
//! at compile time. The state system tracks the progression from empty builder
//! to complete scenario document.

use std::marker::PhantomData;

/// Base trait for all builder states
///
/// This trait is implemented by all state types to enable generic
/// programming over builder states while maintaining type safety.
pub trait BuilderState {}

/// Trait marking states that have a valid file header
///
/// States implementing this trait have completed the header setup
/// and can proceed to configure catalog locations and other elements.
pub trait AfterHeader: BuilderState {}

/// Trait marking states that can add entities
///
/// States implementing this trait have the necessary infrastructure
/// (header, catalogs, road network) to support entity addition.
pub trait CanAddEntities: BuilderState {}

/// Trait marking states that have valid entities
///
/// States implementing this trait have at least one entity defined
/// and can proceed to build the complete scenario.
pub trait HasValidEntities: BuilderState {}

/// Trait marking states that can build a complete document
///
/// States implementing this trait have all required elements
/// and can produce a valid OpenSCENARIO document.
pub trait CanBuild: BuilderState {}

/// Trait marking document types
///
/// This trait distinguishes between different OpenSCENARIO document
/// types (scenario, catalog, parameter variation) for type safety.
pub trait DocumentType {}

// State type definitions

/// Initial empty state - no configuration has been done
#[derive(Debug, Clone, Copy)]
pub struct Empty;

/// State after file header has been configured
#[derive(Debug, Clone, Copy)]
pub struct HasHeader;

/// State after catalog locations have been configured
#[derive(Debug, Clone, Copy)]
pub struct HasCatalogLocations;

/// State after road network has been configured
#[derive(Debug, Clone, Copy)]
pub struct HasRoadNetwork;

/// State after entities section has been initialized
#[derive(Debug, Clone, Copy)]
pub struct HasEntities;

/// State representing a complete, buildable scenario
#[derive(Debug, Clone, Copy)]
pub struct Complete;

// Document type markers

/// Marker for scenario documents
#[derive(Debug, Clone, Copy)]
pub struct ScenarioDocument;

/// Marker for catalog documents
#[derive(Debug, Clone, Copy)]
pub struct CatalogDocument;

/// Marker for parameter variation documents
#[derive(Debug, Clone, Copy)]
pub struct ParameterVariationDocument;

// Implement BuilderState for all state types
impl BuilderState for Empty {}
impl BuilderState for HasHeader {}
impl BuilderState for HasCatalogLocations {}
impl BuilderState for HasRoadNetwork {}
impl BuilderState for HasEntities {}
impl BuilderState for Complete {}

// Implement capability traits for appropriate states
impl AfterHeader for HasHeader {}
impl AfterHeader for HasCatalogLocations {}
impl AfterHeader for HasRoadNetwork {}
impl AfterHeader for HasEntities {}
impl AfterHeader for Complete {}

impl CanAddEntities for HasRoadNetwork {}
impl CanAddEntities for HasEntities {}
impl CanAddEntities for Complete {}

impl HasValidEntities for HasEntities {}
impl HasValidEntities for Complete {}

impl CanBuild for HasEntities {}
impl CanBuild for Complete {}

// Implement DocumentType for document markers
impl DocumentType for ScenarioDocument {}
impl DocumentType for CatalogDocument {}
impl DocumentType for ParameterVariationDocument {}

/// Typed builder container with phantom types for state and document type
///
/// This container uses phantom types to track the current builder state
/// and document type at compile time, enabling type-safe method calls.
#[derive(Debug)]
pub struct TypedBuilder<S: BuilderState, D: DocumentType> {
    _state: PhantomData<S>,
    _document_type: PhantomData<D>,
}

impl<S: BuilderState, D: DocumentType> TypedBuilder<S, D> {
    /// Create a new typed builder in the given state
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            _document_type: PhantomData,
        }
    }

    /// Transition to a new state
    pub fn transition<NewS: BuilderState>(self) -> TypedBuilder<NewS, D> {
        TypedBuilder::new()
    }

    /// Convert to a different document type
    pub fn as_document_type<NewD: DocumentType>(self) -> TypedBuilder<S, NewD> {
        TypedBuilder::new()
    }
}

impl<S: BuilderState, D: DocumentType> Clone for TypedBuilder<S, D> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<S: BuilderState, D: DocumentType> Copy for TypedBuilder<S, D> {}

impl<S: BuilderState, D: DocumentType> Default for TypedBuilder<S, D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        // Test that we can create builders in different states
        let _empty: TypedBuilder<Empty, ScenarioDocument> = TypedBuilder::new();
        let _header: TypedBuilder<HasHeader, ScenarioDocument> = TypedBuilder::new();
        let _complete: TypedBuilder<Complete, ScenarioDocument> = TypedBuilder::new();
    }

    #[test]
    fn test_state_transition_methods() {
        let empty: TypedBuilder<Empty, ScenarioDocument> = TypedBuilder::new();
        let _header: TypedBuilder<HasHeader, ScenarioDocument> = empty.transition();
    }

    #[test]
    fn test_document_type_conversion() {
        let scenario: TypedBuilder<Empty, ScenarioDocument> = TypedBuilder::new();
        let _catalog: TypedBuilder<Empty, CatalogDocument> = scenario.as_document_type();
    }

    #[test]
    fn test_trait_implementations() {
        // Test that states implement the correct traits
        fn _test_after_header<T: AfterHeader>() {}
        fn _test_can_add_entities<T: CanAddEntities>() {}
        fn _test_has_valid_entities<T: HasValidEntities>() {}
        fn _test_can_build<T: CanBuild>() {}

        // These should compile without errors
        _test_after_header::<HasHeader>();
        _test_can_add_entities::<HasRoadNetwork>();
        _test_has_valid_entities::<HasEntities>();
        _test_can_build::<Complete>();
    }

    #[test]
    fn test_zero_runtime_overhead() {
        // Verify that phantom types have zero size
        assert_eq!(std::mem::size_of::<TypedBuilder<Empty, ScenarioDocument>>(), 0);
        assert_eq!(std::mem::size_of::<TypedBuilder<Complete, CatalogDocument>>(), 0);
    }
}
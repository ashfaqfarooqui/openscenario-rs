//! Builder state system for compile-time safety
//!
//! This module defines the type state system that ensures builders can only
//! transition through valid states, preventing invalid scenarios from being
//! constructed at compile time.

use std::marker::PhantomData;

/// Base trait for all builder states
/// 
/// This trait serves as a marker for the type state system. All builder states
/// must implement this trait to be used in the type state machine.
pub trait BuilderState {}

/// Trait marking states that have a valid file header
/// 
/// Methods requiring document metadata (like adding entities or setting
/// catalog locations) require this state to ensure the header is set first.
pub trait AfterHeader: BuilderState {}

/// Trait marking states that can accept entity additions
/// 
/// Only scenarios with proper foundation (header, catalogs, road network)
/// can accept entity additions. This prevents adding entities without context.
pub trait CanAddEntities: BuilderState {}

/// Trait marking states with valid entity registry
/// 
/// Actions and conditions that reference entities require this state to
/// ensure the entities exist and can be validated.
pub trait HasValidEntities: BuilderState {}

/// Trait marking states that can be built into final documents
/// 
/// Only completed builders that have satisfied all requirements can be
/// built into final OpenSCENARIO documents.
pub trait CanBuild: BuilderState {}

/// Trait marking document type capability
/// 
/// Different document types (Scenario, Catalog, ParameterVariation) have
/// different requirements and capabilities.
pub trait DocumentType: BuilderState {}

// Core builder states representing the progression of scenario construction

/// Initial empty state - no data set yet
/// 
/// Builders start in this state and can only set basic document properties
/// like the file header. This ensures document metadata is set first.
pub struct Empty;

/// State after file header is set
/// 
/// From this state, builders can set catalog locations, road networks,
/// and other document-level properties that depend on having metadata.
pub struct HasHeader;

/// State after catalog locations are configured
/// 
/// Catalog locations define where to find reusable components. This must
/// be set before entities that might reference catalog items.
pub struct HasCatalogLocations;

/// State after road network is defined
/// 
/// Road network provides the spatial context for the scenario. Must be
/// set before entities that will be positioned on roads.
pub struct HasRoadNetwork;

/// State with entities defined
/// 
/// Entities are the core participants in scenarios. Once defined, they
/// can be referenced in stories, actions, and conditions.
pub struct HasEntities;

/// State with complete storyboard
/// 
/// The storyboard defines the dynamic behavior of the scenario. This is
/// the final required element for a complete scenario.
pub struct Complete;

// Document type markers

/// Marker for scenario documents
/// 
/// Scenario documents contain entities, storyboards, and define concrete
/// simulation scenarios with dynamic behavior.
pub struct ScenarioDocument;

/// Marker for catalog documents
/// 
/// Catalog documents contain reusable component definitions that can be
/// referenced by scenario documents.
pub struct CatalogDocument;

/// Marker for parameter variation documents
/// 
/// Parameter variation documents define parameter distributions for
/// generating multiple scenario variants from templates.
pub struct ParameterVariationDocument;

// State trait implementations

impl BuilderState for Empty {}
impl BuilderState for HasHeader {}
impl AfterHeader for HasHeader {}
impl BuilderState for HasCatalogLocations {}
impl AfterHeader for HasCatalogLocations {}
impl BuilderState for HasRoadNetwork {}
impl AfterHeader for HasRoadNetwork {}
impl CanAddEntities for HasRoadNetwork {}
impl BuilderState for HasEntities {}
impl AfterHeader for HasEntities {}
impl HasValidEntities for HasEntities {}
impl CanBuild for HasEntities {}
impl BuilderState for Complete {}
impl AfterHeader for Complete {}
impl HasValidEntities for Complete {}
impl CanBuild for Complete {}

// Document type trait implementations

impl BuilderState for ScenarioDocument {}
impl DocumentType for ScenarioDocument {}
impl BuilderState for CatalogDocument {}
impl DocumentType for CatalogDocument {}
impl BuilderState for ParameterVariationDocument {}
impl DocumentType for ParameterVariationDocument {}

/// Type-safe builder container
/// 
/// This struct uses phantom types to track the current state and document type
/// at compile time, ensuring only valid method calls are possible.
pub struct TypedBuilder<S: BuilderState, D: DocumentType = ScenarioDocument> {
    /// Phantom data for state tracking
    pub _state: PhantomData<S>,
    /// Phantom data for document type tracking  
    pub _doc_type: PhantomData<D>,
}

impl<S: BuilderState, D: DocumentType> TypedBuilder<S, D> {
    /// Create a new typed builder in the given state
    /// 
    /// This is used internally by builder implementations to transition
    /// between states while maintaining type safety.
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            _doc_type: PhantomData,
        }
    }
    
    /// Transition to a new state
    /// 
    /// This method allows builders to transition between states while
    /// maintaining the document type. Used for state transitions.
    pub fn transition<NewS: BuilderState>(self) -> TypedBuilder<NewS, D> {
        TypedBuilder::new()
    }
    
    /// Change document type
    /// 
    /// This method allows changing the document type while maintaining
    /// the current state. Used when document type is determined later.
    pub fn as_document_type<NewD: DocumentType>(self) -> TypedBuilder<S, NewD> {
        TypedBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_traits() {
        // Test that states implement expected traits
        fn assert_builder_state<T: BuilderState>() {}
        fn assert_after_header<T: AfterHeader>() {}
        fn assert_can_add_entities<T: CanAddEntities>() {}
        fn assert_has_valid_entities<T: HasValidEntities>() {}
        fn assert_can_build<T: CanBuild>() {}

        assert_builder_state::<Empty>();
        assert_builder_state::<HasHeader>();
        assert_after_header::<HasHeader>();
        assert_builder_state::<HasRoadNetwork>();
        assert_can_add_entities::<HasRoadNetwork>();
        assert_builder_state::<HasEntities>();
        assert_has_valid_entities::<HasEntities>();
        assert_can_build::<HasEntities>();
    }

    #[test]
    fn test_document_types() {
        fn assert_document_type<T: DocumentType>() {}

        assert_document_type::<ScenarioDocument>();
        assert_document_type::<CatalogDocument>();
        assert_document_type::<ParameterVariationDocument>();
    }

    #[test]
    fn test_typed_builder() {
        let builder: TypedBuilder<Empty, ScenarioDocument> = TypedBuilder::new();
        let _transitioned: TypedBuilder<HasHeader, ScenarioDocument> = builder.transition();
        
        let builder2: TypedBuilder<HasHeader, ScenarioDocument> = TypedBuilder::new();
        let _changed: TypedBuilder<HasHeader, CatalogDocument> = builder2.as_document_type();
    }
}
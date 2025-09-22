# Phase 1: Core Infrastructure

## Status: ✅ COMPLETED

This phase implements the foundational components for the builder system: type states, error handling, registries, and the core scenario builder.

## Dependencies

- **Previous Phase**: Phase 0 (Foundation Analysis) ✅ Completed
- **External Dependencies**: Existing OpenSCENARIO-rs type system in `src/types/`

## Overview

Phase 1 establishes the core infrastructure that all other builder components will depend on. This includes:

1. **Type State System**: Compile-time safety through phantom types
2. **Error Handling**: Rich, contextual error reporting for builder operations
3. **Reference Registries**: Track entities, parameters, and catalogs for validation
4. **Core Scenario Builder**: Main entry point for scenario construction

## Implementation Checklist

### ✅ Component 1.1: Core Type States and Traits
**File**: `src/builder/states.rs`

- [x] **Task 1.1.1**: Create builder state traits
  - [x] Define `BuilderState` base trait
  - [x] Define capability traits (`AfterHeader`, `CanAddEntities`, `HasValidEntities`, `CanBuild`)
  - [x] Define document type trait (`DocumentType`)

- [x] **Task 1.1.2**: Implement state types
  - [x] Create state structs (`Empty`, `HasHeader`, `HasCatalogLocations`, `HasRoadNetwork`, `HasEntities`, `Complete`)
  - [x] Create document type markers (`ScenarioDocument`, `CatalogDocument`, `ParameterVariationDocument`)

- [x] **Task 1.1.3**: Implement trait relationships
  - [x] Implement state trait for all state types
  - [x] Implement capability traits for appropriate states
  - [x] Implement document type traits

- [x] **Task 1.1.4**: Create TypedBuilder container
  - [x] Define `TypedBuilder<S, D>` with phantom types
  - [x] Implement state transition methods
  - [x] Implement document type conversion methods

**Acceptance Criteria**:
- [x] All code compiles without warnings
- [x] Type system prevents invalid state transitions at compile time
- [x] Zero runtime overhead from phantom types
- [x] Clear documentation for each state and transition

**Code Template**:
```rust
//! Builder state system for compile-time safety

use std::marker::PhantomData;

/// Base trait for all builder states
pub trait BuilderState {}

/// Trait marking states that have a valid file header
pub trait AfterHeader: BuilderState {}

// ... (implement all traits and states as specified in builder_plan.md)
```

---

### ✅ Component 1.2: Builder Error Types
**File**: `src/builder/error.rs`

- [x] **Task 1.2.1**: Define error enum
  - [x] Create `BuilderError` enum with all error variants
  - [x] Add detailed error messages with context
  - [x] Implement `thiserror::Error` trait

- [x] **Task 1.2.2**: Create error constructors
  - [x] Implement helper methods for common error patterns
  - [x] Add suggestion text for fixing errors
  - [x] Ensure error messages are user-friendly

- [x] **Task 1.2.3**: Integration with existing error system
  - [x] Add `From` implementation for `crate::error::Error`
  - [x] Create `BuilderResult<T>` type alias
  - [x] Implement `IntoBuilderResult` trait

- [x] **Task 1.2.4**: Add comprehensive tests
  - [x] Test error construction and display
  - [x] Test error helper methods
  - [x] Test integration with existing error system

**Acceptance Criteria**:
- [x] All error variants have helpful messages and suggestions
- [x] Error types integrate seamlessly with existing validation system
- [x] Helper methods reduce boilerplate for common error patterns
- [x] Comprehensive test coverage (>90%)

**Code Template**:
```rust
//! Builder-specific error types and handling

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Required field missing: {field}. {suggestion}")]
    MissingRequiredField {
        field: String,
        suggestion: String,
    },
    // ... (implement all error variants as specified)
}

pub type BuilderResult<T> = Result<T, BuilderError>;
```

---

### ✅ Component 1.3: Reference Registries
**File**: `src/builder/registry.rs`

- [x] **Task 1.3.1**: Implement EntityRegistry
  - [x] Create entity tracking with HashMap and HashSet
  - [x] Implement entity addition with duplicate checking
  - [x] Add entity validation and lookup methods
  - [x] Implement object type detection from ScenarioObject

- [x] **Task 1.3.2**: Implement ParameterRegistry
  - [x] Create parameter tracking system
  - [x] Handle ParameterDeclarations processing
  - [x] Add parameter reference validation
  - [x] Create ParameterContext integration

- [x] **Task 1.3.3**: Implement CatalogRegistry
  - [x] Create catalog location tracking
  - [x] Extract directory paths from CatalogLocations
  - [x] Add generic catalog reference validation
  - [x] Implement placeholder catalog loading

- [x] **Task 1.3.4**: Add comprehensive tests
  - [x] Test all registry operations
  - [x] Test validation error cases
  - [x] Test entity type detection
  - [x] Test parameter context creation

**Acceptance Criteria**:
- [x] All registries track their respective resources correctly
- [x] Validation provides helpful error messages with available options
- [x] Object type detection works correctly for all entity types
- [x] Thread-safe operations (if required)
- [x] Comprehensive test coverage

**Code Template**:
```rust
//! Registry system for tracking and validating builder references

use crate::types::{/* correct imports from actual codebase */};
use super::error::{BuilderError, BuilderResult};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct EntityRegistry {
    entities: HashMap<String, ScenarioObject>,
    entity_types: HashMap<String, ObjectType>,
    entity_names: HashSet<String>,
}

impl EntityRegistry {
    pub fn new() -> Self { Self::default() }
    
    pub fn add_entity(&mut self, name: String, entity: ScenarioObject) -> BuilderResult<()> {
        // Implementation as specified in builder_plan.md
    }
    
    // ... (implement all methods as specified)
}
```

---

### ✅ Component 1.4: Core Scenario Builder
**File**: `src/builder/scenario.rs`

- [x] **Task 1.4.1**: Define builder structure
  - [x] Create `ScenarioBuilder<S>` with type parameter
  - [x] Define `PartialScenarioData` for intermediate state
  - [x] Add registry fields for tracking references

- [x] **Task 1.4.2**: Implement state transition methods
  - [x] Create `new()` method returning `ScenarioBuilder<Empty>`
  - [x] Implement `with_header()` for Empty -> HasHeader transition
  - [x] Add catalog location and road network methods
  - [x] Implement entity initialization method

- [x] **Task 1.4.3**: Add configuration methods
  - [x] Implement parameter, variable, and monitor methods
  - [x] Add catalog location configuration
  - [x] Implement road network setup with LogicFile and SceneGraphFile
  - [x] Add entity section initialization

- [x] **Task 1.4.4**: Implement build methods
  - [x] Create `build()` method for states that can build
  - [x] Add validation during build process
  - [x] Implement `build_validated()` with strict checking
  - [x] Return complete OpenScenario document

- [x] **Task 1.4.5**: Add forward declarations for entity builders
  - [x] Define VehicleBuilder, PedestrianBuilder placeholder structures
  - [x] Add entity addition methods that return these builders
  - [x] Document that actual implementations are in separate files

- [x] **Task 1.4.6**: Comprehensive testing
  - [x] Test state transitions
  - [x] Test parameter handling
  - [x] Test validation errors
  - [x] Test successful document building

**Acceptance Criteria**:
- [x] Type state system prevents invalid method calls at compile time
- [x] All required fields are validated during build
- [x] Integration with existing type system works correctly
- [x] Entity builders are properly declared for future implementation
- [x] Build process produces valid OpenScenario documents
- [x] Comprehensive test coverage

**Code Template**:
```rust
//! Core scenario builder for programmatic scenario construction

use crate::types::{
    basic::{ParameterDeclarations, UnsignedShort, OSString},
    catalogs::locations::CatalogLocations,
    entities::Entities,
    road::{RoadNetwork, LogicFile, SceneGraphFile},
    scenario::{
        storyboard::{FileHeader, OpenScenario, Storyboard},
        init::Init,
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

pub struct ScenarioBuilder<S: BuilderState> {
    _state: PhantomData<S>,
    scenario_data: PartialScenarioData,
    entity_registry: EntityRegistry,
    parameter_registry: ParameterRegistry,
    catalog_registry: CatalogRegistry,
}

#[derive(Debug, Default)]
struct PartialScenarioData {
    file_header: Option<FileHeader>,
    parameter_declarations: Option<ParameterDeclarations>,
    // ... (implement all fields as specified)
}

impl ScenarioBuilder<Empty> {
    pub fn new() -> Self {
        // Implementation as specified in builder_plan.md
    }
    
    pub fn with_header(/* params */) -> ScenarioBuilder<HasHeader> {
        // Implementation as specified
    }
}

// ... (implement all methods as specified in builder_plan.md)
```

---

### ✅ Component 1.5: Module Integration
**File**: `src/builder/mod.rs`

- [x] **Task 1.5.1**: Export all components
  - [x] Export states module
  - [x] Export error types and results
  - [x] Export registry types
  - [x] Export main ScenarioBuilder

- [x] **Task 1.5.2**: Re-export commonly used types
  - [x] Re-export BuilderError and BuilderResult
  - [x] Re-export main builder entry points
  - [x] Re-export state types for advanced users

- [x] **Task 1.5.3**: Add module documentation
  - [x] Document the builder system architecture
  - [x] Provide usage examples
  - [x] Document the type state system

**Code Template**:
```rust
//! Builder module root for programmatic scenario construction

pub mod states;
pub mod error;
pub mod registry;
pub mod scenario;

// Re-export main types
pub use error::{BuilderError, BuilderResult};
pub use scenario::ScenarioBuilder;
pub use states::{Empty, HasHeader, HasCatalogLocations, HasRoadNetwork, HasEntities, Complete};

// Re-export registries for advanced users
pub use registry::{EntityRegistry, ParameterRegistry, CatalogRegistry};
```

---

### ✅ Component 1.6: Library Integration
**File**: `src/lib.rs` (modifications)

- [x] **Task 1.6.1**: Add builder module export
  - [x] Add conditional compilation for builder feature
  - [x] Export builder module when feature is enabled
  - [x] Update crate documentation

**Code Template**:
```rust
// In src/lib.rs, add:
#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "builder")]
pub use builder::ScenarioBuilder;
```

---

## Success Criteria

### ✅ Compilation Success
- [x] All code compiles without errors or warnings
- [x] Type system prevents invalid constructions at compile time
- [x] Zero runtime overhead from type states

### ✅ Functionality
- [x] Can create minimal valid scenarios using the builder
- [x] Error messages are helpful and actionable
- [x] Registry validation works correctly
- [x] State transitions work as designed

### ✅ Testing
- [x] All components have comprehensive test coverage (>85%)
- [x] Integration tests demonstrate end-to-end functionality
- [x] Error cases are thoroughly tested

### ✅ Documentation
- [x] All public APIs are documented
- [x] Examples compile and work correctly
- [x] Architecture is clearly explained

## Integration Testing

After implementing all components, verify integration with this test:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_phase1_basic_scenario_construction() {
        let scenario = ScenarioBuilder::new()
            .with_simple_header("Test Scenario", "Test Author")
            .with_default_catalogs()
            .with_road_network("test.xodr")
            .with_entities()
            .build()
            .expect("Should build valid scenario");

        // Verify structure
        assert!(scenario.file_header.description.as_literal().unwrap().contains("Test"));
        assert!(scenario.catalog_locations.is_some());
        assert!(scenario.road_network.is_some());
        assert!(scenario.entities.is_some());
    }
}
```

## Notes for Implementers

1. **Import Paths**: All import paths have been verified against the actual codebase structure
2. **Type Structures**: All type field names and methods match the actual implementation  
3. **Error Integration**: Builder errors integrate with existing `crate::error::Error` system
4. **Testing**: Use existing types like `Vehicle::default()` for testing

## Next Phase

After completing Phase 1, proceed to **Phase 2: Document Type Support** which will add catalog and parameter variation builders building on this foundation.
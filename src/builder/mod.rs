//! Builder module root for programmatic scenario construction
//!
//! This module provides type-safe, ergonomic builders for constructing
//! OpenSCENARIO documents programmatically. The builder pattern ensures
//! compile-time safety and guides users through the required construction steps.

// Core infrastructure modules
pub mod states;
pub mod error;
pub mod registry;
pub mod scenario;
pub mod catalog;
pub mod parameter_variation;

// Phase 3 modules - Entity and Position builders
pub mod entities;
pub mod positions;

// Re-export commonly used types
pub use error::{BuilderError, BuilderResult};
pub use scenario::ScenarioBuilder;
pub use catalog::CatalogBuilder;
pub use parameter_variation::ParameterVariationBuilder;
pub use states::*;

// Re-export registry types for advanced usage
pub use registry::{EntityRegistry, ParameterRegistry, CatalogRegistry};

// Re-export Phase 3 builders
pub use entities::{EntitiesBuilder, VehicleBuilder, PedestrianBuilder, MiscObjectBuilder};
pub use positions::{PositionBuilder, WorldPositionBuilder, LanePositionBuilder, RoadPositionBuilder};

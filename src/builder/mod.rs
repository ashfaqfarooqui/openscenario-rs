//! Builder module root for programmatic OpenSCENARIO document construction
//!
//! This module provides a comprehensive builder system for creating all types of
//! OpenSCENARIO documents programmatically with compile-time safety and validation.
//!
//! # Architecture
//!
//! The builder system uses a type-state pattern to ensure that documents can only
//! be constructed in valid sequences. The main components are:
//!
//! - **States**: Type-safe state transitions preventing invalid construction
//! - **Errors**: Rich error handling with helpful suggestions
//! - **Registries**: Track entities, parameters, and catalogs for validation
//! - **Document Builders**: Specialized builders for each document type
//!
//! # Document Types
//!
//! The builder system supports all three OpenSCENARIO document types:
//!
//! ## Scenario Documents
//! 
//! Create concrete scenario files with entities and storyboards:
//!
//! ```rust
//! use openscenario_rs::builder::ScenarioBuilder;
//!
//! let scenario = ScenarioBuilder::new()
//!     .with_simple_header("My Scenario", "Author")
//!     .with_default_catalogs()?
//!     .with_road_network("road.xodr")
//!     .with_entities()
//!     .build()?;
//! ```
//!
//! ## Catalog Documents
//!
//! Create reusable entity catalogs:
//!
//! ```rust
//! use openscenario_rs::builder::CatalogBuilder;
//!
//! let catalog = CatalogBuilder::new("VehicleCatalog")
//!     .with_simple_header("Vehicle Library", "Fleet Manager")
//!     .add_vehicle("sedan")
//!         .with_model("GenericSedan")
//!         .with_dimensions(4.5, 1.8, 1.4)
//!         .with_performance(200.0, 8.0, 10.0)
//!         .finish_vehicle()
//!     .build()?;
//! ```
//!
//! ## Parameter Variation Documents
//!
//! Create parameter distributions for scenario variants:
//!
//! ```rust
//! use openscenario_rs::builder::ParameterVariationBuilder;
//!
//! let param_var = ParameterVariationBuilder::new()
//!     .with_simple_header("Speed Variations", "Test Engineer")
//!     .add_deterministic_distribution("initial_speed")
//!         .with_range(20.0, 60.0, 5.0)
//!         .finish()
//!     .build()?;
//! ```
//!
//! # Type Safety
//!
//! The builder prevents invalid operations at compile time:
//!
//! ```rust,compile_fail
//! // This won't compile - can't build without required elements
//! let scenario = ScenarioBuilder::new().build();
//! ```
//!
//! # State Transitions
//!
//! All builders progress through these states:
//! 1. `Empty` - Initial state
//! 2. `HasHeader` - After setting file header (can build for some document types)
//! 3. Additional states for scenario builders:
//!    - `HasCatalogLocations` - After setting catalog locations
//!    - `HasRoadNetwork` - After setting road network
//!    - `HasEntities` - After initializing entities (can build)
//!    - `Complete` - Fully configured scenario

pub mod states;
pub mod error;
pub mod registry;
pub mod scenario;
pub mod catalog;
pub mod parameter_variation;
pub mod entities;
pub mod positions;
pub mod actions;
pub mod conditions;
pub mod triggers;
pub mod events;
pub mod storyboard;


// Re-export main types for convenience
pub use error::{BuilderError, BuilderResult};
pub use scenario::ScenarioBuilder;
pub use catalog::CatalogBuilder;
pub use parameter_variation::ParameterVariationBuilder;

// Re-export state types for advanced users
pub use states::{
    Empty, HasHeader, HasCatalogLocations, HasRoadNetwork, HasEntities, Complete,
    ScenarioDocument, CatalogDocument, ParameterVariationDocument,
    BuilderState, AfterHeader, CanAddEntities, HasValidEntities, CanBuild, DocumentType,
};

// Re-export registries for advanced users
pub use registry::{EntityRegistry, ParameterRegistry, CatalogRegistry};

// Re-export entity builders
pub use entities::{VehicleBuilder, PedestrianBuilder, MiscObjectBuilder, EntityBuilder, EntitiesBuilder};

// Re-export position builders
pub use positions::{
    WorldPositionBuilder, RelativePositionBuilder, RoadPositionBuilder, LanePositionBuilder,
    PositionBuilder, UnifiedPositionBuilder,
};

// Re-export catalog entity builders
pub use catalog::{
    CatalogVehicleBuilder, CatalogControllerBuilder, CatalogPedestrianBuilder,
    CatalogEnvironmentBuilder, CatalogManeuverBuilder, CatalogTrajectoryBuilder,
    CatalogRouteBuilder,
};

// Re-export distribution builders
pub use parameter_variation::{
    DeterministicDistributionBuilder, StochasticDistributionBuilder,
};

// Re-export action builders
pub use actions::{
    TeleportActionBuilder, LongitudinalActionBuilder, LateralActionBuilder,
    SynchronizeActionBuilder, FollowTrajectoryActionBuilder, SpeedActionBuilder,
    LaneChangeActionBuilder, LaneOffsetActionBuilder, ActivateControllerActionBuilder,
    ControllerActionBuilder, OverrideControllerActionBuilder, AppearanceActionBuilder,
    LightActionBuilder, AnimationActionBuilder, TrafficSignalActionBuilder,
    TrafficSwarmActionBuilder, TrafficSourceActionBuilder, UnifiedActionBuilder,
};

// Re-export condition builders
pub use conditions::{
    EndOfRoadConditionBuilder, CollisionConditionBuilder, OffroadConditionBuilder,
    TimeHeadwayConditionBuilder, TimeToCollisionConditionBuilder, AccelerationConditionBuilder,
    StandStillConditionBuilder, SpeedConditionBuilder, RelativeSpeedConditionBuilder,
    ParameterConditionBuilder, VariableConditionBuilder, SimulationTimeConditionBuilder,
    StoryboardElementConditionBuilder, ReachPositionConditionBuilder, DistanceConditionBuilder,
    RelativeDistanceConditionBuilder, TraveledDistanceConditionBuilder, UnifiedConditionBuilder,
};

// Re-export trigger and event builders
pub use triggers::TriggerBuilder;
pub use events::EventBuilder;

// Re-export storyboard builders
pub use storyboard::{StoryboardBuilder, StoryBuilder, ActBuilder, ManeuverBuilder};

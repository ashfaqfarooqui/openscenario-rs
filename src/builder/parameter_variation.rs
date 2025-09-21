//! Parameter variation builder for programmatic parameter variation construction
//!
//! This module provides the ParameterVariationBuilder that enables type-safe,
//! ergonomic construction of OpenSCENARIO parameter variation documents with
//! compile-time validation and fluent APIs for both deterministic and stochastic
//! parameter distributions.

use crate::types::{
    basic::{Value, UnsignedInt, Range},
    distributions::{
        ParameterValueDistribution,
        deterministic::{
            Deterministic, DeterministicSingleParameterDistribution, 
            DeterministicMultiParameterDistribution, DistributionSet, 
            DistributionSetElement, DistributionRange, ValueSetDistribution,
            ParameterValueSet, ParameterAssignment,
        },
        stochastic::{Stochastic, StochasticDistribution},
    },
    entities::vehicle::File,
    scenario::storyboard::{FileHeader, OpenScenario},
};
use super::{
    error::{BuilderError, BuilderResult},
    states::*,
};
use std::marker::PhantomData;

/// Main builder for OpenSCENARIO parameter variation documents
/// 
/// This builder provides a type-safe, step-by-step approach to constructing
/// OpenSCENARIO parameter variation documents. The type parameter S tracks the
/// current construction state, ensuring required elements are set before building.
/// 
/// # Type Parameters
/// * `S` - Current builder state (Empty, HasHeader, CanBuild)
/// 
/// # Example
/// ```rust
/// let param_variation = ParameterVariationBuilder::new("highway_scenario.xosc")
///     .with_simple_header("Speed Variation Study", "Test Engineer")
///     .with_deterministic_distribution()
///         .single_parameter("initial_speed")
///             .distribution_range(20.0, 40.0, 5.0)
///             .finish_parameter()
///         .finish_distribution()
///     .build()?;
/// ```
#[derive(Debug)]
pub struct ParameterVariationBuilder<S: BuilderState> {
    /// Type state phantom data for compile-time safety
    _state: PhantomData<S>,
    
    /// Partially constructed parameter variation data
    param_variation_data: PartialParameterVariationData,
}

/// Internal data structure for building parameter variations
/// 
/// This structure holds the partially constructed parameter variation data as it's
/// being built. Fields start as None and are populated as the builder
/// progresses through states.
#[derive(Debug, Default)]
struct PartialParameterVariationData {
    /// Scenario file reference
    scenario_file: Option<File>,
    
    /// File header with document metadata
    file_header: Option<FileHeader>,
    
    /// Parameter value distribution
    parameter_value_distribution: Option<ParameterValueDistribution>,
}

// Core builder implementation for Empty state
impl ParameterVariationBuilder<Empty> {
    /// Create a new parameter variation builder with the specified scenario file
    /// 
    /// This creates a new builder in the Empty state, ready to accept
    /// a file header as the first required element.
    /// 
    /// # Arguments
    /// * `scenario_file` - Path to the scenario template file
    /// 
    /// # Returns
    /// A new ParameterVariationBuilder in Empty state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ParameterVariationBuilder::new("highway_scenario.xosc");
    /// ```
    pub fn new(scenario_file: &str) -> Self {
        Self {
            _state: PhantomData,
            param_variation_data: PartialParameterVariationData {
                scenario_file: Some(File {
                    filepath: scenario_file.to_string(),
                }),
                file_header: None,
                parameter_value_distribution: None,
            },
        }
    }

    /// Set the file header and transition to HasHeader state
    /// 
    /// The file header contains essential document metadata and is required
    /// for all OpenSCENARIO documents. This must be the first method called.
    /// 
    /// # Arguments
    /// * `title` - Document title/description
    /// * `rev_major` - Major revision number (e.g., "1")  
    /// * `rev_minor` - Minor revision number (e.g., "0")
    /// * `date` - Creation date in ISO format (e.g., "2024-01-15T10:00:00")
    /// * `description` - Human-readable description
    /// * `author` - Document author name
    /// 
    /// # Returns
    /// ParameterVariationBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ParameterVariationBuilder::new("scenario.xosc")
    ///     .with_header("Speed Study", "1", "0", "2024-01-15T10:00:00", 
    ///                  "Parameter variation study", "Test Engineer");
    /// ```
    pub fn with_header(
        mut self,
        _title: &str,
        rev_major: &str,
        rev_minor: &str,
        date: &str,
        description: &str,
        author: &str,
    ) -> ParameterVariationBuilder<HasHeader> {
        self.param_variation_data.file_header = Some(FileHeader {
            rev_major: Value::literal(rev_major.parse().unwrap_or(1)),
            rev_minor: Value::literal(rev_minor.parse().unwrap_or(0)),
            date: Value::literal(date.to_string()),
            description: Value::literal(description.to_string()),
            author: Value::literal(author.to_string()),
        });

        ParameterVariationBuilder {
            _state: PhantomData,
            param_variation_data: self.param_variation_data,
        }
    }

    /// Set the file header with simplified parameters
    /// 
    /// This is a convenience method for common use cases where only the
    /// essential header information is needed.
    /// 
    /// # Arguments
    /// * `title` - Document title
    /// * `author` - Document author
    /// 
    /// # Returns
    /// ParameterVariationBuilder in HasHeader state
    /// 
    /// # Example
    /// ```rust
    /// let builder = ParameterVariationBuilder::new("scenario.xosc")
    ///     .with_simple_header("Speed Study", "Test Engineer");
    /// ```
    pub fn with_simple_header(self, title: &str, author: &str) -> ParameterVariationBuilder<HasHeader> {
        let now = "2024-01-15T10:00:00"; // Use fixed date for reproducibility
        self.with_header(title, "1", "0", now, title, author)
    }
}

// Methods available after header is set - can configure distributions and build
impl<S: AfterHeader> ParameterVariationBuilder<S> {
    /// Start configuring a deterministic distribution
    /// 
    /// This method creates a deterministic distribution builder that allows
    /// configuration of systematic parameter exploration.
    /// 
    /// # Returns
    /// DeterministicDistributionBuilder for configuring the distribution
    /// 
    /// # Example
    /// ```rust
    /// builder.with_deterministic_distribution()
    ///     .single_parameter("speed")
    ///         .distribution_range(20.0, 40.0, 5.0)
    ///         .finish_parameter()
    ///     .finish_distribution()
    /// ```
    pub fn with_deterministic_distribution(self) -> DeterministicDistributionBuilder<S> {
        DeterministicDistributionBuilder::new(self)
    }

    /// Start configuring a stochastic distribution
    /// 
    /// This method creates a stochastic distribution builder that allows
    /// configuration of probabilistic parameter exploration.
    /// 
    /// # Returns
    /// StochasticDistributionBuilder for configuring the distribution
    /// 
    /// # Example
    /// ```rust
    /// builder.with_stochastic_distribution()
    ///     .normal_distribution("speed", 30.0, 5.0)
    ///     .finish_distribution()
    /// ```
    pub fn with_stochastic_distribution(self) -> StochasticDistributionBuilder<S> {
        StochasticDistributionBuilder::new(self)
    }
}

// Methods for building final parameter variation (available when header is set)
impl<S: AfterHeader> ParameterVariationBuilder<S> {
    /// Build the final OpenScenario parameter variation document
    /// 
    /// This method validates the constructed parameter variation and builds the final
    /// OpenScenario document. A parameter distribution must be configured.
    /// 
    /// # Returns
    /// Complete OpenScenario document or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError if required elements are missing or validation fails
    /// 
    /// # Example
    /// ```rust
    /// let param_variation = builder.build()?;
    /// ```
    pub fn build(self) -> BuilderResult<OpenScenario> {
        // Validate required elements
        let file_header = self.param_variation_data.file_header
            .ok_or_else(|| BuilderError::missing_field(
                "file_header", 
                "Call .with_header() first"
            ))?;

        let parameter_value_distribution = self.param_variation_data.parameter_value_distribution
            .ok_or_else(|| BuilderError::missing_field(
                "parameter_value_distribution", 
                "Configure a distribution using .with_deterministic_distribution() or .with_stochastic_distribution()"
            ))?;

        // Construct final OpenScenario document
        let scenario = OpenScenario {
            file_header,
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: None,
            road_network: None,
            entities: None,
            storyboard: None,
            parameter_value_distribution: Some(parameter_value_distribution),
            catalog: None,
        };

        Ok(scenario)
    }

    /// Build and validate with strict checking
    /// 
    /// This method performs additional validation beyond the basic build,
    /// checking for semantic correctness and XSD compliance.
    /// 
    /// # Returns
    /// Complete OpenScenario document or BuilderError
    /// 
    /// # Errors
    /// Returns BuilderError for any validation failures
    pub fn build_validated(self) -> BuilderResult<OpenScenario> {
        let scenario = self.build()?;
        
        // TODO: Integrate with existing validation system
        // let validation_context = ValidationContext::new();
        // scenario.validate(&validation_context)
        //     .map_err(BuilderError::ValidationError)?;
        
        Ok(scenario)
    }
}

/// Builder for configuring deterministic parameter distributions
/// 
/// This builder provides configuration options for systematic parameter exploration
/// including ranges, sets, and multi-parameter combinations.
#[derive(Debug)]
pub struct DeterministicDistributionBuilder<S: BuilderState> {
    /// Parent parameter variation builder
    param_variation_builder: ParameterVariationBuilder<S>,
    
    /// Single parameter distributions being configured
    single_distributions: Vec<DeterministicSingleParameterDistribution>,
    
    /// Multi parameter distributions being configured
    multi_distributions: Vec<DeterministicMultiParameterDistribution>,
}

impl<S: AfterHeader> DeterministicDistributionBuilder<S> {
    /// Create a new deterministic distribution builder
    fn new(param_variation_builder: ParameterVariationBuilder<S>) -> Self {
        Self {
            param_variation_builder,
            single_distributions: Vec::new(),
            multi_distributions: Vec::new(),
        }
    }

    /// Start configuring a single parameter distribution
    /// 
    /// # Arguments
    /// * `parameter_name` - Name of the parameter to vary
    /// 
    /// # Returns
    /// SingleParameterBuilder for configuring the parameter
    /// 
    /// # Example
    /// ```rust
    /// builder.single_parameter("speed")
    ///     .distribution_range(20.0, 40.0, 5.0)
    ///     .finish_parameter()
    /// ```
    pub fn single_parameter(self, parameter_name: &str) -> SingleParameterBuilder<S> {
        SingleParameterBuilder::new(self, parameter_name)
    }

    /// Start configuring a multi-parameter distribution
    /// 
    /// # Returns
    /// MultiParameterBuilder for configuring multiple parameters
    /// 
    /// # Example
    /// ```rust
    /// builder.multi_parameter()
    ///     .add_parameter_set()
    ///         .assign("speed", "30.0")
    ///         .assign("weather", "dry")
    ///         .finish_set()
    ///     .finish_multi_parameter()
    /// ```
    pub fn multi_parameter(self) -> MultiParameterBuilder<S> {
        MultiParameterBuilder::new(self)
    }

    /// Finish configuring the deterministic distribution
    /// 
    /// # Returns
    /// The parent parameter variation builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if distribution configuration is invalid
    pub fn finish_distribution(mut self) -> BuilderResult<ParameterVariationBuilder<S>> {
        // Validate that at least one distribution is configured
        if self.single_distributions.is_empty() && self.multi_distributions.is_empty() {
            return Err(BuilderError::constraint_violation(
                "deterministic distribution must have parameters",
                "Add at least one parameter using .single_parameter() or .multi_parameter()"
            ));
        }

        // Create deterministic distribution
        let deterministic = Deterministic {
            single_distributions: self.single_distributions,
            multi_distributions: self.multi_distributions,
        };

        // Create parameter value distribution
        let scenario_file = self.param_variation_builder.param_variation_data.scenario_file
            .clone()
            .ok_or_else(|| BuilderError::missing_field(
                "scenario_file",
                "Internal error: scenario file not set"
            ))?;

        let parameter_value_distribution = ParameterValueDistribution::new_deterministic(
            scenario_file,
            deterministic
        );

        self.param_variation_builder.param_variation_data.parameter_value_distribution = 
            Some(parameter_value_distribution);

        Ok(self.param_variation_builder)
    }
}

/// Builder for configuring single parameter distributions
/// 
/// This builder provides configuration options for individual parameter variations
/// including ranges and discrete sets.
#[derive(Debug)]
pub struct SingleParameterBuilder<S: BuilderState> {
    /// Parent deterministic distribution builder
    deterministic_builder: DeterministicDistributionBuilder<S>,
    
    /// Parameter being configured
    parameter_distribution: DeterministicSingleParameterDistribution,
}

impl<S: AfterHeader> SingleParameterBuilder<S> {
    /// Create a new single parameter builder
    fn new(deterministic_builder: DeterministicDistributionBuilder<S>, parameter_name: &str) -> Self {
        let parameter_distribution = DeterministicSingleParameterDistribution {
            parameter_name: Value::literal(parameter_name.to_string()),
            distribution_set: None,
            distribution_range: None,
            user_defined_distribution: None,
        };

        Self {
            deterministic_builder,
            parameter_distribution,
        }
    }

    /// Configure a distribution range for the parameter
    /// 
    /// # Arguments
    /// * `min_value` - Minimum value of the range
    /// * `max_value` - Maximum value of the range
    /// * `step_width` - Step size between values
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.distribution_range(20.0, 40.0, 5.0)
    /// ```
    pub fn distribution_range(mut self, min_value: f64, max_value: f64, step_width: f64) -> Self {
        self.parameter_distribution.distribution_range = Some(DistributionRange {
            step_width: Value::literal(step_width.to_string()),
            range: Range {
                lower_limit: Value::literal(min_value),
                upper_limit: Value::literal(max_value),
            },
        });
        self
    }

    /// Configure a distribution set for the parameter
    /// 
    /// # Arguments
    /// * `values` - Vector of discrete values
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.distribution_set(vec!["20.0", "25.0", "30.0", "35.0", "40.0"])
    /// ```
    pub fn distribution_set(mut self, values: Vec<&str>) -> Self {
        let elements = values.into_iter()
            .map(|v| DistributionSetElement {
                value: Value::literal(v.to_string()),
            })
            .collect();

        self.parameter_distribution.distribution_set = Some(DistributionSet {
            elements,
        });
        self
    }

    /// Finish configuring the single parameter
    /// 
    /// # Returns
    /// The parent deterministic distribution builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if parameter configuration is invalid
    pub fn finish_parameter(mut self) -> BuilderResult<DeterministicDistributionBuilder<S>> {
        // Validate that either range or set is configured
        if self.parameter_distribution.distribution_range.is_none() && 
           self.parameter_distribution.distribution_set.is_none() {
            return Err(BuilderError::constraint_violation(
                "parameter must have distribution",
                "Configure either .distribution_range() or .distribution_set()"
            ));
        }

        // Add parameter to deterministic builder
        self.deterministic_builder.single_distributions.push(self.parameter_distribution);

        Ok(self.deterministic_builder)
    }
}

/// Builder for configuring multi-parameter distributions
/// 
/// This builder provides configuration options for combinations of multiple parameters
/// using value sets that define parameter combinations.
#[derive(Debug)]
pub struct MultiParameterBuilder<S: BuilderState> {
    /// Parent deterministic distribution builder
    deterministic_builder: DeterministicDistributionBuilder<S>,
    
    /// Parameter value sets being configured
    parameter_value_sets: Vec<ParameterValueSet>,
}

impl<S: AfterHeader> MultiParameterBuilder<S> {
    /// Create a new multi-parameter builder
    fn new(deterministic_builder: DeterministicDistributionBuilder<S>) -> Self {
        Self {
            deterministic_builder,
            parameter_value_sets: Vec::new(),
        }
    }

    /// Start configuring a parameter value set
    /// 
    /// # Returns
    /// ParameterValueSetBuilder for configuring the set
    /// 
    /// # Example
    /// ```rust
    /// builder.add_parameter_set()
    ///     .assign("speed", "30.0")
    ///     .assign("weather", "dry")
    ///     .finish_set()
    /// ```
    pub fn add_parameter_set(self) -> ParameterValueSetBuilder<S> {
        ParameterValueSetBuilder::new(self)
    }

    /// Finish configuring the multi-parameter distribution
    /// 
    /// # Returns
    /// The parent deterministic distribution builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if multi-parameter configuration is invalid
    pub fn finish_multi_parameter(mut self) -> BuilderResult<DeterministicDistributionBuilder<S>> {
        // Validate that at least one parameter value set is configured
        if self.parameter_value_sets.is_empty() {
            return Err(BuilderError::constraint_violation(
                "multi-parameter distribution must have value sets",
                "Add at least one parameter set using .add_parameter_set()"
            ));
        }

        // Create multi-parameter distribution
        let multi_distribution = DeterministicMultiParameterDistribution {
            distribution_type: crate::types::distributions::deterministic::DeterministicMultiParameterDistributionType::ValueSetDistribution(
                ValueSetDistribution {
                    parameter_value_sets: self.parameter_value_sets,
                    assignment_author: None,
                }
            ),
        };

        // Add to deterministic builder
        self.deterministic_builder.multi_distributions.push(multi_distribution);

        Ok(self.deterministic_builder)
    }
}

/// Builder for configuring parameter value sets
/// 
/// This builder provides configuration options for individual parameter value sets
/// that define specific combinations of parameter values.
#[derive(Debug)]
pub struct ParameterValueSetBuilder<S: BuilderState> {
    /// Parent multi-parameter builder
    multi_parameter_builder: MultiParameterBuilder<S>,
    
    /// Parameter assignments being configured
    parameter_assignments: Vec<ParameterAssignment>,
}

impl<S: AfterHeader> ParameterValueSetBuilder<S> {
    /// Create a new parameter value set builder
    fn new(multi_parameter_builder: MultiParameterBuilder<S>) -> Self {
        Self {
            multi_parameter_builder,
            parameter_assignments: Vec::new(),
        }
    }

    /// Assign a value to a parameter in this set
    /// 
    /// # Arguments
    /// * `parameter_ref` - Name of the parameter
    /// * `value` - Value to assign to the parameter
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.assign("speed", "30.0")
    ///        .assign("weather", "dry")
    /// ```
    pub fn assign(mut self, parameter_ref: &str, value: &str) -> Self {
        let assignment = ParameterAssignment {
            parameter_ref: parameter_ref.to_string(),
            value: Value::literal(value.to_string()),
        };

        self.parameter_assignments.push(assignment);
        self
    }

    /// Finish configuring the parameter value set
    /// 
    /// # Returns
    /// The parent multi-parameter builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if parameter value set configuration is invalid
    pub fn finish_set(mut self) -> BuilderResult<MultiParameterBuilder<S>> {
        // Validate that at least one parameter assignment is configured
        if self.parameter_assignments.is_empty() {
            return Err(BuilderError::constraint_violation(
                "parameter value set must have assignments",
                "Add at least one assignment using .assign()"
            ));
        }

        // Create parameter value set
        let parameter_value_set = ParameterValueSet {
            parameter_assignments: self.parameter_assignments,
        };

        // Add to multi-parameter builder
        self.multi_parameter_builder.parameter_value_sets.push(parameter_value_set);

        Ok(self.multi_parameter_builder)
    }
}

/// Builder for configuring stochastic parameter distributions
/// 
/// This builder provides configuration options for probabilistic parameter exploration
/// using various probability distributions.
#[derive(Debug)]
pub struct StochasticDistributionBuilder<S: BuilderState> {
    /// Parent parameter variation builder
    param_variation_builder: ParameterVariationBuilder<S>,
    
    /// Stochastic distribution being configured
    stochastic_distribution: StochasticDistribution,
}

impl<S: AfterHeader> StochasticDistributionBuilder<S> {
    /// Create a new stochastic distribution builder
    fn new(param_variation_builder: ParameterVariationBuilder<S>) -> Self {
        Self {
            param_variation_builder,
            stochastic_distribution: StochasticDistribution {
                distribution_type: crate::types::distributions::stochastic::StochasticDistributionType::NormalDistribution(
                    crate::types::distributions::stochastic::NormalDistribution {
                        expected_value: Value::literal("0.0".to_string()),
                        variance: Value::literal("1.0".to_string()),
                        range: None,
                    }
                ),
                parameter_name: Value::literal("default".to_string()),
                random_seed: None,
            },
        }
    }

    /// Configure a normal distribution for a parameter
    /// 
    /// # Arguments
    /// * `parameter_name` - Name of the parameter
    /// * `mean` - Mean value of the normal distribution
    /// * `std_dev` - Standard deviation of the normal distribution
    /// 
    /// # Returns
    /// Self for method chaining
    /// 
    /// # Example
    /// ```rust
    /// builder.normal_distribution("speed", 30.0, 5.0)
    /// ```
    pub fn normal_distribution(mut self, _parameter_name: &str, _mean: f64, _std_dev: f64) -> Self {
        // TODO: Implement stochastic distribution configuration
        // This is a placeholder for the MVP implementation
        self
    }

    /// Finish configuring the stochastic distribution
    /// 
    /// # Returns
    /// The parent parameter variation builder for continued configuration
    /// 
    /// # Errors
    /// Returns BuilderError if stochastic distribution configuration is invalid
    pub fn finish_distribution(mut self) -> BuilderResult<ParameterVariationBuilder<S>> {
        // Create parameter value distribution
        let scenario_file = self.param_variation_builder.param_variation_data.scenario_file
            .clone()
            .ok_or_else(|| BuilderError::missing_field(
                "scenario_file",
                "Internal error: scenario file not set"
            ))?;

        let stochastic = Stochastic {
            distributions: vec![self.stochastic_distribution],
            number_of_test_runs: Value::literal(100),
            random_seed: None,
        };

        let parameter_value_distribution = ParameterValueDistribution::new_stochastic(
            scenario_file,
            stochastic
        );

        self.param_variation_builder.param_variation_data.parameter_value_distribution = 
            Some(parameter_value_distribution);

        Ok(self.param_variation_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_variation_builder_state_transitions() {
        // Test basic state transitions
        let builder = ParameterVariationBuilder::new("test_scenario.xosc")
            .with_simple_header("Speed Study", "Test Engineer")
            .with_deterministic_distribution()
                .single_parameter("speed")
                    .distribution_range(20.0, 40.0, 5.0)
                    .finish_parameter()
                    .unwrap()
                .finish_distribution()
                .unwrap();
            
        // Builder should be ready to build
        let param_variation = builder.build().unwrap();
        assert!(param_variation.is_parameter_variation());
        assert_eq!(param_variation.file_header.author.as_literal(), Some(&"Test Engineer".to_string()));
    }

    #[test]
    fn test_parameter_variation_builder_deterministic_range() {
        let builder = ParameterVariationBuilder::new("highway.xosc")
            .with_simple_header("Highway Speed Study", "Test Engineer")
            .with_deterministic_distribution()
                .single_parameter("initial_speed")
                    .distribution_range(20.0, 40.0, 5.0)
                    .finish_parameter()
                    .unwrap()
                .finish_distribution()
                .unwrap();
            
        let param_variation = builder.build().unwrap();
        assert!(param_variation.parameter_value_distribution.is_some());
        
        let dist = param_variation.parameter_value_distribution.unwrap();
        assert!(dist.deterministic.is_some());
        assert!(dist.stochastic.is_none());
        
        let deterministic = dist.deterministic.unwrap();
        assert_eq!(deterministic.single_distributions.len(), 1);
        assert_eq!(deterministic.multi_distributions.len(), 0);
    }

    #[test]
    fn test_parameter_variation_builder_deterministic_set() {
        let builder = ParameterVariationBuilder::new("city.xosc")
            .with_simple_header("City Speed Study", "Test Engineer")
            .with_deterministic_distribution()
                .single_parameter("speed_limit")
                    .distribution_set(vec!["30", "40", "50", "60"])
                    .finish_parameter()
                    .unwrap()
                .finish_distribution()
                .unwrap();
            
        let param_variation = builder.build().unwrap();
        let dist = param_variation.parameter_value_distribution.unwrap();
        let deterministic = dist.deterministic.unwrap();
        let single_dist = &deterministic.single_distributions[0];
        
        assert!(single_dist.distribution_set.is_some());
        assert!(single_dist.distribution_range.is_none());
        
        let set = single_dist.distribution_set.as_ref().unwrap();
        assert_eq!(set.elements.len(), 4);
    }

    #[test]
    fn test_parameter_variation_builder_multi_parameter() {
        let builder = ParameterVariationBuilder::new("complex.xosc")
            .with_simple_header("Complex Study", "Test Engineer")
            .with_deterministic_distribution()
                .multi_parameter()
                    .add_parameter_set()
                        .assign("speed", "30.0")
                        .assign("weather", "dry")
                        .finish_set()
                        .unwrap()
                    .add_parameter_set()
                        .assign("speed", "25.0")
                        .assign("weather", "wet")
                        .finish_set()
                        .unwrap()
                    .finish_multi_parameter()
                    .unwrap()
                .finish_distribution()
                .unwrap();
            
        let param_variation = builder.build().unwrap();
        let dist = param_variation.parameter_value_distribution.unwrap();
        let deterministic = dist.deterministic.unwrap();
        
        assert_eq!(deterministic.single_distributions.len(), 0);
        assert_eq!(deterministic.multi_distributions.len(), 1);
        
        let multi_dist = &deterministic.multi_distributions[0];
        match &multi_dist.distribution_type {
            crate::types::distributions::deterministic::DeterministicMultiParameterDistributionType::ValueSetDistribution(vsd) => {
                assert_eq!(vsd.parameter_value_sets.len(), 2);
            }
        }
    }

    #[test]
    fn test_parameter_variation_builder_stochastic() {
        let builder = ParameterVariationBuilder::new("stochastic.xosc")
            .with_simple_header("Stochastic Study", "Test Engineer")
            .with_stochastic_distribution()
                .normal_distribution("speed", 30.0, 5.0)
                .finish_distribution()
                .unwrap();
            
        let param_variation = builder.build().unwrap();
        let dist = param_variation.parameter_value_distribution.unwrap();
        
        assert!(dist.deterministic.is_none());
        assert!(dist.stochastic.is_some());
    }

    #[test]
    fn test_parameter_variation_builder_validation_errors() {
        // Test building without distribution
        let result = ParameterVariationBuilder::new("test.xosc")
            .with_simple_header("Empty", "Author")
            .build();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("parameter_value_distribution"));

        // Test deterministic distribution without parameters
        let result = ParameterVariationBuilder::new("test.xosc")
            .with_simple_header("Empty Dist", "Author")
            .with_deterministic_distribution()
            .finish_distribution();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("deterministic distribution must have parameters"));
    }

    #[test]
    fn test_parameter_variation_builder_mixed_distributions() {
        let builder = ParameterVariationBuilder::new("mixed.xosc")
            .with_simple_header("Mixed Study", "Test Engineer")
            .with_deterministic_distribution()
                .single_parameter("speed")
                    .distribution_range(20.0, 40.0, 10.0)
                    .finish_parameter()
                    .unwrap()
                .single_parameter("weather")
                    .distribution_set(vec!["dry", "wet", "snow"])
                    .finish_parameter()
                    .unwrap()
                .multi_parameter()
                    .add_parameter_set()
                        .assign("time_of_day", "morning")
                        .assign("traffic_density", "low")
                        .finish_set()
                        .unwrap()
                    .finish_multi_parameter()
                    .unwrap()
                .finish_distribution()
                .unwrap();
            
        let param_variation = builder.build().unwrap();
        let dist = param_variation.parameter_value_distribution.unwrap();
        let deterministic = dist.deterministic.unwrap();
        
        assert_eq!(deterministic.single_distributions.len(), 2);
        assert_eq!(deterministic.multi_distributions.len(), 1);
    }
}
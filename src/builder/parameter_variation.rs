//! Parameter variation builder for programmatic parameter variation document construction
//!
//! This module provides the ParameterVariationBuilder for creating OpenSCENARIO parameter
//! variation documents with type-safe state transitions and comprehensive validation.

use crate::types::{
    basic::{Value, OSString, UnsignedInt},
    scenario::storyboard::{FileHeader, OpenScenario},
    distributions::{
        ParameterValueDistribution, Deterministic, Stochastic,
        deterministic::{
            DeterministicSingleParameterDistribution, DistributionSet, DistributionSetElement,
            DistributionRange,
        },
        stochastic::{
            StochasticDistribution, StochasticDistributionType,
            NormalDistribution, UniformDistribution, PoissonDistribution, LogNormalDistribution,
            Range,
        },
    },
    entities::vehicle::File,
};
use super::{
    error::{BuilderError, BuilderResult},
    states::*,
};
use std::marker::PhantomData;

/// Parameter variation builder with type-safe state transitions
pub struct ParameterVariationBuilder<S: BuilderState> {
    _state: PhantomData<S>,
    param_var_data: PartialParameterVariationData,
}

/// Internal data structure for building parameter variations
#[derive(Debug, Default)]
struct PartialParameterVariationData {
    file_header: Option<FileHeader>,
    scenario_file: Option<File>,
    deterministic_distributions: Vec<DeterministicSingleParameterDistribution>,
    stochastic_distributions: Vec<StochasticDistribution>,
}

impl ParameterVariationBuilder<Empty> {
    /// Create a new parameter variation builder
    pub fn new() -> ParameterVariationBuilder<HasHeader> {
        ParameterVariationBuilder {
            _state: PhantomData,
            param_var_data: PartialParameterVariationData::default(),
        }
    }
}

impl ParameterVariationBuilder<HasHeader> {
    /// Set the file header and keep in HasHeader state
    pub fn with_header(
        mut self,
        author: String,
        date: String,
        description: String,
        rev_major: u16,
        rev_minor: u16,
    ) -> Self {
        let header = FileHeader {
            author: Value::literal(author),
            date: Value::literal(date),
            description: Value::literal(description),
            rev_major: Value::literal(rev_major),
            rev_minor: Value::literal(rev_minor),
        };

        self.param_var_data.file_header = Some(header);
        self
    }

    /// Set a simple header with default values
    pub fn with_simple_header(self, description: &str, author: &str) -> Self {
        self.with_header(
            author.to_string(),
            "2024-01-01T00:00:00".to_string(),
            description.to_string(),
            1,
            3,
        )
    }

    /// Set the scenario file that this parameter variation applies to
    pub fn with_scenario_file(mut self, filepath: &str) -> Self {
        self.param_var_data.scenario_file = Some(File {
            filepath: filepath.to_string(),
        });
        self
    }

    /// Add a deterministic distribution and return a deterministic distribution builder
    pub fn add_deterministic_distribution(self, parameter_name: &str) -> DeterministicDistributionBuilder {
        DeterministicDistributionBuilder::new(self, parameter_name.to_string())
    }

    /// Add a stochastic distribution and return a stochastic distribution builder
    pub fn add_stochastic_distribution(self, parameter_name: &str) -> StochasticDistributionBuilder {
        StochasticDistributionBuilder::new(self, parameter_name.to_string())
    }

    /// Build the complete parameter variation document
    pub fn build(self) -> BuilderResult<OpenScenario> {
        // Validate required fields
        let file_header = self.param_var_data.file_header
            .ok_or_else(|| BuilderError::missing_field("file_header", "Call with_header() or with_simple_header() first"))?;

        let scenario_file = self.param_var_data.scenario_file
            .unwrap_or_else(|| File { filepath: "scenario.xosc".to_string() });

        // Validate that at least one distribution is defined
        if self.param_var_data.deterministic_distributions.is_empty() && 
           self.param_var_data.stochastic_distributions.is_empty() {
            return Err(BuilderError::validation_error(
                "No distributions defined in parameter variation",
                "Add at least one distribution using add_deterministic_distribution() or add_stochastic_distribution()"
            ));
        }

        // Build the parameter value distribution
        let parameter_value_distribution = if !self.param_var_data.deterministic_distributions.is_empty() {
            ParameterValueDistribution::new_deterministic(
                scenario_file,
                Deterministic {
                    single_distributions: self.param_var_data.deterministic_distributions,
                    multi_distributions: Vec::new(), // Not implemented in this phase
                }
            )
        } else {
            ParameterValueDistribution::new_stochastic(
                scenario_file,
                Stochastic {
                    distributions: self.param_var_data.stochastic_distributions,
                    number_of_test_runs: Value::literal(1),
                    random_seed: None,
                }
            )
        };

        // Build the complete OpenScenario document as a parameter variation
        Ok(OpenScenario {
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
        })
    }
}

// Distribution builders

/// Builder for deterministic distributions
pub struct DeterministicDistributionBuilder {
    parent: ParameterVariationBuilder<HasHeader>,
    distribution_data: PartialDeterministicDistributionData,
}

#[derive(Debug, Default)]
struct PartialDeterministicDistributionData {
    parameter_name: String,
    values: Option<Vec<String>>,
    range_start: Option<f64>,
    range_end: Option<f64>,
    range_step: Option<f64>,
}

impl DeterministicDistributionBuilder {
    fn new(parent: ParameterVariationBuilder<HasHeader>, parameter_name: String) -> Self {
        Self {
            parent,
            distribution_data: PartialDeterministicDistributionData {
                parameter_name,
                ..Default::default()
            },
        }
    }

    /// Set explicit values for the distribution
    pub fn with_values(mut self, values: Vec<&str>) -> Self {
        self.distribution_data.values = Some(values.into_iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set a range for the distribution (start, end, step)
    pub fn with_range(mut self, start: f64, end: f64, step: f64) -> Self {
        self.distribution_data.range_start = Some(start);
        self.distribution_data.range_end = Some(end);
        self.distribution_data.range_step = Some(step);
        self
    }

    /// Finish building the deterministic distribution and return to the parameter variation builder
    pub fn finish(mut self) -> ParameterVariationBuilder<HasHeader> {
        let distribution = if let Some(values) = self.distribution_data.values {
            // Create distribution set
            let elements: Vec<DistributionSetElement> = values.into_iter()
                .map(|v| DistributionSetElement {
                    value: Value::literal(v),
                })
                .collect();

            DeterministicSingleParameterDistribution {
                parameter_name: Value::literal(self.distribution_data.parameter_name),
                distribution_set: Some(DistributionSet { elements }),
                distribution_range: None,
                user_defined_distribution: None,
            }
        } else if let (Some(start), Some(end), Some(step)) = (
            self.distribution_data.range_start,
            self.distribution_data.range_end,
            self.distribution_data.range_step,
        ) {
            // Create distribution range
            DeterministicSingleParameterDistribution {
                parameter_name: Value::literal(self.distribution_data.parameter_name),
                distribution_set: None,
                distribution_range: Some(DistributionRange {
                    step_width: Value::literal(step),
                    range: crate::types::basic::Range {
                        lower_limit: Value::literal(start),
                        upper_limit: Value::literal(end),
                    },
                }),
                user_defined_distribution: None,
            }
        } else {
            // Default to a single value
            DeterministicSingleParameterDistribution {
                parameter_name: Value::literal(self.distribution_data.parameter_name),
                distribution_set: Some(DistributionSet {
                    elements: vec![DistributionSetElement {
                        value: Value::literal("0.0".to_string()),
                    }],
                }),
                distribution_range: None,
                user_defined_distribution: None,
            }
        };

        // Add the distribution to the parent parameter variation builder
        self.parent.param_var_data.deterministic_distributions.push(distribution);
        self.parent
    }
}

/// Builder for stochastic distributions
pub struct StochasticDistributionBuilder {
    parent: ParameterVariationBuilder<HasHeader>,
    distribution_data: PartialStochasticDistributionData,
}

#[derive(Debug, Default)]
struct PartialStochasticDistributionData {
    parameter_name: String,
    distribution_type: Option<String>,
    mean: Option<f64>,
    std_dev: Option<f64>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    lambda: Option<f64>,
}

impl StochasticDistributionBuilder {
    fn new(parent: ParameterVariationBuilder<HasHeader>, parameter_name: String) -> Self {
        Self {
            parent,
            distribution_data: PartialStochasticDistributionData {
                parameter_name,
                ..Default::default()
            },
        }
    }

    /// Set as normal distribution with mean and standard deviation
    pub fn normal_distribution(mut self, mean: f64, std_dev: f64) -> Self {
        self.distribution_data.distribution_type = Some("normal".to_string());
        self.distribution_data.mean = Some(mean);
        self.distribution_data.std_dev = Some(std_dev);
        self
    }

    /// Set as uniform distribution with min and max values
    pub fn uniform_distribution(mut self, min_value: f64, max_value: f64) -> Self {
        self.distribution_data.distribution_type = Some("uniform".to_string());
        self.distribution_data.min_value = Some(min_value);
        self.distribution_data.max_value = Some(max_value);
        self
    }

    /// Set as Poisson distribution with lambda parameter
    pub fn poisson_distribution(mut self, lambda: f64) -> Self {
        self.distribution_data.distribution_type = Some("poisson".to_string());
        self.distribution_data.lambda = Some(lambda);
        self
    }

    /// Set as log-normal distribution with mean and standard deviation
    pub fn log_normal_distribution(mut self, mean: f64, std_dev: f64) -> Self {
        self.distribution_data.distribution_type = Some("log_normal".to_string());
        self.distribution_data.mean = Some(mean);
        self.distribution_data.std_dev = Some(std_dev);
        self
    }

    /// Finish building the stochastic distribution and return to the parameter variation builder
    pub fn finish(mut self) -> ParameterVariationBuilder<HasHeader> {
        let distribution_type = match self.distribution_data.distribution_type.as_deref() {
            Some("normal") => {
                StochasticDistributionType::NormalDistribution(NormalDistribution {
                    expected_value: OSString::literal(self.distribution_data.mean.unwrap_or(0.0).to_string()),
                    variance: OSString::literal(self.distribution_data.std_dev.unwrap_or(1.0).powi(2).to_string()),
                    range: None,
                })
            }
            Some("uniform") => {
                StochasticDistributionType::UniformDistribution(UniformDistribution {
                    range: Range {
                        lower_limit: OSString::literal(self.distribution_data.min_value.unwrap_or(0.0).to_string()),
                        upper_limit: OSString::literal(self.distribution_data.max_value.unwrap_or(1.0).to_string()),
                    },
                })
            }
            Some("poisson") => {
                StochasticDistributionType::PoissonDistribution(PoissonDistribution {
                    expected_value: OSString::literal(self.distribution_data.lambda.unwrap_or(1.0).to_string()),
                    range: None,
                })
            }
            Some("log_normal") => {
                StochasticDistributionType::LogNormalDistribution(LogNormalDistribution {
                    expected_value: OSString::literal(self.distribution_data.mean.unwrap_or(0.0).to_string()),
                    variance: OSString::literal(self.distribution_data.std_dev.unwrap_or(1.0).powi(2).to_string()),
                    range: None,
                })
            }
            _ => {
                // Default to normal distribution
                StochasticDistributionType::NormalDistribution(NormalDistribution {
                    expected_value: OSString::literal("0.0".to_string()),
                    variance: OSString::literal("1.0".to_string()),
                    range: None,
                })
            }
        };

        let distribution = StochasticDistribution {
            distribution_type,
            parameter_name: OSString::literal(self.distribution_data.parameter_name.clone()),
            random_seed: None,
        };

        // Add the distribution to the parent parameter variation builder
        self.parent.param_var_data.stochastic_distributions.push(distribution);
        self.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_variation_builder_basic_construction() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Test Parameter Variation", "Test Author")
            .with_scenario_file("test_scenario.xosc")
            .add_deterministic_distribution("speed")
                .with_values(vec!["10.0", "20.0", "30.0"])
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        assert!(param_var.parameter_value_distribution.is_some());
        
        let param_dist = param_var.parameter_value_distribution.unwrap();
        assert_eq!(param_dist.scenario_file.filepath, "test_scenario.xosc");
        assert!(param_dist.deterministic.is_some());
        assert!(param_dist.stochastic.is_none());
    }

    #[test]
    fn test_parameter_variation_builder_deterministic_range() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Range Test", "Test Author")
            .add_deterministic_distribution("initial_speed")
                .with_range(20.0, 60.0, 5.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        let deterministic = param_dist.deterministic.unwrap();
        assert_eq!(deterministic.single_distributions.len(), 1);
        
        let single_dist = &deterministic.single_distributions[0];
        assert_eq!(single_dist.parameter_name.as_literal().unwrap(), "initial_speed");
        assert!(single_dist.distribution_range.is_some());
        assert!(single_dist.distribution_set.is_none());
    }

    #[test]
    fn test_parameter_variation_builder_stochastic_normal() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Stochastic Test", "Test Author")
            .add_stochastic_distribution("weather_condition")
                .normal_distribution(25.0, 5.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        assert!(param_dist.stochastic.is_some());
        assert!(param_dist.deterministic.is_none());
        
        let stochastic = param_dist.stochastic.unwrap();
        assert_eq!(stochastic.distributions.len(), 1);
        
        match &stochastic.distributions[0].distribution_type {
            StochasticDistributionType::NormalDistribution(normal) => {
                assert_eq!(normal.expected_value.as_literal().unwrap(), "25");
                assert_eq!(normal.variance.as_literal().unwrap(), "25");
            }
            _ => panic!("Expected normal distribution"),
        }
    }

    #[test]
    fn test_parameter_variation_builder_stochastic_uniform() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Uniform Test", "Test Author")
            .add_stochastic_distribution("random_factor")
                .uniform_distribution(0.0, 1.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        let stochastic = param_dist.stochastic.unwrap();
        
        match &stochastic.distributions[0].distribution_type {
            StochasticDistributionType::UniformDistribution(uniform) => {
                assert_eq!(uniform.range.lower_limit.as_literal().unwrap(), "0");
                assert_eq!(uniform.range.upper_limit.as_literal().unwrap(), "1");
            }
            _ => panic!("Expected uniform distribution"),
        }
    }

    #[test]
    fn test_parameter_variation_builder_multiple_distributions() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Multiple Distributions", "Test Author")
            .add_deterministic_distribution("speed")
                .with_values(vec!["30.0", "50.0"])
                .finish()
            .add_deterministic_distribution("distance")
                .with_range(100.0, 500.0, 50.0)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        assert!(param_var.is_parameter_variation());
        let param_dist = param_var.parameter_value_distribution.unwrap();
        let deterministic = param_dist.deterministic.unwrap();
        assert_eq!(deterministic.single_distributions.len(), 2);
    }

    #[test]
    fn test_parameter_variation_builder_empty_fails() {
        let result = ParameterVariationBuilder::new()
            .with_simple_header("Empty Parameter Variation", "Test Author")
            .build();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BuilderError::ValidationError { .. }));
    }

    #[test]
    fn test_parameter_variation_builder_poisson_distribution() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("Poisson Test", "Test Author")
            .add_stochastic_distribution("event_count")
                .poisson_distribution(3.5)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        let param_dist = param_var.parameter_value_distribution.unwrap();
        let stochastic = param_dist.stochastic.unwrap();
        
        match &stochastic.distributions[0].distribution_type {
            StochasticDistributionType::PoissonDistribution(poisson) => {
                assert_eq!(poisson.expected_value.as_literal().unwrap(), "3.5");
            }
            _ => panic!("Expected Poisson distribution"),
        }
    }

    #[test]
    fn test_parameter_variation_builder_log_normal_distribution() {
        let param_var = ParameterVariationBuilder::new()
            .with_simple_header("LogNormal Test", "Test Author")
            .add_stochastic_distribution("response_time")
                .log_normal_distribution(2.0, 0.5)
                .finish()
            .build()
            .expect("Should build valid parameter variation");

        let param_dist = param_var.parameter_value_distribution.unwrap();
        let stochastic = param_dist.stochastic.unwrap();
        
        match &stochastic.distributions[0].distribution_type {
            StochasticDistributionType::LogNormalDistribution(log_normal) => {
                assert_eq!(log_normal.expected_value.as_literal().unwrap(), "2");
                assert_eq!(log_normal.variance.as_literal().unwrap(), "0.25");
            }
            _ => panic!("Expected log-normal distribution"),
        }
    }
}
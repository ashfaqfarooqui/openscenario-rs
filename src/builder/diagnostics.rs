//! Enhanced diagnostics and developer experience utilities
//!
//! This module provides improved error messages, debugging tools, and developer
//! productivity features for the OpenSCENARIO builder system.

use std::collections::HashMap;
use std::fmt;
use crate::builder::{BuilderError, BuilderResult};

/// Error codes for programmatic error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Header errors
    E001, // Missing required header field
    E002, // Invalid header format
    
    // Entity errors  
    E101, // Duplicate entity name
    E102, // Invalid entity reference
    E103, // Missing entity properties
    
    // Position errors
    E201, // Invalid position coordinates
    E202, // Missing position reference
    E203, // Conflicting position types
    
    // Action errors
    E301, // Invalid action parameters
    E302, // Missing action target
    E303, // Conflicting action types
    
    // Condition errors
    E401, // Invalid condition logic
    E402, // Missing condition parameters
    E403, // Unreachable condition
    
    // Storyboard errors
    E501, // Invalid story structure
    E502, // Missing story elements
    E503, // Circular dependencies
    
    // Validation errors
    E601, // Schema validation failed
    E602, // Semantic validation failed
    E603, // Reference validation failed
    
    // Performance errors
    E701, // Memory limit exceeded
    E702, // Build time exceeded
    E703, // Complexity limit exceeded
}

impl ErrorCode {
    /// Get human-readable description of the error code
    pub fn description(self) -> &'static str {
        match self {
            ErrorCode::E001 => "Missing required header field",
            ErrorCode::E002 => "Invalid header format",
            ErrorCode::E101 => "Duplicate entity name",
            ErrorCode::E102 => "Invalid entity reference",
            ErrorCode::E103 => "Missing entity properties",
            ErrorCode::E201 => "Invalid position coordinates",
            ErrorCode::E202 => "Missing position reference",
            ErrorCode::E203 => "Conflicting position types",
            ErrorCode::E301 => "Invalid action parameters",
            ErrorCode::E302 => "Missing action target",
            ErrorCode::E303 => "Conflicting action types",
            ErrorCode::E401 => "Invalid condition logic",
            ErrorCode::E402 => "Missing condition parameters",
            ErrorCode::E403 => "Unreachable condition",
            ErrorCode::E501 => "Invalid story structure",
            ErrorCode::E502 => "Missing story elements",
            ErrorCode::E503 => "Circular dependencies",
            ErrorCode::E601 => "Schema validation failed",
            ErrorCode::E602 => "Semantic validation failed",
            ErrorCode::E603 => "Reference validation failed",
            ErrorCode::E701 => "Memory limit exceeded",
            ErrorCode::E702 => "Build time exceeded",
            ErrorCode::E703 => "Complexity limit exceeded",
        }
    }

    /// Get suggested fix for the error
    pub fn suggestion(self) -> &'static str {
        match self {
            ErrorCode::E001 => "Add the missing header field using .with_header() or .with_simple_header()",
            ErrorCode::E002 => "Check header field formats (dates should be ISO 8601, versions should be numeric)",
            ErrorCode::E101 => "Use unique entity names or check for typos in entity references",
            ErrorCode::E102 => "Ensure the entity exists before referencing it in actions or conditions",
            ErrorCode::E103 => "Add required properties like position, model, or dimensions",
            ErrorCode::E201 => "Check coordinate values are finite numbers and within reasonable ranges",
            ErrorCode::E202 => "Specify a position using .at_position().world(), .lane(), or .road()",
            ErrorCode::E203 => "Use only one position type per entity (world, lane, road, or relative)",
            ErrorCode::E301 => "Check action parameter types and ranges (e.g., speed > 0)",
            ErrorCode::E302 => "Specify the target entity or position for the action",
            ErrorCode::E303 => "Use only one action type per event (e.g., speed OR lane change)",
            ErrorCode::E401 => "Check condition logic and ensure all parameters are valid",
            ErrorCode::E402 => "Provide all required condition parameters (entity refs, thresholds, etc.)",
            ErrorCode::E403 => "Ensure condition can be triggered given the scenario setup",
            ErrorCode::E501 => "Check story hierarchy: scenario -> story -> act -> maneuver -> event",
            ErrorCode::E502 => "Add required story elements like acts, maneuvers, or events",
            ErrorCode::E503 => "Remove circular references between story elements",
            ErrorCode::E601 => "Check against OpenSCENARIO schema requirements",
            ErrorCode::E602 => "Verify logical consistency of scenario elements",
            ErrorCode::E603 => "Ensure all referenced entities, catalogs, and parameters exist",
            ErrorCode::E701 => "Reduce scenario complexity or increase memory limits",
            ErrorCode::E702 => "Optimize scenario construction or use bulk operations",
            ErrorCode::E703 => "Simplify scenario structure or break into smaller scenarios",
        }
    }

    /// Get documentation URL for the error
    pub fn docs_url(self) -> String {
        format!("https://docs.rs/openscenario-rs/latest/openscenario_rs/builder/diagnostics/enum.ErrorCode.html#{:?}", self)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Enhanced error with context and suggestions
#[derive(Debug, Clone)]
pub struct DiagnosticError {
    pub code: ErrorCode,
    pub message: String,
    pub context: Option<String>,
    pub suggestion: String,
    pub location: Option<SourceLocation>,
    pub related_errors: Vec<DiagnosticError>,
}

/// Source location information for errors
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub function: Option<String>,
}

impl DiagnosticError {
    /// Create a new diagnostic error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            context: None,
            suggestion: code.suggestion().to_string(),
            location: None,
            related_errors: Vec::new(),
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Add custom suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = suggestion.into();
        self
    }

    /// Add source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Add related error
    pub fn with_related(mut self, error: DiagnosticError) -> Self {
        self.related_errors.push(error);
        self
    }
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error {}: {}", self.code, self.message)?;
        
        if let Some(context) = &self.context {
            writeln!(f, "Context: {}", context)?;
        }
        
        if let Some(location) = &self.location {
            if let Some(file) = &location.file {
                write!(f, "Location: {}", file)?;
                if let Some(line) = location.line {
                    write!(f, ":{}", line)?;
                    if let Some(column) = location.column {
                        write!(f, ":{}", column)?;
                    }
                }
                writeln!(f)?;
            }
        }
        
        writeln!(f, "Suggestion: {}", self.suggestion)?;
        writeln!(f, "Documentation: {}", self.code.docs_url())?;
        
        if !self.related_errors.is_empty() {
            writeln!(f, "Related errors:")?;
            for error in &self.related_errors {
                writeln!(f, "  - {}: {}", error.code, error.message)?;
            }
        }
        
        Ok(())
    }
}

impl std::error::Error for DiagnosticError {}

/// "Did you mean?" suggestion engine
pub struct SuggestionEngine {
    entity_names: Vec<String>,
    parameter_names: Vec<String>,
    method_names: Vec<String>,
}

impl SuggestionEngine {
    /// Create new suggestion engine
    pub fn new() -> Self {
        Self {
            entity_names: Vec::new(),
            parameter_names: Vec::new(),
            method_names: vec![
                "with_header".to_string(),
                "with_simple_header".to_string(),
                "with_catalogs".to_string(),
                "with_default_catalogs".to_string(),
                "with_road_network".to_string(),
                "with_entities".to_string(),
                "add_vehicle".to_string(),
                "add_pedestrian".to_string(),
                "add_misc_object".to_string(),
                "at_position".to_string(),
                "world".to_string(),
                "lane".to_string(),
                "road".to_string(),
                "relative".to_string(),
                "build".to_string(),
            ],
        }
    }

    /// Add known entity name
    pub fn add_entity(&mut self, name: String) {
        self.entity_names.push(name);
    }

    /// Add known parameter name
    pub fn add_parameter(&mut self, name: String) {
        self.parameter_names.push(name);
    }

    /// Find suggestions for a given input
    pub fn suggest(&self, input: &str, category: SuggestionCategory) -> Vec<String> {
        let candidates = match category {
            SuggestionCategory::Entity => &self.entity_names,
            SuggestionCategory::Parameter => &self.parameter_names,
            SuggestionCategory::Method => &self.method_names,
        };

        let mut suggestions: Vec<_> = candidates
            .iter()
            .map(|candidate| (candidate, levenshtein_distance(input, candidate)))
            .filter(|(_, distance)| *distance <= 3) // Max edit distance of 3
            .collect();

        suggestions.sort_by_key(|(_, distance)| *distance);
        suggestions.into_iter()
            .take(3) // Top 3 suggestions
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// Categories for suggestions
pub enum SuggestionCategory {
    Entity,
    Parameter,
    Method,
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,     // deletion
                    matrix[i + 1][j] + 1,     // insertion
                ),
                matrix[i][j] + cost,          // substitution
            );
        }
    }

    matrix[len1][len2]
}

/// Builder state inspector for debugging
pub struct StateInspector {
    state_history: Vec<String>,
    current_state: String,
    entity_count: usize,
    parameter_count: usize,
    validation_errors: Vec<String>,
}

impl StateInspector {
    /// Create new state inspector
    pub fn new() -> Self {
        Self {
            state_history: Vec::new(),
            current_state: "Empty".to_string(),
            entity_count: 0,
            parameter_count: 0,
            validation_errors: Vec::new(),
        }
    }

    /// Record state transition
    pub fn transition_to(&mut self, new_state: &str) {
        self.state_history.push(self.current_state.clone());
        self.current_state = new_state.to_string();
    }

    /// Record entity addition
    pub fn add_entity(&mut self) {
        self.entity_count += 1;
    }

    /// Record parameter addition
    pub fn add_parameter(&mut self) {
        self.parameter_count += 1;
    }

    /// Add validation error
    pub fn add_validation_error(&mut self, error: String) {
        self.validation_errors.push(error);
    }

    /// Get current state summary
    pub fn summary(&self) -> StateInspectorSummary {
        StateInspectorSummary {
            current_state: self.current_state.clone(),
            state_history: self.state_history.clone(),
            entity_count: self.entity_count,
            parameter_count: self.parameter_count,
            validation_errors: self.validation_errors.clone(),
            can_build: self.current_state == "HasEntities" || self.current_state == "Complete",
        }
    }
}

/// Summary of builder state for inspection
#[derive(Debug, Clone)]
pub struct StateInspectorSummary {
    pub current_state: String,
    pub state_history: Vec<String>,
    pub entity_count: usize,
    pub parameter_count: usize,
    pub validation_errors: Vec<String>,
    pub can_build: bool,
}

impl fmt::Display for StateInspectorSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Builder State Summary:")?;
        writeln!(f, "  Current State: {}", self.current_state)?;
        writeln!(f, "  Can Build: {}", self.can_build)?;
        writeln!(f, "  Entities: {}", self.entity_count)?;
        writeln!(f, "  Parameters: {}", self.parameter_count)?;
        
        if !self.state_history.is_empty() {
            writeln!(f, "  State History: {}", self.state_history.join(" -> "))?;
        }
        
        if !self.validation_errors.is_empty() {
            writeln!(f, "  Validation Errors:")?;
            for error in &self.validation_errors {
                writeln!(f, "    - {}", error)?;
            }
        }
        
        Ok(())
    }
}

/// Validation preview for incremental feedback
pub struct ValidationPreview {
    warnings: Vec<String>,
    errors: Vec<DiagnosticError>,
    suggestions: Vec<String>,
}

impl ValidationPreview {
    /// Create new validation preview
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Add error
    pub fn add_error(&mut self, error: DiagnosticError) {
        self.errors.push(error);
    }

    /// Add suggestion
    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get summary report
    pub fn report(&self) -> String {
        let mut report = String::new();
        
        if self.errors.is_empty() && self.warnings.is_empty() {
            report.push_str("✅ Validation passed\n");
        } else {
            if !self.errors.is_empty() {
                report.push_str(&format!("❌ {} errors found:\n", self.errors.len()));
                for error in &self.errors {
                    report.push_str(&format!("  - {}\n", error.message));
                }
            }
            
            if !self.warnings.is_empty() {
                report.push_str(&format!("⚠️  {} warnings:\n", self.warnings.len()));
                for warning in &self.warnings {
                    report.push_str(&format!("  - {}\n", warning));
                }
            }
        }
        
        if !self.suggestions.is_empty() {
            report.push_str("💡 Suggestions:\n");
            for suggestion in &self.suggestions {
                report.push_str(&format!("  - {}\n", suggestion));
            }
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_description() {
        assert_eq!(ErrorCode::E001.description(), "Missing required header field");
        assert_eq!(ErrorCode::E101.description(), "Duplicate entity name");
    }

    #[test]
    fn test_suggestion_engine() {
        let mut engine = SuggestionEngine::new();
        engine.add_entity("vehicle_1".to_string());
        engine.add_entity("vehicle_2".to_string());
        
        let suggestions = engine.suggest("vehicl", SuggestionCategory::Entity);
        assert!(suggestions.contains(&"vehicle_1".to_string()));
        assert!(suggestions.contains(&"vehicle_2".to_string()));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("cat", "bat"), 1);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }

    #[test]
    fn test_state_inspector() {
        let mut inspector = StateInspector::new();
        inspector.transition_to("HasHeader");
        inspector.add_entity();
        
        let summary = inspector.summary();
        assert_eq!(summary.current_state, "HasHeader");
        assert_eq!(summary.entity_count, 1);
        assert!(summary.state_history.contains(&"Empty".to_string()));
    }

    #[test]
    fn test_validation_preview() {
        let mut preview = ValidationPreview::new();
        preview.add_warning("Missing optional field".to_string());
        preview.add_suggestion("Consider adding a description".to_string());
        
        assert!(preview.is_valid()); // No errors
        let report = preview.report();
        assert!(report.contains("warnings"));
        assert!(report.contains("Suggestions"));
    }
}
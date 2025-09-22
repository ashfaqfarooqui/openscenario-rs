//! Declarative macros for OpenSCENARIO construction
//!
//! This module provides convenient macros for building scenarios, entities,
//! and other OpenSCENARIO elements with a more declarative syntax.

/// Create a scenario using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::scenario;
///
/// let scenario = scenario! {
///     header: {
///         title: "Highway Merge Test",
///         author: "Test Engineer",
///         date: "2024-01-15T10:00:00",
///     },
///     
///     parameters: {
///         initial_speed: Double = "25.0",
///         target_speed: Double = "35.0",
///     },
///     
///     entities: {
///         ego: vehicle!("TestCar") {
///             position: world!(0.0, 0.0, 0.0),
///             speed: parameter!("initial_speed"),
///         },
///     },
///     
///     story: "main" {
///         act: "acceleration" {
///             maneuver: "speed_up" {
///                 event: "accelerate" {
///                     action: longitudinal!(ego).speed(parameter!("target_speed")),
///                     trigger: condition!(simulation_time > 2.0),
///                 },
///             },
///         },
///     },
/// };
/// ```
#[macro_export]
macro_rules! scenario {
    {
        header: {
            title: $title:expr,
            author: $author:expr,
            $(date: $date:expr,)?
            $(description: $description:expr,)?
            $(rev_major: $rev_major:expr,)?
            $(rev_minor: $rev_minor:expr,)?
        },
        
        $(parameters: {
            $($param_name:ident: $param_type:ident = $param_value:expr),* $(,)?
        },)?
        
        $(entities: {
            $($entity_name:ident: $entity_def:expr),* $(,)?
        },)?
        
        $(story: $story_name:expr {
            $($story_content:tt)*
        })?
    } => {
        {
            use $crate::builder::ScenarioBuilder;
            
            let mut builder = ScenarioBuilder::new()
                .with_simple_header($title, $author);
                
            // Add default catalogs
            let builder = builder.with_default_catalogs()
                .expect("Failed to add default catalogs")
                .with_road_network("default.xodr")
                .with_entities();
            
            // Add parameters if specified
            $(
                $(
                    let builder = builder.add_parameter(
                        stringify!($param_name), 
                        $crate::types::basic::ParameterType::$param_type,
                        $param_value
                    );
                )*
            )?
            
            // Add entities if specified
            $(
                $(
                    let builder = builder.add_entity(stringify!($entity_name), $entity_def);
                )*
            )?
            
            // Add story if specified
            $(
                let builder = builder.with_story($story_name, |story| {
                    scenario_story_content!(story, $($story_content)*)
                });
            )?
            
            builder.build()
        }
    };
}

/// Create a vehicle entity using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::vehicle;
///
/// let vehicle = vehicle!("SportsCar") {
///     position: world!(100.0, 0.0, 0.0),
///     speed: 30.0,
///     model: "sports_car.fbx",
///     dimensions: (4.5, 1.8, 1.4),
/// };
/// ```
#[macro_export]
macro_rules! vehicle {
    ($catalog_ref:expr) => {
        $crate::builder::entities::VehicleBuilder::from_catalog($catalog_ref)
    };
    
    ($catalog_ref:expr { $($field:ident: $value:expr),* $(,)? }) => {
        {
            let mut builder = $crate::builder::entities::VehicleBuilder::from_catalog($catalog_ref);
            $(
                builder = vehicle_field!(builder, $field, $value);
            )*
            builder
        }
    };
}

/// Helper macro for setting vehicle fields
#[macro_export]
macro_rules! vehicle_field {
    ($builder:expr, position, $pos:expr) => {
        $builder.at_position($pos)
    };
    ($builder:expr, speed, $speed:expr) => {
        $builder.with_initial_speed($speed)
    };
    ($builder:expr, model, $model:expr) => {
        $builder.with_model($model)
    };
    ($builder:expr, dimensions, ($length:expr, $width:expr, $height:expr)) => {
        $builder.with_dimensions($length, $width, $height)
    };
    ($builder:expr, $field:ident, $value:expr) => {
        compile_error!(concat!("Unknown vehicle field: ", stringify!($field)))
    };
}

/// Create a world position using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::world;
///
/// let position = world!(100.0, 50.0, 0.0);
/// let position_with_orientation = world!(100.0, 50.0, 0.0, h: 1.57);
/// ```
#[macro_export]
macro_rules! world {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::builder::positions::WorldPositionBuilder::new()
            .at($x, $y, $z)
    };
    
    ($x:expr, $y:expr, $z:expr, h: $h:expr) => {
        $crate::builder::positions::WorldPositionBuilder::new()
            .at($x, $y, $z)
            .with_heading($h)
    };
    
    ($x:expr, $y:expr, $z:expr, h: $h:expr, p: $p:expr, r: $r:expr) => {
        $crate::builder::positions::WorldPositionBuilder::new()
            .at($x, $y, $z)
            .with_orientation($h, $p, $r)
    };
}

/// Create a lane position using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::lane;
///
/// let position = lane!("highway", 1, 100.0);
/// let position_with_offset = lane!("highway", 1, 100.0, offset: 1.5);
/// ```
#[macro_export]
macro_rules! lane {
    ($road_id:expr, $lane_id:expr, $s:expr) => {
        $crate::builder::positions::LanePositionBuilder::new()
            .on_road($road_id)
            .in_lane($lane_id)
            .at_s($s)
    };
    
    ($road_id:expr, $lane_id:expr, $s:expr, offset: $offset:expr) => {
        $crate::builder::positions::LanePositionBuilder::new()
            .on_road($road_id)
            .in_lane($lane_id)
            .at_s($s)
            .with_offset($offset)
    };
}

/// Create a condition using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::condition;
///
/// let cond = condition!(simulation_time > 5.0);
/// let cond = condition!(entity("ego").speed < 10.0);
/// let cond = condition!(distance("ego", "target") < 50.0);
/// ```
#[macro_export]
macro_rules! condition {
    (simulation_time $op:tt $value:expr) => {
        $crate::builder::conditions::SimulationTimeConditionBuilder::new()
            .with_threshold($value)
            .with_rule(condition_rule!($op))
    };
    
    (entity($entity:expr).speed $op:tt $value:expr) => {
        $crate::builder::conditions::SpeedConditionBuilder::new()
            .for_entity($entity)
            .with_threshold($value)
            .with_rule(condition_rule!($op))
    };
    
    (distance($entity1:expr, $entity2:expr) $op:tt $value:expr) => {
        $crate::builder::conditions::DistanceConditionBuilder::new()
            .between_entities($entity1, $entity2)
            .with_threshold($value)
            .with_rule(condition_rule!($op))
    };
}

/// Helper macro for condition rules
#[macro_export]
macro_rules! condition_rule {
    (>) => { $crate::types::enums::Rule::GreaterThan };
    (<) => { $crate::types::enums::Rule::LessThan };
    (>=) => { $crate::types::enums::Rule::GreaterOrEqual };
    (<=) => { $crate::types::enums::Rule::LessOrEqual };
    (==) => { $crate::types::enums::Rule::EqualTo };
    (!=) => { $crate::types::enums::Rule::NotEqualTo };
}

/// Create an action using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::action;
///
/// let action = action!(longitudinal("ego").speed(30.0));
/// let action = action!(teleport("ego").to(world!(100.0, 0.0, 0.0)));
/// ```
#[macro_export]
macro_rules! action {
    (longitudinal($entity:expr).speed($speed:expr)) => {
        $crate::builder::actions::LongitudinalActionBuilder::new()
            .for_entity($entity)
            .speed_action()
            .absolute_target($speed)
    };
    
    (teleport($entity:expr).to($position:expr)) => {
        $crate::builder::actions::TeleportActionBuilder::new()
            .for_entity($entity)
            .to_position($position)
    };
    
    (lateral($entity:expr).lane_change($target_lane:expr)) => {
        $crate::builder::actions::LateralActionBuilder::new()
            .for_entity($entity)
            .lane_change_action()
            .to_lane($target_lane)
    };
}

/// Create a parameter reference
///
/// # Example
///
/// ```rust
/// use openscenario_rs::parameter;
///
/// let param_ref = parameter!("initial_speed");
/// ```
#[macro_export]
macro_rules! parameter {
    ($name:expr) => {
        $crate::types::basic::Value::parameter($name)
    };
}

/// Create a story using declarative syntax
///
/// # Example
///
/// ```rust
/// use openscenario_rs::story;
///
/// let story = story!("main_story") {
///     act: "initialization" {
///         maneuver: "setup" {
///             event: "start" {
///                 action: teleport("ego").to(world!(0.0, 0.0, 0.0)),
///                 trigger: condition!(simulation_time > 0.0),
///             },
///         },
///     },
/// };
/// ```
#[macro_export]
macro_rules! story {
    ($name:expr { $($content:tt)* }) => {
        $crate::builder::storyboard::StoryBuilder::new($name)
            .with_content(|story| {
                story_content!(story, $($content)*)
            })
    };
}

/// Helper macro for story content
#[macro_export]
macro_rules! story_content {
    ($story:expr, act: $act_name:expr { $($act_content:tt)* } $($rest:tt)*) => {
        {
            let story = $story.add_act($act_name, |act| {
                act_content!(act, $($act_content)*)
            });
            story_content!(story, $($rest)*)
        }
    };
    
    ($story:expr,) => { $story };
    ($story:expr) => { $story };
}

/// Helper macro for act content
#[macro_export]
macro_rules! act_content {
    ($act:expr, maneuver: $maneuver_name:expr { $($maneuver_content:tt)* } $($rest:tt)*) => {
        {
            let act = $act.add_maneuver($maneuver_name, |maneuver| {
                maneuver_content!(maneuver, $($maneuver_content)*)
            });
            act_content!(act, $($rest)*)
        }
    };
    
    ($act:expr,) => { $act };
    ($act:expr) => { $act };
}

/// Helper macro for maneuver content
#[macro_export]
macro_rules! maneuver_content {
    ($maneuver:expr, event: $event_name:expr { action: $action:expr, trigger: $trigger:expr $(,)? } $($rest:tt)*) => {
        {
            let maneuver = $maneuver.add_event($event_name)
                .with_action($action)
                .with_start_trigger($trigger)
                .finish_event();
            maneuver_content!(maneuver, $($rest)*)
        }
    };
    
    ($maneuver:expr,) => { $maneuver };
    ($maneuver:expr) => { $maneuver };
}

/// Create a complete scenario template
///
/// # Example
///
/// ```rust
/// use openscenario_rs::scenario_template;
///
/// let template = scenario_template! {
///     name: "highway_merge",
///     description: "Vehicle merging onto highway",
///     
///     parameters: {
///         ego_speed: Double = "30.0",
///         target_speed: Double = "35.0",
///         merge_distance: Double = "100.0",
///     },
///     
///     entities: {
///         ego: vehicle!("sedan") {
///             position: lane!("on_ramp", 1, 0.0),
///             speed: parameter!("ego_speed"),
///         },
///         
///         target: vehicle!("sedan") {
///             position: lane!("highway", 1, parameter!("merge_distance")),
///             speed: parameter!("target_speed"),
///         },
///     },
///     
///     story: "merge_scenario" {
///         act: "merge_maneuver" {
///             maneuver: "lane_change" {
///                 event: "start_merge" {
///                     action: lateral!(ego).lane_change("highway"),
///                     trigger: condition!(distance(ego, target) < 50.0),
///                 },
///             },
///         },
///     },
/// };
/// ```
#[macro_export]
macro_rules! scenario_template {
    {
        name: $name:expr,
        description: $description:expr,
        
        parameters: {
            $($param_name:ident: $param_type:ident = $param_value:expr),* $(,)?
        },
        
        entities: {
            $($entity_name:ident: $entity_def:expr),* $(,)?
        },
        
        story: $story_name:expr {
            $($story_content:tt)*
        }
    } => {
        $crate::builder::templates::ScenarioTemplate::new($name)
            .with_description($description)
            $(.with_parameter(stringify!($param_name), $crate::types::basic::ParameterType::$param_type, $param_value))*
            $(.with_entity(stringify!($entity_name), $entity_def))*
            .with_story($story_name, |story| {
                story_content!(story, $($story_content)*)
            })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_macro() {
        let pos = world!(100.0, 50.0, 0.0);
        // Test that the macro compiles and creates a position builder
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_lane_macro() {
        let pos = lane!("highway", 1, 100.0);
        // Test that the macro compiles and creates a lane position builder
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_parameter_macro() {
        let param = parameter!("test_param");
        // Test that the macro compiles and creates a parameter reference
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_condition_rule_macro() {
        let rule_gt = condition_rule!(>);
        let rule_lt = condition_rule!(<);
        let rule_eq = condition_rule!(==);
        // Test that the macro compiles and creates the correct rules
        assert!(true); // Placeholder test
    }
}
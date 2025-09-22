//! Integration tests for Phase 3 advanced features
//!
//! Tests the complete storyboard builder system including triggers, events,
//! stories, acts, maneuvers, and their integration with the scenario builder.

use openscenario_rs::builder::ScenarioBuilder;

#[test]
fn test_complete_scenario_construction() {
    let scenario = ScenarioBuilder::new()
        .with_simple_header("Integration Test", "Developer")
        .with_default_catalogs().unwrap()
        .with_road_network("test.xodr")
        .with_entities()
        .with_storyboard()
            .add_init_action()
                .teleport("test_vehicle")
                .to_position().world(0.0, 0.0, 0.0)
                .finish_action().unwrap()
                .finish_init()
            .add_story("test_story")
                .add_act("test_act")
                    .add_maneuver_group("test_group")
                        .add_maneuver("test_maneuver")
                            .add_event("test_event")
                                .add_action()
                                    .teleport("test_vehicle")
                                    .to_position().world(10.0, 0.0, 0.0)
                                    .finish_action().unwrap()
                                    .finish_action()
                                .with_start_trigger()
                                    .simulation_time()
                                    .greater_than(1.0)
                                    .finish_condition().unwrap()
                                    .finish_trigger().unwrap()
                                .finish_event().unwrap()
                            .finish_maneuver().unwrap()
                        .finish_maneuver_group().unwrap()
                    .finish_act().unwrap()
                .finish_story().unwrap()
            .finish_storyboard().unwrap()
        .build()
        .expect("Should build complete scenario");

    // Verify complete structure
    assert!(scenario.entities.is_some());
    assert!(scenario.storyboard.is_some());
    
    let storyboard = scenario.storyboard.unwrap();
    assert!(storyboard.stories.len() > 0);
}

#[test]
fn test_trigger_builder() {
    use openscenario_rs::builder::triggers::TriggerBuilder;
    
    let trigger = TriggerBuilder::new()
        .simulation_time()
        .greater_than(2.0)
        .finish_condition().unwrap()
        .finish()
        .expect("Should build trigger");
    
    assert_eq!(trigger.condition_groups.len(), 1);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
}

#[test]
fn test_event_builder() {
    use openscenario_rs::builder::events::EventBuilder;
    use openscenario_rs::types::enums::Priority;
    
    let event = EventBuilder::new()
        .name("test_event")
        .priority(Priority::Overwrite)
        .add_action()
            .name("test_action")
            .teleport("ego")
            .to_position().world(0.0, 0.0, 0.0)
            .finish_action().unwrap()
            .finish_action()
        .finish()
        .expect("Should build event");
    
    assert_eq!(event.name.value(), "test_event");
    assert_eq!(event.priority, Some(Priority::Overwrite));
}

#[test]
fn test_storyboard_builder() {
    use openscenario_rs::builder::storyboard::StoryboardBuilder;
    
    let storyboard = StoryboardBuilder::new()
        .add_init_action()
            .teleport("ego")
            .to_position().world(0.0, 0.0, 0.0)
            .finish_action().unwrap()
            .finish_init()
        .add_story("main_story")
            .add_act("main_act")
                .add_maneuver_group("ego_group")
                    .add_actor("ego")
                    .add_maneuver("acceleration")
                        .add_event("start_acceleration")
                            .add_action()
                                .longitudinal("ego")
                                .speed_action()
                                .absolute_target(30.0)
                                .finish_action().unwrap()
                                .finish_action().unwrap()
                            .with_start_trigger()
                                .simulation_time()
                                .greater_than(1.0)
                                .finish_condition().unwrap()
                                .finish_trigger().unwrap()
                            .finish_event().unwrap()
                        .finish_maneuver().unwrap()
                    .finish_maneuver_group().unwrap()
                .finish_act().unwrap()
            .finish_story().unwrap()
        .finish_storyboard()
        .expect("Should build storyboard");
    
    assert_eq!(storyboard.stories.len(), 1);
    assert_eq!(storyboard.stories[0].acts.len(), 1);
}

#[test]
fn test_complex_trigger_logic() {
    use openscenario_rs::builder::triggers::TriggerBuilder;
    
    let trigger = TriggerBuilder::new()
        .add_condition_group()
            .simulation_time()
            .greater_than(2.0)
            .finish_condition_for_group().unwrap()
            .speed()
            .entity("ego")
            .greater_than(25.0)
            .finish_condition_for_group().unwrap()
            .finish_group()
        .finish()
        .expect("Should build complex trigger");
    
    // Should have one condition group with two conditions (AND logic)
    assert_eq!(trigger.condition_groups.len(), 1);
    assert_eq!(trigger.condition_groups[0].conditions.len(), 2);
}

#[test]
fn test_multiple_stories() {
    use openscenario_rs::builder::storyboard::StoryboardBuilder;
    
    let storyboard = StoryboardBuilder::new()
        .add_story("story1")
            .add_act("act1")
                .add_maneuver_group("group1")
                    .add_actor("ego")
                    .add_maneuver("maneuver1")
                        .add_event("event1")
                            .add_action()
                                .longitudinal("ego")
                                .speed_action()
                                .absolute_target(20.0)
                                .finish_action().unwrap()
                                .finish_action().unwrap()
                            .finish_event().unwrap()
                        .finish_maneuver().unwrap()
                    .finish_maneuver_group().unwrap()
                .finish_act().unwrap()
            .finish_story().unwrap()
        .add_story("story2")
            .add_act("act2")
                .add_maneuver_group("group2")
                    .add_actor("target")
                    .add_maneuver("maneuver2")
                        .add_event("event2")
                            .add_action()
                                .longitudinal("target")
                                .speed_action()
                                .absolute_target(15.0)
                                .finish_action().unwrap()
                                .finish_action().unwrap()
                            .finish_event().unwrap()
                        .finish_maneuver().unwrap()
                    .finish_maneuver_group().unwrap()
                .finish_act().unwrap()
            .finish_story().unwrap()
        .finish_storyboard()
        .expect("Should build storyboard with multiple stories");
    
    assert_eq!(storyboard.stories.len(), 2);
    assert_eq!(storyboard.stories[0].name.value(), "story1");
    assert_eq!(storyboard.stories[1].name.value(), "story2");
}

#[test]
fn test_validation_errors() {
    use openscenario_rs::builder::triggers::TriggerBuilder;
    use openscenario_rs::builder::events::EventBuilder;
    use openscenario_rs::builder::storyboard::StoryboardBuilder;
    
    // Test trigger without conditions
    let result = TriggerBuilder::new().finish();
    assert!(result.is_err());
    
    // Test event without action
    let result = EventBuilder::new().finish();
    assert!(result.is_err());
    
    // Test story without acts
    let result = StoryboardBuilder::new()
        .add_story("empty_story")
        .finish_story();
    assert!(result.is_err());
}
//! Basic test to verify ByEntityCondition works

use openscenario_rs::types::{
    conditions::{ByEntityCondition, EntityCondition},
    scenario::triggers::TriggeringEntities,
    basic::{Double, OSString},
    enums::Rule,
};

// Helper function to create default TriggeringEntities for tests
fn default_triggering_entities() -> TriggeringEntities {
    TriggeringEntities::default()
}

#[test]
fn test_by_entity_condition_basic() {
    // Test that we can create a basic ByEntityCondition
    let default_condition = ByEntityCondition::default();
    
    match default_condition.entity_condition {
        EntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(10.0));
            assert_eq!(speed.rule, Rule::GreaterThan);
            assert_eq!(speed.entity_ref, OSString::literal("DefaultEntity".to_string()));
        }
        _ => panic!("Expected default to be Speed condition"),
    }
}

#[test]
fn test_by_entity_condition_variants() {
    // Test that we can create a speed condition using the new API
    let triggering_entities = default_triggering_entities();
    let speed_condition = ByEntityCondition::speed(
        triggering_entities,
        25.0,
        Rule::GreaterThan,
        "test_vehicle"
    );
    
    match speed_condition.entity_condition {
        EntityCondition::Speed(_) => {
            // This should work
            assert!(true);
        }
        _ => panic!("Expected Speed condition"),
    }
    
    // Test that other variants exist by creating conditions with the new API
    let triggering_entities = default_triggering_entities();
    let _test_variants = |condition: ByEntityCondition| {
        match condition.entity_condition {
            EntityCondition::Speed(_) => {},
            EntityCondition::ReachPosition(_) => {},
            EntityCondition::Distance(_) => {},
            EntityCondition::RelativeDistance(_) => {},
            EntityCondition::Acceleration(_) => {},
            EntityCondition::StandStill(_) => {},
            EntityCondition::Collision(_) => {},
            EntityCondition::OffRoad(_) => {},
            EntityCondition::EndOfRoad(_) => {},
            EntityCondition::TimeHeadway(_) => {},
            EntityCondition::TimeToCollision(_) => {},
        }
    };
}
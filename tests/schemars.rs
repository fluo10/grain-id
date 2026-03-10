#![cfg(all(feature = "schemars", feature = "serde"))]

use caretta_id::CarettaId;
use schemars::{JsonSchema, SchemaGenerator, generate::SchemaSettings};

fn validate_jsonschema(id: CarettaId) {
    let schema = serde_json::Value::from(CarettaId::json_schema(&mut SchemaGenerator::new(SchemaSettings::openapi3())));
    let instance = serde_json::to_value(id).unwrap();

    jsonschema::validate(&schema, &instance).unwrap();
}

#[test]
fn validate_nil() {
    validate_jsonschema(CarettaId::NIL);
}

#[test]
fn validate_max() {
    validate_jsonschema(CarettaId::MAX);
}

#[test]
fn validate_random() {
    for _ in 0..16 {
        validate_jsonschema(CarettaId::random());
    }
}
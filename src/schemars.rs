use schemars::{JsonSchema, json_schema};

use crate::CarettaId;

impl JsonSchema for CarettaId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CarettaId".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        "caretta_id::CarettaId".into()
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "title": "CarettaId",
            "description": "Base32 encoded 7 digit ID",
            "type": "string",
            "minLength" : 7,
            "maxLength" : 7,
            "pattern": "^[a-zA-Z0-9]{7}$",
            "example": "0a1b2c3"
        })
    }
    fn inline_schema() -> bool {
        true
    }
}
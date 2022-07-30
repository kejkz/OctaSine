use super::{utils::*, ParameterValue};
use crate::common::OPERATOR_MOD_INDEX_STEPS;

#[derive(Debug, Clone, Copy)]
pub struct OperatorModOutValue(f32);

impl Default for OperatorModOutValue {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ParameterValue for OperatorModOutValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: String) -> Option<Self> {
        const MIN: f32 = OPERATOR_MOD_INDEX_STEPS[0];
        const MAX: f32 = OPERATOR_MOD_INDEX_STEPS[OPERATOR_MOD_INDEX_STEPS.len() - 1];

        parse_valid_f32(text, MIN, MAX).map(Self)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &OPERATOR_MOD_INDEX_STEPS[..],
            value,
        ))
    }
    fn to_patch(self) -> f32 {
        map_value_to_parameter_value_with_steps(&OPERATOR_MOD_INDEX_STEPS[..], self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}

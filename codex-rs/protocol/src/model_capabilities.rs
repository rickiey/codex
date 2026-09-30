use std::num::NonZeroU32;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::openai_models::ReasoningEffort;

/// User-supplied capabilities for one model ID, independent of the provider catalog.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelCapabilities {
    /// Context window in tokens. Unknown models retain the 272k fallback if omitted.
    pub context_window: Option<NonZeroU32>,
    /// Maximum tokens to request in a single Responses completion.
    pub max_output_tokens: Option<NonZeroU32>,
    /// Default effort, overridden by an explicit turn or global effort setting.
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Efforts offered in the model picker; omit to offer only the default effort.
    #[serde(default)]
    pub supported_reasoning_efforts: Vec<ReasoningEffort>,
}

use crate::cli::{OnMissingLanguageDefinition, OnMissingToolBinary};

pub mod tools;

pub const DEFAULT_TEST_FORMATTER_TIMEOUT: u64 = 0;

pub const DEFAULT_TEST_DEBUG_ENABLED: bool = true;

pub const DEFAULT_ON_MISSING_TOOL_BINARY: OnMissingToolBinary = OnMissingToolBinary::Ignore;

pub const DEFAULT_ON_MISSING_LANGUAGE_DEFINITION: OnMissingLanguageDefinition =
    OnMissingLanguageDefinition::Ignore;

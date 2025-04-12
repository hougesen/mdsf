use crate::cli::OnMissingToolBinary;

pub mod tools;

pub const DEFAULT_TEST_FORMATTER_TIMEOUT: u64 = 0;

pub const DEFAULT_TEST_DEBUG_ENABLED: bool = true;

pub const DEFAULT_ERROR_ON_MISSING_TOOL: OnMissingToolBinary = OnMissingToolBinary::Allow;

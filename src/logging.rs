//! Logging utilities for fs-chaot
//!
//! This module provides structured logging macros that work across both
//! server and client contexts. Logs include structured fields for better
//! filtering and debugging.

/// Macro to log server function entry with structured fields
///
/// # Examples
///
/// ```ignore
/// log_server_fn!("get_card_by_id", card_id = 123);
/// log_server_fn!("update_card", card_id = 25, owned = true);
/// ```
#[cfg(feature = "server")]
#[macro_export]
macro_rules! log_server_fn {
    ($fn_name:expr) => {
        tracing::info!(fn_name = $fn_name, "server function called");
    };
    ($fn_name:expr, $($key:tt = $value:expr),+ $(,)?) => {
        tracing::info!(fn_name = $fn_name, $($key = tracing::field::debug(&$value)),+, "server function called");
    };
}

/// Macro to log database operations with structured fields
///
/// # Examples
///
/// ```ignore
/// log_db_op!("INSERT", table = "cards", card_id = 123);
/// log_db_op!("UPDATE", table = "cards", card_id = 25, owned = true);
/// ```
#[cfg(feature = "server")]
#[macro_export]
macro_rules! log_db_op {
    ($op:expr, $($key:tt = $value:expr),+ $(,)?) => {
        tracing::debug!(operation = $op, $($key = tracing::field::debug(&$value)),+, "database operation");
    };
}

/// Macro to log card ownership changes
///
/// # Examples
///
/// ```ignore
/// log_ownership_change!(25, false, true);
/// ```
#[cfg(feature = "server")]
#[macro_export]
macro_rules! log_ownership_change {
    ($card_id:expr, $old_state:expr, $new_state:expr) => {
        tracing::info!(
            card_id = $card_id,
            old_owned = $old_state,
            new_owned = $new_state,
            "card ownership changed"
        );
    };
}

// Client-side stubs (no-op when not on server)
#[cfg(not(feature = "server"))]
#[macro_export]
macro_rules! log_server_fn {
    ($($tt:tt)*) => {};
}

#[cfg(not(feature = "server"))]
#[macro_export]
macro_rules! log_db_op {
    ($($tt:tt)*) => {};
}

#[cfg(not(feature = "server"))]
#[macro_export]
macro_rules! log_ownership_change {
    ($($tt:tt)*) => {};
}

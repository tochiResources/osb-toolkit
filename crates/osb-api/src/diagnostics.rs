//! The diagnostics library wire types, re-exported from `osb-runtime` (the
//! source of truth) so the contract, the paths and the generated SDK models keep
//! one definition.

pub use osb_runtime::diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticReport, DiagnosticSeverity,
};

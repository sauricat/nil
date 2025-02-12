mod base;
mod builtin;
mod def;
mod diagnostic;
mod ide;

#[cfg(test)]
mod tests;

pub use base::{Change, FileId, FilePos, FileRange, InFile};
pub use diagnostic::{Diagnostic, DiagnosticKind, Severity};
pub use ide::{
    Analysis, AnalysisHost, CompletionItem, CompletionItemKind, NavigationTarget, RootDatabase,
};

//! Object inheritance / `super` override precedence, plus object introspection
//! (`std.length`, `std.objectHas`). Each fixture is a self-contained jsonnet
//! program that evaluates to `true` via `std.assertEqual`. Mirrors `suite.rs`.
use std::path::PathBuf;

use jrsonnet_evaluator::{
    FileImportResolver, State, Val,
    trace::{CompactFormat, PathResolver, TraceFormat},
};
use jrsonnet_stdlib::ContextInitializer;

mod common;
use common::ContextInitializer as TestContextInitializer;

fn run_case(file: &str) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("object_override");
    path.push(file);

    let mut s = State::builder();
    s.context_initializer((
        ContextInitializer::new(PathResolver::new_cwd_fallback()),
        TestContextInitializer,
    ))
    .import_resolver(FileImportResolver::default());
    let s = s.build();

    let trace_format = CompactFormat::default();

    match s.import(&*path) {
        Ok(Val::Bool(true)) => {}
        Ok(Val::Bool(false)) => panic!("test {} returned false", path.display()),
        Ok(_) => panic!("test {} returned wrong type as result", path.display()),
        Err(e) => panic!(
            "test {} failed:\n{}",
            path.display(),
            trace_format.format(&e).unwrap()
        ),
    }
}

#[test]
fn child_field_overrides_super() {
    run_case("child_field_overrides_super.jsonnet");
}

#[test]
fn self_reference_uses_override() {
    run_case("self_reference_uses_override.jsonnet");
}

#[test]
fn disjoint_fields_merge() {
    run_case("disjoint_fields_merge.jsonnet");
}

#[test]
fn object_length() {
    run_case("object_length.jsonnet");
}

#[test]
fn object_has_field() {
    run_case("object_has_field.jsonnet");
}

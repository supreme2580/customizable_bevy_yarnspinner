//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cs>
//!
//! ## Implementation notes
//!
//! Because Rust has no concept of a current global culture setting, the test `TestCompilationShouldNotBeCultureDependent` was omitted.
//! The test `TestNumberPlurals` was moved to a unit test in the `runtime` crate because it fits better there.

#[cfg(feature = "bevy")]
use bevy::prelude::World;
use std::collections::HashMap;
use test_base::prelude::*;
use yarnspinner::compiler::*;
use yarnspinner::core::*;
use yarnspinner::runtime::*;

mod test_base;

#[test]
fn test_example_script() {
    let path = test_data_path().join("Example.yarn");
    let test_plan = path.with_extension("testplan");

    let result = Compiler::default().read_file(path).compile().unwrap();

    TestBase::default()
        .with_runtime_errors_do_not_cause_failure()
        .with_compilation(result)
        .read_test_plan(test_plan)
        .run_standard_testcase();
}

#[test]
fn can_compile_space_demo() {
    let test_base = TestBase::default();
    let sally_path = space_demo_scripts_path().join("Sally.yarn");
    let ship_path = space_demo_scripts_path().join("Ship.yarn");

    let _result_sally = Compiler::new()
        .read_file(&sally_path)
        .extend_library(test_base.dialogue.library().clone())
        .compile()
        .unwrap();
    let _result_sally_and_ship = Compiler::new()
        .read_file(&sally_path)
        .read_file(ship_path)
        .extend_library(test_base.dialogue.library().clone())
        .compile()
        .unwrap();
}

#[test]
#[should_panic]
fn test_merging_nodes() {
    let test_base = TestBase::default();
    let sally_path = space_demo_scripts_path().join("Sally.yarn");
    let ship_path = space_demo_scripts_path().join("Ship.yarn");

    let result_sally = Compiler::default()
        .read_file(&sally_path)
        .extend_library(test_base.dialogue.library().clone())
        .compile()
        .unwrap();
    let result_sally_and_ship = Compiler::default()
        .read_file(&sally_path)
        .read_file(ship_path)
        .extend_library(test_base.dialogue.library().clone())
        .compile()
        .unwrap();

    // Loading code with the same contents should throw
    let _combined_not_working = Program::combine(vec![
        result_sally.program.unwrap(),
        result_sally_and_ship.program.unwrap(),
    ]);
}

#[test]
fn test_end_of_notes_with_options_not_added() {
    let path = test_data_path().join("SkippedOptions.yarn");
    let result = Compiler::default().read_file(path).compile().unwrap();

    let mut dialogue = TestBase::default().with_compilation(result).dialogue;
    dialogue.set_node("Start").unwrap();

    #[cfg(feature = "bevy")]
    let mut world = World::default();

    let mut has_options = false;
    while dialogue.can_continue() {
        #[cfg(feature = "bevy")]
        let events = dialogue.continue_with_world(&mut world);
        #[cfg(not(feature = "bevy"))]
        let events = dialogue.continue_();
        let events =
            events.unwrap_or_else(|e| panic!("Encountered error while running dialogue: {e}"));
        if events
            .iter()
            .any(|event| matches!(event, DialogueEvent::Options(_)))
        {
            has_options = true;
            break;
        }
    }
    assert!(!has_options);
}

#[test]
fn test_node_headers() {
    let path = test_data_path().join("Headers.yarn");
    let result = Compiler::default().read_file(&path).compile().unwrap();
    let program = result.program.as_ref().unwrap();
    assert_eq!(program.nodes.len(), 6);

    for tag in &["one", "two", "three"].map(|s| s.to_owned()) {
        assert!(program.nodes["Tags"].tags.contains(tag));
    }

    let headers: HashMap<_, _> = vec![
        ("EmptyTags", vec![("title", "EmptyTags"), ("tags", "")]),
        (
            "ArbitraryHeaderWithValue",
            vec![
                ("title", "ArbitraryHeaderWithValue"),
                ("arbitraryheader", "some-arbitrary-text"),
            ],
        ),
        ("Tags", vec![("title", "Tags"), ("tags", "one two three")]),
        ("SingleTagOnly", vec![("title", "SingleTagOnly")]),
        (
            "Comments",
            vec![("title", "Comments"), ("tags", "one two three")],
        ),
        (
            "LotsOfHeaders",
            vec![
                ("contains", "lots"),
                ("title", "LotsOfHeaders"),
                ("this", "node"),
                ("of", ""),
                ("blank", ""),
                ("others", "are"),
                ("headers", ""),
                ("some", "are"),
                ("not", ""),
            ],
        ),
    ]
    .into_iter()
    .collect();
    assert_eq!(program.nodes.len(), headers.len());
    for (node_name, expected_headers) in headers {
        let node = &program.nodes[node_name];
        assert_eq!(node.headers.len(), expected_headers.len());
        for header in &node.headers {
            let expected_header = expected_headers
                .iter()
                .find(|(k, _)| k == &header.key)
                .unwrap();
            assert_eq!(header.value, expected_header.1);
        }
    }

    let path = path.to_string_lossy().to_string();

    assert!(result.file_tags.contains_key(&path));
    assert_eq!(1, result.file_tags.len());
    assert!(result.file_tags[&path].contains(&"file_header".to_owned()));
    assert_eq!(1, result.file_tags[&path].len());
}

#[test]
fn test_invalid_characters_in_node_title() {
    let path = test_data_path().join("InvalidNodeTitle.yarn");
    let result = Compiler::default().read_file(path).compile();
    assert!(result.is_err());
}

#[test]
fn test_sources() {
    for file in [
        "TestCases",
        "TestCases/ParseFailures",
        // ## Implementation note: this directory does not exist
        // "Issues"
    ]
    .iter()
    .flat_map(TestBase::file_sources)
    {
        println!("INFO: Loading file {}", file.display());
        let path = test_data_path().join(&file);
        let test_plan = path.with_extension("testplan");

        let test_base = TestBase::default().extend_library(|library| {
            library.add_function("add_three_operands", |a: i32, b: i32, c: i32| a + b + c);
        });
        let result = Compiler::default()
            .read_file(&path)
            .extend_library(test_base.dialogue.library().clone())
            .compile();

        if !test_plan.exists() {
            // No test plan for this file exists, which indicates that
            // the file is not expected to compile. We'll actually make
            // it a test failure if it _does_ compile.
            assert!(
                result.is_err(),
                "{} is expected to have compile errors",
                file.display()
            );
        } else {
            let compilation = result.unwrap();
            let mut test_base = test_base
                .read_test_plan(test_plan)
                .with_compilation(compilation)
                .extend_library(|library| {
                    library
                        .add_function("dummy_bool", || true)
                        .add_function("dummy_number", || 1)
                        .add_function("dummy_string", || "string".to_owned());
                });

            // If this file contains a Start node, run the test case
            // (otherwise, we're just testing its parseability, which
            // we did in the last line)
            if test_base.dialogue.node_exists("Start") {
                test_base.run_standard_testcase();
            }
        }
    }
}

#[test]
#[should_panic]
fn crashes_on_command_expression_evaluating_whitespace() {
    let result = Compiler::from_test_source("<<{0} {\"   \"}>>")
        .compile()
        .unwrap();
    TestBase::new()
        .with_compilation(result)
        .run_standard_testcase();
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;
use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::verdict::Verdict;
use crap4rust::traits::reporter::Reporter;
use crap4rust_cli::system::json_reporter::JsonReporter;
use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string;

fn compact(rendered: &str) -> String {
    to_string(&from_str::<Value>(rendered).expect("valid JSON")).expect("serialize")
}

fn report_of(function: FunctionReport) -> ProjectReport {
    ProjectReport::new(
        String::from("test"),
        1,
        0,
        0.0,
        Verdict::Clean,
        vec![function],
    )
}

fn sample() -> FunctionReport {
    FunctionReport {
        package_name: String::from("cargo-crap4rust"),
        name: String::from("compute_crap_score"),
        relative_file: String::from("src/app.rs"),
        line: 42,
        complexity: 5,
        coverage: 0.5,
        crap_score: 22.5,
        verdict: Verdict::Warn,
    }
}

#[test]
fn render_an_empty_report_is_its_pretty_json_and_a_newline() {
    // Arrange
    let reporter = JsonReporter::new();
    let report = ProjectReport::new(String::from("test"), 0, 0, 0.0, Verdict::Clean, vec![]);

    // Act
    let output = reporter.render(&report, &Config::default());

    // Assert
    assert_eq!(
        output,
        concat!(
            "{\n",
            "  \"scope_name\": \"test\",\n",
            "  \"total_functions\": 0,\n",
            "  \"crappy_functions\": 0,\n",
            "  \"crappy_percent\": 0.0,\n",
            "  \"verdict\": \"Clean\",\n",
            "  \"functions\": []\n",
            "}\n",
        )
    );
}

#[test]
fn render_serializes_all_fields_of_a_function_to_json() {
    // Arrange
    let report = report_of(sample());

    // Act
    let json = compact(&JsonReporter::new().render(&report, &Config::default()));

    // Assert
    assert!(json.contains(r#""package_name":"cargo-crap4rust""#));
    assert!(json.contains(r#""name":"compute_crap_score""#));
    assert!(json.contains(r#""relative_file":"src/app.rs""#));
    assert!(json.contains(r#""line":42"#));
    assert!(json.contains(r#""complexity":5"#));
    assert!(json.contains(r#""coverage":0.5"#));
    assert!(json.contains(r#""crap_score":22.5"#));
    assert!(json.contains(r#""verdict":"Warn""#));
}

#[test]
fn render_serializes_the_reports_own_fields() {
    // Arrange
    let report = report_of(FunctionReport {
        package_name: String::from("test"),
        name: String::from("foo"),
        relative_file: String::from("src/lib.rs"),
        line: 10,
        complexity: 1,
        coverage: 1.0,
        crap_score: 1.0,
        verdict: Verdict::Clean,
    });

    // Act
    let json = compact(&JsonReporter::new().render(&report, &Config::default()));

    // Assert
    assert!(json.contains(r#""scope_name":"test""#));
    assert!(json.contains(r#""verdict":"Clean""#));
    assert!(json.contains(r#""name":"foo""#));
}

#[test]
fn render_writes_a_verdict_as_its_pascal_case_variant_name() {
    // Arrange
    let report = report_of(sample());

    // Act
    let rendered = JsonReporter::new().render(&report, &Config::default());

    // Assert
    let document = from_str::<Value>(&rendered).expect("valid JSON");
    let json = to_string(&document["functions"][0]["verdict"]).expect("serialize verdict");
    assert_eq!(json, "\"Warn\"");
}

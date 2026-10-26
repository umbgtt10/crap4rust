// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crap4rust::invocation::config::Config;
use crap4rust::reporting::function_report::FunctionReport;
use crap4rust::reporting::human_reporter::HumanReporter;
use crap4rust::reporting::project_report::ProjectReport;
use crap4rust::reporting::verdict::Verdict;
use crap4rust::traits::reporter::Reporter;

fn test_config() -> Config {
    Config::default()
        .with_threshold(30.0)
        .with_warn_threshold(20.0)
        .with_project_threshold(5.0)
        .with_strict(true)
}

#[test]
fn render_with_crappy_functions_shows_table() {
    // Arrange
    let reporter = HumanReporter::new();
    let report = ProjectReport::new(
        String::from("test"),
        1,
        1,
        100.0,
        Verdict::Crappy,
        vec![FunctionReport {
            package_name: String::from("pkg"),
            name: String::from("risky_fn"),
            relative_file: String::from("src/lib.rs"),
            line: 10,
            complexity: 10,
            crap_score: 110.0,
            verdict: Verdict::Crappy,
            ..Default::default()
        }],
    );
    let config = test_config();

    // Act
    let output = reporter.render(&report, &config);

    // Assert
    assert_eq!(
        output,
        concat!(
            "crap4rust report for test\n",
            "\n",
            "package  name      file        line  complexity    coverage        crap  verdict\n",
            "-------  --------  ----------  ----  ----------  ----------  ----------  -------\n",
            "pkg      risky_fn  src/lib.rs    10          10        0.0%       110.0  crappy \n",
            "\n",
            "summary: total_functions=1 crappy_functions=1 crappy_percent=100.0% threshold=30.0 project_threshold=5.0% verdict=crappy\n",
        )
    );
}

#[test]
fn render_with_no_clean_functions_shows_threshold_message() {
    // Arrange
    let reporter = HumanReporter::new();
    let report = ProjectReport::new(String::from("test"), 0, 0, 0.0, Verdict::Clean, vec![]);
    let config = test_config();

    // Act
    let output = reporter.render(&report, &config);

    // Assert
    assert_eq!(
        output,
        concat!(
            "crap4rust report for test\n",
            "\n",
            "No functions at or above the threshold of 30.0.\n",
            "\n",
            "summary: total_functions=0 crappy_functions=0 crappy_percent=0.0% threshold=30.0 project_threshold=5.0% verdict=clean\n",
        )
    );
}

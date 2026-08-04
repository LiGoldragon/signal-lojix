use std::process::Command;

#[test]
fn default_dependency_tree_does_not_pull_text_or_retired_signal_crates() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    for forbidden_crate in ["nota", "nota-codec", "signal-core"] {
        assert!(
            !tree.contains(forbidden_crate),
            "default dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}

#[test]
fn dotos_text_feature_is_the_only_text_projection_opt_in() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "dotos-text",
        ])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    assert!(
        tree.contains("dotos"),
        "dotos-text feature should opt into dotos:\n{tree}"
    );
    for forbidden_crate in ["nota-codec", "signal-core"] {
        assert!(
            !tree.contains(forbidden_crate),
            "dotos-text dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}

use std::path::PathBuf;

use snapbox::{
    cmd::{cargo_bin, Command},
    data::Data,
    utils::current_dir,
};

fn fp(paths: &[&str], use_current_dir: bool) -> String {
    let path = if use_current_dir {
        current_dir!()
    } else {
        PathBuf::new().join("tests")
    };

    paths
        .iter()
        .fold(path.join("fixtures").join("lint"), |acc, p| acc.join(p))
        .to_string_lossy()
        .to_string()
}

fn run_on_fixture(path: &str, args: &[&str], fail: bool) {
    let plan = fp(&[path], true);
    let snapshot = current_dir!().join("snapshots").join("lint").join(path);

    let assert = Command::new(cargo_bin!("ods"))
        .args(["--color", "always", "lint"])
        .arg(plan)
        .args(args)
        .assert();

    let assert = if fail {
        assert.failure()
    } else {
        assert.success()
    };

    assert
        .stderr_eq(Data::read_from(&snapshot.join("stderr.txt"), None))
        .stdout_eq(Data::read_from(&snapshot.join("stdout.txt"), None));
}

#[test]
fn non_existent() {
    run_on_fixture("non_existent", &[], true);
}

#[test]
fn empty() {
    run_on_fixture("empty.yaml", &[], true);
}

#[test]
fn change_levels() {
    run_on_fixture("change_levels", &[], true);
}

#[test]
fn files_non_existent() {
    run_on_fixture(
        "files_non_existent",
        &[&fp(&["non_existent.yaml"], false)],
        true,
    );
}

#[test]
fn files_outside_plan() {
    run_on_fixture(
        "files_outside_plan",
        &[&fp(&["files", "good.yaml"], false)],
        true,
    );
}

#[test]
fn files() {
    run_on_fixture("files", &[&fp(&["files", "good.yaml"], false)], false);
}

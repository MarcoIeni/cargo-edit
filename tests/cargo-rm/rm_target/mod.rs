use cargo_test_support::compare::assert_ui;
use cargo_test_support::Project;

use crate::init_registry;
use crate::CargoCommand;
use cargo_test_support::curr_dir;

#[cargo_test]
fn case() {
    init_registry();
    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("rm")
        .args(["--target", "x86_64-unknown-linux-gnu", "dbus"])
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}

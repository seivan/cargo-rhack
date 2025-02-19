use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

type TestResult<T> = Result<T, Box<dyn Error>>;

fn fixtures_dir() -> PathBuf {
    ["tests", "fixtures", "workspaces_test"].iter().collect()
}

#[test]
fn test_workspace_patching_passes() -> TestResult<()> {
    let fixture_path = fixtures_dir();

    let mut edit_runner = Command::new(env!("CARGO_BIN_EXE_cargo-rhack"));
    _ = edit_runner
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let edit_child = edit_runner
        .args(["rhack", "edit", "toml_edit"])
        .current_dir(fixture_path.canonicalize()?)
        .spawn()?;

    let _edit_output = edit_child.wait_with_output()?;

    // TODO: Test other conditions, e.g. if the directory has been created etc.
    // format of the table
    // for now it panics if patch section can't be created

    let mut undo_runner = Command::new(env!("CARGO_BIN_EXE_cargo-rhack"));
    _ = undo_runner
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let _undo_child = undo_runner
        .args(["rhack", "undo"])
        .current_dir(fixture_path.canonicalize()?)
        .spawn()?;

    // TODO: Test other conditions, e.g. if the directory has removed
    // diff against fixtures
    Ok(())
}

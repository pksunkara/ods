use std::{
    error::Error,
    path::{Path, PathBuf},
};

use snapbox::{
    cmd::{cargo_bin, Command},
    utils::current_dir,
    Data,
};
use tryfn::{Case, Harness};

fn setup(path: PathBuf) -> Case {
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let expected = Data::read_from(
        &current_dir!()
            .join("snapshots")
            .join("lint")
            .join("rules")
            .join(format!("{name}.txt")),
        None,
    );

    Case {
        name,
        fixture: path,
        expected,
    }
}

fn test(path: &Path) -> Result<Data, Box<dyn Error>> {
    let cmd = Command::new(cargo_bin!("ods"))
        .args(["--color", "always", "lint", "--no-fail"])
        .arg(path)
        .output()?;

    assert!(cmd.status.success());

    Ok(Data::from(cmd.stdout))
}

fn main() {
    Harness::new(current_dir!().join("fixtures").join("lint"), setup, test)
        .select(["rules/*"])
        .test();
}

use anyhow::Result;
use tempfile::tempdir;

#[test]
fn mycodex_does_not_read_codex_home() -> Result<()> {
    let codex_home = tempdir()?;
    let mycodex_home = tempdir()?;
    std::fs::write(codex_home.path().join("config.toml"), "this is not TOML")?;
    std::fs::write(
        mycodex_home.path().join("config.toml"),
        "model_provider = \"openai\"\n",
    )?;

    assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("mycodex")?)
        .env("CODEX_HOME", codex_home.path())
        .env("MYCODEX_HOME", mycodex_home.path())
        .arg("features")
        .arg("list")
        .assert()
        .success();
    Ok(())
}

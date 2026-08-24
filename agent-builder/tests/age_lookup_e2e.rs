use std::{
    env,
    fs,
    path::Path,
    process::Command,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

const FIXTURE_AGE: i64 = 37;

#[test]
#[ignore = "requires COPILOT_API_KEY and OPENAI_API_KEY for a live model request"]
fn age_lookup_returns_exact_json() {
    require_credentials();

    let fixture = materialize_fixture();
    let output = Command::new(env!("CARGO_BIN_EXE_agent-builder"))
        .current_dir(&fixture)
        .arg("How old is the person?")
        .arg("--file")
        .arg(fixture.join("files/person.txt"))
        .arg("--config")
        .arg(fixture.join("agent-builder.toml"))
        .output()
        .expect("agent-builder binary should start");

    assert!(
        output.status.success(),
        "agent-builder failed:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );

    let response = String::from_utf8(output.stdout)
        .expect("agent-builder response should be UTF-8");
    assert_eq!(response.trim(), r#"{"age":37}"#);

    let response: serde_json::Value =
        serde_json::from_str(response.trim()).expect("response should be JSON");
    let object = response
        .as_object()
        .expect("response should be a JSON object");
    assert_eq!(object.len(), 1, "response must not contain extra keys");
    assert!(object.contains_key("age"), "response must contain `age`");
    assert_eq!(
        object["age"].as_i64(),
        Some(FIXTURE_AGE),
        "`age` must be the fixture's integer age",
    );

    fs::remove_dir_all(&fixture).expect("temporary fixture should be removable");
}

fn require_credentials() {
    for variable in ["COPILOT_API_KEY", "OPENAI_API_KEY"] {
        assert!(
            env::var_os(variable).is_some(),
            "{variable} must be set to run the live age lookup E2E test",
        );
    }
}

fn materialize_fixture() -> std::path::PathBuf {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test-fixtures/age-lookup");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();
    let destination = env::temp_dir().join(format!("agent-builder-age-lookup-{unique}"));
    copy_dir_recursive(&source, &destination);
    destination
}

fn copy_dir_recursive(
    source: &Path,
    destination: &Path,
) {
    fs::create_dir_all(destination).expect("fixture destination should be created");
    for entry in fs::read_dir(source).expect("fixture source should be readable") {
        let entry = entry.expect("fixture directory entry should be readable");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path)
                .expect("fixture file should be copied");
        }
    }
}
use std::path::Path;

use anyhow::{
    Context,
    Result,
};

pub fn read_attachment(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| {
        format!("failed to read attachment {}", path.display())
    })
}

pub fn assemble_prompt(
    request: &str,
    attachment_path: &Path,
    attachment_contents: &str,
) -> String {
    let filename = attachment_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("attachment");

    format!(
        "Request:\n{request}\n\nAttached file: {filename}\n--- BEGIN ATTACHMENT ---\n{attachment_contents}\n--- END ATTACHMENT ---"
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::assemble_prompt;

    #[test]
    fn includes_request_filename_and_attachment_contents() {
        let prompt = assemble_prompt(
            "What is this person's age?",
            Path::new("fixtures/person.txt"),
            "Name: Ada\nAge: 37",
        );

        assert!(prompt.contains("What is this person's age?"));
        assert!(prompt.contains("person.txt"));
        assert!(prompt.contains("Name: Ada\nAge: 37"));
        assert!(prompt.contains("--- BEGIN ATTACHMENT ---"));
        assert!(prompt.contains("--- END ATTACHMENT ---"));
    }
}

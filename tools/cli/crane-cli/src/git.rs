use std::{
    ffi::OsStr,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

use crate::CraneError;

#[derive(Debug, Clone)]
pub struct GitRepo {
    path: PathBuf,
}

impl GitRepo {
    pub fn open(path: &Path) -> Result<Self, CraneError> {
        let canonical = path.canonicalize().map_err(|error| {
            CraneError::BadRequest(format!(
                "failed to resolve repository path {}: {error}",
                path.display()
            ))
        })?;
        Ok(Self { path: canonical })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn assert_repo(&self) -> Result<(), CraneError> {
        let output = self
            .command(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map_err(CraneError::Io)?;
        if output.status.success() {
            Ok(())
        } else {
            Err(CraneError::BadRequest(format!(
                "{} is not a git repository",
                self.path.display()
            )))
        }
    }

    pub fn command<I, S>(
        &self,
        args: I,
    ) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new("git");
        command.current_dir(&self.path).args(args);
        command
    }

    pub fn output_text<I, S>(
        &self,
        args: I,
    ) -> Result<String, CraneError>
    where
        I: Clone + IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = self.command(args.clone());
        let output = command.output().map_err(CraneError::Io)?;
        if !output.status.success() {
            return Err(CraneError::CommandFailed {
                cwd: self.path.clone(),
                command: format_command(args),
                status: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr)
                    .trim()
                    .to_string(),
            });
        }

        String::from_utf8(output.stdout).map_err(CraneError::Utf8)
    }

    pub fn run<I, S>(
        &self,
        args: I,
    ) -> Result<(), CraneError>
    where
        I: Clone + IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = self.command(args.clone());
        let status = command.status().map_err(CraneError::Io)?;
        if status.success() {
            Ok(())
        } else {
            Err(CraneError::CommandFailed {
                cwd: self.path.clone(),
                command: format_command(args),
                status: status.code(),
                stderr: String::new(),
            })
        }
    }

    pub fn rev_parse(
        &self,
        spec: &str,
    ) -> Result<String, CraneError> {
        Ok(self
            .output_text(["rev-parse", spec])?
            .lines()
            .next()
            .unwrap_or_default()
            .trim()
            .to_string())
    }

    pub fn first_touching_commit(
        &self,
        source_ref: &str,
        paths: &[String],
    ) -> Result<Option<String>, CraneError> {
        let mut args = vec![
            "rev-list".to_string(),
            "--reverse".to_string(),
            source_ref.to_string(),
            "--".to_string(),
        ];
        args.extend(paths.iter().cloned());

        let output = self.output_text(args)?;
        Ok(output
            .lines()
            .find(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string()))
    }

    pub fn is_clean(&self) -> Result<bool, CraneError> {
        Ok(self.output_text(["status", "--short"])?.trim().is_empty())
    }

    pub fn checkout(
        &self,
        branch: &str,
    ) -> Result<(), CraneError> {
        self.run(["checkout", branch])
    }

    pub fn merge_allow_unrelated(
        &self,
        branch: &str,
        message: &str,
    ) -> Result<(), CraneError> {
        self.run([
            "merge",
            "--allow-unrelated-histories",
            "--no-ff",
            "-m",
            message,
            branch,
        ])
    }

    pub fn delete_ref_if_exists(
        &self,
        ref_name: &str,
    ) -> Result<(), CraneError> {
        let status = self
            .command(["update-ref", "-d", ref_name])
            .status()
            .map_err(CraneError::Io)?;
        if status.success() || status.code() == Some(1) {
            Ok(())
        } else {
            Err(CraneError::CommandFailed {
                cwd: self.path.clone(),
                command: format!("git update-ref -d {ref_name}"),
                status: status.code(),
                stderr: String::new(),
            })
        }
    }
}

pub fn normalize_branch_ref(branch: &str) -> String {
    if branch.starts_with("refs/") {
        branch.to_string()
    } else {
        format!("refs/heads/{branch}")
    }
}

fn format_command<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut rendered = String::from("git");
    for arg in args {
        rendered.push(' ');
        rendered.push_str(&arg.as_ref().to_string_lossy());
    }
    rendered
}

use crate::version::update::VersionUpdate;
use crate::SheepError;
use std::path::PathBuf;

pub struct ScriptRunner {
    working_dir: PathBuf,
    version_update: VersionUpdate,
}

impl ScriptRunner {
    pub fn new(working_dir: PathBuf, version_update: VersionUpdate) -> Self {
        Self {
            working_dir,
            version_update,
        }
    }

    pub fn run(&self, script: Option<String>, script_name: &str) -> Result<(), SheepError> {
        if script.is_none() {
            // Skip it empty
            return Ok(());
        }

        println!("📜 Running script {script_name}");

        let mut command = std::process::Command::new("sh");
        command
            .arg("-c")
            .arg(script.unwrap())
            .current_dir(&self.working_dir);

        // Set environment variables
        command.env(
            "SHEEPIT_CURRENT_VERSION",
            self.version_update.current_version.to_string(),
        );
        command.env(
            "SHEEPIT_NEXT_VERSION",
            self.version_update.next_version.to_string(),
        );

        let output = command.output()?;

        if !output.status.success() {
            let error_message = format!("Failed to run script {}", script_name);
            return Err(SheepError::new(&error_message));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_script() {
        let working_dir = PathBuf::from("/tmp");
        let version_update = VersionUpdate::new("1.0.0", "1.0.1");
        let script = Some("echo 'Hello, world!'".to_string());
        let script_name = "test_script";
        let runner = ScriptRunner::new(working_dir, version_update);
        let result = runner.run(script, script_name);
        assert!(result.is_ok());
    }
}

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub pid: u32,
    pub cwd: String,
    pub command: String,
    pub destination: String,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllProcesses {
    #[serde(default)]
    pub process: Vec<Process>,
}

fn processes_path() -> Result<PathBuf, String> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| "could not determine home directory".to_string())?;
    Ok(home_dir.join(".config").join("pesn").join("processes.toml"))
}

impl AllProcesses {
    pub fn load() -> Result<AllProcesses, String> {
        let path = processes_path()?;
        if !path.exists() {
            return Ok(AllProcesses { process: vec![] });
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read processes file: {e}"))?;
        let processes: AllProcesses =
            toml::from_str(&content).map_err(|e| format!("failed to parse processes file: {e}"))?;
        Ok(processes)
    }

    pub fn get_process_info(&self, pid: &u32) -> Option<&Process> {
        self.process.iter().find(|p| &p.pid == pid)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = processes_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create processes directory: {e}"))?;
        }
        let content =
            toml::to_string(self).map_err(|e| format!("failed to serialize processes: {e}"))?;
        std::fs::write(&path, content)
            .map_err(|e| format!("failed to write processes file: {e}"))?;
        Ok(())
    }
}

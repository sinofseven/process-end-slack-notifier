use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct ProcessInfo {
    pub command: String,
    pub cwd: PathBuf,
}

pub fn resolve_process(pid: u32) -> Result<ProcessInfo, String> {
    let raw =
        fs::read(format!("/proc/{pid}/cmdline")).map_err(|e| format!("fs::read failed: {e}"))?;
    if raw.is_empty() {
        return Err("resolve_process failed: empty cmdline (kernel thread or zombie)".to_string());
    }
    let trimmed = if raw.last() == Some(&0) {
        &raw[..raw.len() - 1]
    } else {
        &raw
    };
    let command = trimmed
        .split(|&b| b == 0)
        .map(|s| String::from_utf8_lossy(s))
        .collect::<Vec<_>>()
        .join(" ");

    let cwd = fs::read_link(format!("/proc/{pid}/cwd"))
        .map_err(|e| format!("fs::read_link failed: {e}"))?;

    Ok(ProcessInfo { command, cwd })
}

pub fn find_terminated_pids(pids: &[u32]) -> Vec<u32> {
    pids.iter()
        .copied()
        .filter(|&pid| !Path::new(&format!("/proc/{pid}")).exists())
        .collect()
}

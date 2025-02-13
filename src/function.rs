use std::io::Write;

pub fn add_log<T>(path: &str, status: &str, message: &str, time: &str, warning: Option<T>) -> std::io::Result<()> {
    match warning {
        Some(_) => {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(path)?;
            file.write_all(format!("\n\n\n[S: {}  T: {}]  M:  {}\n\n\n", status, time, message).to_uppercase().as_bytes())?;
        }
        None => {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(path)?;
            file.write_all(format!("[S: {}  T: {}]  M:  {}\n", status, time, message).as_bytes())?;
        }
    }

    Ok(())
}
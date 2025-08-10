use core::fmt;
use std::{
    error::Error,
    fs,
    path::PathBuf,
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use daemonize::Daemonize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaffeineSession {
    pub proccess_id: String,
    pub start_time: u64,
    pub session_length: Option<u64>,
}

#[derive(Debug)]
pub enum SessionError {
    ConflictingSession,
    NoActiveSession,
}

impl Error for SessionError {}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionError::ConflictingSession => write!(f, "A session already exists"),
            SessionError::NoActiveSession => write!(f, "Couldn't find an active session"),
        }
    }
}

/// The session path. **/tmp/caffeine-session.json**
pub fn get_session_path() -> PathBuf {
    PathBuf::from("/tmp/caffeine-session.json")
}

/// Inits a caffeine session.
pub fn init_session(seconds: Option<u64>) -> Result<CaffeineSession, Box<dyn Error>> {
    let seconds_str = if let Some(seconds) = seconds {
        seconds.to_string()
    } else {
        String::from("infinity")
    };

    let process = Command::new("systemd-inhibit")
        .arg("--what=idle")
        .arg("sleep")
        .arg(seconds_str)
        .spawn()?;

    let process_id = process.id();

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    Ok(CaffeineSession {
        proccess_id: process_id.to_string(),
        start_time,
        session_length: if let Some(seconds) = seconds {
            Some(seconds)
        } else {
            None
        },
    })
}

/// Inits a protected caffeine session (recommended). It writes a file so that no other session can be also active
pub fn init_protected_session(seconds: Option<u64>) -> Result<CaffeineSession, Box<dyn Error>> {
    let session = get_session();

    if session.is_some() {
        return Err(Box::new(SessionError::ConflictingSession));
    }

    let new_session = init_session(seconds)?;

    let json = serde_json::to_string(&new_session)?;

    fs::write(get_session_path(), &json)?;

    if let Some(seconds) = seconds {
        let daemon = Daemonize::new();

        if let Ok(_) = daemon.start() {
            thread::sleep(Duration::from_secs(seconds));

            let _ = fs::remove_file(get_session_path());
        }
    }

    Ok(new_session)
}

/// Ends the given session
pub fn end_session(session: CaffeineSession) -> Result<(), Box<dyn Error>> {
    Command::new("kill")
        .arg(&session.proccess_id)
        .spawn()
        .expect("Error killing caffeine session");

    Ok(())
}

pub fn end_protected_session() -> Result<(), Box<dyn Error>> {
    let session = get_session();

    if session.is_none() {
        return Err(Box::new(SessionError::NoActiveSession));
    }

    end_session(session.unwrap())?;

    fs::remove_file(get_session_path())?;

    Ok(())
}

/// Get the active session
pub fn get_session() -> Option<CaffeineSession> {
    let session_json = fs::read_to_string(get_session_path());

    if session_json.is_err() {
        return None;
    }

    let session = serde_json::from_str(&session_json.unwrap());

    if session.is_err() {
        return None;
    }

    Some(session.unwrap())
}

impl CaffeineSession {
    pub fn get_elapsed_time(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let elapsed = UNIX_EPOCH + Duration::from_secs(now - self.start_time);

        let datetime: DateTime<Utc> = elapsed.into();

        datetime.format("%Hh %Mm %Ss").to_string()
    }

    pub fn get_session_length(&self) -> Option<String> {
        return if let Some(session_length) = self.session_length {
            let timestamp = UNIX_EPOCH + Duration::from_secs(session_length);

            let datetime: DateTime<Utc> = timestamp.into();

            Some(datetime.format("%Hh %Mm %Ss").to_string())
        } else {
            None
        };
    }

    pub fn get_remaining_time(&self) -> Option<String> {
        return if let Some(session_length) = self.session_length {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let timestamp =
                UNIX_EPOCH + Duration::from_secs((session_length + self.start_time) - now);

            let datetime: DateTime<Utc> = timestamp.into();

            Some(datetime.format("%Hh %Mm %Ss").to_string())
        } else {
            None
        };
    }
}

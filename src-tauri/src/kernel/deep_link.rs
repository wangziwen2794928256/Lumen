use crate::database::{DbError, Result};
// Parse the raw string, not a URL normalizer: percent encoding, path traversal and query strings are rejected.
pub fn route(value: &str) -> Result<String> {
    let body = value
        .strip_prefix("lumen://")
        .ok_or_else(DbError::invalid)?;
    // Windows Shell normalizes authority-only URLs by adding a root slash.
    if matches!(body, "today" | "today/") {
        return Ok("/".into());
    }
    if matches!(body, "calendar" | "calendar/") {
        return Ok("/calendar".into());
    }
    let (kind, id) = body.split_once('/').ok_or_else(DbError::invalid)?;
    match kind {
        "journal" => {
            let date = chrono::NaiveDate::parse_from_str(id, "%Y-%m-%d")
                .map_err(|_| DbError::invalid())?;
            if date.format("%Y-%m-%d").to_string() != id {
                return Err(DbError::invalid());
            }
            Ok(format!("/journal?date={id}"))
        }
        "artifact" | "journey" => {
            let uuid = uuid::Uuid::parse_str(id).map_err(|_| DbError::invalid())?;
            if uuid.to_string() != id {
                return Err(DbError::invalid());
            }
            Ok(if kind == "artifact" {
                format!("/vault?artifact={id}")
            } else {
                format!("/roadmap?journey={id}")
            })
        }
        _ => Err(DbError::invalid()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_routes_only() {
        for value in [
            "lumen://today",
            "lumen://today/",
            "lumen://calendar",
            "lumen://calendar/",
            "lumen://journal/2026-10-08",
            "lumen://artifact/00000000-0000-4000-8000-000000000001",
            "lumen://journey/00000000-0000-4000-8000-000000000001",
        ] {
            assert!(route(value).is_ok());
        }
        for value in [
            "file:///lumen.db",
            "lumen://today//",
            "lumen://calendar//",
            "lumen://today?path=C:/secret",
            "lumen://artifact/../../lumen.db",
            "lumen://journal/2026-02-30",
            "lumen://journal/2026-1-1",
            "lumen://artifact/%2e%2e",
            "lumen://today#x",
            "lumen://calendar\n",
            "LUMEN://today",
            "lumen://journey/00000000000040008000000000000001",
        ] {
            assert!(route(value).is_err(), "{value}");
        }
    }
}

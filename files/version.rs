//! Static build info for Powermill Autodesk.

pub const BUILD_VERSION: &str  = "v2.1.4";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "058304247a88";
pub const BUILD_TIMESTAMP: u64 = 1791855191;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Powermill Autodesk", BUILD_VERSION, BUILD_CHANNEL)
}

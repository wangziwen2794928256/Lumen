use super::Adapter;
// Boundary prepared, native NSWorkspace/session notifications deferred.
pub const ADAPTER: Adapter = Adapter {
    name: "macos",
    session_events: false,
    attachment_viewer: false,
};

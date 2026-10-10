use super::Adapter;
// Boundary prepared, logind/portal implementation deferred.
pub const ADAPTER: Adapter = Adapter {
    name: "linux",
    session_events: false,
    attachment_viewer: false,
};

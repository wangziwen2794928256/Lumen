pub mod official {
    pub mod music {
        pub fn initialize(app: &tauri::AppHandle, db: crate::database::Database) {
            crate::music::initialize(app, db);
        }
    }
    pub mod insights {
        // One migrated Official Module; stable domain implementation stays in place.
        pub fn initialize(db: crate::database::Database) {
            crate::insights::spawn_monthly(db);
        }
    }
}

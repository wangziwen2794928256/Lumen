export interface DesktopPreferences {
  close_to_tray: boolean;
  notifications_enabled: boolean;
  focus_finished: boolean;
  upcoming_event: boolean;
  weekly_review: boolean;
  backup_failure: boolean;
  revision: number;
}
export interface DesktopStatus {
  preferences: DesktopPreferences;
  autostart: boolean | null;
  notification_permission: string;
  runtime: { tray_ready: boolean; error_code: string | null };
  onboarding: boolean;
}

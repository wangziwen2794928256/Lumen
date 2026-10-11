# English language pack and localization

Chinese remains the default, with English added as en. Resources live in `src/locales/en/messages.json`, lookup/persistence in `src/lib/locale.ts` and the picker in `src/features/appearance/LanguagePanel.tsx`. Existing static Chinese copy is the source key. translate maps developer-authored interface literals only, never database bodies. Navigation selects Manifest.name[locale].

## Use

Settings → Appearance → Interface language. Save edits, select English and apply/reload. localStorage key lumen.locale.v1 accepts en/zh-CN. Invalid values or read failure fall back to Chinese. Save failure stays visible. Language changes do not alter SQLite, schema or user text. Reload happens through an explicit action rather than partially changing mounted copy. Save work first.

## Contributing translations

Call translate('Chinese source copy') for new static UI and add the matching English JSON entry. Never pass user.title or entry.body. Source changes must update resources to prevent silent Chinese fallback. Prefer complete sentence templates. Future interpolation/plurals can evolve separately rather than relying on grammatical concatenation. Another locale needs the Locale type, resource loading, picker, manifest name/schema, date formatting and tests.

## Verification

Units cover default/invalid values, persistence, English lookup and unchanged unknown user content. E2E covers switching both ways, reload persistence, 13 routes in two themes at three sizes, no Chinese interface remnants except language self-names, and no horizontal overflow. Empty browser states do not prove every native IPC error or OS dialog is localized.

## Coverage

The pack contains 905 existing distinct static frontend strings, plus bilingual picker copy. Records, filenames, exam sources and custom goals retain their original text. OS file pickers follow the system. Existing Rust tray/notification and some backend diagnostic messages remain Chinese. A future native locale service and stable error-code mapping should unify these. Complete system-wide language parity is not claimed.

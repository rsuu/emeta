const GROUP_COMIKEY: &str = "8d8ecf83-8d42-4f8c-add8-60963f9f28d9";
const GROUP_INKR: &str = "caa63201-4a17-4b7f-95ff-ed884a2b7e60";
const GROUP_MANGA_HOT: &str = "319c1b10-cbd0-4f55-a46e-c4ee17e65139";
const GROUP_MANGA_PLUS: &str = "4f1de6a2-f0c5-4ac5-bce5-02c7dbb67deb";

const BLOCKED_GROUPS_PREF: &str = "blockedGroups";
const BLOCKED_UPLOADER_PREF: &str = "blockedUploader";

const ALT_TITLES_IN_DESC_PREF: &str = "altTitlesInDesc";

const TRY_USING_FIRST_VOLUME_COVER_PREF: &str = "tryUsingFirstVolumeCover";
const TRY_USING_FIRST_VOLUME_COVER_DEFAULT: bool = false;

const HAS_SANITIZED_UUIDS_PREF: &str = "hasSanitizedUuids";

const TAG_GROUP_CONTENT: &str = "content";
const TAG_GROUP_FORMAT: &str = "format";
const TAG_GROUP_GENRE: &str = "genre";
const TAG_GROUP_THEME: &str = "theme";

const TAG_ANTHOLOGY_UUID: &str = "51d83883-4103-437c-b4b1-731cb73d786c";
const TAG_ONE_SHOT_UUID: &str = "0234a31e-a729-4e28-9d6a-3f87c4966b9e";

fn get_has_sanitized_uuids_pref_key(dex_lang: &str) -> String {
    format!("{}_{}", HAS_SANITIZED_UUIDS_PREF, dex_lang)
}

fn get_try_using_first_volume_cover_pref_key(dex_lang: &str) -> String {
    format!("{}_{}", TRY_USING_FIRST_VOLUME_COVER_PREF, dex_lang)
}

fn get_alt_titles_in_desc_pref_key(dex_lang: &str) -> String {
    format!("{}_{}", ALT_TITLES_IN_DESC_PREF, dex_lang)
}

fn get_blocked_groups_pref_key(dex_lang: &str) -> String {
    format!("{}_{}", BLOCKED_GROUPS_PREF, dex_lang)
}

fn get_blocked_uploader_pref_key(dex_lang: &str) -> String {
    format!("{}_{}", BLOCKED_UPLOADER_PREF, dex_lang)
}

//! Project links (Help menu, About, CLI, MCP). This build is a fork of storytold/designcraft; its
//! links point at the fork's repository.

use serde_json::{Value, json};

/// The app's slug on GitHub.
pub const APP: &str = "designcraft";
/// DesignCraft's source repository (this fork).
pub const GITHUB: &str = "https://github.com/685ks/designcraft";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/685ks/designcraft/issues";
/// The project this build is a fork of (named in the About window).
pub const UPSTREAM: &str = "https://github.com/storytold/designcraft";
/// The About window's provenance line. A legal notice, shown in English in every interface language.
pub const FORK_NOTE: &str = "A fork of storytold/designcraft, not affiliated with or endorsed by ArtCraft.";
/// Shown in the About window when the Arthavan Projects launcher started the app
/// (`ARTHAVAN_LAUNCHER=1`). DesignCraft has no updater of its own and makes no update requests; the
/// launcher installs and updates it from the 685ks/designcraft releases.
pub const LAUNCHER_UPDATES_NOTE: &str = "Updates are managed by Arthavan Projects";

/// Whether the Arthavan Projects launcher started this process (`ARTHAVAN_LAUNCHER=1`).
pub fn launcher_managed() -> bool {
    launcher_managed_from(std::env::var("ARTHAVAN_LAUNCHER").ok().as_deref())
}

fn launcher_managed_from(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.trim() == "1")
}

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(GITHUB.ends_with(&format!("/685ks/{APP}")));
        assert!(UPSTREAM.ends_with(&format!("/storytold/{APP}")));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
        assert_eq!(all().as_object().map(|o| o.len()), Some(2));
    }

    /// This fork carries no ArtCraft community or website links (house rebrand rule).
    #[test]
    fn no_link_points_at_artcraft_services() {
        for k in ["discord", "website", "appPage"] {
            assert!(get(k).is_none(), "{k}");
        }
        for url in [GITHUB, ISSUES, UPSTREAM] {
            for banned in ["artcraft", "discord", "storyteller.ai"] {
                assert!(!url.to_ascii_lowercase().contains(banned), "{url}");
            }
        }
    }

    #[test]
    fn launcher_flag_is_read_from_arthavan_launcher() {
        assert!(launcher_managed_from(Some("1")));
        assert!(!launcher_managed_from(None));
        assert!(!launcher_managed_from(Some("0")));
        assert!(!launcher_managed_from(Some("")));
    }
}

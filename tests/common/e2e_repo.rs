//! The GitHub repo the live suites (`e2e`, `parity`) run against.
//!
//! Defaults to the upstream testing repo. Forks point the suites at their own
//! throwaway repo with `JJPR_E2E_REPO=owner/repo`, and optionally
//! `JJPR_E2E_CLONE_URL` to clone over HTTPS (e.g. when only a gh credential
//! helper is configured).

use std::sync::OnceLock;

const DEFAULT_OWNER: &str = "michaeldhopkins";
const DEFAULT_REPO: &str = "forge-e2e-sandbox";

fn repo_slug() -> &'static (String, String) {
    static SLUG: OnceLock<(String, String)> = OnceLock::new();
    SLUG.get_or_init(|| match std::env::var("JJPR_E2E_REPO") {
        Ok(v) => {
            let (owner, repo) = v
                .split_once('/')
                .unwrap_or_else(|| panic!("JJPR_E2E_REPO must be 'owner/repo', got '{v}'"));
            (owner.to_string(), repo.to_string())
        }
        Err(_) => (DEFAULT_OWNER.to_string(), DEFAULT_REPO.to_string()),
    })
}

pub fn owner() -> &'static str {
    &repo_slug().0
}

pub fn repo() -> &'static str {
    &repo_slug().1
}

/// `owner/repo`, as `gh --repo` and the REST API paths take it.
pub fn full_repo() -> String {
    format!("{}/{}", owner(), repo())
}

/// Clone URL for the testing repo. Defaults to SSH.
pub fn clone_url() -> String {
    std::env::var("JJPR_E2E_CLONE_URL")
        .unwrap_or_else(|_| format!("git@github.com:{}.git", full_repo()))
}

//! Help → community and project links (Visualab Studios, the website, this app's page and its
//! GitHub repository). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// The app id.
pub const APP_ID: &str = "vectorcraft";
pub const DISCORD_URL: &str = "https://www.visualabstudios.com";
pub const WEBSITE_URL: &str = "https://www.visualabstudios.com";

/// This app's page on the website.
pub fn app_page_url() -> String {
    format!("{WEBSITE_URL}/vl-tools/vl-vector")
}

/// This app's source repository.
pub fn github_url() -> String {
    "https://github.com/wasigonzi/visualab-vector".to_string()
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Visit Visualab Studios", ["Help"], None, "{} → {url} Visualab Studios", always, |_, _| url(DISCORD_URL.into())),
        cmd!(query "help.website", "Visualab Studios Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "VL Vector on visualabstudios.com", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "VL Vector on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {discord, website, appPage, github}", always, |_, _| {
            Ok(json!({ "discord": DISCORD_URL, "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], "https://www.visualabstudios.com");
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://www.visualabstudios.com/vl-tools/vl-vector");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/wasigonzi/visualab-vector");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://www.visualabstudios.com");
    }
}

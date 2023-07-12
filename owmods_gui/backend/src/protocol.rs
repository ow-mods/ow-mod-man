use regex::Regex;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolInstallType {
    InstallMod,
    InstallURL,
    InstallPreRelease,
    InstallZip,
    Unknown,
}

impl ProtocolInstallType {
    pub fn parse(raw_str: &str) -> Self {
        match raw_str {
            "install-mod" => Self::InstallMod,
            "install-url" => Self::InstallURL,
            "install-prerelease" => Self::InstallPreRelease,
            "install-zip" => Self::InstallZip,
            _ => Self::Unknown,
        }
    }
}

#[allow(rustdoc::bare_urls)]
/// Represents a payload receive by a protocol handler (link from the website)
/// All URLs should start with owmods://
/// Then they should follow with the install type they want like `install-mod` or `install-url`
/// Finally they should have the payload for the install
///
/// If an invalid install type is given the [ProtocolInstallType] will be set to [ProtocolInstallType::Unknown]
///
/// Some examples of valid URIs are:
/// - owmods://install-mod/Bwc9876.TimeSaver
/// - owmods://install-url/https://example.com/Mod.zip
/// - owmods://install-zip//home/user/Downloads/Mod.zip
/// - owmods://install-prerelease/Raicuparta.NomaiVR
#[typeshare]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolPayload {
    pub install_type: ProtocolInstallType,
    pub payload: String,
}

impl ProtocolPayload {
    fn failed() -> Self {
        Self {
            install_type: ProtocolInstallType::Unknown,
            payload: "".to_string(),
        }
    }

    pub fn parse(raw_str: &str) -> Self {
        let re = Regex::new(r"^owmods://([^/]+)/(.+)$").unwrap();
        if let Some(matches) = re.captures(raw_str) {
            let install_type = matches
                .get(1)
                .map(|m| ProtocolInstallType::parse(m.as_str()))
                .unwrap_or(ProtocolInstallType::Unknown);
            let payload = matches.get(2).map(|m| m.as_str());
            if let Some(payload) = payload {
                Self {
                    install_type,
                    payload: payload.to_string(),
                }
            } else {
                Self::failed()
            }
        } else {
            Self::failed()
        }
    }
}

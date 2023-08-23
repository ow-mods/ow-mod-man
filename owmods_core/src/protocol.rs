use regex::Regex;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Represents the type of install that should be done when a protocol link is clicked
/// This is used to determine what to do with the payload
#[typeshare]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolInstallType {
    /// Install a mod from the mod database
    InstallMod,
    /// Install a mod from a URL
    InstallURL,
    /// Install a mod's prerelease from the database
    InstallPreRelease,
    /// Install a mod from a zip file
    InstallZip,
    /// Unknown install type, means the protocol link was invalid and therefore should be ignored
    Unknown,
}

impl ProtocolInstallType {
    /// Parse a string into a [ProtocolInstallType]
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
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolPayload {
    /// The type of install that should be done
    pub install_type: ProtocolInstallType,
    /// The payload for the install
    pub payload: String,
}

impl ProtocolPayload {
    fn failed() -> Self {
        Self {
            install_type: ProtocolInstallType::Unknown,
            payload: "".to_string(),
        }
    }

    /// Parse a string into a [ProtocolPayload]
    /// If the string is invalid the [ProtocolInstallType] will be set to [ProtocolInstallType::Unknown]
    /// and the payload will be set to an empty string
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_payload() {
        let payload = ProtocolPayload::parse("owmods://install-mod/Bwc9876.TimeSaver");
        assert!(matches!(
            payload.install_type,
            ProtocolInstallType::InstallMod
        ));
        assert_eq!(payload.payload, "Bwc9876.TimeSaver");

        let payload = ProtocolPayload::parse("owmods://install-url/https://example.com/Mod.zip");
        assert!(matches!(
            payload.install_type,
            ProtocolInstallType::InstallURL
        ));
        assert_eq!(payload.payload, "https://example.com/Mod.zip");

        let payload = ProtocolPayload::parse("owmods://install-zip//home/user/Downloads/Mod.zip");
        assert!(matches!(
            payload.install_type,
            ProtocolInstallType::InstallZip
        ));
        assert_eq!(payload.payload, "/home/user/Downloads/Mod.zip");

        let payload = ProtocolPayload::parse("owmods://install-prerelease/Raicuparta.NomaiVR");
        assert!(matches!(
            payload.install_type,
            ProtocolInstallType::InstallPreRelease
        ));
        assert_eq!(payload.payload, "Raicuparta.NomaiVR");
    }

    #[test]
    fn test_protocol_payload_invalid() {
        let payload = ProtocolPayload::parse("ow://asdf");
        assert!(matches!(payload.install_type, ProtocolInstallType::Unknown));
        assert_eq!(payload.payload, "");

        let payload = ProtocolPayload::parse("owmods://install-mod");
        assert!(matches!(payload.install_type, ProtocolInstallType::Unknown));
        assert_eq!(payload.payload, "");

        let payload = ProtocolPayload::parse("owmods://install-url");
        assert!(matches!(payload.install_type, ProtocolInstallType::Unknown));
        assert_eq!(payload.payload, "");

        let payload = ProtocolPayload::parse("owmods://install-zip");
        assert!(matches!(payload.install_type, ProtocolInstallType::Unknown));
        assert_eq!(payload.payload, "");

        let payload = ProtocolPayload::parse("owmods://install-prerelease");
        assert!(matches!(payload.install_type, ProtocolInstallType::Unknown));
        assert_eq!(payload.payload, "");
    }
}

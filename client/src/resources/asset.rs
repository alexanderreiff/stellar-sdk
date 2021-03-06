use resources::Amount;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Assets are the units that are traded on the Stellar Network.
/// An asset consists of an type, code, and issuer.
/// Any asset can be traded for any other asset.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/asset.html>

/// An identifer is the type, code, and issuer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssetIdentifier {
    /// Stellar Lumens!
    Native,
    /// Asset with a 4 character code
    CreditAlphanum4(AssetId),
    /// Asset with a 12 character code
    CreditAlphanum12(AssetId),
}

/// Struct containing code and issuer
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AssetId {
    code: String,
    issuer: String,
}

/// A convenience struct used for deserializing AssetIdentifier
#[derive(Serialize, Deserialize, Debug)]
pub struct IntermediateAssetIdentifier {
    asset_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_issuer: Option<String>,
}

impl<'de> Deserialize<'de> for AssetIdentifier {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: IntermediateAssetIdentifier = IntermediateAssetIdentifier::deserialize(d)?;
        AssetIdentifier::new(&rep.asset_type, rep.asset_code, rep.asset_issuer)
            .map_err(de::Error::custom)
    }
}

impl Serialize for AssetIdentifier {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rep = match *self {
            AssetIdentifier::Native => IntermediateAssetIdentifier {
                asset_type: "native".to_string(),
                asset_code: None,
                asset_issuer: None,
            },
            _ => IntermediateAssetIdentifier {
                asset_type: self.asset_type().to_string(),
                asset_code: Some(self.code().to_string()),
                asset_issuer: Some(self.issuer().to_string()),
            },
        };
        rep.serialize(s)
    }
}

impl AssetIdentifier {
    /// The type of this asset: “credit_alphanum4”, or “credit_alphanum12”.
    /// Returns a slice that lives as long as the asset does.
    pub fn asset_type(&self) -> &str {
        match *self {
            AssetIdentifier::Native => &"native",
            AssetIdentifier::CreditAlphanum4(_) => &"credit_alphanum4",
            AssetIdentifier::CreditAlphanum12(_) => &"credit_alphanum12",
        }
    }

    /// The code of this asset.
    /// Returns a slice that lives as long as the asset does.
    pub fn code(&self) -> &str {
        match *self {
            AssetIdentifier::Native => &"XLM",
            AssetIdentifier::CreditAlphanum4(ref asset_id) => &asset_id.code,
            AssetIdentifier::CreditAlphanum12(ref asset_id) => &asset_id.code,
        }
    }

    /// The code of this asset as a result.
    pub fn asset_code(&self) -> Option<String> {
        match *self {
            AssetIdentifier::Native => None,
            AssetIdentifier::CreditAlphanum4(ref asset_id) => Some(asset_id.code.clone()),
            AssetIdentifier::CreditAlphanum12(ref asset_id) => Some(asset_id.code.clone()),
        }
    }

    /// The issuer of this asset.  This corresponds to the id of an account.
    /// Returns a slice that lives as long as the asset does.
    pub fn issuer(&self) -> &str {
        match *self {
            AssetIdentifier::Native => &"Stellar Foundation",
            AssetIdentifier::CreditAlphanum4(ref asset_id) => &asset_id.issuer,
            AssetIdentifier::CreditAlphanum12(ref asset_id) => &asset_id.issuer,
        }
    }

    /// The issuer of this asset as a result
    pub fn asset_issuer(&self) -> Option<String> {
        match *self {
            AssetIdentifier::Native => None,
            AssetIdentifier::CreditAlphanum4(ref asset_id) => Some(asset_id.issuer.clone()),
            AssetIdentifier::CreditAlphanum12(ref asset_id) => Some(asset_id.issuer.clone()),
        }
    }

    /// Returns true if this is the native lumen on the network
    pub fn is_native(&self) -> bool {
        &AssetIdentifier::Native == self
    }

    /// A new Asset can be a native stellar, or a fully identified asset
    pub fn new(
        asset_type: &str,
        code: Option<String>,
        issuer: Option<String>,
    ) -> Result<AssetIdentifier, String> {
        match asset_type {
            "native" => Ok(AssetIdentifier::Native),
            "credit_alphanum4" => Ok(AssetIdentifier::CreditAlphanum4(AssetId {
                code: code.unwrap(),
                issuer: issuer.unwrap(),
            })),
            "credit_alphanum12" => Ok(AssetIdentifier::CreditAlphanum12(AssetId {
                code: code.unwrap(),
                issuer: issuer.unwrap(),
            })),
            _ => Err("Invalid Asset Type.".to_string()),
        }
    }

    /// A type safe way of creating a native asset AssetIdentifier
    pub fn native() -> AssetIdentifier {
        AssetIdentifier::Native
    }

    /// A type safe way of creating an alphanum4 asset AssetIdentifier
    pub fn alphanum4(code: &str, issuer: &str) -> AssetIdentifier {
        AssetIdentifier::CreditAlphanum4(AssetId {
            code: code.to_string(),
            issuer: issuer.to_string(),
        })
    }

    /// A type safe way of creating an alphanum12 asset AssetIdentifier
    pub fn alphanum12(code: &str, issuer: &str) -> AssetIdentifier {
        AssetIdentifier::CreditAlphanum12(AssetId {
            code: code.to_string(),
            issuer: issuer.to_string(),
        })
    }
}

#[cfg(test)]
mod asset_identifier_tests {
    use super::*;
    use serde_json;

    fn asset_json() -> &'static str {
        include_str!("../../fixtures/asset.json")
    }

    fn native_asset_json() -> &'static str {
        include_str!("../../fixtures/native_asset.json")
    }

    #[test]
    fn it_parses_native_assets_from_json() {
        let native_asset: AssetIdentifier = serde_json::from_str(&native_asset_json()).unwrap();
        assert_eq!(native_asset.asset_type(), "native");
        assert_eq!(native_asset.code(), "XLM");
        assert_eq!(native_asset.asset_code(), None);
        assert_eq!(native_asset.issuer(), "Stellar Foundation");
        assert_eq!(native_asset.asset_issuer(), None);
        assert!(native_asset.is_native());
    }

    #[test]
    fn it_parses_an_identifier() {
        let asset: AssetIdentifier = serde_json::from_str(&asset_json()).unwrap();
        assert_eq!(asset.asset_type(), "credit_alphanum4");
        assert_eq!(asset.code(), "USD");
        assert_eq!(
            asset.issuer(),
            "GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG"
        );
        assert!(!asset.is_native());
    }

    #[test]
    fn it_serializes_non_native_assets() {
        let asset: AssetIdentifier = serde_json::from_str(&asset_json()).unwrap();
        assert_eq!(
            serde_json::to_string(&asset).unwrap(),
            "{\
             \"asset_type\":\"credit_alphanum4\",\
             \"asset_code\":\"USD\",\
             \"asset_issuer\":\"GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG\"\
             }"
        );
    }

    #[test]
    fn it_serializes_native_assets() {
        let native_asset: AssetIdentifier = serde_json::from_str(&native_asset_json()).unwrap();
        assert_eq!(
            serde_json::to_string(&native_asset).unwrap(),
            "{\
             \"asset_type\":\"native\"\
             }"
        );
    }

    #[test]
    fn it_creates_a_native_asset() {
        let asset: AssetIdentifier = AssetIdentifier::native();
        assert_eq!(asset.asset_type(), "native");
        assert_eq!(asset.code(), "XLM");
        assert_eq!(asset.asset_code(), None);
        assert_eq!(asset.issuer(), "Stellar Foundation");
        assert_eq!(asset.asset_issuer(), None);
        assert!(asset.is_native());
    }

    #[test]
    fn it_creates_an_alphanum4_asset() {
        let asset: AssetIdentifier = AssetIdentifier::alphanum4("ABCD", "ISSUER");
        assert_eq!(asset.asset_type(), "credit_alphanum4");
        assert_eq!(asset.code(), "ABCD");
        assert_eq!(asset.issuer(), "ISSUER");
        assert!(!asset.is_native());
    }

    #[test]
    fn it_creates_an_alphanum12_asset() {
        let asset: AssetIdentifier = AssetIdentifier::alphanum12("ABCD", "ISSUER");
        assert_eq!(asset.asset_type(), "credit_alphanum12");
        assert_eq!(asset.code(), "ABCD");
        assert_eq!(asset.issuer(), "ISSUER");
        assert!(!asset.is_native());
    }
}

/// Permissions around who can own an asset and whether or
/// not the asset issuer can freeze the asset.
#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
}

impl Flags {
    /// Creates a new set of flags
    pub fn new(auth_required: bool, auth_revocable: bool) -> Flags {
        Flags {
            auth_required,
            auth_revocable,
        }
    }

    /// If this field is true it means the anchor must approve anyone who wants to
    /// hold its credit, allowing it to control who its customers are
    pub fn is_auth_required(&self) -> bool {
        self.auth_required
    }

    /// If this field is true it means the anchor can freeze credit held by another account. When
    /// credit is frozen for a particular account, that account can only send the credit back to
    /// the anchor–it can’t transfer the credit to any other account. This setting allows the
    /// issuing account to revoke credit that it accidentally issued or that was obtained
    /// improperly.
    pub fn is_auth_revocable(&self) -> bool {
        self.auth_revocable
    }
}

/// Assets are the units that are traded on the Stellar Network.
/// An asset consists of an type, code, and issuer.
/// Any asset can be traded for any other asset.
///
/// <https://www.stellar.org/developers/horizon/reference/resources/asset.html>
#[derive(Debug, Clone)]
pub struct Asset {
    asset_identifier: AssetIdentifier,
    amount: Amount,
    num_accounts: u32,
    flags: Flags,
}

#[derive(Deserialize, Debug)]
pub struct IntermediateAsset {
    asset_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_issuer: Option<String>,
    amount: Amount,
    num_accounts: u32,
    flags: Flags,
}

impl<'de> Deserialize<'de> for Asset {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rep: IntermediateAsset = IntermediateAsset::deserialize(d)?;
        let asset_identifier: Result<AssetIdentifier, D::Error> =
            AssetIdentifier::new(&rep.asset_type, rep.asset_code, rep.asset_issuer)
                .map_err(de::Error::custom);
        Ok(Asset {
            asset_identifier: asset_identifier.unwrap(),
            amount: rep.amount,
            num_accounts: rep.num_accounts,
            flags: rep.flags,
        })
    }
}

impl Asset {
    /// The identifier of this asset.
    pub fn identifier(&self) -> &AssetIdentifier {
        &self.asset_identifier
    }

    /// The type of this asset: “credit_alphanum4”, or “credit_alphanum12”.
    /// Returns a slice that lives as long as the asset does.
    pub fn asset_type(&self) -> &str {
        &self.asset_identifier.asset_type()
    }

    /// The code of this asset.
    /// Returns a slice that lives as long as the asset does.
    pub fn code(&self) -> &str {
        &self.asset_identifier.code()
    }

    /// The issuer of this asset.  This corresponds to the id of an account.
    /// Returns a slice that lives as long as the asset does.
    pub fn issuer(&self) -> &str {
        &self.asset_identifier.issuer()
    }

    /// The number of units of credit issued for this asset.
    /// This number is scaled by 10 million to display the number if the format a
    /// user would expect it in.
    ///
    /// <https://www.stellar.org/developers/guides/concepts/assets.html#amount-precision-and-representation>
    pub fn amount(&self) -> Amount {
        self.amount
    }

    /// The number of accounts that: 1) trust this asset and 2) where if the asset has the
    /// auth_required flag then the account is authorized to hold the asset.
    /// Returns an unsigned 32-bit integer
    pub fn num_accounts(&self) -> u32 {
        self.num_accounts
    }

    /// If this field is true it means the anchor must approve anyone who wants to
    /// hold its credit, allowing it to control who its customers are
    pub fn is_auth_required(&self) -> bool {
        self.flags.auth_required
    }

    /// If this field is true it means the anchor can freeze credit held by another account. When
    /// credit is frozen for a particular account, that account can only send the credit back to
    /// the anchor–it can’t transfer the credit to any other account. This setting allows the
    /// issuing account to revoke credit that it accidentally issued or that was obtained
    /// improperly.
    pub fn is_auth_revocable(&self) -> bool {
        self.flags.auth_revocable
    }

    /// Returns the flags associated with this asset.
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

#[cfg(test)]
mod asset_tests {
    use super::*;
    use serde_json;

    fn asset_json() -> &'static str {
        include_str!("../../fixtures/asset.json")
    }

    #[test]
    fn it_parses_an_asset_from_json() {
        let asset: Asset = serde_json::from_str(&asset_json()).unwrap();
        assert_eq!(asset.asset_type(), "credit_alphanum4");
        assert_eq!(asset.code(), "USD");
        assert_eq!(
            asset.issuer(),
            "GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG"
        );
        assert_eq!(asset.amount(), Amount::new(1000000000));
        assert_eq!(asset.num_accounts(), 91547871);
        assert!(!asset.is_auth_required());
        assert!(!asset.flags().is_auth_required());
        assert!(asset.is_auth_revocable());
        assert!(asset.flags().is_auth_revocable());
        assert_eq!(
            asset.identifier(),
            &AssetIdentifier::alphanum4(
                "USD",
                "GBAUUA74H4XOQYRSOW2RZUA4QL5PB37U3JS5NE3RTB2ELJVMIF5RLMAG"
            ),
        );
    }
}

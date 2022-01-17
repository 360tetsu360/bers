use serde::{Deserialize, Serialize};
use serde_json::Value;

// Jwt encoded player data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimatedImage {
    #[serde(rename = "AnimationExpression")]
    pub animation_expression: u32,
    #[serde(rename = "Frames")]
    pub frames: f32,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageHeight")]
    pub image_height: u32,
    #[serde(rename = "ImageWidth")]
    pub image_width: u32,
    #[serde(rename = "Type")]
    pub animation_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonaPiece {
    #[serde(rename = "IsDefault")]
    pub is_default: bool,
    #[serde(rename = "PackId")]
    pub pack_id: String,
    #[serde(rename = "PieceId")]
    pub piece_id: String,
    #[serde(rename = "PieceType")]
    pub piece_type: String,
    #[serde(rename = "ProductId")]
    pub product_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PieceTintColor {
    #[serde(rename = "Colors")]
    pub color: Vec<String>,
    #[serde(rename = "PieceType")]
    pub piece_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    #[serde(rename = "AnimatedImageData")]
    pub animated_image_data: Vec<AnimatedImage>,
    #[serde(rename = "ArmSize")]
    pub arm_size: String,
    #[serde(rename = "CapeData")]
    pub cape_data: String,
    #[serde(rename = "CapeId")]
    pub cape_id: String,
    #[serde(rename = "CapeImageHeight")]
    pub cape_image_height: u32,
    #[serde(rename = "CapeImageWidth")]
    pub cpae_image_width: u32,
    #[serde(rename = "CapeOnClassicSkin")]
    pub cape_on_classic_skin: bool,
    #[serde(rename = "ClientRandomId")]
    pub client_random_id: u64,
    #[serde(rename = "CurrentInputMode")]
    pub current_input_mode: u32,
    #[serde(rename = "DefaultInputMode")]
    pub default_input_mode: u32,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "DeviceModel")]
    pub device_model: String,
    #[serde(rename = "DeviceOS")]
    pub device_os: u32,
    #[serde(rename = "GameVersion")]
    pub game_version: String,
    #[serde(rename = "GuiScale")]
    pub gui_scale: u32,
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    #[serde(rename = "PersonaPieces")]
    pub persona_pieces: Vec<PersonaPiece>,
    #[serde(rename = "PersonaSkin")]
    pub persona_skin: bool,
    #[serde(rename = "PieceTintColors")]
    pub piece_tint_colors: Vec<PieceTintColor>,
    #[serde(rename = "PlatformOfflineId")]
    pub platform_offline_id: String,
    #[serde(rename = "PlatformOnlineId")]
    pub platform_online_id: String,
    #[serde(rename = "PlayFabId")]
    pub play_fab_id: String,
    #[serde(rename = "PremiumSkin")]
    pub premium_skin: bool,
    #[serde(rename = "SelfSignedId")]
    pub self_signed_id: String,
    #[serde(rename = "ServerAddress")]
    pub server_address: String,
    #[serde(rename = "SkinAnimationData")]
    pub skin_animation_data: String,
    #[serde(rename = "SkinColor")]
    pub skin_color: String,
    #[serde(rename = "SkinData")]
    pub skin_data: String,
    #[serde(rename = "SkinGeometryData")]
    pub skin_geometry_data: String,
    #[serde(rename = "SkinGeometryDataEngineVersion")]
    pub skin_geometry_data_engine_version: String,
    #[serde(rename = "SkinId")]
    pub skin_id: String,
    #[serde(rename = "SkinImageHeight")]
    pub skin_image_height: u32,
    #[serde(rename = "SkinImageWidth")]
    pub skin_image_width: u32,
    #[serde(rename = "SkinResourcePatch")]
    pub skin_resource_patch: String,
    #[serde(rename = "ThirdPartyName")]
    pub third_party_name: String,
    #[serde(rename = "ThirdPartyNameOnly")]
    pub third_party_name_only: bool,
    #[serde(rename = "UIProfile")]
    pub ui_profile: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtraData {
    #[serde(rename = "XUID")]
    pub xuid: String,
    #[serde(rename = "identity")]
    pub identity: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
}

impl ExtraData {
    pub fn from_value(value: &Value) -> Option<Self> {
        let xuid_raw = match value.get("XUID") {
            Some(p) => p,
            None => {
                eprintln!("xuid don't exists");
                return None;
            }
        };
        let xuid = match xuid_raw.as_str() {
            Some(p) => p.to_owned(),
            None => {
                eprintln!("xuid is not string");
                return None;
            }
        };
        let identity_raw = match value.get("identity") {
            Some(p) => p,
            None => {
                eprintln!("identity don't exists");
                return None;
            }
        };
        let identity = match identity_raw.as_str() {
            Some(p) => p.to_owned(),
            None => {
                eprintln!("identity is not string");
                return None;
            }
        };
        let display_name_raw = match value.get("displayName") {
            Some(p) => p,
            None => {
                eprintln!("displayName don't exists");
                return None;
            }
        };
        let display_name = match display_name_raw.as_str() {
            Some(p) => p.to_owned(),
            None => {
                eprintln!("displayName is not string");
                return None;
            }
        };
        let title_id_raw = match value.get("titleId") {
            Some(p) => p,
            None => {
                eprintln!("titleId don't exists");
                return None;
            }
        };
        let title_id = match title_id_raw.as_str() {
            Some(p) => p.to_owned(),
            None => {
                eprintln!("titleId is not string");
                return None;
            }
        };
        Some(Self {
            xuid,
            identity,
            display_name,
            title_id,
        })
    }
}

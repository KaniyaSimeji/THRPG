use bitflags::bitflags;
use extension_config::ExtensionConfig;
use indexmap::IndexMap;

pub struct ExtensionManager {
    extensions: IndexMap<String, (ExtensionConfig, Extension)>,
    authority: ExtensionAuthority,
}

#[derive(Debug, Clone, Copy)]
pub enum ExtensionAuthority {
    Standard,
    Strict,
    Custom(AuthorityFlags),
}

bitflags! {
    pub struct AuthorityFlags:u32 {
            const SEND_MESSAGE = 0b00000001;
            const FILE_SYSTEM = 0b00000010;
            const RPG_DATA_READ = 0b00000100;
            const READ_MESSAGE = 0b00000011;
            const DELETE_MESSAGE = 0b00000101;

            const MESSAGE = Self::SEND_MESSAGE.bits | Self::READ_MESSAGE.bits | Self::DELETE_MESSAGE.bits;
    }
}

#[derive(Debug, Clone)]
struct Extension {}

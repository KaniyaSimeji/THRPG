use crate::extension_config::{ExtensionConfig, Extensiontype};
use bitflags::bitflags;
use indexmap::IndexMap;
use wasmer::Imports;


#[derive(Debug,Clone)]
pub struct ExtensionManager {
    extensions: IndexMap<String, ExtensionConfig>,
    authority: ExtensionAuthority,
    import_object: Imports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            const RPG_DATA_WRITE = 0b00000111;

            const MESSAGE = Self::SEND_MESSAGE.bits | Self::READ_MESSAGE.bits | Self::DELETE_MESSAGE.bits;
            const RPG_DATA = Self::RPG_DATA_READ.bits | Self::RPG_DATA_WRITE.bits;
    }
}

impl ExtensionAuthority {
    pub fn authority_bits(&self) -> AuthorityFlags {
        match self {
            ExtensionAuthority::Custom(flags) => *flags,
            ExtensionAuthority::Standard => AuthorityFlags::MESSAGE | AuthorityFlags::RPG_DATA,
            ExtensionAuthority::Strict => AuthorityFlags::MESSAGE | AuthorityFlags::RPG_DATA_READ,
        }
    }

    pub fn strict() -> Self {
        ExtensionAuthority::Strict
    }

    pub fn custom(flags: u32) -> Option<Self> {
        let bits = AuthorityFlags::from_bits(flags)?;
        Some(ExtensionAuthority::Custom(bits))
    }
}

impl Default for ExtensionAuthority {
    fn default() -> Self {
        ExtensionAuthority::Standard
    }
}


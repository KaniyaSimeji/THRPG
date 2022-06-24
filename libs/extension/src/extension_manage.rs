use crate::extension_config::{ExtensionConfig, Extensiontype};
use bitflags::bitflags;
use indexmap::IndexMap;
use std::io::Read;
use wasmer::{ChainableNamedResolver, Exports, Function, ImportObject, Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiEnv, WasiState};

pub struct ExtensionManager {
    extensions: IndexMap<String, ExtensionConfig>,
    authority: ExtensionAuthority,
    import_object: ImportObject,
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

    pub fn authority(strict: bool, flags: Option<u32>) -> Self {
        if strict {
            ExtensionAuthority::Strict
        } else {
            if let Some(flags) = flags {
                ExtensionAuthority::Custom(AuthorityFlags::from_bits(flags).unwrap())
            } else {
                ExtensionAuthority::Standard
            }
        }
    }
}

impl Default for ExtensionAuthority {
    fn default() -> Self {
        ExtensionAuthority::Standard
    }
}

impl ExtensionManager {
    pub fn to_manager(store: crate::store::ExtensionStore, authority: ExtensionAuthority) -> Self {
        let mut map: IndexMap<String, ExtensionConfig> = IndexMap::new();
        store.import().into_iter().for_each(|extension| {
            map.insert(extension.name().to_string(), extension);
        });
        Self {
            extensions: map,
            authority,
            import_object: ImportObject::new(),
        }
    }

    pub fn add_exports(&mut self, objects: Vec<Exports>) -> &mut Self {
        objects.iter().for_each(|export| {
            self.import_object.register("env", export.clone());
        });
        self
    }

    pub fn wasi_state_setting() -> WasiEnv {
        let input = Pipe::new();
        let output = Pipe::new();
        let wasi_env = WasiState::new("thrpg")
            .map_dir("/host", ".")
            .unwrap()
            .stdin(Box::new(input))
            .stdout(Box::new(output))
            .finalize()
            .unwrap();
        wasi_env
    }

    pub fn boot_extension(mut self, wasi_object: ImportObject) {
        self.extensions
            .into_iter()
            .map(|(_, extension_config)| {
                let entry_path = extension_config.entry_file();
                let mut buf = Vec::new();
                let content = std::fs::File::open(entry_path)
                    .unwrap()
                    .read_to_end(&mut buf)
                    .unwrap();
                let bytes = buf.as_slice();
                if extension_config.extension_type() == &Extensiontype::Features {
                    let module = Module::new(&Store::default(), bytes).unwrap();
                    let instance =
                        Instance::new(&module, &self.import_object.chain_back(wasi_object))
                            .unwrap();
                    instance
                } else if extension_config.extension_type() == &Extensiontype::Contents {
                    let strg = String::from_utf8(bytes.to_vec());
                    todo!()
                } else {
                    todo!()
                }
            })
            .collect();
    }
}

struct ExtensionEnv {
    config: ExtensionConfig,
    instance: Instance,
}

use std::{collections::HashMap, path::Path};

use fluent::{FluentResource, bundle::FluentBundle, FluentArgs};
pub use i18n_embed::DesktopLanguageRequester;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
pub use i18n_embed_fl;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use thrpg_utils::dir_files;
use tokio::fs::read_to_string;
pub use unic_langid::LanguageIdentifier;

#[deprecated]
static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    loader.load_fallback_language(&Localizations).unwrap();
    loader
});

#[derive(RustEmbed)]
#[folder = "../../i18n"]
pub struct Localizations;

#[deprecated]
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

#[deprecated]
#[macro_export]
macro_rules! tfl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

#[deprecated]
pub fn appear_enemy<T: Into<String>>(name: T) -> String {
    tfl!("appear-enemy", name = name.into())
}

pub async fn load_fluent_resource<P:AsRef<Path>>(path: P) -> anyhow::Result<Vec<FluentResource>> {
    let mut vec = Vec::new();
    for resource_path in dir_files(path.as_ref()).await? {
        let content = read_to_string(resource_path).await?;
        let resource = FluentResource::try_new(content).map_err(|(resource,error)|anyhow::anyhow!("
            Error resource:{:?} 
            why:{:?}", resource, error
        ))?;
        vec.push(resource)
    }

    Ok(vec)
}

pub fn link_languages<F>(languages: F) -> anyhow::Result<Vec<FluentBundle<FluentResource,intl_memoizer::IntlLangMemoizer>>>
where F: FnOnce(&mut HashMap<LanguageIdentifier, FluentResource>) -> &mut HashMap<LanguageIdentifier,FluentResource>
{
    let mut map = HashMap::new();
    languages(&mut map);
    let bundler = map.into_iter().map(|(identifier, resource)| {
        let mut bundle = FluentBundle::new(vec![identifier]);
        bundle.add_resource(resource).unwrap();
        bundle
    }).collect();
    Ok(bundler)
}

pub fn get_msg<T:Into<String>,U,V>(bundle: FluentBundle<FluentResource,intl_memoizer::IntlLangMemoizer>, id: T, args: Option<&FluentArgs>) -> Option<String> {
    let raw_msg = bundle.get_message(&id.into());
    let mut err = Vec::new();
    let msg = bundle.format_pattern(raw_msg?.value()?, args, &mut err);
    Some(msg.to_string())
}

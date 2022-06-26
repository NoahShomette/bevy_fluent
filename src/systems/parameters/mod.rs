//! System parameters
//!
//! Any entity located directly in this module is
//! [`SystemParam`](bevy::ecs::system::SystemParam).

use crate::{exts::fluent::BundleExt, BundleAsset, Locale, Localization};
use bevy::{ecs::system::SystemParam, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

/// Localization builder
#[derive(SystemParam)]
pub struct LocalizationBuilder<'w, 's> {
    assets: Res<'w, Assets<BundleAsset>>,
    locale: Res<'w, Locale>,
    #[system_param(ignore)]
    phantom_data: PhantomData<&'s ()>,
}

impl LocalizationBuilder<'_, '_> {
    pub fn build<'a>(
        &self,
        handles: impl IntoIterator<Item = &'a Handle<BundleAsset>>,
    ) -> Localization {
        let locale_entries: HashMap<_, _> = handles
            .into_iter()
            .map(|handle| {
                let asset = self.assets.get(handle).unwrap();
                (asset.locale(), Entry { handle, asset })
            })
            .collect();
        let locales = self.locale.fallback_chain(locale_entries.keys().cloned());
        let mut localization = Localization::new();
        for locale in locales {
            localization.insert(
                locale_entries[locale].handle,
                locale_entries[locale].asset.clone(),
            );
        }
        localization
    }
}

struct Entry<'a> {
    handle: &'a Handle<BundleAsset>,
    asset: &'a BundleAsset,
}
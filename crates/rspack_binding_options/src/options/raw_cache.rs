use serde::Deserialize;

#[cfg(feature = "node-api")]
use napi_derive::napi;

use rspack_core::{
  CacheOptions, CompilerOptionsBuilder, FileSystemCacheOptions, MemoryCacheOptions,
};

use crate::RawOption;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "node-api")]
#[napi(object)]
pub struct RawCacheOptions {
  pub cache_type: String,
  pub max_generations: u32,
  pub max_age: u32,
  pub profile: bool,
  pub build_dependencies: Vec<String>,
  pub cache_directory: String,
  pub cache_location: String,
  pub name: String,
  pub version: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[cfg(not(feature = "node-api"))]
pub struct RawCacheOptions {
  pub cache_type: String,
  pub max_generations: u32,
  pub max_age: u32,
  pub profile: bool,
  pub build_dependencies: Vec<String>,
  pub cache_directory: String,
  pub cache_location: String,
  pub name: String,
  pub version: String,
}

impl RawOption<CacheOptions> for RawCacheOptions {
  fn to_compiler_option(self, _options: &CompilerOptionsBuilder) -> anyhow::Result<CacheOptions> {
    let Self {
      cache_type,
      max_generations,
      max_age,
      profile,
      build_dependencies,
      cache_directory,
      cache_location,
      name,
      version,
    } = self;

    Ok(match cache_type.as_str() {
      "memory" => CacheOptions::Memory(MemoryCacheOptions { max_generations }),
      "filesystem" => CacheOptions::FileSystem(FileSystemCacheOptions {
        max_age,
        profile,
        build_dependencies,
        cache_directory,
        cache_location,
        name,
        version,
      }),
      _ => CacheOptions::Disabled,
    })
  }

  fn fallback_value(_: &CompilerOptionsBuilder) -> Self {
    Default::default()
  }
}
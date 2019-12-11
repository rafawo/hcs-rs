// Copyright  rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

use crate::schema::utils::is_default;
use serde::{Deserialize, Serialize};

#[cfg(feature = "19h1")]
impl std::default::Default for MemoryBackingPageSize {
    fn default() -> Self {
        MemoryBackingPageSize::Small
    }
}

#[cfg(feature = "19h1")]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MemoryBackingPageSize {
    /// Small (4KB) page size unit
    Small,
    /// Large (2MB) page size unit
    Large,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Memory {
    #[serde(rename = "SizeInMB")]
    pub size_in_mb: u64,

    /// If enabled, then the VM's memory is backed by the Windows pagefile rather than physically
    /// backed, statically allocated memory.
    #[serde(
        default,
        rename = "AllowOvercommit",
        skip_serializing_if = "is_default"
    )]
    pub allow_overcommit: bool,

    /// The preferred page size unit (chunk size) used when allocating backing pages for the VM.
    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "BackingPageSize",
        skip_serializing_if = "is_default"
    )]
    pub backing_page_size: Option<MemoryBackingPageSize>,

    /// If enabled, then each backing page is physically pinned on first access.
    #[cfg(feature = "19h1")]
    #[serde(rename = "PinBackingPages")]
    pub pin_backing_pages: bool,

    /// If enabled, then backing page chunks smaller than the backing page size are never used unless
    /// the system is under extreme memory pressure. If the backing page size is Small, then it is
    /// forced to Large when this option is enabled.
    #[cfg(feature = "19h1")]
    #[serde(rename = "ForbidSmallBackingPages")]
    pub forbid_small_backing_pages: bool,

    /// If enabled, then the memory hot hint feature is exposed to the VM, allowing it to prefetch
    /// pages into its working set. (if supported by the guest operating system).
    #[serde(default, rename = "EnableHotHint", skip_serializing_if = "is_default")]
    pub enable_hot_hint: bool,

    /// If enabled, then the memory cold hint feature is exposed to the VM, allowing it to trim zeroed
    /// pages from its working set (if supported by the guest operating system).
    #[serde(default, rename = "EnableColdHint", skip_serializing_if = "is_default")]
    pub enable_cold_hint: bool,

    /// If enabled, then the memory cold discard hint feature is exposed to the VM, allowing it to trim
    /// non-zeroed pages from the working set (if supported by the guest operating system).
    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "EnableColdDiscardHint",
        skip_serializing_if = "is_default"
    )]
    pub enable_cold_discard_hint: bool,

    /// If enabled, then commit is not charged for each backing page until first access.
    #[serde(
        default,
        rename = "EnableDeferredCommit",
        skip_serializing_if = "is_default"
    )]
    pub enable_deferred_commit: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Processor {
    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(default, rename = "Limit", skip_serializing_if = "is_default")]
    pub limit: Option<u64>,

    #[serde(default, rename = "Weight", skip_serializing_if = "is_default")]
    pub weight: Option<u64>,

    #[serde(
        default,
        rename = "ExposeVirtualizationExtensions",
        skip_serializing_if = "is_default"
    )]
    pub expose_virtualization_extensions: bool,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "EnablePerfmonPmu",
        skip_serializing_if = "is_default"
    )]
    pub enable_perfmon_pmu: bool,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "EnablePerfmonPebs",
        skip_serializing_if = "is_default"
    )]
    pub enable_perfmon_pebs: bool,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "EnablePerfmonLbr",
        skip_serializing_if = "is_default"
    )]
    pub enable_perfmon_lbr: bool,

    #[cfg(feature = "19h1")]
    #[serde(
        default,
        rename = "EnablePerfmonIpt",
        skip_serializing_if = "is_default"
    )]
    pub enable_perfmon_ipt: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Topology {
    #[serde(rename = "Memory")]
    pub memory: Memory,

    #[serde(rename = "Processor")]
    pub processor: Processor,
}

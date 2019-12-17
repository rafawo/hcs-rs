# hcs-rs
Rust wrapper of Host Compute Service API

## Overview

This project is a collection of Rust libraries that wrap functionality exposed by the [Host Compute Service](https://blogs.technet.microsoft.com/virtualization/2017/01/27/introducing-the-host-compute-service-hcs/).

HCS (Host Compute Service for short) APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

Public documentation for the HCS is yet to be released.

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK.
  - By default, RS5 (10.0.17763.0) API surface is always built.
  - Optionally, 19H1 (10.0.18362.0) can be included through feature `19h1`.
- **amd64** architecture.

## Wrapped Windows 10 SDK APIs

**_Note: This section includes the paths in the Windows SDK for the header and lib files based on the default installation path `c:\Program Files (x86)\Windows Kits\10`._**
**_Note: This section shows paths specific to Windows SDK version 10.0.18362.0_**

The relevant Windows 10 SDK files that this project is wrapping are:
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computecore.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computedefs.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computenetwork.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computestorage.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\hypervdevicevirtualization.h
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computecore.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computenetwork.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computestorage.lib
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\vmdevicehost.lib
- C:\Windows\System32\computecore.dll
- C:\Windows\System32\computenetwork.dll
- C:\Windows\System32\computestorage.dll
- C:\Windows\System32\vmdevicehost.dll

All of the above are serviced through the Windows Service `Host Compute Service`, from executable **C:\Windows\System32\vmcompute.exe**.

The C bindings in this crate will remain private and not public to external code. Only the Rust idiomatic wrappers are exposed.

The following table describes the relevant Windows 10 SDK files that this project is wrapping and how they relate to each module:

| hcs-rs file | Overview | HCS API C Header file | .h path in SDK | .lib path in SDK |
| -- | -- | -- | -- | -- |
| [computedefs](/src/compute/defs.rs) | Types and definitions used by the HCS APIs. | computedefs.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computedefs.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computedefs.lib |
| [computecore](/src/computecore/mod.rs) | APIs for the core HCS functionality to manage compute systems. | computecore.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computecore.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computecore.lib |
| [computenetwork](/src/computenetwork/mod.rs) | Types definitions and APIs to interact with the HCN (Host Compute Network). | computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computenetwork.lib |
| [computestorage](/src/computestorage/mod.rs) | APIs for the HCS storage management. | computestorage.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\computestorage.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\computestorage.lib |
| [hypervdevicevirtualization](/src/hypervdevicevirtualization/mod.rs) | Types definitions and APIs for device emulation/virtualization. | hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\vmdevicehost.lib |

## Features
| Feature | Notes |
| -- | -- |
| `bindings` | By default, the raw C bindings to the SDK APIs are private to the crate. Using feature `bindings` makes them public for consumption on client code |
| `schema` | Includes all HCS/HCN schema JSON object model |
| `19h1` | By default, the project has compatibility with RS5. Using feature `19h1` adds 19H1 specific updates to the APIs |
| `utilities` | Includes utility code that provides more Rust abstractions on top of the basic safe wrappers of the C bindings. By default, this crate only exposes the safe wrappers |

## Crates.io version notes

**NOTE: All versions from 0.2.1 and below have been yanked on purpose. Please use the git repo directly for latest active changes, or 0.2.2 as the oldest "stable" version.**

This section briefly describes all published crates.io [versions](https://crates.io/crates/hcs-rs/versions) of this project, ordered from latest to oldest.

- [**0.6.0 Dec 17, 2019**](https://crates.io/crates/hcs-rs/0.6.0)
  - Breaking changes
    - Mark `HdvPciDevice` callback trait functions as `unsafe`, due to its assumption that they should not panic
- [**0.5.1 Dec 16, 2019**](https://crates.io/crates/hcs-rs/0.5.1)
  - Added `#[derive(Clone)]` to `HdvPciDeviceBase` and `HdvPciDeviceBaseWrapper`.
- [**0.5.0 Dec 12, 2019**](https://crates.io/crates/hcs-rs/0.5.0)
  - Breaking changes
    - Added `unsafe` to the functions that expect a callback or some kind of assumption that the provided reference outlives the internal details. It used to not be marked as unsafe, but after seeing several bugs due to getting wrong the lifetime (and Rust compiler not being able to understand it correctly)
  - Fixed wrong schema name for `vmbfs_root_path`
- [**0.4.0 Dec 12, 2019**](https://crates.io/crates/hcs-rs/0.4.0)
  - Fixed documentation of utilities
  - Added a few more utilities to hypervdevicevirtualization
  - Breaking changes
    - Fixed up callback on utilities to expect a reference to a closure's trait object
    - Renamed `get_handle` to `handle` and `get_handle_policy` to `handle_policy`
- [**0.3.0 Dec 11, 2019**](https://crates.io/crates/hcs-rs/0.3.0)
  - Breaking changes
    - hypervdevicevirtualization safe wrappers for reading and writing guest memory now use slices instead of vectors directly
      - In particular, read used to return a newly allocated buffer with the read memory. Now, it reads it directly into the supplied byte slice
    - Introduction of feature `utilities` to separate all the higher abstract Rust abstraction on top of the basic safe wrappers
    - Safe wrapper `enumerate_compute_systems` now supports an optional query parameter
  - Added utilities around `hypervdevicevirtualization` APIs
- [**0.2.2 Dec 10, 2019**](https://crates.io/crates/hcs-rs/0.2.2)
  - Removes unnecessary ispresent bindings
  - Gets a functional schema JSON object model finalized
  - Proper use of features for conditionally compiling support for different windows releases (as of now, only 19h1)
  - Guard schema model behind a `schema` feature
  - Enable feature `bindings` that exposes the raw bindings to the APIs (by default they are private to the crate)
  - Updated dependency to `winutils_rs` to get a fix for WideString CoTaskMemFree/LocalFree wrappers
  - This is the "oldest" version with a somewhat stable state, as well as cleaned up from unnecessary stuff
- [**0.2.1 Dec 10, 2019**](https://crates.io/crates/hcs-rs/0.2.1)
  - Use extern "system" instead of extern "C"
  - Make sure function pointers are wrapped on an Option, to allow for null function pointers
  - SCHEMA NOT COMPLETELY TESTED/FUNCTIONAL
  - YANKED, INCOMPLETE/WRONG IMPLEMENTATION; NOT ADVISED TO USE
- [**0.2.0 Dec 9, 2019**](https://crates.io/crates/hcs-rs/0.2.0)
  - Second version, with basic safe wrappers on top of computecore
  - HCS schema model implementation
  - Updated hypervdevicevirtualization to include newer changes
  - SCHEMA NOT COMPLETELY TESTED/FUNCTIONAL
  - YANKED, INCOMPLETE/WRONG IMPLEMENTATION; NOT ADVISED TO USE
  - Hardcoded dependency to Windows 10 SDK version 10.0.18362.0
    - Make the strict build-time lookup of exports library files opt-in through an environment variable
- [**0.1.0 Jan 8, 2019**](https://crates.io/crates/hcs-rs/0.1.0)
  - First version, with very limited functionality
  - Bindings to the C APIs exposed as thin safe wrappers
  - Lacking a lot of safe abstractions
  - Hardcoded dependency to Windows 10 SDK version 10.0.17763.0
  - YANKED, INCOMPLETE/WRONG IMPLEMENTATION; NOT ADVISED TO USE

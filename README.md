# hcsrs
Rust wrapper of Host Compute Service API

## Overview

This project is a collection of Rust libraries that wrap functionality exposed by the [Host Compute Service](https://blogs.technet.microsoft.com/virtualization/2017/01/27/introducing-the-host-compute-service-hcs/).

HCS (Host Compute Service for short) APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

Public documentation for the HCS is yet to be released.

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK version **10.0.17763.132**.
- **amd64** architecture.

## Workspace layout

**_Note: that the following table also includes the path in the SDK for the header and lib files based on the default Windows SDK installation path `c:\Program Files (x86)\Windows Kits\10`._**

| hcsrs library | Overview | HCS API C Header file | .h path in SDK | .lib path in SDK |
| -- | -- | -- | -- | -- |
| (computecore)[\src\computecore] | APIs for the core HCS functionality to manage compute systems. | computecore.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computecore.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computecore.lib |
| (computedefs)[\src\computedefs] | Types and definitions used by the HCS APIs. | computedefs.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computedefs.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computedefs.lib |
| (computenetwork)[\src\computenetwork] | Types definitions and APIs to interact with the HCN (Host Compute Network).  | computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computenetwork.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computenetwork.lib |
| (computestorage)[\src\computestorage] | APIs for the HCS storage management. | computestorage.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\computestorage.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\computestorage.lib |
| (hypervdevicevirtualization)[\src\hypervdevicevirtualization] | Types definitions and APIs for device emulation/virtualization. | hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\hypervdevicevirtualization.h | C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\vmdevicehost.lib |

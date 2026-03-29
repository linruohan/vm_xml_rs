pub mod common;
pub mod controller;
pub mod disk;
pub mod filesystem;
pub mod graphics_video;
pub mod hostdev;
pub mod input_sound_tpm;
pub mod misc_devices;
pub mod network;
pub mod nvram;
pub mod serial_console;
pub mod smartcard;

pub use common::*;
// 以下导入用于 re-export，使得 crate::model::X 可用
#[allow(unused_imports)]
pub use controller::ControllerConfig;
#[allow(unused_imports)]
pub use disk::{
    BlockIOConfig, DiskConfig, DiskDriver, DiskSource, DiskTarget, EncryptionConfig,
    EncryptionSecret, GeometryConfig, IOtuneConfig, ThrottleFilter,
};
#[allow(unused_imports)]
pub use filesystem::FilesystemConfig;
#[allow(unused_imports)]
pub use graphics_video::{GraphicsConfig, VideoConfig, VideoModel};
#[allow(unused_imports)]
pub use hostdev::{
    HostdevACPI, HostdevAdapter, HostdevAddress, HostdevAuth, HostdevConfig, HostdevDriver,
    HostdevHost, HostdevIOMMUFD, HostdevIQN, HostdevInitiator, HostdevProduct, HostdevROM,
    HostdevSCSIAddress, HostdevSecret, HostdevSource, HostdevVendor,
};
#[allow(unused_imports)]
pub use input_sound_tpm::{
    InputConfig, MemballoonConfig, RngConfig, SoundConfig, TPMBackend, TPMConfig, WatchdogConfig,
};
#[allow(unused_imports)]
pub use misc_devices::{
    AudioConfig, AudioStream, CryptoBackend, CryptoConfig, HubConfig, IommuConfig,
    MemoryDeviceConfig, MemoryDeviceTarget, PanicConfig, PstoreConfig, ShmemConfig, ShmemModel,
    SizeConfig, VsockConfig, VsockSource,
};
#[allow(unused_imports)]
pub use network::{InterfaceConfig, InterfaceModel, InterfaceSource, MacAddress};
#[allow(unused_imports)]
pub use nvram::NVRAMConfig;
#[allow(unused_imports)]
pub use serial_console::{
    ChannelConfig, ChannelTarget, ConsoleConfig, ParallelConfig, SerialConfig,
};
#[allow(unused_imports)]
pub use smartcard::{SmartcardConfig, SmartcardProtocol, SmartcardSource};

pub mod common;
pub mod controller;
pub mod disk;
pub mod filesystem;
pub mod graphics_video;
pub mod hostdev;
pub mod input_sound_tpm;
pub mod lease;
pub mod misc_devices;
pub mod network;
pub mod nvram;
pub mod redirdev;
pub mod serial_console;
pub mod smartcard;

// 以下导入用于 re-export，使得 crate::model::X 可用
#[allow(unused_imports)]
pub use common::{
    AddressConfig, AliasConfig, BootOrderConfig, DevicesConfig, OemString, SMBIOSBlock,
    SMBIOSEntry, SysInfoConfig, SysInfoEntry,
};
#[allow(unused_imports)]
pub use controller::{ControllerConfig, ControllerDriver, ControllerHotplug};
#[allow(unused_imports)]
pub use disk::{
    BlockIOConfig, DiskConfig, DiskDriver, DiskSource, DiskTarget, EncryptionConfig,
    EncryptionSecret, GeometryConfig, HistogramBin, IOtuneConfig, IOtuneMaxConfig,
    LatencyHistogramConfig, MetadataCacheConfig, StatisticInterval, ThrottleFilter,
};
#[allow(unused_imports)]
pub use filesystem::{
    FilesystemBinary, FilesystemCache, FilesystemConfig, FilesystemDriver, FilesystemLock,
    FilesystemSandbox, FilesystemSource, FilesystemTarget, FilesystemThreadPool,
};
#[allow(unused_imports)]
pub use graphics_video::{
    AccelerationConfig, ChannelPolicyConfig, ClipboardConfig, FileTransferConfig, GlConfig,
    GraphicsAudioConfig, GraphicsConfig, ImageConfig, ListenConfig, MouseConfig, ResolutionConfig,
    StreamingConfig, VideoConfig, VideoDriverConfig, VideoModel,
};
#[allow(unused_imports)]
pub use hostdev::{
    HostdevACPI, HostdevAdapter, HostdevAddress, HostdevAuth, HostdevConfig, HostdevDriver,
    HostdevHost, HostdevIOMMUFD, HostdevIQN, HostdevInitiator, HostdevProduct, HostdevROM,
    HostdevSCSIAddress, HostdevSecret, HostdevSource, HostdevVendor,
};
#[allow(unused_imports)]
pub use input_sound_tpm::{
    InputConfig, InputDriver, InputSource, MemballoonConfig, MemballoonStats, RngBackend,
    RngConfig, RngRate, RngSize, SoundAudio, SoundCodec, SoundConfig, TPMBackend, TPMConfig,
    WatchdogConfig,
};
#[allow(unused_imports)]
pub use lease::{LeaseConfig, LeaseTarget};
#[allow(unused_imports)]
pub use misc_devices::{
    AudioConfig, AudioSource, AudioSourceBackend, AudioStream, CryptoBackend, CryptoConfig,
    HubConfig, IommuConfig, IommuDriver, LabelConfig, MemoryDeviceConfig, MemoryDeviceSource,
    MemoryDeviceTarget, MemoryDeviceTargetAddress, MsiConfig, PanicConfig, PstoreConfig,
    ShmemConfig, ShmemModel, ShmemServer, SizeConfig, VsockConfig, VsockSource,
};
#[allow(unused_imports)]
pub use network::{
    AcpiConfig, BackendConfig, BandwidthConfig, DirectionConfig, DriverConfig, DriverGuestOffload,
    DriverHostOffload, GuestConfig, InterfaceConfig, InterfaceModel, InterfaceSource, IpConfig,
    LinkConfig, MacAddress, PortConfig, PortForwardConfig, PortRangeConfig, RomConfig, TuneConfig,
    VirtualPortConfig, VirtualPortParameters, VlanConfig, VlanTag,
};
#[allow(unused_imports)]
pub use nvram::DeviceNVRAMConfig;
#[allow(unused_imports)]
pub use redirdev::{
    RedirdevConfig, RedirdevProtocol, RedirdevSource, RedirfilterConfig, UsbDevFilter,
};
#[allow(unused_imports)]
pub use serial_console::{
    ChannelConfig, ChannelTarget, ConsoleConfig, ConsoleLog, ConsoleSource, ParallelConfig,
    ParallelSource, SerialConfig, SerialLog, SerialSource,
};
#[allow(unused_imports)]
pub use smartcard::{SmartcardConfig, SmartcardProtocol, SmartcardSource};

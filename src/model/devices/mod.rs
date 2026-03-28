pub mod common;
pub mod controller;
pub mod disk;
pub mod filesystem;
pub mod graphics_video;
pub mod input_sound_tpm;
pub mod network;
pub mod serial_console;

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
pub use input_sound_tpm::{
    InputConfig, MemballoonConfig, RngConfig, SoundConfig, TPMBackend, TPMConfig, WatchdogConfig,
};
#[allow(unused_imports)]
pub use network::{InterfaceConfig, InterfaceModel, InterfaceSource, MacAddress};
#[allow(unused_imports)]
pub use serial_console::{
    ChannelConfig, ChannelTarget, ConsoleConfig, ParallelConfig, SerialConfig,
};

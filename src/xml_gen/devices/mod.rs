use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use super::general::write_element;
use crate::model::VMConfig;

pub mod channel_watchdog_rng;
pub mod controller;
pub mod disk;
pub mod filesystem;
pub mod graphics_video;
pub mod hostdev;
pub mod input_sound_tpm;
pub mod misc_devices;
pub mod network;
pub mod serial_console;

/// 写入设备配置（devices 部分）
#[allow(clippy::too_many_arguments)]
pub fn write_devices<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    let devices_elem = BytesStart::new("devices");
    writer.write_event(Event::Start(devices_elem)).map_err(|e| e.to_string())?;

    if let Some(ref emulator) = config.devices.emulator {
        write_element(writer, "emulator", emulator)?;
    }

    // Graphics 和 Video 设备
    if let Some(ref graphics_list) = config.devices.graphics {
        graphics_video::write_graphics(writer, graphics_list)?;
    }
    if let Some(ref video_list) = config.devices.video {
        graphics_video::write_video(writer, video_list)?;
    }

    // Disk 设备
    if let Some(ref disk_list) = config.devices.disk {
        disk::write_disks(writer, disk_list)?;
    }

    // Network 设备
    if let Some(ref iface_list) = config.devices.interface {
        network::write_interfaces(writer, iface_list)?;
    }

    // Input 设备
    if let Some(ref input_list) = config.devices.input {
        input_sound_tpm::write_inputs(writer, input_list)?;
    }

    // TPM 设备
    input_sound_tpm::write_tpm(writer, config.devices.tpm.as_ref())?;

    // Sound 设备
    if let Some(ref sound_list) = config.devices.sound {
        input_sound_tpm::write_sounds(writer, sound_list)?;
    }

    // Filesystem 设备
    if let Some(ref filesystem_list) = config.devices.filesystem {
        filesystem::write_filesystems(writer, filesystem_list)?;
    }

    // Controller 设备
    if let Some(ref controller_list) = config.devices.controller {
        controller::write_controllers(writer, controller_list)?;
    }

    // Serial/Parallel/Console 设备
    if let Some(ref serial_list) = config.devices.serial {
        serial_console::write_serials(writer, serial_list)?;
    }
    if let Some(ref parallel_list) = config.devices.parallel {
        serial_console::write_parallels(writer, parallel_list)?;
    }
    serial_console::write_console(writer, config.devices.console.as_ref())?;

    // Channel/Watchdog/Rng/Memballoon 设备
    if let Some(ref channel_list) = config.devices.channel {
        channel_watchdog_rng::write_channels(writer, channel_list)?;
    }
    channel_watchdog_rng::write_watchdog(writer, config.devices.watchdog.as_ref())?;
    if let Some(ref rng_list) = config.devices.rng {
        channel_watchdog_rng::write_rngs(writer, rng_list)?;
    }
    channel_watchdog_rng::write_memballoon(writer, config.devices.memballoon.as_ref())?;

    // Hub 设备
    if let Some(ref hub_list) = config.devices.hub {
        misc_devices::write_hubs(writer, hub_list)?;
    }

    // Panic 设备
    if let Some(ref panic) = config.devices.panic {
        misc_devices::write_panic(writer, panic)?;
    }

    // Shmem 设备
    if let Some(ref shmem_list) = config.devices.shmem {
        misc_devices::write_shmems(writer, shmem_list)?;
    }

    // Memory Device 设备
    if let Some(ref memory_device_list) = config.devices.memory_device {
        misc_devices::write_memory_devices(writer, memory_device_list)?;
    }

    // IOMMU 设备
    if let Some(ref iommu) = config.devices.iommu {
        misc_devices::write_iommu(writer, iommu)?;
    }

    // Vsock 设备
    if let Some(ref vsock) = config.devices.vsock {
        misc_devices::write_vsock(writer, vsock)?;
    }

    // Crypto 设备
    if let Some(ref crypto) = config.devices.crypto {
        misc_devices::write_crypto(writer, crypto)?;
    }

    // Pstore 设备
    if let Some(ref pstore) = config.devices.pstore {
        misc_devices::write_pstore(writer, pstore)?;
    }

    // Audio 设备
    if let Some(ref audio) = config.devices.audio {
        misc_devices::write_audio(writer, audio)?;
    }

    // Hostdev 设备（主机设备直通）
    if let Some(ref hostdev_list) = config.devices.hostdev {
        hostdev::write_hostdevs(writer, hostdev_list)?;
    }

    writer.write_event(Event::End(BytesEnd::new("devices"))).map_err(|e| e.to_string())?;

    Ok(())
}

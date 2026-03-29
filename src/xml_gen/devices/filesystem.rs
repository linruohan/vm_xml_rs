use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};

use crate::{error::AppError, model::devices::FilesystemConfig};

/// 写入 Filesystem 设备
pub fn write_filesystems<W: std::io::Write>(
    writer: &mut Writer<W>,
    filesystem_list: &[FilesystemConfig],
) -> Result<(), AppError> {
    for filesystem in filesystem_list {
        let mut fs_elem = BytesStart::new("filesystem");
        fs_elem.push_attribute(("type", filesystem.fs_type.as_str()));
        if let Some(ref accessmode) = filesystem.accessmode {
            fs_elem.push_attribute(("accessmode", accessmode.as_str()));
        }
        if let Some(ref multidevs) = filesystem.multidevs {
            fs_elem.push_attribute(("multidevs", multidevs.as_str()));
        }
        if let Some(ref fmode) = filesystem.fmode {
            fs_elem.push_attribute(("fmode", fmode.as_str()));
        }
        if let Some(ref dmode) = filesystem.dmode {
            fs_elem.push_attribute(("dmode", dmode.as_str()));
        }
        writer.write_event(Event::Start(fs_elem)).map_err(|e| e.to_string())?;

        if let Some(ref driver) = filesystem.driver {
            let mut driver_elem = BytesStart::new("driver");
            driver_elem.push_attribute(("type", driver.driver_type.as_str()));
            if let Some(ref format) = driver.format {
                driver_elem.push_attribute(("format", format.as_str()));
            }
            if let Some(ref wrpolicy) = driver.wrpolicy {
                driver_elem.push_attribute(("wrpolicy", wrpolicy.as_str()));
            }
            if let Some(queue) = driver.queue {
                driver_elem.push_attribute(("queue", queue.to_string().as_str()));
            }
            writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref binary) = filesystem.binary {
            let mut binary_elem = BytesStart::new("binary");
            if let Some(ref path) = binary.path {
                binary_elem.push_attribute(("path", path.as_str()));
            }
            if let Some(ref xattr) = binary.xattr {
                binary_elem.push_attribute(("xattr", xattr.as_str()));
            }
            writer.write_event(Event::Start(binary_elem)).map_err(|e| e.to_string())?;

            if let Some(ref cache) = binary.cache {
                let mut cache_elem = BytesStart::new("cache");
                cache_elem.push_attribute(("mode", cache.mode.as_str()));
                writer.write_event(Event::Empty(cache_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref sandbox) = binary.sandbox {
                let mut sandbox_elem = BytesStart::new("sandbox");
                sandbox_elem.push_attribute(("mode", sandbox.mode.as_str()));
                writer.write_event(Event::Empty(sandbox_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref lock) = binary.lock {
                let mut lock_elem = BytesStart::new("lock");
                if let Some(ref posix) = lock.posix {
                    lock_elem.push_attribute(("posix", posix.as_str()));
                }
                if let Some(ref flock) = lock.flock {
                    lock_elem.push_attribute(("flock", flock.as_str()));
                }
                writer.write_event(Event::Empty(lock_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref thread_pool) = binary.thread_pool {
                let mut thread_pool_elem = BytesStart::new("thread_pool");
                thread_pool_elem.push_attribute(("size", thread_pool.size.to_string().as_str()));
                writer.write_event(Event::Empty(thread_pool_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("binary"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref source) = filesystem.source {
            let mut source_elem = BytesStart::new("source");
            if let Some(ref name) = source.name {
                source_elem.push_attribute(("name", name.as_str()));
            }
            if let Some(ref dir) = source.dir {
                source_elem.push_attribute(("dir", dir.as_str()));
            }
            if let Some(ref file) = source.file {
                source_elem.push_attribute(("file", file.as_str()));
            }
            if let Some(ref socket) = source.socket {
                source_elem.push_attribute(("socket", socket.as_str()));
            }
            if let Some(ref usage) = source.usage {
                source_elem.push_attribute(("usage", usage.as_str()));
            }
            if let Some(ref units) = source.units {
                source_elem.push_attribute(("units", units.as_str()));
            }
            writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref target) = filesystem.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("dir", target.dir.as_str()));
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref idmap) = filesystem.idmap {
            let idmap_elem = BytesStart::new("idmap");
            writer.write_event(Event::Start(idmap_elem)).map_err(|e| e.to_string())?;

            if let Some(ref uid) = idmap.uid {
                let mut uid_elem = BytesStart::new("uid");
                uid_elem.push_attribute(("start", uid.start.to_string().as_str()));
                uid_elem.push_attribute(("target", uid.target.to_string().as_str()));
                uid_elem.push_attribute(("count", uid.count.to_string().as_str()));
                writer.write_event(Event::Empty(uid_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref gid) = idmap.gid {
                let mut gid_elem = BytesStart::new("gid");
                gid_elem.push_attribute(("start", gid.start.to_string().as_str()));
                gid_elem.push_attribute(("target", gid.target.to_string().as_str()));
                gid_elem.push_attribute(("count", gid.count.to_string().as_str()));
                writer.write_event(Event::Empty(gid_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("idmap"))).map_err(|e| e.to_string())?;
        }

        if filesystem.readonly.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("readonly")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref space_hard_limit) = filesystem.space_hard_limit {
            let mut limit_elem = BytesStart::new("space_hard_limit");
            limit_elem.push_attribute(("value", space_hard_limit.to_string().as_str()));
            writer.write_event(Event::Empty(limit_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref space_soft_limit) = filesystem.space_soft_limit {
            let mut limit_elem = BytesStart::new("space_soft_limit");
            limit_elem.push_attribute(("value", space_soft_limit.to_string().as_str()));
            writer.write_event(Event::Empty(limit_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("filesystem"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use super::super::general::write_element;
use crate::model::devices::DiskConfig;

/// 写入 Disk 设备
pub fn write_disks<W: std::io::Write>(
    writer: &mut Writer<W>,
    disk_list: &[DiskConfig],
) -> Result<(), String> {
    for disk in disk_list {
        let mut disk_elem = BytesStart::new("disk");
        disk_elem.push_attribute(("type", disk.disk_type.as_str()));
        disk_elem.push_attribute(("device", disk.device.as_str()));
        if let Some(ref snapshot) = disk.snapshot {
            disk_elem.push_attribute(("snapshot", snapshot.as_str()));
        }
        writer.write_event(Event::Start(disk_elem)).map_err(|e| e.to_string())?;

        if let Some(ref alias) = disk.alias {
            let mut alias_elem = BytesStart::new("alias");
            alias_elem.push_attribute(("name", alias.name.as_str()));
            writer.write_event(Event::Empty(alias_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref driver) = disk.driver {
            let mut driver_elem = BytesStart::new("driver");
            driver_elem.push_attribute(("name", driver.name.as_str()));
            driver_elem.push_attribute(("type", driver.driver_type.as_str()));
            if let Some(ref cache) = driver.cache {
                driver_elem.push_attribute(("cache", cache.as_str()));
            }
            if let Some(ref io) = driver.io {
                driver_elem.push_attribute(("io", io.as_str()));
            }
            if let Some(ref ioeventfd) = driver.ioeventfd {
                driver_elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
            }
            if let Some(ref event_idx) = driver.event_idx {
                driver_elem.push_attribute(("event_idx", event_idx.as_str()));
            }
            if let Some(queues) = driver.queues {
                driver_elem.push_attribute(("queues", queues.to_string().as_str()));
            }
            if let Some(queue_size) = driver.queue_size {
                driver_elem.push_attribute(("queue_size", queue_size.to_string().as_str()));
            }
            if let Some(iothread) = driver.iothread {
                driver_elem.push_attribute(("iothread", iothread.to_string().as_str()));
            }
            if let Some(ref discard_no_unref) = driver.discard_no_unref {
                driver_elem.push_attribute(("discard_no_unref", discard_no_unref.as_str()));
            }

            // 写入 iothreads 子元素
            if let Some(ref iothreads) = driver.iothreads {
                writer.write_event(Event::Start(driver_elem)).map_err(|e| e.to_string())?;

                let iothreads_elem = BytesStart::new("iothreads");
                writer.write_event(Event::Start(iothreads_elem)).map_err(|e| e.to_string())?;
                for iothread in iothreads {
                    let mut iothread_elem = BytesStart::new("iothread");
                    iothread_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                    writer.write_event(Event::Empty(iothread_elem)).map_err(|e| e.to_string())?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new("iothreads")))
                    .map_err(|e| e.to_string())?;

                // 写入 statistics 子元素
                if let Some(ref statistics) = driver.statistics {
                    let statistics_elem = BytesStart::new("statistics");
                    writer.write_event(Event::Start(statistics_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref statistic_list) = statistics.statistic {
                        for statistic in statistic_list {
                            let mut statistic_elem = BytesStart::new("statistic");
                            statistic_elem.push_attribute((
                                "interval",
                                statistic.interval.to_string().as_str(),
                            ));
                            writer
                                .write_event(Event::Empty(statistic_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    if let Some(ref latency_histogram_list) = statistics.latency_histogram {
                        for histogram in latency_histogram_list {
                            write_latency_histogram(writer, histogram)?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("statistics")))
                        .map_err(|e| e.to_string())?;
                }

                // 写入 latency-histogram 子元素（直接在 driver 下）
                if let Some(ref latency_histogram_list) = driver.latency_histogram {
                    for histogram in latency_histogram_list {
                        write_latency_histogram(writer, histogram)?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("driver")))
                    .map_err(|e| e.to_string())?;
            } else if driver.statistics.is_some() || driver.latency_histogram.is_some() {
                // 没有 iothreads 但有 statistics 或 latency_histogram
                writer.write_event(Event::Start(driver_elem)).map_err(|e| e.to_string())?;

                if let Some(ref statistics) = driver.statistics {
                    let statistics_elem = BytesStart::new("statistics");
                    writer.write_event(Event::Start(statistics_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref statistic_list) = statistics.statistic {
                        for statistic in statistic_list {
                            let mut statistic_elem = BytesStart::new("statistic");
                            statistic_elem.push_attribute((
                                "interval",
                                statistic.interval.to_string().as_str(),
                            ));
                            writer
                                .write_event(Event::Empty(statistic_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    if let Some(ref latency_histogram_list) = statistics.latency_histogram {
                        for histogram in latency_histogram_list {
                            write_latency_histogram(writer, histogram)?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("statistics")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref latency_histogram_list) = driver.latency_histogram {
                    for histogram in latency_histogram_list {
                        write_latency_histogram(writer, histogram)?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("driver")))
                    .map_err(|e| e.to_string())?;
            } else {
                writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref source) = disk.source {
            let mut source_elem = BytesStart::new("source");
            if let Some(ref file) = source.file {
                source_elem.push_attribute(("file", file.as_str()));
            }
            if let Some(ref dev) = source.dev {
                source_elem.push_attribute(("dev", dev.as_str()));
            }
            if let Some(ref protocol) = source.protocol {
                source_elem.push_attribute(("protocol", protocol.as_str()));
            }
            if let Some(ref name) = source.name {
                source_elem.push_attribute(("name", name.as_str()));
            }
            if let Some(ref startup_policy) = source.startup_policy {
                source_elem.push_attribute(("startupPolicy", startup_policy.as_str()));
            }
            writer.write_event(Event::Start(source_elem)).map_err(|e| e.to_string())?;

            if let Some(ref seclabel_list) = source.seclabel {
                for seclabel in seclabel_list {
                    let mut seclabel_elem = BytesStart::new("seclabel");
                    if let Some(ref relabel) = seclabel.relabel {
                        seclabel_elem.push_attribute(("relabel", relabel.as_str()));
                    }
                    writer.write_event(Event::Empty(seclabel_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref host) = source.host {
                let mut host_elem = BytesStart::new("host");
                host_elem.push_attribute(("name", host.name.as_str()));
                if let Some(ref port) = host.port {
                    host_elem.push_attribute(("port", port.to_string().as_str()));
                }
                writer.write_event(Event::Empty(host_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref auth) = source.auth {
                let mut auth_elem = BytesStart::new("auth");
                if let Some(ref username) = auth.username {
                    auth_elem.push_attribute(("username", username.as_str()));
                }
                writer.write_event(Event::Start(auth_elem)).map_err(|e| e.to_string())?;

                if let Some(ref secret) = auth.secret {
                    let mut secret_elem = BytesStart::new("secret");
                    secret_elem.push_attribute(("type", secret.secret_type.as_str()));
                    secret_elem.push_attribute(("usage", secret.usage.as_str()));
                    writer.write_event(Event::Empty(secret_elem)).map_err(|e| e.to_string())?;
                }

                writer.write_event(Event::End(BytesEnd::new("auth"))).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("source"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref target) = disk.target {
            let mut target_elem = BytesStart::new("target");
            target_elem.push_attribute(("dev", target.dev.as_str()));
            if let Some(ref bus) = target.bus {
                target_elem.push_attribute(("bus", bus.as_str()));
            }
            if let Some(ref tray) = target.tray {
                target_elem.push_attribute(("tray", tray.as_str()));
            }
            if let Some(rotation_rate) = target.rotation_rate {
                target_elem.push_attribute(("rotation_rate", rotation_rate.to_string().as_str()));
            }
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref boot) = disk.boot {
            let mut boot_elem = BytesStart::new("boot");
            boot_elem.push_attribute(("order", boot.order.to_string().as_str()));
            writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
        }

        if disk.readonly.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("readonly")))
                .map_err(|e| e.to_string())?;
        }

        if disk.shareable.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("shareable")))
                .map_err(|e| e.to_string())?;
        }

        if disk.transient.is_some() {
            writer
                .write_event(Event::Empty(BytesStart::new("transient")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref encryption) = disk.encryption {
            let mut encryption_elem = BytesStart::new("encryption");
            encryption_elem.push_attribute(("type", encryption.encryption_type.as_str()));
            writer.write_event(Event::Start(encryption_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("encryption")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref serial) = disk.serial {
            let serial_elem = BytesStart::new("serial");
            writer.write_event(Event::Start(serial_elem)).map_err(|e| e.to_string())?;
            writer.write_event(Event::Text(BytesText::new(serial))).map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("serial"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref wwn) = disk.wwn {
            let wwn_elem = BytesStart::new("wwn");
            writer.write_event(Event::Start(wwn_elem)).map_err(|e| e.to_string())?;
            writer.write_event(Event::Text(BytesText::new(wwn))).map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("wwn"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref vendor) = disk.vendor {
            let vendor_elem = BytesStart::new("vendor");
            writer.write_event(Event::Start(vendor_elem)).map_err(|e| e.to_string())?;
            writer.write_event(Event::Text(BytesText::new(vendor))).map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("vendor"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref geometry) = disk.geometry {
            let mut geometry_elem = BytesStart::new("geometry");
            geometry_elem.push_attribute(("cyls", geometry.cyls.to_string().as_str()));
            geometry_elem.push_attribute(("heads", geometry.heads.to_string().as_str()));
            geometry_elem.push_attribute(("secs", geometry.secs.to_string().as_str()));
            if let Some(ref trans) = geometry.trans {
                geometry_elem.push_attribute(("trans", trans.as_str()));
            }
            writer.write_event(Event::Empty(geometry_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref blockio) = disk.blockio {
            let mut blockio_elem = BytesStart::new("blockio");
            if let Some(logical_block_size) = blockio.logical_block_size {
                blockio_elem.push_attribute((
                    "logical_block_size",
                    logical_block_size.to_string().as_str(),
                ));
            }
            if let Some(physical_block_size) = blockio.physical_block_size {
                blockio_elem.push_attribute((
                    "physical_block_size",
                    physical_block_size.to_string().as_str(),
                ));
            }
            if let Some(discard_granularity) = blockio.discard_granularity {
                blockio_elem.push_attribute((
                    "discard_granularity",
                    discard_granularity.to_string().as_str(),
                ));
            }
            writer.write_event(Event::Empty(blockio_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref iotune) = disk.iotune {
            let iotune_elem = BytesStart::new("iotune");
            writer.write_event(Event::Start(iotune_elem)).map_err(|e| e.to_string())?;

            if let Some(total_bytes_sec) = iotune.total_bytes_sec {
                write_element(writer, "total_bytes_sec", &total_bytes_sec.to_string())?;
            }
            if let Some(read_bytes_sec) = iotune.read_bytes_sec {
                write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
            }
            if let Some(write_bytes_sec) = iotune.write_bytes_sec {
                write_element(writer, "write_bytes_sec", &write_bytes_sec.to_string())?;
            }
            if let Some(total_iops_sec) = iotune.total_iops_sec {
                write_element(writer, "total_iops_sec", &total_iops_sec.to_string())?;
            }
            if let Some(read_iops_sec) = iotune.read_iops_sec {
                write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
            }
            if let Some(write_iops_sec) = iotune.write_iops_sec {
                write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("iotune"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref backenddomain) = disk.backenddomain {
            let mut backenddomain_elem = BytesStart::new("backenddomain");
            backenddomain_elem.push_attribute(("name", backenddomain.as_str()));
            writer.write_event(Event::Empty(backenddomain_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref throttlefilters) = disk.throttlefilters {
            let throttlefilters_elem = BytesStart::new("throttlefilters");
            writer.write_event(Event::Start(throttlefilters_elem)).map_err(|e| e.to_string())?;

            for filter in throttlefilters {
                let mut filter_elem = BytesStart::new("throttlefilter");
                filter_elem.push_attribute(("group", filter.group.as_str()));
                writer.write_event(Event::Empty(filter_elem)).map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("throttlefilters")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref address) = disk.address {
            let mut address_elem = BytesStart::new("address");
            address_elem.push_attribute(("type", address.address_type.as_str()));
            if let Some(controller) = address.controller {
                address_elem.push_attribute(("controller", controller.to_string().as_str()));
            }
            if let Some(bus) = address.bus {
                address_elem.push_attribute(("bus", bus.to_string().as_str()));
            }
            if let Some(target) = address.target {
                address_elem.push_attribute(("target", target.to_string().as_str()));
            }
            if let Some(unit) = address.unit {
                address_elem.push_attribute(("unit", unit.to_string().as_str()));
            }
            if let Some(slot) = address.slot {
                address_elem.push_attribute(("slot", slot.to_string().as_str()));
            }
            if let Some(function) = address.function {
                address_elem.push_attribute(("function", function.to_string().as_str()));
            }
            if let Some(ref domain) = address.domain {
                address_elem.push_attribute(("domain", domain.as_str()));
            }
            if let Some(ref multifunction) = address.multifunction {
                address_elem.push_attribute(("multifunction", multifunction.as_str()));
            }
            if let Some(ref reg) = address.reg {
                address_elem.push_attribute(("reg", reg.as_str()));
            }
            if let Some(ref cssid) = address.cssid {
                address_elem.push_attribute(("cssid", cssid.as_str()));
            }
            if let Some(ssid) = address.ssid {
                address_elem.push_attribute(("ssid", ssid.to_string().as_str()));
            }
            if let Some(ref devno) = address.devno {
                address_elem.push_attribute(("devno", devno.as_str()));
            }
            if let Some(iobase) = address.iobase {
                address_elem.push_attribute(("iobase", iobase.to_string().as_str()));
            }
            if let Some(irq) = address.irq {
                address_elem.push_attribute(("irq", irq.to_string().as_str()));
            }
            writer.write_event(Event::Empty(address_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("disk"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 写入 latency-histogram 元素
fn write_latency_histogram<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &crate::model::devices::disk::LatencyHistogramConfig,
) -> Result<(), String> {
    let mut histogram_elem = BytesStart::new("latency-histogram");
    if let Some(ref histogram_type) = config.histogram_type {
        histogram_elem.push_attribute(("type", histogram_type.as_str()));
    }
    writer.write_event(Event::Start(histogram_elem)).map_err(|e| e.to_string())?;

    if let Some(ref bin_list) = config.bin {
        for bin in bin_list {
            let mut bin_elem = BytesStart::new("bin");
            bin_elem.push_attribute(("start", bin.start.to_string().as_str()));
            writer.write_event(Event::Empty(bin_elem)).map_err(|e| e.to_string())?;
        }
    }

    writer
        .write_event(Event::End(BytesEnd::new("latency-histogram")))
        .map_err(|e| e.to_string())?;
    Ok(())
}

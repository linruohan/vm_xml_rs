use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{error::AppError, model::devices::ControllerConfig};

/// 写入 Controller 设备
pub fn write_controllers<W: std::io::Write>(
    writer: &mut Writer<W>,
    controller_list: &[ControllerConfig],
) -> Result<(), AppError> {
    for controller in controller_list {
        let mut controller_elem = BytesStart::new("controller");
        controller_elem.push_attribute(("type", controller.controller_type.as_str()));
        if let Some(ref index) = controller.index {
            controller_elem.push_attribute(("index", index.to_string().as_str()));
        }
        if let Some(ref model) = controller.model {
            controller_elem.push_attribute(("model", model.as_str()));
        }
        if let Some(ref ports) = controller.ports {
            controller_elem.push_attribute(("ports", ports.to_string().as_str()));
        }
        if let Some(ref vectors) = controller.vectors {
            controller_elem.push_attribute(("vectors", vectors.to_string().as_str()));
        }
        if let Some(ref max_grant_frames) = controller.max_grant_frames {
            controller_elem
                .push_attribute(("maxGrantFrames", max_grant_frames.to_string().as_str()));
        }
        if let Some(ref max_event_channels) = controller.max_event_channels {
            controller_elem
                .push_attribute(("maxEventChannels", max_event_channels.to_string().as_str()));
        }
        writer.write_event(Event::Start(controller_elem)).map_err(|e| e.to_string())?;

        if let Some(ref driver) = controller.driver {
            let mut driver_elem = BytesStart::new("driver");
            if let Some(ref queues) = driver.queues {
                driver_elem.push_attribute(("queues", queues.to_string().as_str()));
            }
            if let Some(ref cmd_per_lun) = driver.cmd_per_lun {
                driver_elem.push_attribute(("cmd_per_lun", cmd_per_lun.to_string().as_str()));
            }
            if let Some(ref max_sectors) = driver.max_sectors {
                driver_elem.push_attribute(("max_sectors", max_sectors.to_string().as_str()));
            }
            if let Some(ref ioeventfd) = driver.ioeventfd {
                driver_elem.push_attribute(("ioeventfd", ioeventfd.as_str()));
            }
            if let Some(ref iothread) = driver.iothread {
                driver_elem.push_attribute(("iothread", iothread.to_string().as_str()));
            }
            writer.write_event(Event::Start(driver_elem)).map_err(|e| e.to_string())?;

            if let Some(ref iothreads) = driver.iothreads {
                let iothreads_elem = BytesStart::new("iothreads");
                writer.write_event(Event::Start(iothreads_elem)).map_err(|e| e.to_string())?;

                for iothread in iothreads {
                    let mut iothread_elem = BytesStart::new("iothread");
                    iothread_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                    writer.write_event(Event::Start(iothread_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref queues) = iothread.queues {
                        for queue in queues {
                            let mut queue_elem = BytesStart::new("queue");
                            queue_elem.push_attribute(("id", queue.id.to_string().as_str()));
                            writer
                                .write_event(Event::Empty(queue_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("iothread")))
                        .map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("iothreads")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("driver"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref master) = controller.master {
            let mut master_elem = BytesStart::new("master");
            if let Some(ref startport) = master.startport {
                master_elem.push_attribute(("startport", startport.to_string().as_str()));
            }
            writer.write_event(Event::Empty(master_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref model_config) = controller.model_elem {
            let mut model_elem = BytesStart::new("model");
            model_elem.push_attribute(("name", model_config.name.as_str()));
            writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref target) = controller.target {
            let mut target_elem = BytesStart::new("target");
            if let Some(ref target_type) = target.target_type {
                target_elem.push_attribute(("type", target_type.as_str()));
            }
            if let Some(chassis_nr) = target.chassis_nr {
                target_elem.push_attribute(("chassisNr", chassis_nr.to_string().as_str()));
            }
            if let Some(chassis) = target.chassis {
                target_elem.push_attribute(("chassis", chassis.to_string().as_str()));
            }
            if let Some(port) = target.port {
                target_elem.push_attribute(("port", port.to_string().as_str()));
            }
            writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref hotplug) = controller.hotplug {
            let mut hotplug_elem = BytesStart::new("hotplug");
            hotplug_elem.push_attribute(("enabled", hotplug.enabled.as_str()));
            writer.write_event(Event::Empty(hotplug_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref pcihole64) = controller.pcihole64 {
            let mut pcihole64_elem = BytesStart::new("pcihole64");
            if let Some(ref unit) = pcihole64.unit {
                pcihole64_elem.push_attribute(("unit", unit.as_str()));
            }
            if let Some(value) = pcihole64.value {
                pcihole64_elem.push_attribute(("value", value.to_string().as_str()));
            }
            writer.write_event(Event::Empty(pcihole64_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref serial) = controller.serial {
            let serial_elem = BytesStart::new("serial");
            writer.write_event(Event::Start(serial_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(serial.as_str())))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("serial"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref address) = controller.address {
            let mut address_elem = BytesStart::new("address");
            address_elem.push_attribute(("type", address.address_type.as_str()));
            if let Some(controller_id) = address.controller {
                address_elem.push_attribute(("controller", controller_id.to_string().as_str()));
            }
            if let Some(bus) = address.bus {
                address_elem.push_attribute(("bus", bus.to_string().as_str()));
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
            writer.write_event(Event::Empty(address_elem)).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("controller"))).map_err(|e| e.to_string())?;
    }
    Ok(())
}

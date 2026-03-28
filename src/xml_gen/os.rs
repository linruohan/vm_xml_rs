use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use super::general::write_element;
use crate::model::VMConfig;

/// 写入系统引导配置（os 部分）
pub fn write_os<W: std::io::Write>(
    writer: &mut Writer<W>,
    config: &VMConfig,
) -> Result<(), String> {
    if let Some(ref os) = config.general.os {
        let mut os_elem = BytesStart::new("os");
        if let Some(ref firmware) = os.firmware {
            os_elem.push_attribute(("firmware", firmware.as_str()));
        }
        writer.write_event(Event::Start(os_elem)).map_err(|e| e.to_string())?;

        {
            let mut type_elem = BytesStart::new("type");
            if let Some(ref arch) = os.arch {
                type_elem.push_attribute(("arch", arch.as_str()));
            }
            if let Some(ref machine) = os.machine {
                type_elem.push_attribute(("machine", machine.as_str()));
            }
            writer.write_event(Event::Start(type_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&os.os_type)))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("type"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref loader) = os.loader {
            let mut loader_elem = BytesStart::new("loader");
            if let Some(ref readonly) = loader.readonly {
                loader_elem.push_attribute(("readonly", readonly.as_str()));
            }
            if let Some(ref loader_type) = loader.loader_type {
                loader_elem.push_attribute(("type", loader_type.as_str()));
            }
            if let Some(ref secure) = loader.secure {
                loader_elem.push_attribute(("secure", secure.as_str()));
            }
            if let Some(ref stateless) = loader.stateless {
                loader_elem.push_attribute(("stateless", stateless.as_str()));
            }
            if let Some(ref format) = loader.format {
                loader_elem.push_attribute(("format", format.as_str()));
            }
            writer.write_event(Event::Start(loader_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&loader.path)))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("loader"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref nvram_list) = os.nvram {
            for nvram in nvram_list {
                let mut nvram_elem = BytesStart::new("nvram");
                if let Some(ref template) = nvram.template {
                    nvram_elem.push_attribute(("template", template.as_str()));
                }
                if let Some(ref template_format) = nvram.template_format {
                    nvram_elem.push_attribute(("templateFormat", template_format.as_str()));
                }
                if let Some(ref nvram_type) = nvram.nvram_type {
                    nvram_elem.push_attribute(("type", nvram_type.as_str()));
                }

                if let Some(ref path) = nvram.path {
                    writer.write_event(Event::Start(nvram_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(path)))
                        .map_err(|e| e.to_string())?;
                } else {
                    writer.write_event(Event::Start(nvram_elem)).map_err(|e| e.to_string())?;
                }

                if let Some(ref source) = nvram.source {
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
                    writer.write_event(Event::Start(source_elem)).map_err(|e| e.to_string())?;

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
                        auth_elem.push_attribute(("username", auth.username.as_str()));
                        writer.write_event(Event::Start(auth_elem)).map_err(|e| e.to_string())?;

                        if let Some(ref secret) = auth.secret {
                            let mut secret_elem = BytesStart::new("secret");
                            secret_elem.push_attribute(("type", secret.secret_type.as_str()));
                            secret_elem.push_attribute(("usage", secret.usage.as_str()));
                            writer
                                .write_event(Event::Empty(secret_elem))
                                .map_err(|e| e.to_string())?;
                        }

                        writer
                            .write_event(Event::End(BytesEnd::new("auth")))
                            .map_err(|e| e.to_string())?;
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("source")))
                        .map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("nvram")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref varstore) = os.varstore {
            let mut varstore_elem = BytesStart::new("varstore");
            varstore_elem.push_attribute(("path", varstore.path.as_str()));
            if let Some(ref template) = varstore.template {
                varstore_elem.push_attribute(("template", template.as_str()));
            }
            writer.write_event(Event::Empty(varstore_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref boot_list) = os.boot {
            for boot in boot_list {
                let mut boot_elem = BytesStart::new("boot");
                boot_elem.push_attribute(("dev", boot.dev.as_str()));
                writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref bootmenu) = os.bootmenu {
            let mut bootmenu_elem = BytesStart::new("bootmenu");
            bootmenu_elem.push_attribute(("enable", bootmenu.enable.as_str()));
            if let Some(ref timeout) = bootmenu.timeout {
                bootmenu_elem.push_attribute(("timeout", timeout.to_string().as_str()));
            }
            writer.write_event(Event::Empty(bootmenu_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref smbios) = os.smbios {
            let mut smbios_elem = BytesStart::new("smbios");
            smbios_elem.push_attribute(("mode", smbios.mode.as_str()));
            writer.write_event(Event::Empty(smbios_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref bios) = os.bios {
            let mut bios_elem = BytesStart::new("bios");
            if let Some(ref useserial) = bios.useserial {
                bios_elem.push_attribute(("useserial", useserial.as_str()));
            }
            if let Some(ref reboot_timeout) = bios.reboot_timeout {
                bios_elem.push_attribute(("rebootTimeout", reboot_timeout.to_string().as_str()));
            }
            writer.write_event(Event::Empty(bios_elem)).map_err(|e| e.to_string())?;
        }

        if let Some(ref kernel) = os.kernel {
            write_element(writer, "kernel", kernel)?;
        }

        if let Some(ref initrd) = os.initrd {
            write_element(writer, "initrd", initrd)?;
        }

        if let Some(ref cmdline) = os.cmdline {
            write_element(writer, "cmdline", cmdline)?;
        }

        if let Some(ref shim) = os.shim {
            write_element(writer, "shim", shim)?;
        }

        if let Some(ref dtb) = os.dtb {
            write_element(writer, "dtb", dtb)?;
        }

        if let Some(ref init) = os.init {
            write_element(writer, "init", init)?;
        }

        if let Some(ref initarg_list) = os.initarg {
            for initarg in initarg_list {
                write_element(writer, "initarg", initarg)?;
            }
        }

        if let Some(ref initenv_list) = os.initenv {
            for initenv in initenv_list {
                let mut initenv_elem = BytesStart::new("initenv");
                initenv_elem.push_attribute(("name", initenv.name.as_str()));
                writer.write_event(Event::Start(initenv_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&initenv.value)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("initenv")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref initdir) = os.initdir {
            write_element(writer, "initdir", initdir)?;
        }

        if let Some(ref inituser) = os.inituser {
            write_element(writer, "inituser", inituser)?;
        }

        if let Some(ref initgroup) = os.initgroup {
            write_element(writer, "initgroup", initgroup)?;
        }

        if let Some(ref idmap) = os.idmap {
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

        if let Some(ref acpi) = os.acpi {
            let acpi_elem = BytesStart::new("acpi");
            writer.write_event(Event::Start(acpi_elem)).map_err(|e| e.to_string())?;

            if let Some(ref table_list) = acpi.table {
                for table in table_list {
                    let mut table_elem = BytesStart::new("table");
                    table_elem.push_attribute(("type", table.table_type.as_str()));
                    writer.write_event(Event::Start(table_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(&table.path)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("table")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("acpi"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref feature_list) = os.feature {
            for feature in feature_list {
                let mut feature_elem = BytesStart::new("feature");
                feature_elem.push_attribute(("enabled", feature.enabled.as_str()));
                feature_elem.push_attribute(("name", feature.name.as_str()));
                writer.write_event(Event::Empty(feature_elem)).map_err(|e| e.to_string())?;
            }
        }

        writer.write_event(Event::End(BytesEnd::new("os"))).map_err(|e| e.to_string())?;
    }

    Ok(())
}

use std::io::Cursor;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::vm_config::VMConfig;

pub struct XMLGenerator;

impl XMLGenerator {
    pub fn generate(config: &VMConfig) -> Result<String, String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| format!("写入 XML 声明失败: {}", e))?;

        let mut domain = BytesStart::new("domain");
        domain.push_attribute(("type", config.general.vm_type.as_str()));
        writer
            .write_event(Event::Start(domain))
            .map_err(|e| format!("写入 domain 标签失败: {}", e))?;

        Self::write_general(&mut writer, config)?;
        Self::write_os(&mut writer, config)?;
        Self::write_cpu(&mut writer, config)?;
        Self::write_memory(&mut writer, config)?;
        Self::write_devices(&mut writer, config)?;
        Self::write_advanced(&mut writer, config)?;

        writer
            .write_event(Event::End(BytesEnd::new("domain")))
            .map_err(|e| format!("关闭 domain 标签失败: {}", e))?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).map_err(|e| format!("转换 UTF-8 失败: {}", e))
    }

    fn write_general<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        Self::write_element(writer, "name", &config.general.name)?;

        if let Some(ref uuid) = config.general.uuid {
            Self::write_element(writer, "uuid", uuid)?;
        }

        if let Some(ref hwuuid) = config.general.hwuuid {
            Self::write_element(writer, "hwuuid", hwuuid)?;
        }

        if let Some(ref genid) = config.general.genid {
            Self::write_element(writer, "genid", genid)?;
        }

        if let Some(ref desc) = config.general.description {
            Self::write_element(writer, "description", desc)?;
        }

        if let Some(ref title) = config.general.title {
            Self::write_element(writer, "title", title)?;
        }

        if let Some(ref metadata) = config.general.metadata {
            let metadata_elem = BytesStart::new("metadata");
            writer.write_event(Event::Start(metadata_elem)).map_err(|e| e.to_string())?;

            for entry in &metadata.entries {
                let mut entry_elem =
                    BytesStart::new(entry.xmlns.split(':').next_back().unwrap_or("entry"));
                entry_elem.push_attribute(("xmlns", entry.xmlns.as_str()));
                writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&entry.value)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new(
                        entry.xmlns.split(':').next_back().unwrap_or("entry"),
                    )))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("metadata"))).map_err(|e| e.to_string())?;
        }

        {
            let mut mem_elem = BytesStart::new("memory");
            if let Some(ref unit) = config.general.memory.unit {
                mem_elem.push_attribute(("unit", unit.as_str()));
            }
            if let Some(ref dump_core) = config.general.memory.dump_core {
                mem_elem.push_attribute(("dumpCore", dump_core.as_str()));
            }
            writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&config.general.memory.value.to_string())))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("memory"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref max_memory) = config.general.max_memory {
            let mut max_mem_elem = BytesStart::new("maxMemory");
            if let Some(ref unit) = max_memory.unit {
                max_mem_elem.push_attribute(("unit", unit.as_str()));
            }
            if let Some(ref slots) = max_memory.slots {
                max_mem_elem.push_attribute(("slots", slots.to_string().as_str()));
            }
            writer.write_event(Event::Start(max_mem_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&max_memory.value.to_string())))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("maxMemory")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref current_memory) = config.general.current_memory {
            let mut current_mem_elem = BytesStart::new("currentMemory");
            if let Some(ref unit) = current_memory.unit {
                current_mem_elem.push_attribute(("unit", unit.as_str()));
            }
            writer.write_event(Event::Start(current_mem_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&current_memory.value.to_string())))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("currentMemory")))
                .map_err(|e| e.to_string())?;
        }

        {
            let mut vcpu_elem = BytesStart::new("vcpu");
            if let Some(ref placement) = config.general.vcpu.placement {
                vcpu_elem.push_attribute(("placement", placement.as_str()));
            }
            if let Some(ref cpuset) = config.general.vcpu.cpuset {
                vcpu_elem.push_attribute(("cpuset", cpuset.as_str()));
            }
            if let Some(ref current) = config.general.vcpu.current {
                vcpu_elem.push_attribute(("current", current.to_string().as_str()));
            }
            writer.write_event(Event::Start(vcpu_elem)).map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&config.general.vcpu.count.to_string())))
                .map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("vcpu"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref vcpus) = config.general.vcpus {
            let vcpus_elem = BytesStart::new("vcpus");
            writer.write_event(Event::Start(vcpus_elem)).map_err(|e| e.to_string())?;

            for vcpu in vcpus {
                let mut vcpu_elem = BytesStart::new("vcpu");
                vcpu_elem.push_attribute(("id", vcpu.id.to_string().as_str()));
                vcpu_elem.push_attribute(("enabled", vcpu.enabled.as_str()));
                vcpu_elem.push_attribute(("hotpluggable", vcpu.hotpluggable.as_str()));
                if let Some(ref order) = vcpu.order {
                    vcpu_elem.push_attribute(("order", order.to_string().as_str()));
                }
                writer.write_event(Event::Empty(vcpu_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("vcpus"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref bootloader) = config.general.bootloader {
            Self::write_element(writer, "bootloader", bootloader)?;
        }

        if let Some(ref bootloader_args) = config.general.bootloader_args {
            Self::write_element(writer, "bootloader_args", bootloader_args)?;
        }

        Ok(())
    }

    fn write_os<W: std::io::Write>(
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
                    writer
                        .write_event(Event::End(BytesEnd::new("type")))
                        .map_err(|e| e.to_string())?;
                }
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
                writer
                    .write_event(Event::End(BytesEnd::new("loader")))
                    .map_err(|e| e.to_string())?;
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
                            writer
                                .write_event(Event::Empty(host_elem))
                                .map_err(|e| e.to_string())?;
                        }

                        if let Some(ref auth) = source.auth {
                            let mut auth_elem = BytesStart::new("auth");
                            auth_elem.push_attribute(("username", auth.username.as_str()));
                            writer
                                .write_event(Event::Start(auth_elem))
                                .map_err(|e| e.to_string())?;

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
                    bios_elem
                        .push_attribute(("rebootTimeout", reboot_timeout.to_string().as_str()));
                }
                writer.write_event(Event::Empty(bios_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref kernel) = os.kernel {
                Self::write_element(writer, "kernel", kernel)?;
            }

            if let Some(ref initrd) = os.initrd {
                Self::write_element(writer, "initrd", initrd)?;
            }

            if let Some(ref cmdline) = os.cmdline {
                Self::write_element(writer, "cmdline", cmdline)?;
            }

            if let Some(ref shim) = os.shim {
                Self::write_element(writer, "shim", shim)?;
            }

            if let Some(ref dtb) = os.dtb {
                Self::write_element(writer, "dtb", dtb)?;
            }

            if let Some(ref init) = os.init {
                Self::write_element(writer, "init", init)?;
            }

            if let Some(ref initarg_list) = os.initarg {
                for initarg in initarg_list {
                    Self::write_element(writer, "initarg", initarg)?;
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
                Self::write_element(writer, "initdir", initdir)?;
            }

            if let Some(ref inituser) = os.inituser {
                Self::write_element(writer, "inituser", inituser)?;
            }

            if let Some(ref initgroup) = os.initgroup {
                Self::write_element(writer, "initgroup", initgroup)?;
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

                writer
                    .write_event(Event::End(BytesEnd::new("idmap")))
                    .map_err(|e| e.to_string())?;
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

    fn write_cpu<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if config.cpu.topology.is_some() || config.cpu.mode.is_some() || config.cpu.model.is_some()
        {
            let mut cpu_elem = BytesStart::new("cpu");
            if let Some(ref mode) = config.cpu.mode {
                cpu_elem.push_attribute(("mode", mode.as_str()));
            }
            writer.write_event(Event::Start(cpu_elem)).map_err(|e| e.to_string())?;

            if let Some(ref topology) = config.cpu.topology {
                let mut topo_elem = BytesStart::new("topology");
                topo_elem.push_attribute(("sockets", topology.sockets.to_string().as_str()));
                topo_elem.push_attribute(("dies", topology.dies.to_string().as_str()));
                topo_elem.push_attribute(("cores", topology.cores.to_string().as_str()));
                topo_elem.push_attribute(("threads", topology.threads.to_string().as_str()));
                writer.write_event(Event::Empty(topo_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref model) = config.cpu.model {
                let mut model_elem = BytesStart::new("model");
                if let Some(ref fallback) = model.fallback {
                    model_elem.push_attribute(("fallback", fallback.as_str()));
                }
                writer.write_event(Event::Start(model_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&model.name)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("model")))
                    .map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("cpu"))).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn write_memory<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref memory_backing) = config.memory_backing {
            let mem_elem = BytesStart::new("memoryBacking");
            writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;

            if let Some(ref hugepages) = memory_backing.hugepages {
                let hugepages_elem = BytesStart::new("hugepages");
                writer.write_event(Event::Start(hugepages_elem)).map_err(|e| e.to_string())?;

                if let Some(ref page_list) = hugepages.page {
                    for page in page_list {
                        let mut page_elem = BytesStart::new("page");
                        page_elem.push_attribute(("size", page.size.as_str()));
                        if let Some(ref unit) = page.unit {
                            page_elem.push_attribute(("unit", unit.as_str()));
                        }
                        if let Some(ref nodeset) = page.nodeset {
                            page_elem.push_attribute(("nodeset", nodeset.as_str()));
                        }
                        writer.write_event(Event::Empty(page_elem)).map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("hugepages")))
                    .map_err(|e| e.to_string())?;
            }

            if memory_backing.nosharepages.is_some() {
                writer
                    .write_event(Event::Empty(BytesStart::new("nosharepages")))
                    .map_err(|e| e.to_string())?;
            }

            if memory_backing.locked.is_some() {
                writer
                    .write_event(Event::Empty(BytesStart::new("locked")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref source) = memory_backing.source {
                let mut source_elem = BytesStart::new("source");
                source_elem.push_attribute(("type", source.source_type.as_str()));
                writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref access) = memory_backing.access {
                let mut access_elem = BytesStart::new("access");
                access_elem.push_attribute(("mode", access.mode.as_str()));
                writer.write_event(Event::Empty(access_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref allocation) = memory_backing.allocation {
                let mut allocation_elem = BytesStart::new("allocation");
                allocation_elem.push_attribute(("mode", allocation.mode.as_str()));
                if let Some(ref threads) = allocation.threads {
                    allocation_elem.push_attribute(("threads", threads.to_string().as_str()));
                }
                writer.write_event(Event::Empty(allocation_elem)).map_err(|e| e.to_string())?;
            }

            if memory_backing.discard.is_some() {
                writer
                    .write_event(Event::Empty(BytesStart::new("discard")))
                    .map_err(|e| e.to_string())?;
            }

            writer
                .write_event(Event::End(BytesEnd::new("memoryBacking")))
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn write_devices<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        let devices_elem = BytesStart::new("devices");
        writer.write_event(Event::Start(devices_elem)).map_err(|e| e.to_string())?;

        if let Some(ref emulator) = config.devices.emulator {
            Self::write_element(writer, "emulator", emulator)?;
        }

        if let Some(ref graphics_list) = config.devices.graphics {
            for g in graphics_list {
                let mut g_elem = BytesStart::new("graphics");
                g_elem.push_attribute(("type", g.graphics_type.as_str()));
                if let Some(ref port) = g.port {
                    g_elem.push_attribute(("port", port.as_str()));
                }
                if let Some(ref autoport) = g.autoport {
                    g_elem.push_attribute(("autoport", autoport.as_str()));
                }
                if let Some(ref listen) = g.listen {
                    g_elem.push_attribute(("listen", listen.as_str()));
                }
                writer.write_event(Event::Empty(g_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref video_list) = config.devices.video {
            for v in video_list {
                writer
                    .write_event(Event::Start(BytesStart::new("video")))
                    .map_err(|e| e.to_string())?;

                let mut model_elem = BytesStart::new("model");
                model_elem.push_attribute(("type", v.model.model_type.as_str()));
                if let Some(vram) = v.model.vram {
                    model_elem.push_attribute(("vram", vram.to_string().as_str()));
                }
                if let Some(heads) = v.model.heads {
                    model_elem.push_attribute(("heads", heads.to_string().as_str()));
                }
                if let Some(ref primary) = v.model.primary {
                    model_elem.push_attribute(("primary", primary.as_str()));
                }
                writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;

                writer
                    .write_event(Event::End(BytesEnd::new("video")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref disk_list) = config.devices.disk {
            for disk in disk_list {
                let mut disk_elem = BytesStart::new("disk");
                disk_elem.push_attribute(("type", disk.disk_type.as_str()));
                disk_elem.push_attribute(("device", disk.device.as_str()));
                writer.write_event(Event::Start(disk_elem)).map_err(|e| e.to_string())?;

                if let Some(ref driver) = disk.driver {
                    let mut driver_elem = BytesStart::new("driver");
                    driver_elem.push_attribute(("name", driver.name.as_str()));
                    driver_elem.push_attribute(("type", driver.driver_type.as_str()));
                    if let Some(ref cache) = driver.cache {
                        driver_elem.push_attribute(("cache", cache.as_str()));
                    }
                    writer.write_event(Event::Empty(driver_elem)).map_err(|e| e.to_string())?;
                }

                if let Some(ref source) = disk.source {
                    let mut source_elem = BytesStart::new("source");
                    if let Some(ref file) = source.file {
                        source_elem.push_attribute(("file", file.as_str()));
                    }
                    if let Some(ref dev) = source.dev {
                        source_elem.push_attribute(("dev", dev.as_str()));
                    }
                    writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
                }

                if let Some(ref target) = disk.target {
                    let mut target_elem = BytesStart::new("target");
                    target_elem.push_attribute(("dev", target.dev.as_str()));
                    if let Some(ref bus) = target.bus {
                        target_elem.push_attribute(("bus", bus.as_str()));
                    }
                    writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
                }

                if disk.readonly.is_some() {
                    writer
                        .write_event(Event::Empty(BytesStart::new("readonly")))
                        .map_err(|e| e.to_string())?;
                }

                writer.write_event(Event::End(BytesEnd::new("disk"))).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref iface_list) = config.devices.interface {
            for iface in iface_list {
                let mut iface_elem = BytesStart::new("interface");
                iface_elem.push_attribute(("type", iface.interface_type.as_str()));
                writer.write_event(Event::Start(iface_elem)).map_err(|e| e.to_string())?;

                if let Some(ref mac) = iface.mac {
                    let mut mac_elem = BytesStart::new("mac");
                    mac_elem.push_attribute(("address", mac.address.as_str()));
                    writer.write_event(Event::Empty(mac_elem)).map_err(|e| e.to_string())?;
                }

                if let Some(ref source) = iface.source {
                    let mut source_elem = BytesStart::new("source");
                    if let Some(ref bridge) = source.bridge {
                        source_elem.push_attribute(("bridge", bridge.as_str()));
                    }
                    if let Some(ref network) = source.network {
                        source_elem.push_attribute(("network", network.as_str()));
                    }
                    writer.write_event(Event::Empty(source_elem)).map_err(|e| e.to_string())?;
                }

                if let Some(ref model) = iface.model {
                    let mut model_elem = BytesStart::new("model");
                    model_elem.push_attribute(("type", model.model_type.as_str()));
                    writer.write_event(Event::Empty(model_elem)).map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("interface")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref input_list) = config.devices.input {
            for input in input_list {
                let mut input_elem = BytesStart::new("input");
                input_elem.push_attribute(("type", input.input_type.as_str()));
                if let Some(ref bus) = input.bus {
                    input_elem.push_attribute(("bus", bus.as_str()));
                }
                writer.write_event(Event::Empty(input_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref tpm) = config.devices.tpm {
            let mut tpm_elem = BytesStart::new("tpm");
            tpm_elem.push_attribute(("model", tpm.model.as_str()));
            writer.write_event(Event::Start(tpm_elem)).map_err(|e| e.to_string())?;

            let mut backend_elem = BytesStart::new("backend");
            backend_elem.push_attribute(("type", tpm.backend.backend_type.as_str()));
            if let Some(ref version) = tpm.backend.version {
                backend_elem.push_attribute(("version", version.as_str()));
            }
            writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;

            writer.write_event(Event::End(BytesEnd::new("tpm"))).map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("devices"))).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn write_advanced<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref sysinfo) = config.devices.sysinfo {
            let mut sysinfo_elem = BytesStart::new("sysinfo");
            sysinfo_elem.push_attribute(("type", sysinfo.sysinfo_type.as_str()));
            writer.write_event(Event::Start(sysinfo_elem)).map_err(|e| e.to_string())?;

            if let Some(ref bios) = sysinfo.bios {
                let bios_elem = BytesStart::new("bios");
                writer.write_event(Event::Start(bios_elem)).map_err(|e| e.to_string())?;

                if let Some(ref entry_list) = bios.entry {
                    for entry in entry_list {
                        let mut entry_elem = BytesStart::new("entry");
                        entry_elem.push_attribute(("name", entry.name.as_str()));
                        writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::Text(BytesText::new(&entry.value)))
                            .map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::End(BytesEnd::new("entry")))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer.write_event(Event::End(BytesEnd::new("bios"))).map_err(|e| e.to_string())?;
            }

            if let Some(ref system) = sysinfo.system {
                let system_elem = BytesStart::new("system");
                writer.write_event(Event::Start(system_elem)).map_err(|e| e.to_string())?;

                if let Some(ref entry_list) = system.entry {
                    for entry in entry_list {
                        let mut entry_elem = BytesStart::new("entry");
                        entry_elem.push_attribute(("name", entry.name.as_str()));
                        writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::Text(BytesText::new(&entry.value)))
                            .map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::End(BytesEnd::new("entry")))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("system")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref base_board_list) = sysinfo.base_board {
                for base_board in base_board_list {
                    let base_board_elem = BytesStart::new("baseBoard");
                    writer.write_event(Event::Start(base_board_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref entry_list) = base_board.entry {
                        for entry in entry_list {
                            let mut entry_elem = BytesStart::new("entry");
                            entry_elem.push_attribute(("name", entry.name.as_str()));
                            writer
                                .write_event(Event::Start(entry_elem))
                                .map_err(|e| e.to_string())?;
                            writer
                                .write_event(Event::Text(BytesText::new(&entry.value)))
                                .map_err(|e| e.to_string())?;
                            writer
                                .write_event(Event::End(BytesEnd::new("entry")))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("baseBoard")))
                        .map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref chassis) = sysinfo.chassis {
                let chassis_elem = BytesStart::new("chassis");
                writer.write_event(Event::Start(chassis_elem)).map_err(|e| e.to_string())?;

                if let Some(ref entry_list) = chassis.entry {
                    for entry in entry_list {
                        let mut entry_elem = BytesStart::new("entry");
                        entry_elem.push_attribute(("name", entry.name.as_str()));
                        writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::Text(BytesText::new(&entry.value)))
                            .map_err(|e| e.to_string())?;
                        writer
                            .write_event(Event::End(BytesEnd::new("entry")))
                            .map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("chassis")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref oem_strings) = sysinfo.oem_strings {
                let oem_elem = BytesStart::new("oemStrings");
                writer.write_event(Event::Start(oem_elem)).map_err(|e| e.to_string())?;

                for oem_string in oem_strings {
                    let entry_elem = BytesStart::new("entry");
                    writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(&oem_string.value)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("oemStrings")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref entry_list) = sysinfo.entry {
                for entry in entry_list {
                    let mut entry_elem = BytesStart::new("entry");
                    entry_elem.push_attribute(("name", entry.name.as_str()));
                    if let Some(ref file) = entry.file {
                        entry_elem.push_attribute(("file", file.as_str()));
                    }
                    writer.write_event(Event::Start(entry_elem)).map_err(|e| e.to_string())?;
                    if let Some(ref value) = entry.value {
                        writer
                            .write_event(Event::Text(BytesText::new(value)))
                            .map_err(|e| e.to_string())?;
                    }
                    writer
                        .write_event(Event::End(BytesEnd::new("entry")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("sysinfo"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref iothreads) = config.iothreads {
            if let Some(ref value) = iothreads.value {
                let iothreads_elem = BytesStart::new("iothreads");
                writer.write_event(Event::Start(iothreads_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&value.to_string())))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("iothreads")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref iothreadids) = iothreads.iothreadids {
                let iothreadids_elem = BytesStart::new("iothreadids");
                writer.write_event(Event::Start(iothreadids_elem)).map_err(|e| e.to_string())?;

                for iothread in iothreadids {
                    let mut io_elem = BytesStart::new("iothread");
                    io_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                    if let Some(min) = iothread.thread_pool_min {
                        io_elem.push_attribute(("thread_pool_min", min.to_string().as_str()));
                    }
                    if let Some(max) = iothread.thread_pool_max {
                        io_elem.push_attribute(("thread_pool_max", max.to_string().as_str()));
                    }

                    if let Some(ref poll) = iothread.poll {
                        writer.write_event(Event::Start(io_elem)).map_err(|e| e.to_string())?;

                        let mut poll_elem = BytesStart::new("poll");
                        poll_elem.push_attribute(("max", poll.max.to_string().as_str()));
                        if let Some(grow) = poll.grow {
                            poll_elem.push_attribute(("grow", grow.to_string().as_str()));
                        }
                        if let Some(shrink) = poll.shrink {
                            poll_elem.push_attribute(("shrink", shrink.to_string().as_str()));
                        }
                        writer.write_event(Event::Empty(poll_elem)).map_err(|e| e.to_string())?;

                        writer
                            .write_event(Event::End(BytesEnd::new("iothread")))
                            .map_err(|e| e.to_string())?;
                    } else {
                        writer.write_event(Event::Empty(io_elem)).map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("iothreadids")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref defaultiothread) = iothreads.defaultiothread {
                let mut default_elem = BytesStart::new("defaultiothread");
                if let Some(min) = defaultiothread.thread_pool_min {
                    default_elem.push_attribute(("thread_pool_min", min.to_string().as_str()));
                }
                if let Some(max) = defaultiothread.thread_pool_max {
                    default_elem.push_attribute(("thread_pool_max", max.to_string().as_str()));
                }
                writer.write_event(Event::Empty(default_elem)).map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref cpu_tuning) = config.cpu_tuning {
            let cputune_elem = BytesStart::new("cputune");
            writer.write_event(Event::Start(cputune_elem)).map_err(|e| e.to_string())?;

            if let Some(ref vcpupin_list) = cpu_tuning.vcpupin {
                for vcpupin in vcpupin_list {
                    let mut pin_elem = BytesStart::new("vcpupin");
                    pin_elem.push_attribute(("vcpu", vcpupin.vcpu.to_string().as_str()));
                    pin_elem.push_attribute(("cpuset", vcpupin.cpuset.as_str()));
                    writer.write_event(Event::Empty(pin_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref emulatorpin) = cpu_tuning.emulatorpin {
                let mut pin_elem = BytesStart::new("emulatorpin");
                pin_elem.push_attribute(("cpuset", emulatorpin.cpuset.as_str()));
                writer.write_event(Event::Empty(pin_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref iothreadpin_list) = cpu_tuning.iothreadpin {
                for iothreadpin in iothreadpin_list {
                    let mut pin_elem = BytesStart::new("iothreadpin");
                    pin_elem
                        .push_attribute(("iothread", iothreadpin.iothread.to_string().as_str()));
                    pin_elem.push_attribute(("cpuset", iothreadpin.cpuset.as_str()));
                    writer.write_event(Event::Empty(pin_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(shares) = cpu_tuning.shares {
                Self::write_element(writer, "shares", &shares.to_string())?;
            }

            if let Some(period) = cpu_tuning.period {
                Self::write_element(writer, "period", &period.to_string())?;
            }

            if let Some(quota) = cpu_tuning.quota {
                Self::write_element(writer, "quota", &quota.to_string())?;
            }

            if let Some(global_period) = cpu_tuning.global_period {
                Self::write_element(writer, "global_period", &global_period.to_string())?;
            }

            if let Some(global_quota) = cpu_tuning.global_quota {
                Self::write_element(writer, "global_quota", &global_quota.to_string())?;
            }

            if let Some(emulator_period) = cpu_tuning.emulator_period {
                Self::write_element(writer, "emulator_period", &emulator_period.to_string())?;
            }

            if let Some(emulator_quota) = cpu_tuning.emulator_quota {
                Self::write_element(writer, "emulator_quota", &emulator_quota.to_string())?;
            }

            if let Some(iothread_period) = cpu_tuning.iothread_period {
                Self::write_element(writer, "iothread_period", &iothread_period.to_string())?;
            }

            if let Some(iothread_quota) = cpu_tuning.iothread_quota {
                Self::write_element(writer, "iothread_quota", &iothread_quota.to_string())?;
            }

            if let Some(ref vcpusched_list) = cpu_tuning.vcpusched {
                for vcpusched in vcpusched_list {
                    let mut sched_elem = BytesStart::new("vcpusched");
                    sched_elem.push_attribute(("vcpus", vcpusched.vcpus.as_str()));
                    sched_elem.push_attribute(("scheduler", vcpusched.scheduler.as_str()));
                    if let Some(priority) = vcpusched.priority {
                        sched_elem.push_attribute(("priority", priority.to_string().as_str()));
                    }
                    writer.write_event(Event::Empty(sched_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref iothreadsched_list) = cpu_tuning.iothreadsched {
                for iothreadsched in iothreadsched_list {
                    let mut sched_elem = BytesStart::new("iothreadsched");
                    sched_elem.push_attribute(("iothreads", iothreadsched.iothreads.as_str()));
                    sched_elem.push_attribute(("scheduler", iothreadsched.scheduler.as_str()));
                    if let Some(priority) = iothreadsched.priority {
                        sched_elem.push_attribute(("priority", priority.to_string().as_str()));
                    }
                    writer.write_event(Event::Empty(sched_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref emulatorsched) = cpu_tuning.emulatorsched {
                let mut sched_elem = BytesStart::new("emulatorsched");
                sched_elem.push_attribute(("scheduler", emulatorsched.scheduler.as_str()));
                if let Some(priority) = emulatorsched.priority {
                    sched_elem.push_attribute(("priority", priority.to_string().as_str()));
                }
                writer.write_event(Event::Empty(sched_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref cachetune_list) = cpu_tuning.cachetune {
                for cachetune in cachetune_list {
                    let mut cache_elem = BytesStart::new("cachetune");
                    cache_elem.push_attribute(("vcpus", cachetune.vcpus.as_str()));
                    writer.write_event(Event::Start(cache_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref cache_list) = cachetune.cache {
                        for cache in cache_list {
                            let mut cache_entry_elem = BytesStart::new("cache");
                            cache_entry_elem.push_attribute(("id", cache.id.to_string().as_str()));
                            cache_entry_elem
                                .push_attribute(("level", cache.level.to_string().as_str()));
                            cache_entry_elem.push_attribute(("type", cache.cache_type.as_str()));
                            cache_entry_elem
                                .push_attribute(("size", cache.size.to_string().as_str()));
                            if let Some(ref unit) = cache.unit {
                                cache_entry_elem.push_attribute(("unit", unit.as_str()));
                            }
                            writer
                                .write_event(Event::Empty(cache_entry_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    if let Some(ref monitor_list) = cachetune.monitor {
                        for monitor in monitor_list {
                            let mut monitor_elem = BytesStart::new("monitor");
                            monitor_elem
                                .push_attribute(("level", monitor.level.to_string().as_str()));
                            monitor_elem.push_attribute(("vcpus", monitor.vcpus.as_str()));
                            writer
                                .write_event(Event::Empty(monitor_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("cachetune")))
                        .map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref memorytune_list) = cpu_tuning.memorytune {
                for memorytune in memorytune_list {
                    let mut mem_elem = BytesStart::new("memorytune");
                    mem_elem.push_attribute(("vcpus", memorytune.vcpus.as_str()));
                    writer.write_event(Event::Start(mem_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref node_list) = memorytune.node {
                        for node in node_list {
                            let mut node_elem = BytesStart::new("node");
                            node_elem.push_attribute(("id", node.id.to_string().as_str()));
                            node_elem
                                .push_attribute(("bandwidth", node.bandwidth.to_string().as_str()));
                            writer
                                .write_event(Event::Empty(node_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("memorytune")))
                        .map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("cputune"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref numa) = config.numa {
            let numa_elem = BytesStart::new("numa");
            writer.write_event(Event::Start(numa_elem)).map_err(|e| e.to_string())?;

            if let Some(ref cell_list) = numa.cell {
                for cell in cell_list {
                    let mut cell_elem = BytesStart::new("cell");
                    cell_elem.push_attribute(("id", cell.id.to_string().as_str()));
                    cell_elem.push_attribute(("cpus", cell.cpus.as_str()));
                    cell_elem.push_attribute(("memory", cell.memory.to_string().as_str()));
                    if let Some(ref unit) = cell.unit {
                        cell_elem.push_attribute(("unit", unit.as_str()));
                    }

                    if let Some(ref memnode_list) = cell.memnode {
                        writer.write_event(Event::Start(cell_elem)).map_err(|e| e.to_string())?;

                        for memnode in memnode_list {
                            let mut memnode_elem = BytesStart::new("memnode");
                            memnode_elem
                                .push_attribute(("cellid", memnode.cellid.to_string().as_str()));
                            memnode_elem.push_attribute(("mode", memnode.mode.as_str()));
                            writer
                                .write_event(Event::Empty(memnode_elem))
                                .map_err(|e| e.to_string())?;
                        }

                        writer
                            .write_event(Event::End(BytesEnd::new("cell")))
                            .map_err(|e| e.to_string())?;
                    } else {
                        writer.write_event(Event::Empty(cell_elem)).map_err(|e| e.to_string())?;
                    }
                }
            }

            writer.write_event(Event::End(BytesEnd::new("numa"))).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn write_element<W: std::io::Write>(
        writer: &mut Writer<W>,
        name: &str,
        value: &str,
    ) -> Result<(), String> {
        writer.write_event(Event::Start(BytesStart::new(name))).map_err(|e| e.to_string())?;
        writer.write_event(Event::Text(BytesText::new(value))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new(name))).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn format_xml(xml: &str) -> Result<String, String> {
        let mut result = String::new();
        let mut indent: usize = 0;
        let in_text = false;

        for line in xml.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("</") {
                indent = indent.saturating_sub(2);
            }

            if !in_text {
                result.push_str(&" ".repeat(indent));
            }
            result.push_str(trimmed);
            result.push('\n');

            if trimmed.starts_with("<?") || trimmed.starts_with("<!") {
                continue;
            }

            if trimmed.starts_with("</") {
                continue;
            }

            if trimmed.starts_with("<") && !trimmed.ends_with("/>") && !trimmed.contains("</") {
                indent += 2;
            }
        }

        Ok(result)
    }
}

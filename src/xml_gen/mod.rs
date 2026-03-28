use std::io::Cursor;

use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::model::VMConfig;

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
        Self::write_events(&mut writer, config)?;
        Self::write_features(&mut writer, config)?;
        Self::write_clock(&mut writer, config)?;
        Self::write_perf(&mut writer, config)?;
        Self::write_iothreads(&mut writer, config)?;
        Self::write_cputune(&mut writer, config)?;
        Self::write_devices(&mut writer, config)?;
        Self::write_advanced(&mut writer, config)?;

        Self::write_numatune(&mut writer, config)?;

        writer
            .write_event(Event::End(BytesEnd::new("domain")))
            .map_err(|e| format!("关闭 domain 标签失败: {}", e))?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).map_err(|e| format!("转换 UTF-8 失败: {}", e))
    }

    fn write_numatune<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref numatune) = config.numatune {
            let numatune_elem = BytesStart::new("numatune");
            writer.write_event(Event::Start(numatune_elem)).map_err(|e| e.to_string())?;

            if let Some(ref memory) = numatune.memory {
                let mut memory_elem = BytesStart::new("memory");
                if let Some(ref mode) = memory.mode {
                    memory_elem.push_attribute(("mode", mode.as_str()));
                }
                if let Some(ref nodeset) = memory.nodeset {
                    memory_elem.push_attribute(("nodeset", nodeset.as_str()));
                }
                if let Some(ref placement) = memory.placement {
                    memory_elem.push_attribute(("placement", placement.as_str()));
                }
                writer.write_event(Event::Empty(memory_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref memnode_list) = numatune.memnode {
                for memnode in memnode_list {
                    let mut memnode_elem = BytesStart::new("memnode");
                    memnode_elem.push_attribute(("cellid", memnode.cellid.to_string().as_str()));
                    if let Some(ref mode) = memnode.mode {
                        memnode_elem.push_attribute(("mode", mode.as_str()));
                    }
                    if let Some(ref nodeset) = memnode.nodeset {
                        memnode_elem.push_attribute(("nodeset", nodeset.as_str()));
                    }
                    writer.write_event(Event::Empty(memnode_elem)).map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("numatune"))).map_err(|e| e.to_string())?;
        }

        Ok(())
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
        if config.cpu.topology.is_some()
            || config.cpu.mode.is_some()
            || config.cpu.model.is_some()
            || config.cpu.vendor.is_some()
            || config.cpu.feature.is_some()
            || config.cpu.cache.is_some()
            || config.cpu.maxphysaddr.is_some()
            || config.cpu.numa.is_some()
        {
            let mut cpu_elem = BytesStart::new("cpu");
            if let Some(ref mode) = config.cpu.mode {
                cpu_elem.push_attribute(("mode", mode.as_str()));
            }
            if let Some(ref match_) = config.cpu.match_ {
                cpu_elem.push_attribute(("match", match_.as_str()));
            }
            if let Some(ref check) = config.cpu.check {
                cpu_elem.push_attribute(("check", check.as_str()));
            }
            if let Some(ref migratable) = config.cpu.migratable {
                cpu_elem.push_attribute(("migratable", migratable.as_str()));
            }
            writer.write_event(Event::Start(cpu_elem)).map_err(|e| e.to_string())?;

            if let Some(ref model) = config.cpu.model {
                let mut model_elem = BytesStart::new("model");
                if let Some(ref fallback) = model.fallback {
                    model_elem.push_attribute(("fallback", fallback.as_str()));
                }
                if let Some(ref vendor_id) = model.vendor_id {
                    model_elem.push_attribute(("vendor_id", vendor_id.as_str()));
                }
                writer.write_event(Event::Start(model_elem)).map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::Text(BytesText::new(&model.name)))
                    .map_err(|e| e.to_string())?;
                writer
                    .write_event(Event::End(BytesEnd::new("model")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref vendor) = config.cpu.vendor {
                Self::write_element(writer, "vendor", vendor)?;
            }

            if let Some(ref topology) = config.cpu.topology {
                let mut topo_elem = BytesStart::new("topology");
                topo_elem.push_attribute(("sockets", topology.sockets.to_string().as_str()));
                if let Some(dies) = topology.dies {
                    topo_elem.push_attribute(("dies", dies.to_string().as_str()));
                }
                if let Some(clusters) = topology.clusters {
                    topo_elem.push_attribute(("clusters", clusters.to_string().as_str()));
                }
                topo_elem.push_attribute(("cores", topology.cores.to_string().as_str()));
                topo_elem.push_attribute(("threads", topology.threads.to_string().as_str()));
                writer.write_event(Event::Empty(topo_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref feature_list) = config.cpu.feature {
                for feature in feature_list {
                    let mut feature_elem = BytesStart::new("feature");
                    feature_elem.push_attribute(("name", feature.name.as_str()));
                    feature_elem.push_attribute(("policy", feature.policy.as_str()));
                    writer.write_event(Event::Empty(feature_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref cache_list) = config.cpu.cache {
                for cache in cache_list {
                    let mut cache_elem = BytesStart::new("cache");
                    if let Some(level) = cache.level {
                        cache_elem.push_attribute(("level", level.to_string().as_str()));
                    }
                    if let Some(ref mode) = cache.mode {
                        cache_elem.push_attribute(("mode", mode.as_str()));
                    }
                    writer.write_event(Event::Empty(cache_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref maxphysaddr) = config.cpu.maxphysaddr {
                let mut maxphysaddr_elem = BytesStart::new("maxphysaddr");
                maxphysaddr_elem.push_attribute(("mode", maxphysaddr.mode.as_str()));
                if let Some(bits) = maxphysaddr.bits {
                    maxphysaddr_elem.push_attribute(("bits", bits.to_string().as_str()));
                }
                if let Some(limit) = maxphysaddr.limit {
                    maxphysaddr_elem.push_attribute(("limit", limit.to_string().as_str()));
                }
                writer.write_event(Event::Empty(maxphysaddr_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref numa) = config.cpu.numa {
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
                        if let Some(ref mem_access) = cell.mem_access {
                            cell_elem.push_attribute(("memAccess", mem_access.as_str()));
                        }
                        if let Some(ref discard) = cell.discard {
                            cell_elem.push_attribute(("discard", discard.as_str()));
                        }

                        if let Some(ref distances) = cell.distances {
                            writer
                                .write_event(Event::Start(cell_elem))
                                .map_err(|e| e.to_string())?;

                            let distances_elem = BytesStart::new("distances");
                            writer
                                .write_event(Event::Start(distances_elem))
                                .map_err(|e| e.to_string())?;

                            if let Some(ref sibling_list) = distances.sibling {
                                for sibling in sibling_list {
                                    let mut sibling_elem = BytesStart::new("sibling");
                                    sibling_elem
                                        .push_attribute(("id", sibling.id.to_string().as_str()));
                                    sibling_elem.push_attribute((
                                        "value",
                                        sibling.value.to_string().as_str(),
                                    ));
                                    writer
                                        .write_event(Event::Empty(sibling_elem))
                                        .map_err(|e| e.to_string())?;
                                }
                            }

                            writer
                                .write_event(Event::End(BytesEnd::new("distances")))
                                .map_err(|e| e.to_string())?;
                            writer
                                .write_event(Event::End(BytesEnd::new("cell")))
                                .map_err(|e| e.to_string())?;
                        } else {
                            writer
                                .write_event(Event::Empty(cell_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }
                }

                if let Some(ref interconnects) = numa.interconnects {
                    let interconnects_elem = BytesStart::new("interconnects");
                    writer
                        .write_event(Event::Start(interconnects_elem))
                        .map_err(|e| e.to_string())?;

                    if let Some(ref latency_list) = interconnects.latency {
                        for latency in latency_list {
                            let mut latency_elem = BytesStart::new("latency");
                            latency_elem.push_attribute((
                                "initiator",
                                latency.initiator.to_string().as_str(),
                            ));
                            latency_elem
                                .push_attribute(("target", latency.target.to_string().as_str()));
                            latency_elem.push_attribute(("type", latency.type_.as_str()));
                            latency_elem
                                .push_attribute(("value", latency.value.to_string().as_str()));
                            if let Some(cache) = latency.cache {
                                latency_elem.push_attribute(("cache", cache.to_string().as_str()));
                            }
                            writer
                                .write_event(Event::Empty(latency_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    if let Some(ref bandwidth_list) = interconnects.bandwidth {
                        for bandwidth in bandwidth_list {
                            let mut bandwidth_elem = BytesStart::new("bandwidth");
                            bandwidth_elem.push_attribute((
                                "initiator",
                                bandwidth.initiator.to_string().as_str(),
                            ));
                            bandwidth_elem
                                .push_attribute(("target", bandwidth.target.to_string().as_str()));
                            bandwidth_elem.push_attribute(("type", bandwidth.type_.as_str()));
                            bandwidth_elem
                                .push_attribute(("value", bandwidth.value.to_string().as_str()));
                            if let Some(ref unit) = bandwidth.unit {
                                bandwidth_elem.push_attribute(("unit", unit.as_str()));
                            }
                            writer
                                .write_event(Event::Empty(bandwidth_elem))
                                .map_err(|e| e.to_string())?;
                        }
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("interconnects")))
                        .map_err(|e| e.to_string())?;
                }

                writer.write_event(Event::End(BytesEnd::new("numa"))).map_err(|e| e.to_string())?;
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
                            writer
                                .write_event(Event::Empty(seclabel_elem))
                                .map_err(|e| e.to_string())?;
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
                        target_elem
                            .push_attribute(("rotation_rate", rotation_rate.to_string().as_str()));
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
                    // 可以添加 encryption 的子元素
                    writer
                        .write_event(Event::End(BytesEnd::new("encryption")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref serial) = disk.serial {
                    let serial_elem = BytesStart::new("serial");
                    writer.write_event(Event::Start(serial_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(serial)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("serial")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref wwn) = disk.wwn {
                    let wwn_elem = BytesStart::new("wwn");
                    writer.write_event(Event::Start(wwn_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(wwn)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("wwn")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref vendor) = disk.vendor {
                    let vendor_elem = BytesStart::new("vendor");
                    writer.write_event(Event::Start(vendor_elem)).map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::Text(BytesText::new(vendor)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("vendor")))
                        .map_err(|e| e.to_string())?;
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
                        Self::write_element(
                            writer,
                            "total_bytes_sec",
                            &total_bytes_sec.to_string(),
                        )?;
                    }
                    if let Some(read_bytes_sec) = iotune.read_bytes_sec {
                        Self::write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
                    }
                    if let Some(write_bytes_sec) = iotune.write_bytes_sec {
                        Self::write_element(
                            writer,
                            "write_bytes_sec",
                            &write_bytes_sec.to_string(),
                        )?;
                    }
                    if let Some(total_iops_sec) = iotune.total_iops_sec {
                        Self::write_element(writer, "total_iops_sec", &total_iops_sec.to_string())?;
                    }
                    if let Some(read_iops_sec) = iotune.read_iops_sec {
                        Self::write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
                    }
                    if let Some(write_iops_sec) = iotune.write_iops_sec {
                        Self::write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
                    }
                    // 可以添加更多 iotune 子元素

                    writer
                        .write_event(Event::End(BytesEnd::new("iotune")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref backenddomain) = disk.backenddomain {
                    let mut backenddomain_elem = BytesStart::new("backenddomain");
                    backenddomain_elem.push_attribute(("name", backenddomain.as_str()));
                    writer
                        .write_event(Event::Empty(backenddomain_elem))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref throttlefilters) = disk.throttlefilters {
                    let throttlefilters_elem = BytesStart::new("throttlefilters");
                    writer
                        .write_event(Event::Start(throttlefilters_elem))
                        .map_err(|e| e.to_string())?;

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
                        address_elem
                            .push_attribute(("controller", controller.to_string().as_str()));
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
        }

        if let Some(ref iface_list) = config.devices.interface {
            for iface in iface_list {
                let mut iface_elem = BytesStart::new("interface");
                iface_elem.push_attribute(("type", iface.interface_type.as_str()));
                if let Some(ref trust_guest_rx_filters) = iface.trust_guest_rx_filters {
                    iface_elem
                        .push_attribute(("trustGuestRxFilters", trust_guest_rx_filters.as_str()));
                }
                writer.write_event(Event::Start(iface_elem)).map_err(|e| e.to_string())?;

                if let Some(ref alias) = iface.alias {
                    let mut alias_elem = BytesStart::new("alias");
                    alias_elem.push_attribute(("name", alias.name.as_str()));
                    writer.write_event(Event::Empty(alias_elem)).map_err(|e| e.to_string())?;
                }

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

                if let Some(ref boot) = iface.boot {
                    let mut boot_elem = BytesStart::new("boot");
                    boot_elem.push_attribute(("order", boot.order.to_string().as_str()));
                    writer.write_event(Event::Empty(boot_elem)).map_err(|e| e.to_string())?;
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

        if let Some(ref filesystem_list) = config.devices.filesystem {
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
                        writer
                            .write_event(Event::Empty(sandbox_elem))
                            .map_err(|e| e.to_string())?;
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
                        thread_pool_elem
                            .push_attribute(("size", thread_pool.size.to_string().as_str()));
                        writer
                            .write_event(Event::Empty(thread_pool_elem))
                            .map_err(|e| e.to_string())?;
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("binary")))
                        .map_err(|e| e.to_string())?;
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

                    writer
                        .write_event(Event::End(BytesEnd::new("idmap")))
                        .map_err(|e| e.to_string())?;
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

                writer
                    .write_event(Event::End(BytesEnd::new("filesystem")))
                    .map_err(|e| e.to_string())?;
            }
        }

        if let Some(ref controller_list) = config.devices.controller {
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
                    controller_elem.push_attribute((
                        "maxEventChannels",
                        max_event_channels.to_string().as_str(),
                    ));
                }
                writer.write_event(Event::Start(controller_elem)).map_err(|e| e.to_string())?;

                if let Some(ref driver) = controller.driver {
                    let mut driver_elem = BytesStart::new("driver");
                    if let Some(ref queues) = driver.queues {
                        driver_elem.push_attribute(("queues", queues.to_string().as_str()));
                    }
                    if let Some(ref cmd_per_lun) = driver.cmd_per_lun {
                        driver_elem
                            .push_attribute(("cmd_per_lun", cmd_per_lun.to_string().as_str()));
                    }
                    if let Some(ref max_sectors) = driver.max_sectors {
                        driver_elem
                            .push_attribute(("max_sectors", max_sectors.to_string().as_str()));
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
                        writer
                            .write_event(Event::Start(iothreads_elem))
                            .map_err(|e| e.to_string())?;

                        for iothread in iothreads {
                            let mut iothread_elem = BytesStart::new("iothread");
                            iothread_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                            writer
                                .write_event(Event::Start(iothread_elem))
                                .map_err(|e| e.to_string())?;

                            if let Some(ref queues) = iothread.queues {
                                for queue in queues {
                                    let mut queue_elem = BytesStart::new("queue");
                                    queue_elem
                                        .push_attribute(("id", queue.id.to_string().as_str()));
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

                    writer
                        .write_event(Event::End(BytesEnd::new("driver")))
                        .map_err(|e| e.to_string())?;
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

                if let Some(ref _target) = controller.target {
                    let target_elem = BytesStart::new("target");
                    writer.write_event(Event::Start(target_elem)).map_err(|e| e.to_string())?;
                    // 可以添加 target 的子元素
                    writer
                        .write_event(Event::End(BytesEnd::new("target")))
                        .map_err(|e| e.to_string())?;
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
                        .write_event(Event::Text(BytesText::new(serial)))
                        .map_err(|e| e.to_string())?;
                    writer
                        .write_event(Event::End(BytesEnd::new("serial")))
                        .map_err(|e| e.to_string())?;
                }

                if let Some(ref address) = controller.address {
                    let mut address_elem = BytesStart::new("address");
                    address_elem.push_attribute(("type", address.address_type.as_str()));
                    if let Some(controller_id) = address.controller {
                        address_elem
                            .push_attribute(("controller", controller_id.to_string().as_str()));
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

                writer
                    .write_event(Event::End(BytesEnd::new("controller")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // 串口设备
        if let Some(ref serial_list) = config.devices.serial {
            for serial in serial_list {
                let mut serial_elem = BytesStart::new("serial");
                serial_elem.push_attribute(("type", serial.serial_type.as_str()));
                if let Some(ref port) = serial.port {
                    serial_elem.push_attribute(("port", port.to_string().as_str()));
                }
                writer.write_event(Event::Start(serial_elem)).map_err(|e| e.to_string())?;

                // SerialConfig 结构体没有 source 字段

                if let Some(ref target) = serial.target {
                    if let Some(port) = target.port {
                        let mut target_elem = BytesStart::new("target");
                        target_elem.push_attribute(("port", port.to_string().as_str()));
                        writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
                    }
                }

                // SerialConfig 结构体没有 address 字段

                writer
                    .write_event(Event::End(BytesEnd::new("serial")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // 并口设备
        if let Some(ref parallel_list) = config.devices.parallel {
            for parallel in parallel_list {
                let mut parallel_elem = BytesStart::new("parallel");
                parallel_elem.push_attribute(("type", parallel.parallel_type.as_str()));
                writer.write_event(Event::Start(parallel_elem)).map_err(|e| e.to_string())?;

                // ParallelConfig 结构体没有 source 字段

                if let Some(ref target) = parallel.target {
                    let mut target_elem = BytesStart::new("target");
                    target_elem.push_attribute(("port", target.port.to_string().as_str()));
                    writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("parallel")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // 控制台设备
        if let Some(ref console) = config.devices.console {
            let mut console_elem = BytesStart::new("console");
            console_elem.push_attribute(("type", console.console_type.as_str()));
            writer.write_event(Event::Start(console_elem)).map_err(|e| e.to_string())?;

            if let Some(ref target) = console.target {
                let mut target_elem = BytesStart::new("target");
                target_elem.push_attribute(("type", target.target_type.as_str()));
                if let Some(ref port) = target.port {
                    target_elem.push_attribute(("port", port.to_string().as_str()));
                }
                writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("console"))).map_err(|e| e.to_string())?;
        }

        // 声音设备
        if let Some(ref sound_list) = config.devices.sound {
            for sound in sound_list {
                let mut sound_elem = BytesStart::new("sound");
                sound_elem.push_attribute(("model", sound.model.as_str()));
                writer.write_event(Event::Start(sound_elem)).map_err(|e| e.to_string())?;

                writer
                    .write_event(Event::End(BytesEnd::new("sound")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // 通道设备
        if let Some(ref channel_list) = config.devices.channel {
            for channel in channel_list {
                let mut channel_elem = BytesStart::new("channel");
                channel_elem.push_attribute(("type", channel.channel_type.as_str()));
                writer.write_event(Event::Start(channel_elem)).map_err(|e| e.to_string())?;

                // ChannelConfig 结构体没有 source 字段

                if let Some(ref target) = channel.target {
                    let mut target_elem = BytesStart::new("target");
                    target_elem.push_attribute(("type", target.target_type.as_str()));
                    target_elem.push_attribute(("name", target.name.as_str()));
                    // ChannelTarget 结构体没有 port 字段
                    writer.write_event(Event::Empty(target_elem)).map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("channel")))
                    .map_err(|e| e.to_string())?;
            }
        }

        // 看门狗设备
        if let Some(ref watchdog) = config.devices.watchdog {
            let mut watchdog_elem = BytesStart::new("watchdog");
            watchdog_elem.push_attribute(("model", watchdog.model.as_str()));
            watchdog_elem.push_attribute(("action", watchdog.action.as_str()));
            writer.write_event(Event::Start(watchdog_elem)).map_err(|e| e.to_string())?;

            writer.write_event(Event::End(BytesEnd::new("watchdog"))).map_err(|e| e.to_string())?;
        }

        // 随机数生成器设备
        if let Some(ref rng_list) = config.devices.rng {
            for rng in rng_list {
                let rng_elem = BytesStart::new("rng");
                writer.write_event(Event::Start(rng_elem)).map_err(|e| e.to_string())?;

                if let Some(ref backend) = rng.backend {
                    let mut backend_elem = BytesStart::new("backend");
                    backend_elem.push_attribute(("model", backend.model.as_str()));
                    backend_elem.push_attribute(("type", backend.rng_type.as_str()));
                    writer.write_event(Event::Empty(backend_elem)).map_err(|e| e.to_string())?;
                }

                writer.write_event(Event::End(BytesEnd::new("rng"))).map_err(|e| e.to_string())?;
            }
        }

        // 内存气球设备
        if let Some(ref memballoon) = config.devices.memballoon {
            let mut memballoon_elem = BytesStart::new("memballoon");
            memballoon_elem.push_attribute(("model", memballoon.model.as_str()));
            writer.write_event(Event::Start(memballoon_elem)).map_err(|e| e.to_string())?;

            writer
                .write_event(Event::End(BytesEnd::new("memballoon")))
                .map_err(|e| e.to_string())?;
        }

        writer.write_event(Event::End(BytesEnd::new("devices"))).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn write_events<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref events) = config.events {
            if let Some(ref on_poweroff) = events.on_poweroff {
                Self::write_element(writer, "on_poweroff", on_poweroff)?;
            }

            if let Some(ref on_reboot) = events.on_reboot {
                Self::write_element(writer, "on_reboot", on_reboot)?;
            }

            if let Some(ref on_crash) = events.on_crash {
                Self::write_element(writer, "on_crash", on_crash)?;
            }

            if let Some(ref on_lockfailure) = events.on_lockfailure {
                Self::write_element(writer, "on_lockfailure", on_lockfailure)?;
            }
        }

        Ok(())
    }

    fn write_features<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref hypervisor_features) = config.hypervisor_features {
            let features_elem = BytesStart::new("features");
            writer.write_event(Event::Start(features_elem)).map_err(|e| e.to_string())?;

            if let Some(ref feature_list) = hypervisor_features.feature {
                for feature in feature_list {
                    let mut feature_elem = BytesStart::new("feature");
                    feature_elem.push_attribute(("enabled", feature.enabled.as_str()));
                    feature_elem.push_attribute(("name", feature.name.as_str()));
                    writer.write_event(Event::Empty(feature_elem)).map_err(|e| e.to_string())?;
                }
            }

            // 可以继续添加其他 features 子元素

            writer.write_event(Event::End(BytesEnd::new("features"))).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn write_clock<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref time_keeping) = config.time_keeping {
            if let Some(ref clock) = time_keeping.clock {
                let mut clock_elem = BytesStart::new("clock");
                clock_elem.push_attribute(("offset", clock.offset.as_str()));
                writer.write_event(Event::Start(clock_elem)).map_err(|e| e.to_string())?;

                if let Some(ref timer_list) = clock.timer {
                    for timer in timer_list {
                        let mut timer_elem = BytesStart::new("timer");
                        timer_elem.push_attribute(("name", timer.name.as_str()));
                        if let Some(ref present) = timer.present {
                            timer_elem.push_attribute(("present", present.as_str()));
                        }
                        if let Some(frequency) = timer.frequency {
                            timer_elem
                                .push_attribute(("frequency", frequency.to_string().as_str()));
                        }
                        if let Some(ref tickpolicy) = timer.tickpolicy {
                            timer_elem.push_attribute(("tickpolicy", tickpolicy.as_str()));
                        }
                        writer.write_event(Event::Empty(timer_elem)).map_err(|e| e.to_string())?;
                    }
                }

                writer
                    .write_event(Event::End(BytesEnd::new("clock")))
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    fn write_perf<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref performance_monitoring) = config.performance_monitoring {
            let perf_elem = BytesStart::new("perf");
            writer.write_event(Event::Start(perf_elem)).map_err(|e| e.to_string())?;

            if let Some(ref event_list) = performance_monitoring.events {
                for event in event_list {
                    let mut event_elem = BytesStart::new("event");
                    event_elem.push_attribute(("name", event.name.as_str()));
                    if let Some(count) = event.count {
                        event_elem.push_attribute(("count", count.to_string().as_str()));
                    }
                    writer.write_event(Event::Empty(event_elem)).map_err(|e| e.to_string())?;
                }
            }

            writer.write_event(Event::End(BytesEnd::new("perf"))).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn write_iothreads<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
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
                    let mut iothread_elem = BytesStart::new("iothread");
                    iothread_elem.push_attribute(("id", iothread.id.to_string().as_str()));
                    if let Some(ref thread_pool_min) = iothread.thread_pool_min {
                        iothread_elem.push_attribute((
                            "thread_pool_min",
                            thread_pool_min.to_string().as_str(),
                        ));
                    }
                    if let Some(ref thread_pool_max) = iothread.thread_pool_max {
                        iothread_elem.push_attribute((
                            "thread_pool_max",
                            thread_pool_max.to_string().as_str(),
                        ));
                    }
                    writer.write_event(Event::Start(iothread_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref poll) = iothread.poll {
                        let mut poll_elem = BytesStart::new("poll");
                        poll_elem.push_attribute(("max", poll.max.to_string().as_str()));
                        if let Some(ref grow) = poll.grow {
                            poll_elem.push_attribute(("grow", grow.to_string().as_str()));
                        }
                        if let Some(ref shrink) = poll.shrink {
                            poll_elem.push_attribute(("shrink", shrink.to_string().as_str()));
                        }
                        writer.write_event(Event::Empty(poll_elem)).map_err(|e| e.to_string())?;
                    }

                    writer
                        .write_event(Event::End(BytesEnd::new("iothread")))
                        .map_err(|e| e.to_string())?;
                }

                writer
                    .write_event(Event::End(BytesEnd::new("iothreadids")))
                    .map_err(|e| e.to_string())?;
            }

            if let Some(ref defaultiothread) = iothreads.defaultiothread {
                let mut defaultiothread_elem = BytesStart::new("defaultiothread");
                if let Some(ref thread_pool_min) = defaultiothread.thread_pool_min {
                    defaultiothread_elem
                        .push_attribute(("thread_pool_min", thread_pool_min.to_string().as_str()));
                }
                if let Some(ref thread_pool_max) = defaultiothread.thread_pool_max {
                    defaultiothread_elem
                        .push_attribute(("thread_pool_max", thread_pool_max.to_string().as_str()));
                }
                writer
                    .write_event(Event::Empty(defaultiothread_elem))
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    fn write_cputune<W: std::io::Write>(
        writer: &mut Writer<W>,
        config: &VMConfig,
    ) -> Result<(), String> {
        if let Some(ref cputune) = config.cpu_tuning {
            let cputune_elem = BytesStart::new("cputune");
            writer.write_event(Event::Start(cputune_elem)).map_err(|e| e.to_string())?;

            if let Some(ref vcpupin_list) = cputune.vcpupin {
                for vcpupin in vcpupin_list {
                    let mut vcpupin_elem = BytesStart::new("vcpupin");
                    vcpupin_elem.push_attribute(("vcpu", vcpupin.vcpu.to_string().as_str()));
                    vcpupin_elem.push_attribute(("cpuset", vcpupin.cpuset.as_str()));
                    writer.write_event(Event::Empty(vcpupin_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref emulatorpin) = cputune.emulatorpin {
                let mut emulatorpin_elem = BytesStart::new("emulatorpin");
                emulatorpin_elem.push_attribute(("cpuset", emulatorpin.cpuset.as_str()));
                writer.write_event(Event::Empty(emulatorpin_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref iothreadpin_list) = cputune.iothreadpin {
                for iothreadpin in iothreadpin_list {
                    let mut iothreadpin_elem = BytesStart::new("iothreadpin");
                    iothreadpin_elem
                        .push_attribute(("iothread", iothreadpin.iothread.to_string().as_str()));
                    iothreadpin_elem.push_attribute(("cpuset", iothreadpin.cpuset.as_str()));
                    writer
                        .write_event(Event::Empty(iothreadpin_elem))
                        .map_err(|e| e.to_string())?;
                }
            }

            if let Some(shares) = cputune.shares {
                Self::write_element(writer, "shares", &shares.to_string())?;
            }

            if let Some(period) = cputune.period {
                Self::write_element(writer, "period", &period.to_string())?;
            }

            if let Some(quota) = cputune.quota {
                Self::write_element(writer, "quota", &quota.to_string())?;
            }

            if let Some(global_period) = cputune.global_period {
                Self::write_element(writer, "global_period", &global_period.to_string())?;
            }

            if let Some(global_quota) = cputune.global_quota {
                Self::write_element(writer, "global_quota", &global_quota.to_string())?;
            }

            if let Some(emulator_period) = cputune.emulator_period {
                Self::write_element(writer, "emulator_period", &emulator_period.to_string())?;
            }

            if let Some(emulator_quota) = cputune.emulator_quota {
                Self::write_element(writer, "emulator_quota", &emulator_quota.to_string())?;
            }

            if let Some(iothread_period) = cputune.iothread_period {
                Self::write_element(writer, "iothread_period", &iothread_period.to_string())?;
            }

            if let Some(iothread_quota) = cputune.iothread_quota {
                Self::write_element(writer, "iothread_quota", &iothread_quota.to_string())?;
            }

            if let Some(ref vcpusched_list) = cputune.vcpusched {
                for vcpusched in vcpusched_list {
                    let mut vcpusched_elem = BytesStart::new("vcpusched");
                    vcpusched_elem.push_attribute(("vcpus", vcpusched.vcpus.as_str()));
                    vcpusched_elem.push_attribute(("scheduler", vcpusched.scheduler.as_str()));
                    if let Some(ref priority) = vcpusched.priority {
                        vcpusched_elem.push_attribute(("priority", priority.to_string().as_str()));
                    }
                    writer.write_event(Event::Empty(vcpusched_elem)).map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref iothreadsched_list) = cputune.iothreadsched {
                for iothreadsched in iothreadsched_list {
                    let mut iothreadsched_elem = BytesStart::new("iothreadsched");
                    iothreadsched_elem
                        .push_attribute(("iothreads", iothreadsched.iothreads.as_str()));
                    iothreadsched_elem
                        .push_attribute(("scheduler", iothreadsched.scheduler.as_str()));
                    if let Some(ref priority) = iothreadsched.priority {
                        iothreadsched_elem
                            .push_attribute(("priority", priority.to_string().as_str()));
                    }
                    writer
                        .write_event(Event::Empty(iothreadsched_elem))
                        .map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref emulatorsched) = cputune.emulatorsched {
                let mut emulatorsched_elem = BytesStart::new("emulatorsched");
                emulatorsched_elem.push_attribute(("scheduler", emulatorsched.scheduler.as_str()));
                if let Some(ref priority) = emulatorsched.priority {
                    emulatorsched_elem.push_attribute(("priority", priority.to_string().as_str()));
                }
                writer.write_event(Event::Empty(emulatorsched_elem)).map_err(|e| e.to_string())?;
            }

            if let Some(ref cachetune_list) = cputune.cachetune {
                for cachetune in cachetune_list {
                    let mut cachetune_elem = BytesStart::new("cachetune");
                    cachetune_elem.push_attribute(("vcpus", cachetune.vcpus.as_str()));
                    writer.write_event(Event::Start(cachetune_elem)).map_err(|e| e.to_string())?;

                    if let Some(ref cache_list) = cachetune.cache {
                        for cache in cache_list {
                            let mut cache_elem = BytesStart::new("cache");
                            if let Some(level) = cache.level {
                                cache_elem.push_attribute(("level", level.to_string().as_str()));
                            }
                            if let Some(ref mode) = cache.mode {
                                cache_elem.push_attribute(("mode", mode.as_str()));
                            }
                            writer
                                .write_event(Event::Empty(cache_elem))
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

            if let Some(ref memorytune_list) = cputune.memorytune {
                for memorytune in memorytune_list {
                    let mut memorytune_elem = BytesStart::new("memorytune");
                    memorytune_elem.push_attribute(("vcpus", memorytune.vcpus.as_str()));
                    writer.write_event(Event::Start(memorytune_elem)).map_err(|e| e.to_string())?;

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

        if let Some(ref memtune) = config.memory_tuning {
            let memtune_elem = BytesStart::new("memtune");
            writer.write_event(Event::Start(memtune_elem)).map_err(|e| e.to_string())?;

            if let Some(hard_limit) = memtune.hard_limit {
                Self::write_element(writer, "hard_limit", &hard_limit.to_string())?;
            }

            if let Some(soft_limit) = memtune.soft_limit {
                Self::write_element(writer, "soft_limit", &soft_limit.to_string())?;
            }

            if let Some(swap_hard_limit) = memtune.swap_hard_limit {
                Self::write_element(writer, "swap_hard_limit", &swap_hard_limit.to_string())?;
            }

            if let Some(guarantee) = memtune.guarantee {
                Self::write_element(writer, "min_guarantee", &guarantee.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new("memtune"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref blkiotune) = config.blockio_tuning {
            let blkiotune_elem = BytesStart::new("blkiotune");
            writer.write_event(Event::Start(blkiotune_elem)).map_err(|e| e.to_string())?;

            if let Some(weight) = blkiotune.weight {
                Self::write_element(writer, "weight", &weight.to_string())?;
            }

            if let Some(ref device_weight_list) = blkiotune.device_weight {
                for device_weight in device_weight_list {
                    let device_elem = BytesStart::new("device");
                    writer.write_event(Event::Start(device_elem)).map_err(|e| e.to_string())?;

                    Self::write_element(writer, "path", &device_weight.dev)?;
                    Self::write_element(writer, "weight", &device_weight.weight.to_string())?;

                    writer
                        .write_event(Event::End(BytesEnd::new("device")))
                        .map_err(|e| e.to_string())?;
                }
            }

            if let Some(ref throttle) = blkiotune.throttle {
                if let Some(read_bytes_sec) = throttle.read_bytes_sec {
                    Self::write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
                }
                if let Some(write_bytes_sec) = throttle.write_bytes_sec {
                    Self::write_element(writer, "write_bytes_sec", &write_bytes_sec.to_string())?;
                }
                if let Some(read_iops_sec) = throttle.read_iops_sec {
                    Self::write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
                }
                if let Some(write_iops_sec) = throttle.write_iops_sec {
                    Self::write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
                }
            }

            writer
                .write_event(Event::End(BytesEnd::new("blkiotune")))
                .map_err(|e| e.to_string())?;
        }

        if let Some(ref resource) = config.resource_partitioning {
            let resource_elem = BytesStart::new("resource");
            writer.write_event(Event::Start(resource_elem)).map_err(|e| e.to_string())?;

            if let Some(ref cpuset) = resource.cpuset {
                Self::write_element(writer, "partition", cpuset)?;
            }

            writer.write_event(Event::End(BytesEnd::new("resource"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref pm) = config.power_management {
            let pm_elem = BytesStart::new("pm");
            writer.write_event(Event::Start(pm_elem)).map_err(|e| e.to_string())?;

            let mut std_elem = BytesStart::new("suspend-to-disk");
            std_elem.push_attribute(("enabled", if pm.suspend_to_disk { "yes" } else { "no" }));
            writer.write_event(Event::Empty(std_elem)).map_err(|e| e.to_string())?;

            let mut stm_elem = BytesStart::new("suspend-to-mem");
            stm_elem.push_attribute(("enabled", if pm.suspend_to_ram { "yes" } else { "no" }));
            writer.write_event(Event::Empty(stm_elem)).map_err(|e| e.to_string())?;

            writer.write_event(Event::End(BytesEnd::new("pm"))).map_err(|e| e.to_string())?;
        }

        if let Some(ref throttlegroup) = config.disk_throttle_group {
            let tg_elem = BytesStart::new("throttlegroups");
            writer.write_event(Event::Start(tg_elem)).map_err(|e| e.to_string())?;

            let tgroup_elem = BytesStart::new("throttlegroup");
            writer.write_event(Event::Start(tgroup_elem)).map_err(|e| e.to_string())?;

            Self::write_element(writer, "group_name", &throttlegroup.name)?;

            if let Some(ref throttle) = throttlegroup.throttle {
                if let Some(read_bytes_sec) = throttle.read_bytes_sec {
                    Self::write_element(writer, "read_bytes_sec", &read_bytes_sec.to_string())?;
                }
                if let Some(write_bytes_sec) = throttle.write_bytes_sec {
                    Self::write_element(writer, "write_bytes_sec", &write_bytes_sec.to_string())?;
                }
                if let Some(read_iops_sec) = throttle.read_iops_sec {
                    Self::write_element(writer, "read_iops_sec", &read_iops_sec.to_string())?;
                }
                if let Some(write_iops_sec) = throttle.write_iops_sec {
                    Self::write_element(writer, "write_iops_sec", &write_iops_sec.to_string())?;
                }
            }

            writer
                .write_event(Event::End(BytesEnd::new("throttlegroup")))
                .map_err(|e| e.to_string())?;

            writer
                .write_event(Event::End(BytesEnd::new("throttlegroups")))
                .map_err(|e| e.to_string())?;
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

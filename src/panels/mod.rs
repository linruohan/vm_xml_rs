pub mod advanced;
mod cpu_panel;
mod devices_panel;
mod general_panel;
#[macro_use]
mod macros;
mod memory_panel;
mod os_panel;
pub(crate) mod utils;

pub use advanced::*;
pub use cpu_panel::CPUPanel;
pub use devices_panel::DevicesPanel;
pub use general_panel::GeneralPanel;
pub use memory_panel::MemoryPanel;
pub use os_panel::OSPanel;

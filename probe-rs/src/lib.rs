//! # Debugging toolset for embedded devices
//!
//!  
//! # Prerequisites
//!
//! - Udev rules
//! - libusb
//!
//! # Examples
//!
//!
//! ## Halting the attached chip
//! ```no_run
//! # use probe_rs::Error;
//! use probe_rs::Probe;
//!
//! // Get a list of all available debug probes.
//! let probes = Probe::list_all();
//!
//! // Use the first probe found.
//! let mut probe = probes[0].open()?;
//!
//! // Attach to a chip.
//! let mut session = probe.attach("nrf52")?;
//!
//! // Select a core.
//! let mut core = session.core(0)?;
//!
//! // Halt the attached core.
//! core.halt(std::time::Duration::from_millis(10))?;
//! # Ok::<(), Error>(())
//! ```
//!
//! ## Reading from RAM
//!
//! ```no_run
//! # use probe_rs::Error;
//! use probe_rs::Session;
//! use probe_rs::MemoryInterface;
//!
//! let mut session = Session::auto_attach("nrf52")?;
//! let mut core = session.core(0)?;
//!
//! // Read a block of 50 32 bit words.
//! let mut buff = [0u32;50];
//! core.read_32(0x2000_0000, &mut buff)?;
//!
//! // Read a single 32 bit word.
//! let word = core.read_word_32(0x2000_0000)?;
//!
//! // Writing is just as simple.
//! let buff = [0u32;50];
//! core.write_32(0x2000_0000, &buff)?;
//!
//! // of course we can also write 8bit words.
//! let buff = [0u8;50];
//! core.write_8(0x2000_0000, &buff)?;
//!
//! # Ok::<(), Error>(())
//! ```
//!
//! probe-rs is built around 5 main interfaces: the [Probe],
//! [Target], [Session], [Memory] and [Core] strucs.

#[macro_use]
extern crate serde;

pub mod architecture;
pub mod config;
mod core;
pub mod debug;
mod error;
pub mod flashing;
mod memory;
mod probe;
mod session;

pub use crate::config::{CoreType, Target};
pub use crate::core::{
    Architecture, Breakpoint, BreakpointId, CommunicationInterface, Core, CoreInformation,
    CoreInterface, CoreList, CoreRegister, CoreRegisterAddress, CoreState, CoreStatus, HaltReason,
    RegisterFile, SpecificCoreState,
};
pub use crate::error::Error;
pub use crate::memory::{Memory, MemoryInterface};
pub use crate::probe::{
    AttachMethod, DebugProbe, DebugProbeError, DebugProbeInfo, DebugProbeSelector, DebugProbeType,
    Probe, ProbeCreationError, WireProtocol,
};
pub use crate::session::Session;

// TODO: Hide behind feature
pub use crate::probe::fake_probe::FakeProbe;

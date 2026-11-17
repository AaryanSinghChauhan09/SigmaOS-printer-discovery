//! SigmaOS-printer-discovery — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_printer_discovery_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-printer-discovery: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-printer-discovery: ok");
}

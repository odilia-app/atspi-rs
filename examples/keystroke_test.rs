use atspi::events::*;

use glib::translate::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    atspi::init()?;
    let listener = DeviceListener::new(|e| {
        println!("Pressed key!");
        true
    });
    atspi::register_keystroke_listener(
        &listener,
        None,
        1,
        EventType::KeyPressedEvent.into_glib() as _,
        KeyListenerSyncType::SYNCHRONOUS | KeyListenerSyncType::ALL_WINDOWS,
    )?;
    Event::main();
    atspi::exit();
    Ok(())
}

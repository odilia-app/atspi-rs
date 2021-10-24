use atspi::events::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    atspi::init()?;
    let listener = EventListener::new(|e| {
        println!(
            "Event {} {{\n\tdetail1: {},\n\tdetail2: {}\n}}",
            e.kind(),
            e.detail1(),
            e.detail2()
        );
    });
    listener.register("object:state-changed:focused")?;
    Event::main();
    atspi::exit();
    Ok(())
}

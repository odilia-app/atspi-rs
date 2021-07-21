use atspi::traits::AccessibleExt;

fn main() {
    if let Err(e) = atspi::init() {
        eprintln!("Error initialising libatspi: {}", e);
        std::process::exit(1);
    }

    let desktop = atspi::desktop(0).expect("Desktop 0 should exist");

    println!("Index Child Count Name");
    for (i, child) in desktop
        .children()
        .expect("Could not get number of children of desktop")
        .enumerate()
    {
        let child = child.expect("Could not get child of desktop");
        let name = child.name().expect("Could not get name");
        let child_count = child.child_count().expect("Could not get child count");
        println!("{:4}  {:4}         {}", i, child_count, name);
    }
}

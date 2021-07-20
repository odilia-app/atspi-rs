use atspi::traits::AccessibleExt;

fn main() {
    if atspi::init() != 0 {
        eprintln!("Error initialising at-spi");
        std::process::exit(1);
    }

    let desktop = atspi::desktop(0).expect("Desktop 0 should exist");

    println!("Index Child Count Name");
    for i in 0..desktop.child_count().expect("Could not get child count") {
        let child = desktop
            .child_at_index(i)
            .expect("Could not get child of desktop");
        let name = child.name().expect("Could not get name");
        let child_count = child.child_count().expect("Could not get child count");
        println!("{:4}  {:4}         {}", i, child_count, name);
    }
}

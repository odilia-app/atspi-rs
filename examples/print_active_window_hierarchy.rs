use atspi::{
    accessible::*,
    state::{StateSetExt, StateType},
};

fn main() {
    if let Err(e) = atspi::init() {
        eprintln!("Error initialising libatspi: {}", e);
        std::process::exit(1);
    }

    if let Some(win) = active_window() {
        let name = win.name().expect("Could not get window name");
        println!("Active window: {:?}", name);
        print_tree(&win, 0);
    } else {
        println!("Could not determine active window");
    }
}

fn active_window() -> Option<Accessible> {
    let desktop = atspi::desktop(0).expect("Desktop 0 should exist");
    for app in desktop
        .children()
        .expect("could not get children of desktop")
        .map(|app| app.expect("Could not get application from desktop"))
    {
        if let Some(active) = app
            .children()
            .expect("Could not get children of application")
            .map(|win| win.expect("could not get window"))
            .find(|win| {
                if let Some(state) = win.state_set() {
                    state.contains(StateType::Active)
                } else {
                    false
                }
            })
        {
            return Some(active);
        }
    }
    None
}

fn print_tree(accessible: &Accessible, level: usize) {
    match accessible.children() {
        Ok(children) => {
            for child in children.filter_map(|c| match c {
                Ok(c) => Some(c),
                Err(e) => {
                    eprintln!("Warning: Could not get child: {}", e);
                    None
                }
            }) {
                print_accessible(&child, level);
                print_tree(&child, level + 1);
            }
        }
        Err(e) => {
            if let Ok(name) = accessible.name() {
                eprintln!("Warning: Could not get children of {}: {}", name, e);
            } else {
                eprintln!("Warning: Could not get children: {}", e);
            }
        }
    }
}

fn print_accessible(accessible: &Accessible, level: usize) {
    let role_name = accessible.role_name().unwrap_or_else(|e| {
        eprintln!("Warning: Could not get role name: {}", e);
        "<unknown>".into()
    });
    let name = accessible.name().unwrap_or_else(|e| {
        eprintln!("Warning: Could not get role name: {}", e);
        "<unknown>".into()
    });
    println!("{0:1$}{2}: {3:?}", "", level * 2, role_name, name);
}

use std::process::Command;
use std::thread;
use std::time;

fn main() {
    let device = "Keychron  Keychron Link "; // YES THE SPACE AT THE END MATTERS I DONT KNOW WHY ITS DUMB
    let sleep_time = time::Duration::from_secs(15);
    let window_to_search_for = String::from("Sober");

    loop {
        thread::sleep(sleep_time);
        if sober_open(&window_to_search_for) {
            enable_preset(device);
        } else {
            continue;
        }

        while sober_open(&window_to_search_for) {
            thread::sleep(sleep_time);
        }

        if !sober_open(&window_to_search_for) {
            disable_preset(device);
        }
    }
}

// Gets the window ID's of all currently opened windows
fn get_window_ids() -> Vec<String> {
    let command_output = Command::new("kdotool").arg("search").output().unwrap();

    let output = String::from_utf8(command_output.stdout).expect("String was not a valid UTF 8");

    output.lines().map(String::from).collect()
}

fn enable_preset(device: &str) {
    let preset = "DVN";

    Command::new("input-remapper-control")
        .arg("--command")
        .arg("start")
        .arg("--device")
        .arg(device)
        .arg("--preset")
        .arg(preset)
        .output()
        .unwrap();
}

fn disable_preset(device: &str) {
    let _ = Command::new("input-remapper-control")
        .arg("--command")
        .arg("stop")
        .arg("--device")
        .arg(device)
        .output();
}

fn sober_open(window_to_search_for: &String) -> bool {
    let window_ids = get_window_ids();

    for id in window_ids {
        let window_name = String::from_utf8(
            Command::new("kdotool")
                .arg("getwindowname")
                .arg(id)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        if window_name.trim() == window_to_search_for {
            return true;
        }
    }

    false
}

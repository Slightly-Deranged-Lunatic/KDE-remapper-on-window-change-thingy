use std::process::Command;
use std::time;
use std::thread;

fn main() {
    let device = "Keychron  Keychron Link "; // YES THE SPACE AT THE END MATTERS I DONT KNOW WHY ITS DUMB
    let sleep_time = time::Duration::from_secs(15);
    let window_to_search_for = String::from("Sober");

    loop {
        thread::sleep(sleep_time);
        if sober_open(&window_to_search_for) {
            enable_present(device);
        } else {
            continue;
        }

        while sober_open(&window_to_search_for) {
            thread::sleep(sleep_time);
        }

        if ! sober_open(&window_to_search_for) {
            disable_preset(device);
        }
    }
}

// Gets the window ID's of all currently opened windows
fn get_window_ids() -> Vec<String> {
    let command_output = Command::new("kdotool")
        .arg("search")
        .output()
        .unwrap();

    let mut output = String::new();

    if command_output.status.success() {
        output = String::from_utf8(command_output.stdout)
            .expect("String was not a valid UTF 8");
    } else {
        panic!("kdotool was not a success, output: {:?}", command_output);
    }
    let window_ids: Vec<String> = output.lines().map(String::from).collect();
    return window_ids
}

fn enable_present(device: &str) {
    let preset = "DVN";

    let output = Command::new("input-remapper-control")
        .arg("--command")
        .arg("start")
        .arg("--device")
        .arg(format!("{}", device))
        .arg("--preset")
        .arg(format!("{}", preset))
        .output()
        .unwrap();
}

fn disable_preset(device: &str) {
    let _ = Command::new("input-remapper-control")
        .arg("--command")
        .arg("stop")
        .arg("--device")
        .arg(format!("{}", device))
        .output();
}

fn sober_open(window_to_search_for: &String) -> bool {
    let window_ids = get_window_ids();


    for id in window_ids {
        let window_name = String::from_utf8(
            Command::new("kdotool")
            .arg("getwindowname")
            .arg(format!("{}", id))
            .output()
            .unwrap()
            .stdout
        ).unwrap();
        if window_name.trim() == window_to_search_for {
            return true;
        }
    }

    return false;
}
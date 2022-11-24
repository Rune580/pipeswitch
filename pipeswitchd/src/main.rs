use std::sync::mpsc::channel;

use pipeswitch_lib::Pipeswitch;
use sdl2::keyboard::Keycode;

mod sdl2_signaller;

fn main() {
    let ps = Pipeswitch::new(None).unwrap();

    let (sender, receiver) = channel();
    sdl2_signaller::open_window_thread(sender);
    while let Ok(keycode) = receiver.recv() {
        println!("starting");
        let state = ps.lock_current_state();
        println!("Current nodes:");
        let mut output = None;
        let mut input = None;
        let mut name = "spotify";
        if keycode == Keycode::A {
            name = "Firefox"
        }
        for (node_id, node) in &state.nodes {
            let node_name = &node.node_name;
            let ports = state.ports_by_node(*node_id);
            let names: Vec<&String> = state
                .ports_by_node(*node_id)
                .into_iter()
                .map(|p| &p.name)
                .collect();
            println!("Node {node_id} '{node_name}' with ports {names:?}");
            if node.node_name == "WEBRTC VoiceEngine" {
                for port in &ports {
                    if port.name == "input_MONO" {
                        input = Some((*port).clone());
                    }
                }
            }
            if node.node_name == name {
                for port in &ports {
                    if port.name == "output_FR" {
                        output = Some((*port).clone());
                    }
                }
            }
        }
        drop(state);

        if let (Some(o), Some(i)) = (output, input) {
            let link = ps.create_link(o, i).unwrap();
            println!("{link:?}");
        }

        if keycode == Keycode::Escape {
            break;
        }
    }
}

mod multiplayer;

use clap::Parser;
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 400;
const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 30.0;
const SPEED: f32 = 2.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    room: Option<String>,
    #[arg(long, default_value_t = false)]
    host: bool,
}

fn conf() -> Conf {
    Conf {
        window_title: "Minimal Macroquad".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn main() {
    let args = Args::parse();
    if args.host && args.name.is_none() && args.room.is_none() {
        bummer::start(multiplayer::PlayerState::default());
    } else {
        macroquad::Window::from_config(conf(), run(args));
    }
}

async fn run(args: Args) {
    let settings = bummer::get_settings();
    if args.host {
        let _ = std::thread::spawn(move || {
            bummer::start(multiplayer::PlayerState::default());
        });
    }
    let my_name = args
        .name
        .expect("argument --name required when not soley hosting");
    let room_id = args
        .room
        .expect("argument --room required when not soley hosting");
    if !multiplayer::check_room(&settings, &room_id) {
        multiplayer::create_room(&settings, &room_id);
    }
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    let mut my_pos = (100f32, 100f32);
    loop {
        clear_background(WHITE);

        draw_rectangle(my_pos.0, my_pos.1, PLAYER_WIDTH, PLAYER_HEIGHT, GREEN);
        draw_text(&my_name, my_pos.0, my_pos.1, PLAYER_WIDTH, BLACK);

        let vertical: f32 = if is_key_down(KeyCode::W) { -1.0 } else { 0.0 }
            + if is_key_down(KeyCode::S) { 1.0 } else { 0.0 };
        let horizontal: f32 = if is_key_down(KeyCode::A) { -1.0 } else { 0.0 }
            + if is_key_down(KeyCode::D) { 1.0 } else { 0.0 };
        my_pos = (
            (my_pos.0 + (horizontal * SPEED)).clamp(0.0, WINDOW_WIDTH as f32 - PLAYER_WIDTH),
            (my_pos.1 + (vertical * SPEED)).clamp(0.0, WINDOW_HEIGHT as f32 - PLAYER_HEIGHT),
        );

        let response = multiplayer::udp(&settings, &socket, &my_name, my_pos.0, my_pos.1);
        let game_message: Result<bummer::udp::data::GameMessage<multiplayer::PlayerState>, _> =
            serde_json::from_str(&response);
        if let Ok(msg) = game_message {
            for (player_name, player_state) in msg.state.data.iter() {
                if player_name != &my_name {
                    let pos = player_state.state.position;
                    draw_rectangle(pos.0, pos.1, PLAYER_WIDTH, PLAYER_HEIGHT, RED);
                    draw_text(player_name, pos.0, pos.1, PLAYER_WIDTH, BLACK);
                }
            }
        }

        next_frame().await
    }
}

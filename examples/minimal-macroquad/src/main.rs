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
    name: String,
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

#[macroquad::main(conf)]
async fn main() {
    let args = Args::parse();
    if args.host {
        let _ = std::thread::spawn(move || {
            bummer::start(multiplayer::PlayerState::default());
        });
        multiplayer::create_room();
    } else {
        multiplayer::check_room();
    }
    let socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let mut my_pos = (100f32, 100f32);
    loop {
        clear_background(WHITE);

        draw_rectangle(my_pos.0, my_pos.1, PLAYER_WIDTH, PLAYER_HEIGHT, GREEN);
        draw_text(&args.name, my_pos.0, my_pos.1, PLAYER_WIDTH, BLACK);

        let vertical: f32 = if is_key_down(KeyCode::W) { -1.0 } else { 0.0 }
            + if is_key_down(KeyCode::S) { 1.0 } else { 0.0 };
        let horizontal: f32 = if is_key_down(KeyCode::A) { -1.0 } else { 0.0 }
            + if is_key_down(KeyCode::D) { 1.0 } else { 0.0 };
        my_pos = (
            (my_pos.0 + (horizontal * SPEED)).clamp(0.0, WINDOW_WIDTH as f32 - PLAYER_WIDTH),
            (my_pos.1 + (vertical * SPEED)).clamp(0.0, WINDOW_HEIGHT as f32 - PLAYER_HEIGHT),
        );

        let response = multiplayer::udp(&socket, &args.name, my_pos.0, my_pos.1);
        let tagged_response: bummer::udp::data::TaggedMessage = serde_json::from_str(&response).unwrap();
        let game_message: bummer::udp::data::GameMessage<multiplayer::PlayerState> =
            serde_json::from_str(&tagged_response.data).unwrap();
        for (player_name, player_state) in game_message.state.data.iter() {
            if player_name != &args.name {
                let pos = player_state.state.position;
                draw_rectangle(pos.0, pos.1, PLAYER_WIDTH, PLAYER_HEIGHT, RED);
                draw_text(player_name, pos.0, pos.1, PLAYER_WIDTH, BLACK);
            }
        }

        next_frame().await
    }
}
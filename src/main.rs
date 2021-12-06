extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate impl_ops;

use crate::num::ToPrimitive;

mod chess;

use chess::chess_piece::ChessPiece;
use chess::chess_player::ChessPlayer;
use chess::position::*;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::path::Path;
use std::time::Duration;
// use std::io;

struct ChessDrawing {}

const CHESS_PIECES_TEXTURES_PATHS: [&str; 12] = [
    "images/white_pawn.png",
    "images/white_rook.png",
    "images/white_knight.png",
    "images/white_bishop.png",
    "images/white_queen.png",
    "images/white_king.png",
    "images/black_pawn.png",
    "images/black_rook.png",
    "images/black_knight.png",
    "images/black_bishop.png",
    "images/black_queen.png",
    "images/black_king.png",
];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

impl ChessDrawing {
    pub fn new() -> Self {
        Self {}
    }
    pub fn draw(&self) {
        assert_eq!(WIDTH % 8, 0);
        assert_eq!(HEIGHT % 8, 0);
        let mut pos: Position = Position::default();
        //let mut pos: Position = rand::random();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let chess_pieces_textures: Vec<Texture> = CHESS_PIECES_TEXTURES_PATHS
            .iter()
            .map(|&p| texture_creator.load_texture(Path::new(p)).unwrap())
            .collect();

        //canvas.set_draw_color(Color::RGB(0, 255, 255));
        //canvas.clear();
        //canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
        let mut first = true;
        let mut up = 0;
        let mut last_up = 0;
        let mut valid_moves: Vec<(usize, usize, usize, usize)> = Vec::new();
        'running: loop {
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            for i in 0..8 {
                for j in 0..8 {
                    let maybe_player = pos.get_player_on_position(i, j);
                    if let Some(player) = maybe_player {
                        let num = match player {
                            ChessPlayer::White => {
                                pos.get_piece_on_position(i, j).unwrap().to_usize().unwrap()
                            }
                            ChessPlayer::Black => {
                                pos.get_piece_on_position(i, j).unwrap().to_usize().unwrap() + 6
                            }
                        };

                        canvas
                            .copy(
                                &chess_pieces_textures[num],
                                None,
                                Rect::new(
                                    (j as i32) * ((WIDTH / 8) as i32),
                                    (7 - i as i32) * ((HEIGHT / 8) as i32),
                                    WIDTH / 8,
                                    HEIGHT / 8,
                                ),
                            )
                            .unwrap();

                        //valid moves
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        for (_, _, to_i, to_j) in valid_moves.iter() {
                            canvas
                                .fill_rect(Rect::new(
                                    (*to_j as i32) * ((WIDTH / 8) as i32),
                                    (7 - *to_i as i32) * ((HEIGHT / 8) as i32),
                                    WIDTH / 80,
                                    HEIGHT / 80,
                                ))
                                .unwrap();
                        }
                        canvas.set_draw_color(Color::RGB(0, 255, 255));
                    }
                }
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            let mouse_state = event_pump.mouse_state();
            if mouse_state.left() && first && up == last_up {
                // println!("A");
                a = mouse_state.x() / ((WIDTH / 8) as i32);
                b = mouse_state.y() / ((HEIGHT / 8) as i32);

                let maybe_player = pos.get_player_on_position(7 - b as usize, a as usize);

                if maybe_player.is_none() || (maybe_player.unwrap() != pos.get_player_on_move()) {
                } else {
                    valid_moves = pos
                        .get_valid_moves()
                        .iter()
                        .filter(|(k, l, _, _)| *k == (7 - b as usize) && *l == (a as usize))
                        .cloned()
                        .collect();

                    first = false;
                    last_up += 1;
                }
            } else if mouse_state.left() && (!first) && up == last_up {
                // println!("B");
                c = mouse_state.x() / ((WIDTH / 8) as i32);
                d = mouse_state.y() / ((HEIGHT / 8) as i32);
                let maybe_player = pos.get_player_on_position(7 - d as usize, c as usize);
                if maybe_player.is_some() && maybe_player.unwrap() == pos.get_player_on_move() {
                    first = true;
                } else {
                    let m = pos.clone().make_move(
                        7 - b as usize,
                        a as usize,
                        7 - d as usize,
                        c as usize,
                    );
                    if m {
                        println!("Good move!");
                        pos.make_move(7 - b as usize, a as usize, 7 - d as usize, c as usize);
                    } else {
                        println!("Bad move!");
                    }
                    first = true;
                    last_up += 1;
                }
            } else if (!mouse_state.left()) && last_up != up {
                // println!("C");
                up += 1;
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

fn main() {
    let draw = ChessDrawing::new();
    draw.draw();
    /*for _ in 0..100 {
        let pos: Position = rand::random();
        println!("Pos is:\n{}", pos);
    }

    return;*/

    /*let mut pos: Position = Position::default();
    loop {
        println!("Pos is:\n{}", pos);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let inputs: Vec<usize> = line
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        pos.make_move(inputs[0], inputs[1], inputs[2], inputs[3]);
    }*/
}

#![allow(unused)]

use std::panic::set_hook;
use std::sync::{LazyLock, Mutex, MutexGuard};
use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
extern "C" {
    fn render() -> ();
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
    #[wasm_bindgen(js_namespace=console)]
    fn error(s: String);
}

struct Attacker {
    x: u16,
    y: u16,
}

struct World {
    world: [[Option<Object>; 47]; 27],
}

#[derive(Clone, Copy)]
enum Object {
    Path(Dir),
    Castle,
    Spawner(Dir),
}

#[derive(Clone, Copy)]
enum Dir {
    U,
    L,
    D,
    R,
}

static ATTACKERS: LazyLock<Mutex<Vec<Attacker>>> = LazyLock::new(|| Mutex::new(vec![]));
static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| {
    Mutex::new(World {
        world: [[const { None }; 47]; 27],
    })
});

#[wasm_bindgen]
pub fn game_start() {
    init();
    init_world(1);
}

fn init() {
    set_hook(Box::new(console_error_panic_hook::hook));
}

fn init_world(world_number: u8) {
    parse(match world_number {
        1 => {
            "\
            r>>>>>>>>>>>>v..................................\
            .............v..................................\
            .............v..................................\
            .............v...........................c......\
            .............v...........................^......\
            .............v...........................^......\
            .............v...........................^......\
            .............v...........................^......\
            .............v...........................^......\
            .............v...........................^......\
            .............>>>>>>>>>>>>>>>>>v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................v..........^......\
            ..............................>>>>>>>>>>>^......\
            ................................................\
            ................................................\
            ................................................\
            ................................................"
        }
        _ => {
            error("Error, invalid world_number".to_string());
            panic!()
        }
    });
}

/// Input string will be 47*27 (1269)
/// Requires `WORLD`
///
/// ## Path
///
/// `^` &rarr; upward\
/// `<` &rarr; left\
/// `v` &rarr; downward\
/// `>` &rarr; right\
///
/// ## Spawner
///
/// `u` &rarr; upward\
/// `a` &rarr; left\
/// `s` &rarr; downward\
/// `d` &rarr; right\
///
/// ## Others
///
/// `c` &rarr; castle\
/// `.` &rarr; empty space
fn parse(level: &str) {
    let mut world: MutexGuard<'_, World> = WORLD.lock().unwrap();
    let mut char_iter: std::str::Chars<'_> = level.chars();

    for y in 0..27 {
        for x in 0..47 {
            world.world[y][x] = match char_iter.next() {
                Some('^') => Some(Object::Path(Dir::U)),
                Some('<') => Some(Object::Path(Dir::L)),
                Some('v') => Some(Object::Path(Dir::D)),
                Some('>') => Some(Object::Path(Dir::R)),

                Some('u') => Some(Object::Spawner(Dir::U)),
                Some('l') => Some(Object::Spawner(Dir::L)),
                Some('d') => Some(Object::Spawner(Dir::D)),
                Some('r') => Some(Object::Spawner(Dir::R)),

                Some('c') => Some(Object::Castle),
                Some('.') => None,

                _ => {
                    error("Wrong Level Encoding".to_string());
                    panic!();
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn game_tick() {}

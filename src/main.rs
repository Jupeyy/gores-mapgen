mod grid_test;
use grid_test::*;

use ndarray::Array2;

use egui::{epaint::Shadow, Color32, Frame, Label, Margin, Rect};
use macroquad::prelude::*;

const LEVEL_SIZE: usize = 100;

fn window_frame() -> Frame {
    Frame {
        fill: Color32::from_gray(0),
        inner_margin: Margin::same(5.0),
        shadow: Shadow::NONE,
        ..Default::default()
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        ..Default::default()
    }
}

enum ShiftDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

// using my own position vector to meet ndarray's indexing standard using usize
// while glam has nice performance benefits, the amount of expensive operations
// on the position vector will be very limited, so this should be fine..
#[derive(Debug, Default)]
struct WalkerPos {
    x: usize, 
    y: usize
}

impl WalkerPos {
    fn as_index(&self) -> [usize;2] {
        [self.x, self.y]
    }  

    fn shift(&mut self, shift: ShiftDirection) {
        match shift {
            ShiftDirection::UP => {self.y -= 1},
            ShiftDirection::RIGHT => {self.x += 1},
            ShiftDirection::DOWN => {self.y -= 1},
            ShiftDirection::LEFT => {self.x -= 1}
        }
    }
}

// this walker is indeed very cute
#[derive(Default, Debug)]
struct CuteWalker {
    pos: WalkerPos,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_rect: Rect = Rect::EVERYTHING;
    let mut grid = Array2::from_elem((LEVEL_SIZE, LEVEL_SIZE), BlockType::Empty);
    let mut walker = CuteWalker::default();

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::SidePanel::right("right_panel").show(egui_ctx, |ui| {
                ui.label("hello world");
                ui.separator();
            });

            egui::Window::new("yeah")
                .frame(window_frame())
                .show(egui_ctx, |ui| {
                    ui.add(Label::new("this is some UI stuff"));
                    ui.button("text").clicked();
                });

            main_rect = egui_ctx.available_rect();
        });

        // TODO: add proper mouse input xd
        // if main_rect.contains(mouse_position().into()) {
        //     handle_mouse_inputs(&mut display_factor, &mut display_shift);
        // }

        if is_key_released(KeyCode::Enter) {
            grid[walker.pos.as_index()] = BlockType::Filled;
            walker.pos.shift(ShiftDirection::RIGHT);
            dbg!(&walker);
        }

        let available_length = f32::min(main_rect.width(), main_rect.height()); // TODO: assumes square

        let display_factor = available_length / LEVEL_SIZE as f32;

        draw_grid_blocks(&grid, display_factor, vec2(0.0, 0.0));

        egui_macroquad::draw();
        next_frame().await
    }
}

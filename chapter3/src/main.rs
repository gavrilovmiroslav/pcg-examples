mod state;
mod util;

use crate::state::*;
use crate::util::configure_text_styles;

use notan::prelude::*;
use notan::draw::{CreateDraw, Draw, DrawConfig, DrawShapes};
use notan_egui::*;
use crate::panel::Side;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default().title("Cellular Automata (chapter 3)");

    notan::init_with(setup)
        .add_config(win)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .draw(drawing_fn)
        .build()
}

fn setup(_: &mut Graphics) -> State {
    State::default()
}

fn draw_automaton(draw: &mut Draw, state: &State) {
    let mut n = 0;
    let padding = (50.0, 50.0);
    let matrix = state.get_matrix();

    for i in 0i32..state.config.size.0 as i32 {
        for j in 0i32..state.config.size.1 as i32 {
            if matrix[n] == 1 {
                draw.rect((padding.0 + i as f32 * 10.0, padding.1 + j as f32 * 10.0), (10.0, 10.0));
            }
            n += 1;
        }
    }
}

fn drawing_fn(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let mut changes_made = false;

    let output = plugins.egui(|ctx| {
        use notan_egui::*;

        configure_text_styles(&ctx);

        SidePanel::new(Side::Right, "Cellular Automata").show(&ctx, |ui| {
            ui.heading("Generations");
            ui.label("The cellular automaton will go through this many generations");
            if Slider::new(&mut state.config.generations, 1usize..=10usize).ui(ui).changed() {
                changes_made = true;
            }

            ui.heading("Birth factor");
            ui.label("With this many neighbors around, a field will become a stone");
            ui.horizontal(|ui| {
                for i in 0..8 {
                    if ui.checkbox(&mut state.config.birth[i], "").changed() {
                        changes_made = true;
                    }
                }
            });

            ui.heading("Survival");
            ui.label("With this many neighbors around, a stone will stay a stone");
            ui.horizontal(|ui| {
                for i in 0..8 {
                    if ui.checkbox(&mut state.config.survival[i], "").changed() {
                        changes_made = true;
                    }
                }
            });

            ui.spacing();

            if ui.button("Save to Gallery").clicked() {
                state.save_to_gallery();
            }

            ui.separator();

            ui.heading("Gallery");

            for config in &state.gallery {
                if ui.button(config.to_string()).clicked() {
                    state.config.set_to(config);
                    changes_made = true;
                }
            }
        });
    });

    if changes_made {
        state.update();
    }

    draw_automaton(&mut draw, &state);

    gfx.render(&draw);
    gfx.render(&output);
}
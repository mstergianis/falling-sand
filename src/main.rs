use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::core::math::Vector2;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
// use raylib::RaylibHandle;
// use std::array;
// use std::collections::VecDeque;
// use std::fmt::Debug;

const GRAV: f32 = 50.0;
const BACKGROUND_COLOR: Color = Color::BLACK;
const FONT_SIZE: i32 = 60;
const BOX_HEIGHT: i32 = FONT_SIZE + 20;

enum GameState {
    Starting,
    Paused,
    Running,
}

struct Particle {
    kind: ParticleKind,
    pos: Vector2,
    vel: Vector2,
}

impl Particle {
    fn color(&self) -> Color {
        self.kind.color()
    }
}

#[derive(Eq, PartialEq)]
enum ParticleKind {
    Sand,
}

impl ParticleKind {
    fn color(&self) -> Color {
        match self {
            ParticleKind::Sand => Color::SANDYBROWN,
        }
    }

    fn name(&self) -> &str {
        match self {
            ParticleKind::Sand => "Sand",
        }
    }
}

impl Default for ParticleKind {
    fn default() -> Self {
        Self::Sand
    }
}

struct Dim {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

trait InBoundary {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn in_boundary(&self, pos: Vector2) -> bool {
        pos.x >= self.x() as f32
            && pos.x <= (self.x() + self.width()) as f32
            && pos.y >= self.y() as f32
            && pos.y <= (self.y() + self.height()) as f32
    }
}

impl InBoundary for Dim {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

struct Sandbox {
    dim: Dim,
    particles: Vec<Particle>,
    selected_particle: ParticleKind,
}

impl Sandbox {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Sandbox {
            dim: Dim {
                x,
                y,
                width,
                height,
            },
            particles: Vec::new(),
            selected_particle: ParticleKind::default(),
        }
    }

    fn spawn_particle(&mut self, kind: ParticleKind, pos: Vector2) {
        self.particles.push(Particle {
            kind,
            pos,
            vel: Vector2 { x: 0.0, y: 0.0 },
        });
    }

    fn evolve(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.vel.y += (GRAV * dt).clamp(-100.0, 100.0);
            particle.pos.y += particle.vel.y * dt;
        }

        self.particles.retain(|p| self.dim.in_boundary(p.pos));
    }

    fn in_boundary(&self, pos: Vector2) -> bool {
        self.dim.in_boundary(pos)
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, sandbox: &Sandbox) {
    draw.clear_background(BACKGROUND_COLOR);

    // draw border
    draw.draw_rectangle_lines(
        sandbox.dim.x,
        sandbox.dim.y,
        sandbox.dim.width,
        sandbox.dim.height,
        Color::WHEAT,
    );

    for particle in &sandbox.particles {
        draw.draw_rectangle(
            particle.pos.x as i32,
            particle.pos.y as i32,
            2,
            2,
            particle.color(),
        );
    }

    const PADDING: i32 = 30;
    let sand_selector = ParticleSelector::new(
        sandbox.dim.x + sandbox.dim.width + PADDING,
        sandbox.dim.y,
        draw.measure_text(ParticleKind::Sand.name(), FONT_SIZE),
        ParticleKind::Sand,
    );
    sand_selector.draw(&sandbox.selected_particle, draw);

    // draw help
    // draw.draw_text(
    //     "Help: ",
    //     window.active_width + step_size as i32,
    //     step_size as i32 * 3,
    //     20,
    //     Color::BLUE,
    // );
    // draw.draw_text(
    //     "Use the arrow keys to move",
    //     window.active_width + step_size as i32,
    //     (step_size as f32 * 3.5) as i32,
    //     10,
    //     Color::RED,
    // );
    // draw.draw_text(
    //     "and P to pause.",
    //     window.active_width + step_size as i32,
    //     (step_size as f32 * 3.75) as i32,
    //     10,
    //     Color::RED,
    // );

    // draw.draw_text(
    //     "Or, you can use W,A,S,D to move",
    //     window.active_width + step_size as i32,
    //     (step_size as f32 * 4.0) as i32,
    //     10,
    //     Color::RED,
    // );
    // draw.draw_text(
    //     "and space to pause.",
    //     window.active_width + step_size as i32,
    //     (step_size as f32 * 4.25) as i32,
    //     10,
    //     Color::RED,
    // );
}

struct ParticleSelector {
    dim: Dim,
    kind: ParticleKind,
}

impl ParticleSelector {
    fn new(x: i32, y: i32, text_width: i32, kind: ParticleKind) -> Self {
        Self {
            kind,
            dim: Dim {
                x,
                y,
                width: text_width + 20,
                height: BOX_HEIGHT,
            },
        }
    }

    fn draw(&self, selected_particle: &ParticleKind, draw: &mut RaylibDrawHandle) {
        if self.kind == *selected_particle {
            draw.draw_rectangle(
                self.dim.x - 10,
                self.dim.y,
                self.dim.width,
                BOX_HEIGHT,
                self.kind.color(),
            );

            draw.draw_text(
                self.kind.name(),
                self.dim.x,
                self.dim.y + 10,
                FONT_SIZE,
                BACKGROUND_COLOR,
            );

            return;
        }

        draw.draw_rectangle_lines(
            self.dim.x - 10,
            self.dim.y,
            self.dim.width,
            BOX_HEIGHT,
            self.kind.color(),
        );

        draw.draw_text(
            self.kind.name(),
            self.dim.x,
            self.dim.y + 10,
            FONT_SIZE,
            self.kind.color(),
        );
    }

    fn in_boundary(&self, pos: Vector2) -> bool {
        self.dim.in_boundary(pos)
    }
}

struct WindowDimensions {
    width: i32,
    height: i32,
}

impl WindowDimensions {
    fn new(width: i32, height: i32) -> WindowDimensions {
        WindowDimensions { width, height }
    }
}

fn is_pause_key(key: &Option<KeyboardKey>) -> bool {
    match key {
        Some(KeyboardKey::KEY_P | KeyboardKey::KEY_SPACE) => true,
        _ => false,
    }
}

fn main() {
    const WINDOW_WIDTH: i32 = 1920;
    const WINDOW_HEIGHT: i32 = 1080;

    let mut game_state = GameState::Starting;

    let mut sandbox = Sandbox::new(40, 40, 1000, 1000);

    let (mut rl, thd) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("Falling Sand")
        .build();

    const FPS: i32 = 60;
    rl.set_target_fps(FPS as u32);

    while !rl.window_should_close() {
        match game_state {
            GameState::Starting => {
                game_state = GameState::Running;
            }

            GameState::Running => {
                let dt = rl.get_frame_time();
                sandbox.evolve(dt);
                let key = rl.get_key_pressed();
                if is_pause_key(&key) {
                    game_state = GameState::Paused;
                    continue;
                }

                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
                    let mouse_pos = rl.get_mouse_position();
                    if sandbox.in_boundary(mouse_pos) {
                        sandbox.spawn_particle(ParticleKind::Sand, mouse_pos);
                    }
                }

                {
                    let mut draw = rl.begin_drawing(&thd);
                    draw_game(&mut draw, &sandbox);
                }
            }

            GameState::Paused => {
                {
                    let mut draw = rl.begin_drawing(&thd);
                    write_center(
                        "Paused. Press P to resume\n",
                        &mut draw,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                        50,
                    );
                }
                let key = rl.get_key_pressed();

                if is_pause_key(&key) {
                    game_state = GameState::Running
                }
            }
        }
    }
}

fn write_center(
    text: &str,
    draw: &mut RaylibDrawHandle,
    screen_width: i32,
    screen_height: i32,
    fontsize: i32,
) {
    let text_width = draw.measure_text(text, fontsize);
    let textx = screen_width / 2 - text_width / 2;
    let texty = screen_height / 2 - fontsize;

    draw.draw_rectangle(
        textx - 10,
        texty - 10,
        text_width + 20,
        fontsize + 20,
        Color::WHEAT,
    );
    draw.draw_text(text, textx, texty, fontsize, Color::RED);
}

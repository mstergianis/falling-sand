use std::slice::Iter;

use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::core::math::Vector2;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

const FPS: i32 = 60;
const GRAV: f32 = 50.0;
const BACKGROUND_COLOR: Color = Color::BLACK;
const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const WINDOW_MARGIN: i32 = 40;

fn main() {
    let mut game_state = GameState::Starting;

    let mut sandbox = Sandbox::new(WINDOW_MARGIN, WINDOW_MARGIN, 1000, 1000);

    let (mut rl, thd) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("Falling Sand")
        .build();

    rl.set_target_fps(FPS as u32);

    while !rl.window_should_close() {
        match game_state {
            GameState::Starting => {
                game_state = GameState::Running;
            }

            GameState::Running => {
                let mouse_pos = rl.get_mouse_position();

                let dt = rl.get_frame_time();
                sandbox.evolve(dt);

                let key = rl.get_key_pressed();
                if is_pause_key(&key) {
                    game_state = GameState::Paused;
                    continue;
                }

                if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
                    if sandbox.in_boundary(mouse_pos) {
                        sandbox.spawn_particle(mouse_pos);
                    }
                }

                if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
                    sandbox.particle_selector_clicked(mouse_pos);
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
        particle.draw(draw);
    }

    sandbox
        .selectors
        .iter()
        .for_each(|s| s.draw(&sandbox.selected_particle, draw));
}

struct Sandbox {
    dim: Dim,
    particles: Vec<Particle>,
    selected_particle: ParticleKind,
    selectors: SelectorGrid,
}

impl Sandbox {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            dim: Dim {
                x,
                y,
                width,
                height,
            },
            particles: Vec::new(),
            selected_particle: ParticleKind::default(),
            selectors: SelectorGrid::new(
                x + width,
                y,
                vec![ParticleKind::Sand, ParticleKind::Wall],
            ),
        }
    }

    fn spawn_particle(&mut self, pos: Vector2) {
        self.particles
            .push(Particle::new(self.selected_particle, pos));
    }

    fn evolve(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.evolve(dt);
        }
        self.particles.retain(|p| self.dim.in_boundary(p.pos));
    }

    fn in_boundary(&self, pos: Vector2) -> bool {
        self.dim.in_boundary(pos)
    }

    fn particle_selector_clicked(&mut self, pos: Vector2) {
        for selector in &self.selectors.selectors {
            if selector.in_boundary(pos) {
                self.selected_particle = selector.kind;
                return;
            }
        }
    }
}

enum GameState {
    Starting,
    Paused,
    Running,
}

struct Particle {
    kind: ParticleKind,
    pos: Vector2,
    width: i32,
    height: i32,
    vel: Vector2,
}

impl Particle {
    fn new(kind: ParticleKind, pos: Vector2) -> Self {
        let (width, height) = match kind {
            ParticleKind::Sand => (2, 2),
            ParticleKind::Wall => (4, 4),
        };
        Self {
            kind,
            pos,
            width,
            height,
            vel: match kind {
                ParticleKind::Sand => Vector2 { x: 0.0, y: 0.0 },
                ParticleKind::Wall => Vector2 { x: 0.0, y: 0.0 },
            },
        }
    }
    fn color(&self) -> Color {
        self.kind.color()
    }

    fn draw(&self, draw: &mut RaylibDrawHandle) {
        draw.draw_rectangle(
            self.pos.x as i32,
            self.pos.y as i32,
            self.width,
            self.height,
            self.color(),
        );
    }

    fn evolve(&mut self, dt: f32) {
        match self.kind {
            ParticleKind::Sand => {
                self.vel.y += (GRAV * dt).clamp(-100.0, 100.0);
                self.pos.y += self.vel.y * dt;
            }
            ParticleKind::Wall => {}
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum ParticleKind {
    Sand,
    Wall,
}

impl ParticleKind {
    fn color(&self) -> Color {
        match self {
            ParticleKind::Sand => Color::SANDYBROWN,
            ParticleKind::Wall => Color::GRAY,
        }
    }

    fn name(&self) -> &str {
        match self {
            ParticleKind::Sand => "Sand",
            ParticleKind::Wall => "Wall",
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

struct SelectorGrid {
    dim: Dim,
    selectors: Vec<ParticleSelector>,
}

impl SelectorGrid {
    fn new(x: i32, y: i32, kinds: Vec<ParticleKind>) -> Self {
        const X_PADDING: i32 = 10;
        const Y_PADDING: i32 = 15;

        let mut sg = Self {
            dim: Dim {
                x: x + X_PADDING,
                y,
                width: WINDOW_WIDTH - x - X_PADDING - WINDOW_MARGIN,
                height: WINDOW_HEIGHT - y - WINDOW_MARGIN,
            },
            selectors: Vec::with_capacity(kinds.len()),
        };

        {
            let mut x = sg.dim.x;
            let mut y = sg.dim.y;
            for kind in kinds {
                sg.selectors.push(ParticleSelector::new(x, y, kind));

                let new_y = y + sg.selectors[sg.selectors.len() - 1].dim.height + Y_PADDING;
                if new_y > sg.dim.height {
                    x += sg.selectors[sg.selectors.len() - 1].dim.width + X_PADDING;
                    y = sg.dim.y;
                    continue;
                }

                y = new_y;
            }
        }

        sg
    }

    fn iter(&self) -> Iter<'_, ParticleSelector> {
        self.selectors.iter()
    }
}

struct ParticleSelector {
    dim: Dim,
    kind: ParticleKind,
}

const FONT_SIZE: i32 = 60;
impl ParticleSelector {
    fn new(x: i32, y: i32, kind: ParticleKind) -> Self {
        const BOX_HEIGHT: i32 = FONT_SIZE + 20;
        const BOX_WIDTH: i32 = 240;
        Self {
            kind,
            dim: Dim {
                x,
                y,
                width: BOX_WIDTH,
                height: BOX_HEIGHT,
            },
        }
    }

    fn draw(&self, selected_particle: &ParticleKind, draw: &mut RaylibDrawHandle) {
        let text_width = draw.measure_text(self.kind.name(), FONT_SIZE);
        assert!(
            self.dim.width > text_width,
            "dim.width = {}, text_width = {}",
            self.dim.width,
            text_width
        );
        let width_diff = (self.dim.width - text_width) / 2;

        assert!(
            self.dim.height > FONT_SIZE,
            "dim.height = {}, FONT_SIZE = {}",
            self.dim.height,
            FONT_SIZE,
        );
        let height_diff = (self.dim.height - FONT_SIZE) / 2;

        if self.kind == *selected_particle {
            draw.draw_rectangle(
                self.dim.x,
                self.dim.y,
                self.dim.width,
                self.dim.height,
                self.kind.color(),
            );

            draw.draw_text(
                self.kind.name(),
                self.dim.x + width_diff,
                self.dim.y + height_diff,
                FONT_SIZE,
                BACKGROUND_COLOR,
            );

            return;
        }

        draw.draw_rectangle_lines(
            self.dim.x,
            self.dim.y,
            self.dim.width,
            self.dim.height,
            self.kind.color(),
        );

        draw.draw_text(
            self.kind.name(),
            self.dim.x + width_diff,
            self.dim.y + height_diff,
            FONT_SIZE,
            self.kind.color(),
        );
    }

    fn in_boundary(&self, pos: Vector2) -> bool {
        self.dim.in_boundary(pos)
    }
}

fn is_pause_key(key: &Option<KeyboardKey>) -> bool {
    match key {
        Some(KeyboardKey::KEY_P | KeyboardKey::KEY_SPACE) => true,
        _ => false,
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

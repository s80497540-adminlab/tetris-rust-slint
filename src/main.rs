slint::include_modules!();

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn to_slint_color(&self) -> slint::Color {
        slint::Color::from_rgb_u8(self.r, self.g, self.b)
    }
}

// Tetromino colors
const COLORS: [Color; 7] = [
    Color { r: 0, g: 255, b: 255 },   // I - Cyan
    Color { r: 255, g: 255, b: 0 },   // O - Yellow
    Color { r: 128, g: 0, b: 128 },   // T - Purple
    Color { r: 0, g: 255, b: 0 },     // S - Green
    Color { r: 255, g: 0, b: 0 },     // Z - Red
    Color { r: 0, g: 0, b: 255 },     // J - Blue
    Color { r: 255, g: 165, b: 0 },   // L - Orange
];

// Tetromino shapes (4x4 grid representation)
const SHAPES: [[[bool; 4]; 4]; 7] = [
    // I
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // O
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // T
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // S
    [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
    // Z
    [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // J
    [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // L
    [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
];

#[derive(Clone)]
struct Tetromino {
    shape: [[bool; 4]; 4],
    color: Color,
    x: i32,
    y: i32,
}

impl Tetromino {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let shape_index = rng.gen_range(0..7);
        
        Tetromino {
            shape: SHAPES[shape_index],
            color: COLORS[shape_index],
            x: (GRID_WIDTH as i32 / 2) - 2,
            y: 0,
        }
    }

    fn rotate(&mut self) {
        let mut new_shape = [[false; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_shape[i][j] = self.shape[3 - j][i];
            }
        }
        self.shape = new_shape;
    }
}

struct GameState {
    grid: [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT],
    current_piece: Option<Tetromino>,
    score: i32,
    game_over: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
            current_piece: None,
            score: 0,
            game_over: false,
        }
    }

    fn start(&mut self) {
        self.grid = [[None; GRID_WIDTH]; GRID_HEIGHT];
        self.current_piece = Some(Tetromino::new());
        self.score = 0;
        self.game_over = false;
    }

    fn can_move(&self, piece: &Tetromino, dx: i32, dy: i32) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if piece.shape[i][j] {
                    let new_x = piece.x + j as i32 + dx;
                    let new_y = piece.y + i as i32 + dy;

                    // Check boundaries
                    if new_x < 0 || new_x >= GRID_WIDTH as i32 || new_y >= GRID_HEIGHT as i32 {
                        return false;
                    }

                    // Check collision with existing pieces (only if not at top)
                    if new_y >= 0 && self.grid[new_y as usize][new_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn move_left(&mut self) {
        if let Some(piece) = &self.current_piece {
            let moved_piece = Tetromino {
                x: piece.x - 1,
                ..piece.clone()
            };
            if self.can_move(&moved_piece, 0, 0) {
                self.current_piece = Some(moved_piece);
            }
        }
    }

    fn move_right(&mut self) {
        if let Some(piece) = &self.current_piece {
            let moved_piece = Tetromino {
                x: piece.x + 1,
                ..piece.clone()
            };
            if self.can_move(&moved_piece, 0, 0) {
                self.current_piece = Some(moved_piece);
            }
        }
    }

    fn move_down(&mut self) -> bool {
        if let Some(piece) = &self.current_piece {
            let moved_piece = Tetromino {
                y: piece.y + 1,
                ..piece.clone()
            };
            if self.can_move(&moved_piece, 0, 0) {
                self.current_piece = Some(moved_piece);
                return true;
            } else {
                self.lock_piece();
                return false;
            }
        }
        false
    }

    fn rotate(&mut self) {
        if let Some(piece) = &self.current_piece {
            let mut rotated = piece.clone();
            rotated.rotate();
            if self.can_move(&rotated, 0, 0) {
                self.current_piece = Some(rotated);
            }
        }
    }

    fn drop(&mut self) {
        while self.move_down() {}
    }

    fn lock_piece(&mut self) {
        if let Some(piece) = &self.current_piece {
            for i in 0..4 {
                for j in 0..4 {
                    if piece.shape[i][j] {
                        let x = piece.x + j as i32;
                        let y = piece.y + i as i32;
                        
                        if y >= 0 && y < GRID_HEIGHT as i32 && x >= 0 && x < GRID_WIDTH as i32 {
                            self.grid[y as usize][x as usize] = Some(piece.color);
                        }
                    }
                }
            }
        }

        self.clear_lines();
        self.spawn_new_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        
        for y in (0..GRID_HEIGHT).rev() {
            if self.grid[y].iter().all(|cell| cell.is_some()) {
                // Remove this line
                for yy in (1..=y).rev() {
                    self.grid[yy] = self.grid[yy - 1];
                }
                self.grid[0] = [None; GRID_WIDTH];
                lines_cleared += 1;
            }
        }

        // Score: 100 per line, bonus for multiple lines
        if lines_cleared > 0 {
            self.score += lines_cleared * 100 * lines_cleared;
        }
    }

    fn spawn_new_piece(&mut self) {
        let new_piece = Tetromino::new();
        if self.can_move(&new_piece, 0, 0) {
            self.current_piece = Some(new_piece);
        } else {
            self.game_over = true;
        }
    }

    fn get_display_grid(&self) -> Vec<Vec<Cell>> {
        let mut display_grid = Vec::new();

        for y in 0..GRID_HEIGHT {
            let mut row = Vec::new();
            for x in 0..GRID_WIDTH {
                let mut cell = Cell {
                    color: slint::Color::from_rgb_u8(0, 0, 0),
                    filled: false,
                };

                // Draw locked pieces
                if let Some(color) = self.grid[y][x] {
                    cell = Cell {
                        color: color.to_slint_color(),
                        filled: true,
                    };
                }

                // Draw current piece
                if let Some(ref piece) = self.current_piece {
                    for i in 0..4 {
                        for j in 0..4 {
                            if piece.shape[i][j] {
                                let px = piece.x + j as i32;
                                let py = piece.y + i as i32;
                                
                                if py >= 0 && py == y as i32 && px == x as i32 {
                                    cell = Cell {
                                        color: piece.color.to_slint_color(),
                                        filled: true,
                                    };
                                }
                            }
                        }
                    }
                }

                row.push(cell);
            }
            display_grid.push(row);
        }

        display_grid
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    let game_state = Rc::new(RefCell::new(GameState::new()));

    // Update display
    let update_display = {
        let ui_weak = ui.as_weak();
        let game_state = game_state.clone();
        move || {
            let ui = ui_weak.unwrap();
            let state = game_state.borrow();
            
            let grid = state.get_display_grid();
            let grid_model: Vec<slint::ModelRc<Cell>> = grid
                .iter()
                .map(|row| {
                    slint::ModelRc::new(slint::VecModel::from(row.clone()))
                })
                .collect();
            
            ui.set_grid(slint::ModelRc::new(slint::VecModel::from(grid_model)));
            ui.set_score(state.score);
            ui.set_game_over(state.game_over);
        }
    };

    // Start game callback
    let start_game_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            game_state.borrow_mut().start();
            update_display();
        }
    };
    ui.on_start_game(start_game_callback);

    // Move left callback
    let move_left_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            if !game_state.borrow().game_over {
                game_state.borrow_mut().move_left();
                update_display();
            }
        }
    };
    ui.on_move_left(move_left_callback);

    // Move right callback
    let move_right_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            if !game_state.borrow().game_over {
                game_state.borrow_mut().move_right();
                update_display();
            }
        }
    };
    ui.on_move_right(move_right_callback);

    // Move down callback
    let move_down_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            if !game_state.borrow().game_over {
                game_state.borrow_mut().move_down();
                update_display();
            }
        }
    };
    ui.on_move_down(move_down_callback);

    // Rotate callback
    let rotate_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            if !game_state.borrow().game_over {
                game_state.borrow_mut().rotate();
                update_display();
            }
        }
    };
    ui.on_rotate(rotate_callback);

    // Drop callback
    let drop_callback = {
        let game_state = game_state.clone();
        let update_display = update_display.clone();
        move || {
            if !game_state.borrow().game_over {
                game_state.borrow_mut().drop();
                update_display();
            }
        }
    };
    ui.on_drop(drop_callback);

    // Game loop timer
    let timer = slint::Timer::default();
    let game_state_timer = game_state.clone();
    let update_display_timer = update_display.clone();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(500),
        move || {
            if !game_state_timer.borrow().game_over && game_state_timer.borrow().current_piece.is_some() {
                game_state_timer.borrow_mut().move_down();
                update_display_timer();
            }
        },
    );

    update_display();
    ui.run()
}
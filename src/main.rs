use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]
async fn main() {
    let row_count = 100;
    let col_count = 140;
    let mut game_board = setup_board(row_count, col_count);
    let mut is_paused = false;
    let mut is_erase_mode = false;
    let mut grid_state = 0;
    loop {
        clear_background(RED);
        draw_text(&format!("FPS: {0}", get_fps()), 40.0, 40.0, 55.0, YELLOW);

        if !is_paused {
            game_board = update_board(&mut game_board, row_count, col_count);
        }

        draw_board(&game_board, row_count, col_count);

        handle_key_inputs(&mut game_board, row_count, col_count);

         draw_clear_button(&mut game_board, row_count, col_count, 705.0, 60.0);
         draw_grid_button(&mut grid_state, 705.0, 100.0);
         start_pause_button(&mut is_paused, 705.0, 140.0);
         erase_button(&mut game_board, row_count.try_into().unwrap(), col_count.try_into().unwrap(), &mut is_erase_mode, 705.0, 180.0);
 
         draw_board_grid(CELLSIZE as f32, row_count.try_into().unwrap(), col_count.try_into().unwrap(), grid_state.try_into().unwrap());
         next_frame().await;
    }
}

fn setup_board(row_count: i32, col_count: i32) -> Vec<Particle> {
    let cell_count = row_count * col_count;
    let mut game_board: Vec<Particle> = vec![Particle(VOID, vec2(0.0, 0.0), false); cell_count as usize];
    for i in 0..row_count {
        for j in 0..col_count {
            game_board[(i * col_count + j) as usize] = Particle(VOID, vec2(0.0, 0.0), false);
        }
    }
    game_board
}

fn draw_board(game_board: &[Particle], row_count: i32, col_count: i32) {
    for i in 0..row_count {
        for j in 0..col_count {
            let cell: Particle = game_board[((i * col_count) + j) as usize];
            draw_rectangle(
                (j as u32 * CELLSIZE) as f32,
                (i as u32 * CELLSIZE) as f32 + 60.0,
                CELLSIZE as f32,
                CELLSIZE as f32,
                cell.0.color,
            )
        }
    }
}

fn update_board(game_board: &mut [Particle], row_count: i32, col_count: i32) -> Vec<Particle> {
    let frame_time = get_frame_time();
    for i in 0..row_count {
        for j in 0..col_count {
            let cellpos: usize = (i * col_count + j) as usize;
            game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;
            for _k in 0..game_board[cellpos].1.y as i32 {
                if (i + _k) < (row_count)
                    && game_board[cellpos].0.mass
                        > game_board[((i + _k) * col_count + j) as usize].0.mass
                    && game_board[cellpos].2
                {
                    game_board.swap(cellpos, (((i + _k) * col_count) + j) as usize);
                    game_board[(((i + _k) * col_count) + j) as usize].2 = false;
                } else if (i + _k) >= (row_count) {
                    game_board[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    continue;
                }
            }
            game_board[cellpos].2 = true;
        }
    }
    handle_mouse_input(game_board, row_count, col_count);
    game_board.to_vec()
}

fn handle_mouse_input(game_board: &mut [Particle], row_count: i32, col_count: i32) {
    let btn = MouseButton::Left;
    let rbtn = MouseButton::Right;
    let mbtn = MouseButton::Middle;
    if is_mouse_button_down(btn) || is_mouse_button_down(rbtn) || is_mouse_button_down(mbtn)
        || is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::D) {
        let cursor_position = mouse_position();
        if cursor_position.0 > CELLSIZE as f32
            && cursor_position.0 < (CELLSIZE * col_count as u32) as f32
            && cursor_position.1 > CELLSIZE as f32
            && cursor_position.1 < (CELLSIZE * row_count as u32) as f32
        {
            let x = (cursor_position.0 as u32 / CELLSIZE) - 1;
            let y = ((cursor_position.1 - 55.0) as u32 / CELLSIZE) - 1;
            let material = if is_mouse_button_down(btn) {
                WATER
            } else if is_mouse_button_down(rbtn) {
                SAND 
            } else {
                SOLID
            };
            game_board[(y * col_count as u32 + x) as usize] =
                Particle(material, vec2(0.0, 1.0), false);
        }
    }
}

fn handle_key_inputs(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) {
    if is_key_pressed(KeyCode::R) {
        *game_board = setup_board(row_count, col_count);
    }

    let cursor_position = mouse_position();
    
    if cursor_position.0 > CELLSIZE as f32
        && cursor_position.0 < (CELLSIZE * col_count as u32) as f32
        && cursor_position.1 > CELLSIZE as f32
        && cursor_position.1 < (CELLSIZE * row_count as u32) as f32
    {
        let x = ((cursor_position.0 - CELLSIZE as f32) / CELLSIZE as f32) as u32;
        let y = (((cursor_position.1 - 60.0) / CELLSIZE as f32) as u32).min(row_count as u32 - 1);
        
        let x = x.min(col_count as u32 - 1);
        
        let material = if is_key_down(KeyCode::A) {
            POWDER
        } else if is_key_down(KeyCode::S) {
            LIQUID
        } else if is_key_down(KeyCode::W) {
            GAS
        } else if is_key_down(KeyCode::D) {
            PLASMA
        } else {
            return;
        };
        
        let index = (y * col_count as u32 + x) as usize;
        if index < game_board.len() {
            game_board[index] = Particle(material, vec2(0.0, 1.0), false);
        }
    }
}

#[derive(Copy, Clone)]
enum Phase {
    Void,
    Solid { hardness: u8 },
    Powder { coarseness: f32 },
    Liquid { viscosity: f32 },
    Gas { viscosity: f32 },
    Plasma { viscosity: f32 },
}

fn solve_particle(
    game_board: &mut [Particle],
    phase: Phase,
    row_count: i32,
    col_count: i32,
    i: i32,
    j: i32,
) {
    let frame_time = get_frame_time();
    match phase {
        Phase::Void => {}
        Phase::Solid { hardness: _u8 } => {}
        Phase::Powder { coarseness: _f32 } => {
            let cellpos: usize = (i * col_count + j) as usize;
            game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;
            for _k in 0..game_board[cellpos].1.y as i32 {
                if (i + _k) < (row_count)
                    && game_board[cellpos].0.mass
                        > game_board[((i + _k) * col_count + j) as usize].0.mass
                    && game_board[cellpos].2
                {
                    game_board.swap(cellpos, (((i + _k) * col_count) + j) as usize);
                    game_board[(((i + _k) * col_count) + j) as usize].2 = false;
                } else if (i + _k) >= (row_count) {
                    game_board[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    continue;
                }
            }
            game_board[cellpos].2 = true;
        }
        Phase::Liquid { viscosity: _f32 } => {
            let cellpos: usize = (i * col_count + j) as usize;
            game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;
            for _k in 0..game_board[cellpos].1.y as i32 {
                if (i + _k) < (row_count)
                    && game_board[cellpos].0.mass
                        > game_board[((i + _k) * col_count + j) as usize].0.mass
                    && game_board[cellpos].2
                {
                    game_board.swap(cellpos, (((i + _k) * col_count) + j) as usize);
                    game_board[(((i + _k) * col_count) + j) as usize].2 = false;
                } else if (i + _k) >= (row_count) {
                    game_board[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    continue;
                }
            }
            game_board[cellpos].2 = true;
        }
        Phase::Gas { viscosity: _f32 } => {}
        Phase::Plasma { viscosity: _f32 } => {}
    }
}

const CELLSIZE: u32 = 5;
const GRAVITY: f32 = 9.81;

static VOID: Material = Material {
    mass: 0.0,
    coarseness: 0.0,
    hardness: 5,
    viscosity: 0.0,
    phase: Phase::Void,
    flammability: 0.0,
    color: color_u8!(0, 0, 0, 100),
};

static WATER: Material = Material {
    mass: 1.0,
    coarseness: 0.0,
    hardness: 5,
    viscosity: 1.0,
    phase: Phase::Liquid { viscosity: 1.0 },
    flammability: 1.0,
    color: BLUE,
};

static SAND: Material = Material {
    mass: 1.682,
    hardness: 5,
    viscosity: 0.0,
    coarseness: 1.0,
    phase: Phase::Powder { coarseness: 1.0 },
    flammability: 0.0,
    color: color_u8!(203, 189, 147, 255),
};

static SOLID: Material = Material {
    mass: 1.682,
    coarseness: 0.0,
    viscosity: 0.0,
    hardness: 5,
    phase: Phase::Solid { hardness: 5 },
    flammability: 0.0,
    color: BLACK,
};

static POWDER: Material = Material {
    mass: 1.682,
    hardness: 5,
    viscosity: 0.0,
    coarseness: 1.0,
    phase: Phase::Powder { coarseness: 1.0 },
    flammability: 0.0,
    color: RED,
};

static LIQUID: Material = Material {
    mass: 1.682,
    coarseness: 0.0,
    hardness: 5,
    viscosity: 1.0,
    phase: Phase::Liquid { viscosity: 1.0 },
    flammability: 0.0,
    color: BLUE,
};

static GAS: Material = Material {
    mass: 1.682,
    coarseness: 0.0,
    hardness: 5,
    viscosity: 1.0,
    phase: Phase::Gas { viscosity: 1.0 },
    flammability: 0.0,
    color: GRAY,
};

static PLASMA: Material = Material {
    mass: 1.682,
    coarseness: 0.0,
    hardness: 5,
    viscosity: 1.0,
    phase: Phase::Plasma { viscosity: 1.0 },
    flammability: 0.0,
    color: PINK,
};

#[derive(Copy, Clone)]
struct Material {
    mass: f32,        
    coarseness: f32,
    hardness: u8,
    viscosity: f32,
    phase: Phase,      
    flammability: f32, 
    color: Color,      
}

#[derive(Copy, Clone)]
struct Particle(Material, Vec2, bool);

fn clear_board(row_count: i32, col_count: i32) -> Vec<Particle> {
    let mut game_board: Vec<Particle> = Vec::new();
    for _ in 0..row_count {
        for _ in 0..col_count {
            game_board.push(Particle(VOID, vec2(0.0, 0.0), false));
        }
    }
    game_board
}

fn draw_clear_button(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32, x: f32, y: f32) {
    let (btn_width, btn_height): (f32, f32) = (100.0, 30.0);
    let mouse_pos: (f32, f32) = mouse_position();
    let mouse_pressed: bool = is_mouse_button_pressed(MouseButton::Left);

    if mouse_pos.0 > x
        && mouse_pos.0 < x + btn_width
        && mouse_pos.1 > y
        && mouse_pos.1 < y + btn_height
        && mouse_pressed
    {
        *game_board = clear_board(row_count, col_count);
    }

    draw_rectangle(x, y, btn_width, btn_height, DARKGRAY);
    draw_text("Clear", x + 10.0, y + 20.0, 20.0, WHITE);
}

fn draw_grid_button(grid_state: &mut usize, x: f32, y: f32) {
    let button_width: f32 = 100.0;
    let button_height: f32 = 30.0;
    let button_color: Color = if *grid_state > 0 { BLUE } else { DARKGRAY };
    let label: &str = match *grid_state {
        0 => "Show Grid",
        1..=6 => "More Grid",
        7 => "Hide Grid",
        _ => "Unknown State",
    };

    draw_rectangle(x, y, button_width, button_height, button_color);
    draw_text(
        label,
        x + 10.0,
        y + 20.0,
        20.0,
        WHITE,
    );

    if is_mouse_button_pressed(MouseButton::Left) && is_mouse_over_button(x, y, button_width, button_height) {
        *grid_state = (*grid_state + 1) % 8;
    }
}

const CELL_AREA_WIDTH: f32 = 100.0 * CELLSIZE as f32;
const CELL_AREA_HEIGHT: f32 = 125.0 * CELLSIZE as f32;
const GRID_SPACING_PX: f32 = 39.0;

fn draw_board_grid(cell_size: f32, row_count: usize, col_count: usize, grid_state: usize) {
    if grid_state == 0 {
        return;
    }

    let line_thickness: f32 = 1.0;
    let line_color: Color = BLACK;

    let grid_width = cell_size * col_count as f32;
    let grid_height = cell_size * row_count as f32;

    let (spacing_x, spacing_y) = match grid_state {
        1 => (grid_width, grid_height),
        2 | 3 | 4 | 5 | 6 => {
            let count: i32 = match grid_state {
                2 => 4,
                3 => 6,
                4 => 8,
                5 => 10,
                6 => 12,
                _ => 0,
            };
            (grid_width / (count + 1) as f32, grid_height / (count + 1) as f32)
        },
        7 => (GRID_SPACING_PX, GRID_SPACING_PX),
        _ => (0.0, 0.0),
    };

    if grid_state == 1 {
        draw_line(0.0, grid_height / 2.0 + 60.0, grid_width, grid_height / 2.0 + 60.0, line_thickness, line_color);
        draw_line(grid_width / 2.0, 60.0, grid_width / 2.0, grid_height + 60.0, line_thickness, line_color);
    } else {
        let mut offset = spacing_y;
        while offset <= grid_height {
            draw_line(0.0, offset + 60.0, grid_width, offset + 60.0, line_thickness, line_color);
            offset += spacing_y;
        }

        offset = spacing_x;
        while offset <= grid_width {
            draw_line(offset, 60.0, offset, grid_height + 60.0, line_thickness, line_color);
            offset += spacing_x;
        }

        if grid_state == 7 {
            draw_line(0.0, 60.0, grid_width, 60.0, line_thickness, line_color);
            draw_line(0.0, grid_height + 60.0, grid_width, grid_height + 60.0, line_thickness, line_color);
            draw_line(0.0, 60.0, 0.0, grid_height + 60.0, line_thickness, line_color);
            draw_line(grid_width, 60.0, grid_width, grid_height + 60.0, line_thickness, line_color);
        }
    }
}

fn is_mouse_over_button(x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mouse_x, mouse_y): (f32, f32) = mouse_position();
    mouse_x > x && mouse_x < x + width && mouse_y > y && mouse_y < y + height
}

pub fn start_pause_button(is_paused: &mut bool, x: f32, y: f32) {
    let button_width: f32 = 100.0;
    let button_height: f32 = 30.0;
    let button_color: Color = if *is_paused { RED } else { GREEN };
    let label: &str = if *is_paused { "Start" } else { "Pause" };


    draw_rectangle(x, y, button_width, button_height, button_color);
    draw_text(label, x + 10.0, y + 20.0, 20.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) && is_mouse_over_button(x, y, button_width, button_height) {
        *is_paused = !*is_paused;
    }
}

fn erase_button(
    board: &mut Vec<Particle>, 
    row_count: usize, 
    col_count: usize, 
    is_erase_mode: &mut bool, 
    button_x: f32, 
    button_y: f32
) {
    let button_width: f32 = 100.0;
    let button_height: f32 = 30.0;
    
    let button_color: Color = if *is_erase_mode { RED } else { DARKGRAY };

    draw_rectangle(button_x, button_y, button_width, button_height, button_color);
    draw_text(
        "Erase", 
        button_x + 10.0, 
        button_y + 20.0, 
        20.0, 
        WHITE
    );

    if is_mouse_button_pressed(MouseButton::Left) && is_mouse_over_button(button_x, button_y, button_width, button_height) {
        *is_erase_mode = !*is_erase_mode;
    }
    if *is_erase_mode {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y): (f32, f32) = mouse_position();
            let col: usize = (mouse_x / CELLSIZE as f32) as usize;
            let row: usize = ((mouse_y - 60.0) / CELLSIZE as f32) as usize;
            if row < row_count && col < col_count {
                let index: usize = row * col_count + col;
                board[index] = Particle(VOID, vec2(0.0, 0.0), false);
            }
        }
    }
}
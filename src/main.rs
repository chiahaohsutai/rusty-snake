use macroquad::prelude::rand::gen_range;
use macroquad::prelude::*;
use std::collections::{HashSet, LinkedList};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn half(val: f32) -> f32 {
    val / 2.0
}

fn up() -> bool {
    is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)
}

fn down() -> bool {
    is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)
}

fn left() -> bool {
    is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)
}

fn right() -> bool {
    is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)
}

async fn render_start_menu(texts: String, size: u16) {
    let color = BLACK;
    let text1 = texts.as_str();
    let text2 = "Press [Enter] to play.";

    loop {
        clear_background(WHITE);
        let dims1 = measure_text(text1, None, size, 1.0);
        let dims2 = measure_text(text2, None, size, 1.0);
        let centerx = half(screen_width());
        let centery = half(screen_height());

        draw_text(
            text1,
            centerx - half(dims1.width),
            centery - dims1.height,
            size as f32,
            color,
        );
        draw_text(
            text2,
            centerx - half(dims2.width),
            centery + dims2.height,
            size as f32,
            color,
        );
        if is_key_down(KeyCode::Enter) {
            break;
        }
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await
    }
}

fn render_background(score: u32, size: u8, spacing: u8) -> Vec<Vec<(f32, f32)>> {
    let font_size = 30;

    let text = format!("Score: {score}");
    let text_dim = measure_text(text.clone().as_str(), None, font_size, 1.0);
    draw_text(
        text.as_str(),
        5.0,
        text_dim.height + 10.0,
        font_size as f32,
        GREEN,
    );
    let offset = text_dim.height + 25.0;

    let block = (size + spacing) as f32;
    let (w, h) = (screen_width(), screen_height() - offset);
    let width_rem = w % block;

    let (x1, y1) = (half(width_rem), offset);
    let (x2, y2) = (x1 + (w - width_rem), h);

    let width = x2 - x1;
    let height = y2 - y1;

    let mut start_x = x1;
    let mut start_y = y1;
    let mut grid_coords = Vec::new();

    for _ in 0..((height / block) as u32) {
        let mut row_coords = Vec::new();

        for _ in 0..((width / block) as u32) {
            draw_rectangle(start_x, start_y, size as f32, size as f32, WHITE);
            row_coords.push((start_x, start_y));
            start_x += block as f32
        }
        grid_coords.push(row_coords);
        start_x = x1;
        start_y += block as f32;
    }
    grid_coords
}

fn render_snake(
    head: &(usize, usize),
    snake: &LinkedList<(usize, usize)>,
    size: u8,
    coords: &Vec<Vec<(f32, f32)>>,
) {
    let size = size as f32;
    let head = coords[head.0][head.1];
    draw_rectangle(head.0, head.1, size, size, LIME);

    for (idx1, idx2) in snake {
        let (x, y) = coords[*idx1][*idx2];
        draw_rectangle(x, y, size, size, LIME);
    }
}

fn render_fruit(fruit: &(usize, usize), size: u8, coords: &Vec<Vec<(f32, f32)>>) {
    let (x, y) = coords[fruit.0][fruit.1];
    let size = size as f32;
    draw_rectangle(x, y, size, size, GOLD)
}

fn random_location(range: (usize, usize), exlude: &HashSet<(usize, usize)>) -> (usize, usize) {
    loop {
        let location = (gen_range(0, range.0), gen_range(0, range.1));
        if !exlude.contains(&location) {
            break location;
        }
    }
}

fn update_dir(curr_dir: Direction) -> Option<Direction> {
    if up() && curr_dir != Direction::DOWN {
        Some(Direction::UP)
    } else if down() && curr_dir != Direction::UP {
        Some(Direction::DOWN)
    } else if left() && curr_dir != Direction::RIGHT {
        Some(Direction::LEFT)
    } else if right() && curr_dir != Direction::LEFT {
        Some(Direction::RIGHT)
    } else {
        None
    }
}

fn update(head: &(usize, usize), dir: &Direction, coords: &Vec<Vec<(f32, f32)>>) -> (usize, usize) {
    let dim = (coords.len() as f32, coords[0].len() as f32);
    let head = (head.0 as f32, head.1 as f32);
    let idx = match dir {
        Direction::UP => ((head.0 - 1.0).rem_euclid(dim.0), head.1),
        Direction::DOWN => ((head.0 + 1.0).rem_euclid(dim.0), head.1),
        Direction::LEFT => (head.0, (head.1 - 1.0).rem_euclid(dim.1)),
        Direction::RIGHT => (head.0, (head.1 + 1.0).rem_euclid(dim.1)),
    };
    (idx.0 as usize, idx.1 as usize)
}

async fn play(size: u8, spacing: u8) -> u32 {
    let mut score = 0;
    let mut game_over = false;

    let mut coords = render_background(score, size, spacing);

    let top_speed = 0.1;
    let mut speed = 0.3;
    let mut last_update = get_time();

    let mut snake_body = LinkedList::new();
    let mut snake_dir = Direction::UP;

    let mut snake_head = (
        half(coords.len() as f32) as usize,
        half(coords[0].len() as f32) as usize,
    );

    let mut locked = false;
    let mut snake_positions = HashSet::new();

    for index in snake_body.iter() {
        snake_positions.insert(*index);
    }
    snake_positions.insert(snake_head);

    let mut fruit = random_location((coords.len(), coords[0].len()), &snake_positions);

    return loop {
        if let Some(dir) = update_dir(snake_dir.clone()) {
            snake_dir = dir;
            locked = !locked;
        }

        if get_time() - last_update > speed {
            last_update = get_time();

            snake_body.push_front(snake_head);
            snake_head = update(&snake_head, &snake_dir, &coords);

            if snake_head == fruit {
                snake_positions.insert(snake_head);
                fruit = random_location((coords.len(), coords[0].len()), &snake_positions);
                score = score + gen_range(5, 10) as u32;

                if speed > top_speed {
                    speed *= 0.9;
                }
            } else {
                snake_positions.insert(snake_head);
                snake_positions.remove(&snake_body.pop_back().unwrap());
            }
            for index in &snake_body {
                if *index == snake_head {
                    game_over = true;
                }
            }
        }

        coords = render_background(score.clone(), size, spacing);
        render_snake(&snake_head, &snake_body, size, &coords);
        render_fruit(&fruit, size, &coords);

        if snake_positions.len() == coords.len() * coords[0].len()
            || game_over
            || is_key_down(KeyCode::Q)
        {
            break score;
        }
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await;
    };
}

#[macroquad::main("Rusty Snake")]
async fn main() {
    let font_size = 30;
    let spacing = 1;
    let size = 20;
    render_start_menu("Welcome to Rusty Snake!".to_string(), font_size).await;

    loop {
        let score = play(size, spacing).await;
        render_start_menu(format!("Your score was {score}"), font_size).await;
    }
}

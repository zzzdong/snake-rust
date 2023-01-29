use std::collections::VecDeque;

use rand::Rng;
use tiny_skia::Color;
use winit::event::VirtualKeyCode;

use crate::renderer::Renderer;

pub struct Game {
    snake: Snake,
    food: Food,
    rect: Size,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Game {
        let head = Point::new(w / 2, h / 2);

        Game {
            snake: Snake::new(head),
            food: Food::new(head),
            rect: Size::new(w, h),
        }
    }

    pub fn init(&mut self) {
        self.place_food()
    }

    fn place_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(0..self.rect.w);
            let y = rand::thread_rng().gen_range(0..self.rect.h);

            let p = Point::new(x, y);

            if !self.snake.hit_test(&p) {
                let food = Food::new(p);
                self.food = food;
                return;
            }
        }
    }

    pub fn render(&self, renderer: &mut impl Renderer) {
        self.snake.render(renderer);
        self.food.render(renderer);
    }

    pub fn tick(&mut self) {
        let next_head = self.snake.next_head();

        if next_head.x < 0 || next_head.x >= self.rect.w || next_head.y < 0 || next_head.y >= self.rect.h {
            return
        }

        if next_head == self.food.pos {
            self.snake.grow(next_head);
            self.place_food();
        } else {
            self.snake.step(next_head);
        }
    }

    pub fn on_key(&mut self, key: VirtualKeyCode) {
        self.snake.on_key(key);
    }
}

#[derive(Debug, Clone)]
struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    fn new(head: Point) -> Snake {
        let mut body = VecDeque::new();

        body.push_back(head);

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn step(&mut self, next_head: Point) {
        self.body.push_front(next_head);
        self.body.pop_back();
    }

    fn grow(&mut self, next_head: Point) {
        self.body.push_front(next_head);
    }

    fn next_head(&self) -> Point {
        let head = self.body.front().unwrap();

        let new_head;

        match self.direction {
            Direction::Up => {
                new_head = Point::new(head.x, head.y-1);
            }
            Direction::Down => {
                new_head = Point::new(head.x, head.y+1);
            }
            Direction::Left => {
                new_head = Point::new(head.x-1, head.y);
            }
            Direction::Right => {
                new_head = Point::new(head.x+1, head.y);
            }
        }

        new_head
    }

    fn hit_test(&self, p: &Point) -> bool {
        for b in &self.body {
            if b.is_same(&p) {
                return true;
            }
        }

        return false;
    }

    fn render(&self, renderer: &mut impl Renderer) {
        let color = Color::from_rgba8(32, 200, 32, 255);

        renderer.draw_points(self.body.iter(), color)
    }

    fn on_key(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::S => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
            VirtualKeyCode::Left | VirtualKeyCode::A => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            VirtualKeyCode::Right | VirtualKeyCode::D => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0x00,
    Down = 0x01,
    Left = 0x10,
    Right = 011,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn is_same(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Food {
    pos: Point,
}

impl Food {
    fn new(pos: Point) -> Food {
        Food { pos }
    }

    fn render(&self, renderer: &mut impl Renderer) {
        let color = Color::from_rgba8(200, 32, 32, 255);
        renderer.draw_points([self.pos].iter(), color)
    }
}

#[derive(Debug, Clone, Copy)]
struct Size {
    w: i32,
    h: i32,
}
impl Size {
    fn new(w: i32, h: i32) -> Size {
        Size { w, h }
    }
}

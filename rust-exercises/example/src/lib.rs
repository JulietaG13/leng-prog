use std::f64::consts::PI;
use std::f64;
use std::mem::replace;

const G: f64 = 9.807;

// Calculate the range (max x) and height (max y) of an oblique shot
// Provide the angle (in degrees) and initial velocity

pub fn oblique_shot(angle: f64, velocity: f64) -> (f64, f64) {
    let radians = angle * PI / 180.0;
    let vx = velocity * radians.cos();
    let vy = velocity * radians.sin();
    let t = 2.0 * vy / G;
    let range = vx * t;
    let time = vy * vy / (2.0 * G);
    (range, time)
}

pub fn max(a: &i64, b: &i64) -> i64 {
    if a > b {
        return *a;
    }
    *b
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Canvas {
    pub width: f64,
    pub height: f64,
    pub shapes: Vec<(Point, Box<dyn Shape>)>,
}

impl Canvas {
    pub fn new(width: f64, height: f64) -> Canvas {
        Canvas {
            width,
            height,
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, place: Point, shape: Box<dyn Shape>) {
        if self.check_in(&place, &(*shape)) {
            self.shapes.push((place, shape));
        }
    }

    pub fn sum_areas(&self) -> f64 {
        self.shapes.iter()
            .map(|(_,s)| (*s).area())
            .sum()
    }

    pub fn shift_all(&mut self, shift_x: f64, shift_y: f64) -> bool {
        let all_in = self.shapes.iter().all(|(p,s)| {
            let new_p = Point { x: p.x + shift_x, y: p.y + shift_y };
            self.check_in(&new_p, &*(*s))
        });
        if !all_in {
            return false;
        }
        self.shapes.iter_mut().for_each(|(p,_)| *p = Point { x: p.x + shift_x, y: p.y + shift_y });
        true
    }

    pub fn check_in(&self, place: &Point, shape: &dyn Shape) -> bool {
        let (bot_left, top_right) = shape.container_box(place);
        if bot_left.x < 0.0 || bot_left.y < 0.0 || top_right.x > self.width || top_right.y > self.height {
            return false;
        }
        true
    }
}

pub trait Shape {
    fn area(&self) -> f64;
    fn container_box(&self, center: &Point) -> (Point, Point);
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    fn container_box(&self, center: &Point) -> (Point, Point) {
        let p1 = Point { x: center.x - self.radius, y: center.y - self.radius};
        let p2 = Point { x: center.x + self.radius, y: center.y + self.radius};
        (p1, p2)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn container_box(&self, center: &Point) -> (Point, Point) {
        let p1 = Point { x: center.x - self.width / 2.0, y: center.y - self.height / 2.0 };
        let p2 = Point { x: center.x + self.width / 2.0, y: center.y + self.height / 2.0 };
        (p1, p2)
    }
}

pub struct Node<T> {
    pub value: T,
    pub next: NodeRef<T>
}

type NodeRef<T> = Option<Box<Node<T>>>;

pub struct MutStack<T> {
    pub head: NodeRef<T>,
}

impl<T> MutStack<T> {
    pub fn new() -> Self {
        MutStack { head: None }
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        let dest = &mut self.head;
        let node = Node {
            value,
            next: replace(dest, None),
        };
        self.head = Some(Box::new(node));
        self
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.value),
            None => None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next;
        Some(old_head.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub struct Person {
    pub name: String,
    pub age: i32,
}

pub trait SaysHi {
    fn says_hi(&self) -> String;
}

impl SaysHi for Person {
    fn says_hi(&self) -> String{
        format!("Hi, I am {}", self.name)
    }
}

pub fn vec_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

pub fn max_min_int(vec: &[i32]) -> Option<(i32, i32)> {
    let min = vec.iter().reduce(|acc, n| if acc < n { acc } else { n });
    let max = vec.iter().reduce(|acc, n| if acc > n { acc } else { n });

    if min.is_none() || max.is_none() {
        return None;
    }
    Some((*min.unwrap(), *max.unwrap()))
}

pub fn filter_a(s: Vec<String>) -> Vec<String> {
    s.into_iter().filter(|s| s.contains("a")).collect()
}

pub fn double(mut s: Vec<i32>) -> Vec<i32> {
    s.iter_mut().map(|x| *x * 2).collect()
}

pub enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>),
    Nil
}

pub fn cons<T>(value: T, tail: LinkedList<T>) -> LinkedList<T> {
    LinkedList::Cons(value, Box::new(tail))
}

pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    s2
}
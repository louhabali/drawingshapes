use rand::Rng;
use raster::Color;

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}

// Define the traits

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);
    fn color(&self) -> Color {
        random_color()
    }
}

// Point structure
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
            color: random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            color: random_color(),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color.clone());
    }
}

// Line structure
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: Point::new(start.x, start.y),
            end: Point::new(end.x, end.y),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Line::new(&p1, &p2)
    }

    pub fn draw_with_color(&self, img: &mut impl Displayable, color: Color) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let mut steps = dx.abs();
        if dy.abs() > steps {
            steps = dy.abs();
        }
        if steps == 0 {
            img.display(self.start.x, self.start.y, color);
            return;
        }
        let mut x = self.start.x as f64;
        let mut y = self.start.y as f64;
        let xinc = (dx as f64) / (steps as f64);
        let yinc = (dy as f64) / (steps as f64);
        let mut i = 0;
        while i <= steps {
            img.display(x as i32, y as i32, color.clone());
            x += xinc;
            y += yinc;
            i += 1;
        }
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut impl Displayable) {
        self.draw_with_color(img, self.color());
    }
}

// Triangle structure
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            c: Point::new(c.x, c.y),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut impl Displayable) {
        let color = self.color();
        Line::new(&self.a, &self.b).draw_with_color(image, color.clone());
        Line::new(&self.b, &self.c).draw_with_color(image, color.clone());
        Line::new(&self.c, &self.a).draw_with_color(image, color.clone());
    }
}

// Rectangle structure
pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        // Ensure the points are arranged as a = top-left and b = bottom-right
        let a = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let b = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));
        Rectangle { a, b }
    }
    pub fn draw_with_color(&self, image: &mut impl Displayable, color: Color) {
        let top_right = Point::new(self.b.x, self.a.y);
        let bottom_left = Point::new(self.a.x, self.b.y);
        Line::new(&self.a, &top_right).draw_with_color(image, color.clone());
        Line::new(&top_right, &self.b).draw_with_color(image, color.clone());
        Line::new(&self.b, &bottom_left).draw_with_color(image, color.clone());
        Line::new(&bottom_left, &self.a).draw_with_color(image, color.clone());
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut impl Displayable) {
        self.draw_with_color(image, self.color())
    }
}

// Circle structure
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let radius = rng.gen_range(0..=height);
        Circle::new(&center, radius)
    }
    pub fn new(p: &Point, r: i32) -> Self {
        Self {
            center: p.clone(),
            radius: r,
        }
    }
}

// impl Drawable for Circle {
//     fn draw(&self, img: &mut impl Displayable) {
//         let mut deg = 0.0;
//         let pere = ((self.radius as f64) * 4.0 * std::f64::consts::PI).max(36.);
//         let color = self.color();
//         while deg <= 360.0 {
//             println!("k");
//             let rad = ((deg as f64) * std::f64::consts::PI) / 180.0;
//             let x = ((self.center.x as f64) + (self.radius as f64) * rad.cos()).round() as i32;
//             let y = ((self.center.y as f64) + (self.radius as f64) * rad.sin()).round() as i32;
//             img.display( x,  y, color.clone());
//             deg += 360.0 / pere;
//         }
//     }
// }

// deffrent method
impl Drawable for Circle {
    fn draw(&self, img: &mut impl Displayable) {
        let mut x: i32 = 0;
        let mut y: i32 = -self.radius;
        let r = self.radius;
        let mut p = x.pow(2) as f32 + (y as f32 + 0.5).powf(2.) - r.pow(2) as f32;
        let cx = self.center.x;
        let cy = self.center.x;

        let color = self.color();
        
        if self.radius == 0 {
            img.display(cx + y, cy - x, color.clone());
        }
        while x < -y {
            if p > 0. {
                y += 1
            }
            p = x.pow(2) as f32 + (y as f32 + 0.5).powf(2.) - r.pow(2) as f32;

            img.display(cx + x, cy + y, color.clone());
            img.display(cx - x, cy + y, color.clone());
            img.display(cx - x, cy - y, color.clone());
            img.display(cx + x, cy - y, color.clone());
            img.display(cx + y, cy + x, color.clone());
            img.display(cx - y, cy + x, color.clone());
            img.display(cx - y, cy - x, color.clone());
            img.display(cx + y, cy - x, color.clone());

            x += 1
        }
    }
}

pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(p1: &Point, radius: i32) -> Self {
        Self {
            center: p1.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            &Point::random(width, height),
            rng.gen_range(0..width.min(height)),
        )
    }
}
impl Drawable for Pentagon {
    fn draw(&self, image: &mut impl Displayable) {
        let mut last_x = self.radius + self.center.x;
        let mut last_y = self.center.y;
        let color = self.color();
        for i in 1..=5 {
            let last_point = Point::new(last_x, last_y);
            let angle = (((i as f64) * (360.0 / 5.0) * std::f64::consts::PI) / 180.0) as f32;
            let x = ((self.radius as f32) * angle.cos() + (self.center.x as f32)).floor() as i32;
            let y = ((self.radius as f32) * angle.sin() + (self.center.y as f32)).floor() as i32;
            last_x = x;
            last_y = y;
            let line = Line::new(&last_point, &Point::new(x, y));
            line.draw_with_color(image, color.clone());
        }
    }
}

pub struct Cube {
    point1: Point,
    side: i32,
}

impl Cube {
    pub fn new(p1: &Point, side: i32) -> Self {
        Self {
            point1: p1.clone(),
            side,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut impl Displayable) {
        let p1 = &self.point1;
        let p2: Point = Point::new(self.point1.x + self.side, self.point1.y);
        let p3: Point = Point::new(self.point1.x + self.side, self.point1.y + self.side);
        let p4: Point = Point::new(self.point1.x, self.point1.y + self.side);
        let d = (p3.x - p1.x).abs() / 3;
        let p1_b = Point::new(p1.x + d, p1.y - d);
        let p2_b: Point = Point::new(p2.x + d, p2.y - d);
        let p3_b: Point = Point::new(p3.x + d, p3.y - d);
        let p4_b: Point = Point::new(p4.x + d, p4.y - d);

        let color = self.color();
        Rectangle::new(p1, &p3).draw_with_color(image, color.clone());
        Rectangle::new(&p1_b, &p3_b).draw_with_color(image, color.clone());
        Line::new(&p1, &p1_b).draw_with_color(image, color.clone());
        Line::new(&p2, &p2_b).draw_with_color(image, color.clone());
        Line::new(&p3, &p3_b).draw_with_color(image, color.clone());
        Line::new(&p4, &p4_b).draw_with_color(image, color.clone());
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl From<[f32; 2]> for Vec2 {
    fn from(arr: [f32; 2]) -> Self {
        Vec2 {
            x: arr[0],
            y: arr[1],
        }
    }
}

use std::ops::Mul;

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

trait AsciiVec2Ext {
    fn to_string(&self) -> String;
}

impl AsciiVec2Ext for Vec2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x.round() as i32, self.y.round() as i32)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl AABB {
    fn include_point(&mut self, point: Vec2) {
        if point.x < self.x_min {
            self.x_min = point.x;
        }
        if point.x > self.x_max {
            self.x_max = point.x;
        }
        if point.y < self.y_min {
            self.y_min = point.y;
        }
        if point.y > self.y_max {
            self.y_max = point.y;
        }
    }
}

struct AsciiCanvas {
    buffer: HashMap<(i32, i32), char>,
    bounds: AABB,
}

impl AsciiCanvas {
    fn new() -> Self {
        AsciiCanvas {
            buffer: HashMap::new(),
            bounds: AABB::default(),
        }
    }

    fn rect(&mut self, center: Vec2, size: Vec2) -> &mut Self {
        let half_width = (size.x / 2.0).ceil() as i32;
        let half_height = (size.y / 2.0).ceil() as i32;
        let center_x = center.x.round() as i32;
        let center_y = center.y.round() as i32;

        for x in -half_width..=half_width {
            let px = center_x + x;
            let py_top = center_y - half_height;
            let py_bottom = center_y + half_height;
            self.buffer.insert((px, py_top), '-');
            self.buffer.insert((px, py_bottom), '-');
            self.bounds.include_point([px as f32, py_top as f32].into());
            self.bounds
                .include_point([px as f32, py_bottom as f32].into());
        }

        for y in -half_height..=half_height {
            let py = center_y + y;
            let px_left = center_x - half_width;
            let px_right = center_x + half_width;
            self.buffer.insert((px_left, py), '|');
            self.buffer.insert((px_right, py), '|');
            self.bounds
                .include_point([px_left as f32, py as f32].into());
            self.bounds
                .include_point([px_right as f32, py as f32].into());
        }

        self
    }

    fn text(&mut self, position: Vec2, text: &str) -> &mut Self {
        let start_x = position.x.round() as i32 - (text.len() as i32 / 2);
        let start_y = position.y.round() as i32;

        for (i, ch) in text.chars().enumerate() {
            let x = start_x + i as i32;
            self.buffer.insert((x, start_y), ch);
            self.bounds.include_point([x as f32, start_y as f32].into());
        }

        self
    }

    fn draw(&self) {
        let width = (self.bounds.x_max - self.bounds.x_min).ceil() as i32 + 1;
        let height = (self.bounds.y_max - self.bounds.y_min).ceil() as i32 + 1;
        let offset_x = self.bounds.x_min.floor() as i32;
        let offset_y = self.bounds.y_min.floor() as i32;

        let mut canvas = vec![vec![' '; width as usize]; height as usize];

        for (&(x, y), &ch) in &self.buffer {
            let canvas_x = (x - offset_x) as usize;
            let canvas_y = (y - offset_y) as usize;
            canvas[canvas_y][canvas_x] = ch;
        }

        for row in canvas.iter().rev() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

struct AsciiDrawer {
    canvas: AsciiCanvas,
    scale: Vec2,
}

impl AsciiDrawer {
    fn new(scale: Vec2) -> Self {
        AsciiDrawer {
            canvas: AsciiCanvas::new(),
            scale,
        }
    }

    fn rect(&mut self, center: Vec2, size: Vec2) -> &mut Self {
        let scaled_center: Vec2 = center * self.scale;
        let scaled_size: Vec2 = size * self.scale;
        self.canvas.rect(scaled_center, scaled_size);
        self
    }

    fn text(&mut self, position: Vec2, text: &str) -> &mut Self {
        let scaled_position: Vec2 = position * self.scale;
        self.canvas.text(scaled_position, text);
        self
    }

    fn rect_with_labels(
        &mut self,
        center: Vec2,
        size: Vec2,
        corners_coords: bool,
        center_coords: bool,
        edge_lengths: bool,
    ) -> &mut Self {
        self.rect(center, size);

        let half_width = size.x / 2.0;
        let half_height = size.y / 2.0;

        let corners = [
            [center.x - half_width, center.y - half_height].into(),
            [center.x + half_width, center.y - half_height].into(),
            [center.x - half_width, center.y + half_height].into(),
            [center.x + half_width, center.y + half_height].into(),
        ];

        if corners_coords {
            for corner in &corners {
                self.text(*corner, &corner.to_string());
            }
        }

        if center_coords {
            self.text(center, &center.to_string());
        }

        if edge_lengths {
            let left_center: Vec2 = [center.x - half_width, center.y].into();
            let bottom_center: Vec2 = [center.x, center.y - half_height].into();
            let edge_length_x = size.x.round() as i32;
            let edge_length_y = size.y.round() as i32;

            self.text(left_center, &edge_length_y.to_string());
            self.text(bottom_center, &edge_length_x.to_string());
        }

        self
    }

    fn draw(&self) {
        self.canvas.draw();
    }
}

fn main() {
    AsciiDrawer::new([5.0, 2.25].into())
        .rect([-1.0, 0.0].into(), [1.0, 1.0].into())
        .rect_with_labels([0.0, 0.0].into(), [10.0, 5.0].into(), true, false, false)
        .rect_with_labels([4.0, 0.0].into(), [6.0, 2.0].into(), false, true, true)
        .draw();
}

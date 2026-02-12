use glam::Vec2;

pub trait Drawing {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn buffer_mut(&mut self) -> &mut [u32];
    fn buffer(&self) -> &[u32];

    fn clear(&mut self, color: u32) {
        self.buffer_mut().fill(color);
    }

    fn world_to_screen(&self, world_pos: Vec2, camera_pos: Vec2) -> Option<Vec2> {
        let w = self.width() as f32;
        let h = self.height() as f32;
        let center = Vec2::new(w / 2.0, h / 2.0);

        let screen_pos = world_pos - camera_pos + center;

        if screen_pos.x < -100.0
            || screen_pos.x > w + 100.0
            || screen_pos.y < -100.0
            || screen_pos.y > h + 100.0
        {
            return None;
        }
        Some(screen_pos)
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        let w = self.width() as i32;
        let h = self.height() as i32;

        if x >= 0 && x < w && y >= 0 && y < h {
            let idx = y as usize * (w as usize) + x as usize;
            self.buffer_mut()[idx] = color;
        }
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, color: u32) {
        let (mut x0, mut y0) = (start.x as i32, start.y as i32);
        let (x1, y1) = (end.x as i32, end.y as i32);

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.draw_pixel(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_arrow_centered(&mut self, pos: Vec2, angle: f32, len: f32, col: u32) {
        let h = len / 2.0;
        let (sin_a, cos_a) = angle.sin_cos();
        let dir = Vec2::new(cos_a, sin_a);

        let s = pos - dir * h;
        let e = pos + dir * h;

        self.draw_line(s, e, col);

        let hl = 4.0;
        let aw = 0.6;

        let p1 = e - Vec2::new((angle + aw).cos(), (angle + aw).sin()) * hl;
        let p2 = e - Vec2::new((angle - aw).cos(), (angle - aw).sin()) * hl;

        self.draw_line(e, p1, col);
        self.draw_line(e, p2, col);
    }

    fn draw_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, color: u32, filled: bool) {
        if !filled {
            self.draw_line(p0, p1, color);
            self.draw_line(p1, p2, color);
            self.draw_line(p2, p0, color);
        } else {
            let mut v = [p0, p1, p2];

            v.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

            let (p0, p1, p2) = (v[0], v[1], v[2]);

            let total_height = p2.y - p0.y;
            if total_height == 0.0 {
                return;
            }

            for y in (p0.y as i32)..=(p2.y as i32) {
                let second_half = y > p1.y as i32 || p1.y == p0.y;
                let segment_height = if second_half {
                    p2.y - p1.y
                } else {
                    p1.y - p0.y
                };

                if segment_height == 0.0 && !second_half {
                    continue;
                }

                let t_a = (y as f32 - p0.y) / total_height;
                let t_b = (y as f32 - (if second_half { p1.y } else { p0.y })) / segment_height;

                let mut x_a = p0.x + (p2.x - p0.x) * t_a;
                let mut x_b = if second_half {
                    p1.x + (p2.x - p1.x) * t_b
                } else {
                    p0.x + (p1.x - p0.x) * t_b
                };

                if x_a > x_b {
                    std::mem::swap(&mut x_a, &mut x_b);
                }

                for x in (x_a as i32)..=(x_b as i32) {
                    self.draw_pixel(x, y, color);
                }
            }
        }
    }
}

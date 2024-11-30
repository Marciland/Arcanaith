use glam::{Mat4, Vec2, Vec3};

pub struct PositionComponent {
    pub xyz: Vec3,
    pub scale: Vec3,
}

impl PositionComponent {
    #[must_use]
    pub fn get_model_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.xyz) * Mat4::from_scale(self.scale)
    }
}

pub struct Quad {
    pub top_right: Vec2,
    pub top_left: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
}

impl Quad {
    pub fn new() -> Self {
        let bottom_left = Vec2 { x: -0.5, y: -0.5 };
        let bottom_right = Vec2 { x: 0.5, y: -0.5 };
        let top_right = Vec2 { x: 0.5, y: 0.5 };
        let top_left = Vec2 { x: -0.5, y: 0.5 };

        Self {
            top_right,
            top_left,
            bottom_left,
            bottom_right,
        }
    }

    pub fn position_is_inside(&self, position: Vec2) -> bool {
        // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        let mut intersections = 0;

        let edges = [
            (self.top_left, self.top_right),
            (self.top_right, self.bottom_right),
            (self.bottom_right, self.bottom_left),
            (self.bottom_left, self.top_left),
        ];

        for (start, end) in &edges {
            if (start.y > position.y) != (end.y > position.y) {
                let slope = (end.x - start.x) / (end.y - start.y);
                let intersect_x = start.x + slope * (position.y - start.y);

                if position.x < intersect_x {
                    intersections += 1;
                }
            }
        }

        intersections % 2 != 0
    }
}

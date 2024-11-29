use glam::{Mat4, Vec2, Vec3};

use super::Vertex;

pub struct PositionComponent {
    pub xyz: Vec3,
    scale: Vec3,
}

pub struct MVP {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl MVP {
    pub fn get_model_matrix(position: &PositionComponent) -> Mat4 {
        Mat4::from_translation(position.xyz) * Mat4::from_scale(position.scale)
    }

    pub fn get_projection() -> Mat4 {
        Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.0, -1.0)
    }
}

pub struct Quad {
    pub top_right: Vec2,
    pub top_left: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Quad {
    pub fn new() -> Self {
        let bottom_left = Vertex {
            position: Vec2 { x: -0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 0.0 },
        };
        let bottom_right = Vertex {
            position: Vec2 { x: 0.5, y: -0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 0.0 },
        };
        let top_right = Vertex {
            position: Vec2 { x: 0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 1.0, y: 1.0 },
        };
        let top_left = Vertex {
            position: Vec2 { x: -0.5, y: 0.5 },
            texture_coordinates: Vec2 { x: 0.0, y: 1.0 },
        };

        Self {
            top_right: top_right.position,
            top_left: top_left.position,
            bottom_left: bottom_left.position,
            bottom_right: bottom_right.position,
            vertices: vec![bottom_left, bottom_right, top_right, top_left],
            indices: vec![0, 1, 2, 2, 3, 0],
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn get_indices(&self) -> &[u16] {
        &self.indices
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

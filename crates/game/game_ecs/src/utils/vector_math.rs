use glam::Vec3;

pub fn vec3_center_point(points: &[Vec3]) -> Vec3 {
    let mut center = Vec3::ZERO;

    for point in points {
        center += point;
    }

    center / points.len() as f32
}

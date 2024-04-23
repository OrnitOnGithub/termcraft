#![allow(dead_code)] // temporary

use colored::CustomColor;
const CAMERA_DISTANCE: f32 = 10.0;

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;


#[cfg(test)]
mod tests {
  use super::*;

    /// Test the distance function
    #[test]
    fn test_distance() {
      let vec1: Vector3 = Vector3 { x: 0.0, y: 4.0, z: -4.0 };
      let vec2: Vector3 = Vector3 { x: 4.0, y: -4.0, z: 4.0 };
      // should be 12
      let actual: f32 = vector3_distance(vec1, vec2);
      let expected: f32 = 12.0;
      assert_eq!(actual, expected);
    }

    /// Test the 2D -> 1D projection function
    #[test]
    fn test_2d_to_1d_projection() {
      // test data
      let camera_position: Vector2 = Vector2 { x: 1.0, y: 1.0 };
      let camera_rot: f32 = 45.0;
      let vertex: Vector2 = Vector2 { x: 12.0, y: 12.0 };

      let actual: f32 = project_2d_to_1d(vertex, camera_position, camera_rot);
      let expected: f32 = 0.0; // in theory camera is looking directly at point

      assert_eq!(actual, expected);
    }

    #[test]
    fn test_vertex_render() {
      // the camera should be pointing directly at the vertex
      let vertex: Vector3 = Vector3 { x: 10.0, y: 10.0, z: 1.0 };
      let camera_position: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
      let camera_rotation_horizontal: f32 = 45.0;
      let camera_rotation_vertical: f32 = 0.0;
      // so expected result is (0, 0)
      let actual: Vector2 = render_vertex(vertex, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
      let expected: Vector2 = Vector2 { x: 0.0, y: 0.0 };

      assert_eq!(actual, expected);
    }
}


fn main() {
  let mut camera_position: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
  let mut camera_rotation_vertical: f32 = 0.0;
  let mut camera_rotation_horizontal: f32 = 0.0;

}

/// 3D point -> 2D point (to be put on screen)
fn render_vertex(vertex: Vector3, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> Vector2 {

  let position_on_vertical_plane: Vector2 = Vector2   { x: vertex.z, y: vertex.x };
  let position_on_horizontal_plane: Vector2 = Vector2 { x: vertex.y, y: vertex.x };  

  let position_camera_vertical_plane: Vector2 =  Vector2   { x: camera_position.z, y: camera_position.x };
  let position_camera_horizontal_plane: Vector2 =  Vector2 { x: camera_position.y, y: camera_position.x };

  let screen_x = project_2d_to_1d(position_on_vertical_plane, position_camera_vertical_plane, camera_rotation_vertical);
  let screen_y = project_2d_to_1d(position_on_horizontal_plane, position_camera_horizontal_plane, camera_rotation_horizontal);

  return Vector2 { x: screen_x, y: screen_y };
}

/// point on 2D plane -> point on 1D axis
fn project_2d_to_1d(vertex: Vector2, camera_position: Vector2, camera_rotation: f32) -> f32 {
  // distance between camera and the vertex
  let camera_vertex_distance: f32 = vector2_distance(camera_position.clone(), vertex.clone());
  // TODO: calculate relative angly directly using relative position
  // absolute angle of vertex
  let vertex_absolute_angle: f32 = f32::atan(vertex.x/vertex.y) * (180.0/PI);
  // angle of vertex relative to camera angle
  let vertex_relative_angle: f32 = camera_rotation - vertex_absolute_angle;
  println!("absolute {:?}", vertex_absolute_angle);
  println!("relative {:?}", vertex_relative_angle);
  
  //      this line represents the screen we project on
  //                          |
  //            ----\         |
  //            |    ----\   \'/    hypotenuse = camera_vertex_distance
  //            |         ----\
  // relative_y |             |----\
  //            |    screen_y |    ----\
  //            |             |          ----\
  //            -----------------------------------X  camera_position
  //                           <---CAMERA_DISTANCE--->
  //            <---------------depth---------------->

  // find depth
  let depth: f32 = f32::cos(vertex_relative_angle) * camera_vertex_distance;
  // find relative_y
  let relative_y: f32 = f32::sin(vertex_relative_angle) * camera_vertex_distance;

  // Now we apply Thales' Theorem to find screen_y
  let screen_y: f32 = (CAMERA_DISTANCE * relative_y) / (depth - CAMERA_DISTANCE);

  return screen_y;
}


/// Calculate the distance between two 3D points.
/// 
/// returns `sqrt((vec2.x - vec1.x)² + (vec2.y - vec1.y)² + (vec2.z - vec1.z)²)`
fn vector3_distance(vec1: Vector3, vec2: Vector3) -> f32 {

  let x_dist_sq: f32 = f32::powf(vec2.x - vec1.x, 2.0);
  let y_dist_sq: f32 = f32::powf(vec2.y - vec1.y, 2.0);
  let z_dist_sq: f32 = f32::powf(vec2.z - vec1.z, 2.0);

  let distance = f32::sqrt(x_dist_sq + y_dist_sq + z_dist_sq);
  return distance;
}

/// Calculate the distance between two 2D points.
/// 
/// returns `sqrt((vec2.x - vec1.x)² + (vec2.y - vec1.y)²)`
fn vector2_distance(vec1: Vector2, vec2: Vector2) -> f32 {

  let x_dist_sq: f32 = f32::powf(vec2.x - vec1.x, 2.0);
  let y_dist_sq: f32 = f32::powf(vec2.y - vec1.y, 2.0);

  let distance = f32::sqrt(x_dist_sq + y_dist_sq);
  return distance;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector3 {
  x : f32,
  y : f32,
  z : f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
  x : f32,
  y : f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle3D {
  a: Vector3,
  b: Vector3,
  c: Vector3,
  normal: Vector3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle2D {
  a: Vector2,
  b: Vector2,
  c: Vector2,
  color: CustomColor,
}
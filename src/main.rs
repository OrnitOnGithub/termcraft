#![allow(dead_code)] // temporary


use colored::*;
const CAMERA_DISTANCE: f32 = 10.0;
const SUN_DIRECTION: Vector3 = Vector3{x: 0.0, y: 0.0, z: -1.0};

const PIXEL_CHAR: String = String::from("MM");

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn dot_product() {
      let vec1: Vector3 = Vector3 { x: 0.0, y: 4.0, z: -4.0 };
      let vec2: Vector3 = Vector3 { x: 4.0, y: -4.0, z: 4.0 };
      let actual: f32 = vector3_dot(vec1, vec2);
      let expected: f32 = -32.0;
      assert_eq!(actual, expected);
    }

    /// Test the distance function
    #[test]
    fn distance() {
      let vec1: Vector3 = Vector3 { x: 0.0, y: 4.0, z: -4.0 };
      let vec2: Vector3 = Vector3 { x: 4.0, y: -4.0, z: 4.0 };
      // should be 12
      let actual: f32 = vector3_distance(vec1, vec2);
      let expected: f32 = 12.0;
      assert_eq!(actual, expected);
    }

    /// Test the 2D -> 1D projection function
    #[test]
    fn projection_2d_to_1d() {
      // test data
      let camera_position: Vector2 = Vector2 { x: 1.0, y: 1.0 };
      let camera_rot: f32 = 45.0;
      let vertex: Vector2 = Vector2 { x: 12.0, y: 12.0 };

      let actual: f32 = project_2d_to_1d(vertex, camera_position, camera_rot);
      let expected: f32 = 0.0; // in theory camera is looking directly at point

      assert_eq!(actual, expected);
    }

    #[test]
    fn vertex_render() {
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

fn draw_screen(screen: Screen) -> () {

}

fn render_triangle(triangle: Triangle3D, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> Triangle2D {
  let vertex_a: Vector2 = render_vertex(triangle.a, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let vertex_b: Vector2 = render_vertex(triangle.b, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let vertex_c: Vector2 = render_vertex(triangle.c, camera_position, camera_rotation_vertical, camera_rotation_horizontal);

  let test_colour: CustomColor = triangle.color;

  return Triangle2D { a: vertex_a, b: vertex_b, c: vertex_c, color: test_colour }
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

  // distance between camera and the vertex
  let camera_vertex_distance: f32 = vector2_distance(camera_position.clone(), vertex.clone());
  // TODO: calculate relative angly directly using relative position
  let camera_vertex_distance_vector: Vector2 = Vector2 { x: vertex.x - camera_position.x, y: vertex.y - camera_position.y };
  // world angle of vertex relative to camera
  let vertex_relative_angle: f32 = f32::atan(camera_vertex_distance_vector.x/camera_vertex_distance_vector.y);
  // relative to camera angle of vertex relative to camera
  let vertex_camera_angle: f32 = camera_rotation * (PI/180.0) - vertex_relative_angle;
  
  // find depth
  let depth: f32 = f32::cos(vertex_camera_angle) * camera_vertex_distance;
  // find relative_y
  let relative_y: f32 = f32::sin(vertex_camera_angle) * camera_vertex_distance;
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

/// Calculate the dot product between two vectors
fn vector3_dot(vec1: Vector3, vec2: Vector3) -> f32 {
  let dot_product = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
  return dot_product
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
  color: CustomColor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle2D {
  a: Vector2,
  b: Vector2,
  c: Vector2,
  color: CustomColor,
}

struct Screen {
  pixels: Vec<Vec<CustomColor>>,
  size_x: usize,
  size_y: usize,
}
impl Screen {
  fn init(mut self) {
    let mut screen: Vec<Vec<CustomColor>> = Vec::new();
    for _ in 0..self.size_x {
      let mut x_row: Vec<CustomColor> = Vec::new();
      for _ in 0..self.size_y {
        x_row.push(CustomColor{r: 0, g: 0, b: 0});
      }
      screen.push(x_row);
    }
    self.pixels = screen;
  }
  fn draw(&self) {
    for column in 0..self.size_x {
      for pixel in 0..self.size_y {
        let color: CustomColor = self.pixels[column][pixel];
        print!("{}", PIXEL_CHAR);
      }
    }
  }
}
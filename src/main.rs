#![allow(dead_code)] // temporary
use colored::*;

// unintuitively, Y is horizontal
const SCREEN_Y_SIZE: usize = 50;
// and X is vertical
const SCREEN_X_SIZE: usize = 30;

const PI: f32 = 3.14159265358979323846264338327950288419716939937510582;

const CAMERA_DISTANCE: f32 = 10.0;
const SUN_DIRECTION: Vector3 = Vector3{x: 0.0, y: 0.0, z: -1.0};


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
      let camera_rot: f32 = 45.0 * (PI/180.0);
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
      let camera_rotation_horizontal: f32 = 45.0 * (PI/180.0);
      let camera_rotation_vertical: f32 = 0.0 * (PI/180.0);
      // so expected result is (0, 0)
      let actual: Vector2 = render_vertex(vertex, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
      let expected: Vector2 = Vector2 { x: 0.0, y: 0.0 };

      assert_eq!(actual, expected);
    }

    #[test]
    fn triangle_contains() {
      let a: Vector2 = Vector2 { x: 0.0, y: 0.0 };
      let c: Vector2 = Vector2 { x: 0.0, y: 12.0 };
      let b: Vector2 = Vector2 { x: 12.0, y: 0.0 };
      let triangle: Triangle2D = Triangle2D { a, b, c, color: CustomColor { r: 0, g: 0, b: 0 } };

      let expected_1: bool = true;
      let actual_1: bool = triangle.contains(Vector2 { x: 2.0, y: 2.0 });
      assert_eq!(actual_1, expected_1);

      let expected_2: bool = false;
      let actual_2: bool = triangle.contains(Vector2 { x: 12.0, y: 12.0 });
      assert_eq!(actual_2, expected_2);
    }

}

fn main() {
  let mut camera_position: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
  // achtung! CAMERA ROTATION IS IN RADIANS
  let mut camera_rotation_vertical: f32 = 0.0 * (PI/180.0);
  let mut camera_rotation_horizontal: f32 = 0.0 * (PI/180.0);

  // initialise the screen
  let mut main_screen: Screen = Screen { pixels: Vec::new(), size_x: SCREEN_X_SIZE, size_y: SCREEN_Y_SIZE };
  main_screen.init();

  // initialise the world
  // put the whole world in memory because reading from disk is slow
  let mut world: Vec<CubeType> = load_world("world.rmc");

  // GAME LOOP
  loop {
    //clearscreen::clear().expect("failed to clear screen");
    
    // update main screen
    main_screen = draw_world();
    // update main screen

    // Draw the screen and sleep for a few milliseconds (to let the screen render)
    main_screen.draw();
    let idle_interval = std::time::Duration::from_millis(10);
    std::thread::sleep(idle_interval);
  }
}

fn draw_world() -> Screen {

  todo!("TODO: draw_world");

  // so the linter shuts up
  return Screen { pixels: Vec::new(), size_x: 0, size_y: 0 };
}

fn load_world(path: &str) -> Vec<CubeType> {
  let mut cubes: Vec<CubeType> = Vec::new();

  let contents: String = std::fs::read_to_string(path)
    .expect("Should have been able to read the file, or not idk");

  for char in contents.as_bytes() {
    match char {

      b'0' => { cubes.push(CubeType::Air)   }
      b'1' => { cubes.push(CubeType::Grass) }
      b'2' => { cubes.push(CubeType::Stone) }
      b'3' => { cubes.push(CubeType::Wood)  }

      _ => {
        panic!("invalid element in save file.");
      }
    }

  }

  return cubes
}

fn render_triangle(triangle: Triangle3D, camera_position: Vector3, camera_rotation_vertical: f32, camera_rotation_horizontal: f32) -> Triangle2D {
  let vertex_a: Vector2 = render_vertex(triangle.a, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let vertex_b: Vector2 = render_vertex(triangle.b, camera_position, camera_rotation_vertical, camera_rotation_horizontal);
  let vertex_c: Vector2 = render_vertex(triangle.c, camera_position, camera_rotation_vertical, camera_rotation_horizontal);

  let test_colour: CustomColor = triangle.color;

  return Triangle2D { a: vertex_a, b: vertex_b, c: vertex_c, color: test_colour }
}

/// 3D point -> 2D point (to be put on screen)██
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
  // find the vector that connects camera and vertex
  let camera_vertex_distance_vector: Vector2 = Vector2 { x: vertex.x - camera_position.x, y: vertex.y - camera_position.y };
  // world angle of vertex relative to camera
  let vertex_relative_angle: f32 = f32::atan(camera_vertex_distance_vector.x/camera_vertex_distance_vector.y);
  // relative to camera angle of vertex relative to camera
  let vertex_camera_angle: f32 = camera_rotation - vertex_relative_angle;
  
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
  //parent_cube_idex: usize,
}
impl Triangle2D {
  /// Check if a triangle contains a certain point
  fn contains(&self, point: Vector2) -> bool {
    // say ABC = triangle and P = point:
    // test if area ABC = area ABP + area ACP + area BCP
    let real_triangle_area = triangle_area(self.clone());

    let mut area_sum: f32 = 0.0;
    area_sum += triangle_area(Triangle2D { a: self.a, b: self.b, c: point, color: CustomColor {r: 0, g: 0, b: 0} });
    area_sum += triangle_area(Triangle2D { a: self.a, b: self.c, c: point, color: CustomColor {r: 0, g: 0, b: 0} });
    area_sum += triangle_area(Triangle2D { a: self.b, b: self.c, c: point, color: CustomColor {r: 0, g: 0, b: 0} });

    return real_triangle_area == area_sum;
  }
}
/// calculate the area of a 2D triangle
fn triangle_area(triangle: Triangle2D) -> f32 {
  let ab: Vector2 = Vector2 { x: triangle.b.x - triangle.a.x, y: triangle.b.y - triangle.a.y };
  let ac: Vector2 = Vector2 { x: triangle.c.x - triangle.a.x, y: triangle.c.y - triangle.a.y };
  let cross_product: f32 = ab.x * ac.y - ab.y * ac.x;
  let area = f32::abs(cross_product);
  return area/2.0;
}

struct Screen {
  pixels: Vec<Vec<CustomColor>>,
  size_x: usize,
  size_y: usize,
}
impl Screen {
  fn init(&mut self) {
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
    let pixel_char: ColoredString = ColoredString::from("██");
    for column in 0..self.size_x {
      for pixel in 0..self.size_y {
        let color: CustomColor = self.pixels[column][pixel];
        print!("{}", pixel_char.clone().custom_color(color));
      }
      println!();
    }
  }
}

/// enumarates all possible cubes
#[derive(Debug, Clone, PartialEq)]
enum CubeType {
  Air,
  Grass,
  Stone,
  Wood,
}
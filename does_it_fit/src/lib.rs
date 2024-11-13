mod areas_volumes;

// use geometrical_shapes as gs;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};
//use areas_volumes;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    match objects {
        GeometricalShapes::Square => {
            return y * x >= areas_volumes::square_area(a) * times;
        },
        GeometricalShapes::Circle => {
            return (y as f64 * x as f64) >= areas_volumes::circle_area(a) * times as f64;
        },
        GeometricalShapes::Rectangle => {
            return y*x >= areas_volumes::rectangle_area(a, b) * times;
        },
        GeometricalShapes::Triangle => {
            return (y as f64 * x as f64) >= areas_volumes::triangle_area(a, b) * times as f64;
        }
    }
}

pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    match objects {
        GeometricalVolumes::Cube => {
            return y * x * z >= areas_volumes::cube_volume(a) * times
        },
        GeometricalVolumes::Sphere => {
            return (y as f64 * x as f64 * z as f64) >= areas_volumes::sphere_volume(a) * times as f64
        },
        GeometricalVolumes::Cone => {
            return (y as f64 * x as f64 * z as f64) >= areas_volumes::cone_volume(a, b) * times as f64
        },
        GeometricalVolumes::Pyramid => {
            return (y as f64 * x as f64 * z as f64) >= areas_volumes::triangular_pyramid_volume(a as f64, b) * times as f64
        },
        GeometricalVolumes::Parallelepiped => {
            return y * x * z >= areas_volumes::parallelepiped_volume(a, b, c) * times
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn no_volumes_shapes() {
//         assert_eq!(true, area_fit(2, 5, areas_volumes::GeometricalShapes::Circle, 0, 2, 1));
//         assert_eq!(true, area_fit(2, 2, areas_volumes::GeometricalShapes::Rectangle, 0, 6, 10));
//         assert_eq!(
//             true,
//             volume_fit(2, 5, 3, areas_volumes::GeometricalVolumes::Cone, 0, 1, 1, 1)
//         );
//         assert_eq!(
//             true,
//             volume_fit(3, 5, 7, areas_volumes::GeometricalVolumes::Parallelepiped, 0, 2, 6, 3)
//         );
//     }

//     #[test]
//     fn equal_size() {
//         assert_eq!(true, area_fit(2, 5, areas_volumes::GeometricalShapes::Square, 1, 2, 5));
//         assert_eq!(
//             true,
//             volume_fit(3, 1, 4, areas_volumes::GeometricalVolumes::Cube, 1, 1, 3, 4)
//         );
//     }

//     #[test]
//     fn all_shapes() {
//         assert_eq!(false, area_fit(3, 5, areas_volumes::GeometricalShapes::Circle, 2, 2, 0));
//         assert_eq!(true, area_fit(8, 6, areas_volumes::GeometricalShapes::Triangle, 5, 5, 2));
//         assert_eq!(true, area_fit(7, 3, areas_volumes::GeometricalShapes::Rectangle, 2, 2, 4));
//         assert_eq!(true, area_fit(5, 5, areas_volumes::GeometricalShapes::Square, 1, 2, 4));
//     }

//     #[test]
//     fn all_volumes() {
//         assert_eq!(
//             true,
//             volume_fit(5, 6, 3, areas_volumes::GeometricalVolumes::Cube, 2, 3, 3, 4)
//         );
//         assert_eq!(
//             false,
//             volume_fit(7, 4, 4, areas_volumes::GeometricalVolumes::Cone, 1, 8, 2, 0)
//         );
//         assert_eq!(
//             true,
//             volume_fit(2, 5, 3, areas_volumes::GeometricalVolumes::Sphere, 1, 1, 1, 1)
//         );
//         assert_eq!(
//             false,
//             volume_fit(2, 5, 3, areas_volumes::GeometricalVolumes::Parallelepiped, 31, 1, 1, 1)
//         );
//         assert_eq!(
//             true,
//             volume_fit(7, 5, 3, areas_volumes::GeometricalVolumes::Pyramid, 3, 3, 2, 1)
//         );
//     }
// }

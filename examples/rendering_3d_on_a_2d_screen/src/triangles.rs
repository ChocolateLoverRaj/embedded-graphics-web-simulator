use crate::data::PointOrVector;

struct RectangularPrism {
    lowest_corner: PointOrVector,
    dimensions: PointOrVector,
}

impl RectangularPrism {
    fn get_triangles_for_rectangular_prism(&self) -> Vec<[PointOrVector; 3]> {
        vec![
            // Back side
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.y_only(),
                self.lowest_corner + self.dimensions.y_only() + self.dimensions.z_only(),
            ],
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.z_only(),
                self.lowest_corner + self.dimensions.z_only() + self.dimensions.y_only(),
            ],
            // Left Side
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.x_only(),
                self.lowest_corner + self.dimensions.x_only() + self.dimensions.z_only(),
            ],
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.z_only(),
                self.lowest_corner + self.dimensions.z_only() + self.dimensions.x_only(),
            ],
            // Bottom side
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.x_only(),
                self.lowest_corner + self.dimensions.x_only() + self.dimensions.y_only(),
            ],
            [
                self.lowest_corner,
                self.lowest_corner + self.dimensions.y_only(),
                self.lowest_corner + self.dimensions.y_only() + self.dimensions.x_only(),
            ],
            // Front side
            [
                self.lowest_corner + self.dimensions.x_only(),
                self.lowest_corner + self.dimensions.x_only() + self.dimensions.y_only(),
                self.lowest_corner
                    + self.dimensions.x_only()
                    + self.dimensions.y_only()
                    + self.dimensions.z_only(),
            ],
            [
                self.lowest_corner + self.dimensions.x_only(),
                self.lowest_corner + self.dimensions.x_only() + self.dimensions.z_only(),
                self.lowest_corner
                    + self.dimensions.x_only()
                    + self.dimensions.z_only()
                    + self.dimensions.y_only(),
            ],
            // Right Side
            [
                self.lowest_corner + self.dimensions.y_only(),
                self.lowest_corner + self.dimensions.y_only() + self.dimensions.x_only(),
                self.lowest_corner
                    + self.dimensions.y_only()
                    + self.dimensions.x_only()
                    + self.dimensions.z_only(),
            ],
            [
                self.lowest_corner + self.dimensions.y_only(),
                self.lowest_corner + self.dimensions.y_only() + self.dimensions.z_only(),
                self.lowest_corner
                    + self.dimensions.y_only()
                    + self.dimensions.z_only()
                    + self.dimensions.x_only(),
            ],
            // Top side
            [
                self.lowest_corner + self.dimensions.z_only(),
                self.lowest_corner + self.dimensions.z_only() + self.dimensions.x_only(),
                self.lowest_corner
                    + self.dimensions.z_only()
                    + self.dimensions.x_only()
                    + self.dimensions.y_only(),
            ],
            [
                self.lowest_corner + self.dimensions.z_only(),
                self.lowest_corner + self.dimensions.z_only() + self.dimensions.y_only(),
                self.lowest_corner
                    + self.dimensions.z_only()
                    + self.dimensions.y_only()
                    + self.dimensions.x_only(),
            ],
        ]
    }
}

pub fn get_triangles() -> Vec<[PointOrVector; 3]> {
    vec![
        RectangularPrism {
            lowest_corner: PointOrVector {
                x: -7.0,
                y: 2.0,
                z: 3.0,
            },
            dimensions: PointOrVector {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
        },
        RectangularPrism {
            lowest_corner: PointOrVector {
                x: -7.0,
                y: -5.0,
                z: 3.0,
            },
            dimensions: PointOrVector {
                x: 6.0,
                y: 4.0,
                z: 5.0,
            },
        },
        // RectangularPrism {
        //     lowest_corner: PointOrVector {
        //         x: -7.0,
        //         y: -4.0,
        //         z: -4.5,
        //     },
        //     dimensions: PointOrVector {
        //         x: 6.0,
        //         y: 6.0,
        //         z: 6.0,
        //     },
        // },
        RectangularPrism {
            lowest_corner: PointOrVector {
                x: -7.0,
                y: 5.0,
                z: -3.0,
            },
            dimensions: PointOrVector {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            },
        },
    ]
    .into_iter()
    .flat_map(|rectangular_prism| rectangular_prism.get_triangles_for_rectangular_prism())
    .collect()
}

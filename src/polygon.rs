use eframe::egui::Pos2;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

pub struct Polygon {
    pub vertices: Vec<Pos2>,
    pub fill_color: [u8; 3],
    pub outline_color: [u8; 3],
    pub outlined: bool,
    pub intersections: BTreeMap<usize, Vec<OrderedFloat<f32>>>,
}

impl Polygon {
    pub fn new(
        vertices: Vec<Pos2>,
        fill_color: [u8; 3],
        outline_color: [u8; 3],
        outlined: bool,
    ) -> Self {
        let intersections = BTreeMap::new();
        Self {
            vertices,
            fill_color,
            outline_color,
            intersections,
            outlined,
        }
    }

    /// Calcula as interseções do polígono com as linhas horizontais.
    pub fn calculate_intersections(&mut self) {
        self.intersections = BTreeMap::new();
        
        let mut counter = 0;
        while counter < self.vertices.len() - 1 {
            let mut x0 = self.vertices[counter].x;
            let mut y0 = self.vertices[counter].y.round();
            let mut x1 = self.vertices[counter + 1].x;
            let mut y1 = self.vertices[counter + 1].y.round();

            if y0 > y1 {
                let x = x0;
                x0 = x1;
                x1 = x;

                let y = y0;
                y0 = y1;
                y1 = y;
            }

            let dx = x1 - x0;
            let dy = y1 - y0;
            let tx = dx / dy;

            let mut x = x0;
            let mut y = y0 as usize;

            while y < y1 as usize {
                let intersections = self.intersections
                    .entry(y)
                    .or_insert(Vec::new());

                intersections.push(x.into());

                x += tx;
                y += 1;
            }

            counter += 1;
        }

        for (_, intersections) in &mut self.intersections {
            intersections.sort();
        }
    }

    /// Verifica se a coordenada (x, y) está dentro do polígono.
    pub fn is_inside(
        &self,
        x: f32,
        y: f32,
    ) -> bool {
        let y = y.round() as usize;
        if let Some(intersections) = self.intersections.get(&y) {
            let mut counter = 0;
            
            while counter < intersections.len() {
                let x_initial = intersections[counter].ceil();
                let x_final = intersections[counter + 1].floor();

                if x >= x_initial && x <= x_final {
                    return true;
                }

                counter += 2;
            }
        }

        false
    }

    /*/// Verifica se a coordenada (x, y) está dentro do polígono.
    pub fn ray_casting(
        &self,
        x: f32,
        y: f32,
    ) -> bool {
        let len = self.vertices.len();
        let mut crossings: usize = 0;

        for i in 1..len {
            let x0 = self.vertices[i - 1].x;
            let y0 = self.vertices[i - 1].y;
            let x1 = self.vertices[i].x;
            let y1 = self.vertices[i].y;

            if y0 <= y && y < y1 && (x - x0) * (y1 - y0) < (x1 - x0) * (y - y0) {
                crossings += 1
            } 
        }

        crossings % 2 == 1
    }*/
}
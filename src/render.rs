use crate::{
    polygon::Polygon,
    utils::pos_to_coords,
};

pub struct Render {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl Render {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height * 4];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    
    pub fn clean_buffer(&mut self) {
        self.buffer = vec![0; self.width * self.height * 4];
    }

    /// Desenha um polígono na tela.
    pub fn draw_polygon(&mut self, polygon: &Polygon) {
        self.fillpoly(polygon);
        if polygon.outlined {
            self.outlined(polygon);
        }
    }

    /// Desenha as arestas dos polígonos.
    pub fn outlined(&mut self, polygon: &Polygon) {
        let mut counter = 0;
        while counter < polygon.vertices.len() - 1 {
            let (
                x0,
                y0,
                x1,
                y1,
            ) = pos_to_coords(&polygon.vertices[counter], &polygon.vertices[counter + 1]);

            self.bresenham(polygon.outline_color, x0, y0, x1, y1);

            counter += 1;
        }
    }

    /// Algoritmo de preenchimento de polígonos.
    fn fillpoly(
        &mut self,
        polygon: &Polygon,
    ) {
        for (i, intersections) in &polygon.intersections {
            let mut counter = 0;

            while counter < intersections.len() {
                let x_initial = intersections[counter].ceil() as usize;
                let x_final = intersections[counter + 1].floor() as usize;

                /*
                if x_initial > x_final {
                    self.paint(*i, x_initial, polygon.fill_color);
                }
                */

                for j in x_initial..=x_final {
                    self.paint(*i, j, polygon.fill_color);
                }

                counter += 2;
            }

            print!("\n");
        }
    }

    /// Algoritmo de Bresenham para desenhar linhas.
    pub fn bresenham(
        &mut self,
        color: [u8; 3],
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
    ) {
        let x0 = x0 as isize;
        let y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
    
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
    
        let mut x = x0;
        let mut y = y0;
    
        let mut err = dx - dy;
    
        loop {
            self.paint(y as usize, x as usize, color);

            if x == x1 && y == y1 {
                break;
            }
    
            let e2 = 2 * err;
    
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
    
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }  

    /// Pinta um pixel no buffer de imagem.
    fn paint(
        &mut self,
        i: usize,
        j: usize,
        color: [u8; 3],
    ) {
        let index = ((i * self.width) + j) * 4;
        self.buffer[index]     = color[0];
        self.buffer[index + 1] = color[1];
        self.buffer[index + 2] = color[2];
        self.buffer[index + 3] = 255;
    }
}
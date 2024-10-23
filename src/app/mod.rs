mod central_panel;
mod side_panel;

use eframe::{App as EframeApp, Frame};
use eframe::egui::{CentralPanel, ColorImage, Context, Pos2, SidePanel};

use central_panel::central_panel;
use side_panel::side_panel;
use std::time::{Duration, Instant};
use crate::constants::SIDE_PANEL_WIDTH;
use crate::polygon::Polygon;
use crate::render::Render;
use crate::utils::pos_to_coords;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Draw,
    Select,
}

pub struct App {
    mode: Mode,
    duration: Duration,

    current_fill_color: [u8; 3],
    current_outline_color: [u8; 3],
    current_drawing_polygon: Vec<Pos2>,
    outlined: bool,

    polygons: Vec<Polygon>,
    selected_polygon: Option<usize>,

    render: Option<Render>,
    image: Option<ColorImage>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Draw,
            duration: Duration::default(),

            current_fill_color: [255, 255, 255],
            current_outline_color: [255, 255, 00],
            current_drawing_polygon: Vec::new(),
            outlined: true,

            polygons: Vec::new(),
            selected_polygon: None,
            
            render: None,
            image: None,
        }
    }
}

impl EframeApp for App {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame,
    ) {
        SidePanel::right("SidePanel")
            .exact_width(SIDE_PANEL_WIDTH as f32)
            .show(ctx, |ui| {
                side_panel(self, ui);
            });
        
        CentralPanel::default()
            .show(ctx, |ui| {
                central_panel(self, ui);
            });
    }
}

impl App {
    pub fn redraw(&mut self) {
        if self.render.is_none() {
            return;
        }

        let start = Instant::now();

        self.render.as_mut().unwrap().clean_buffer();

        self.draw_polygons();
        self.draw_current_drawing_polygon();

        let width = self.render.as_ref().unwrap().get_width();
        let height = self.render.as_ref().unwrap().get_height();
        let buffer = self.render.as_ref().unwrap().get_buffer();
        
        self.image = Some(ColorImage::from_rgba_premultiplied([width, height], &buffer));

        self.duration = start.elapsed();
    }

    fn draw_polygons(&mut self) {
        for polygon in &self.polygons {
            self.render.as_mut().unwrap().draw_polygon(polygon);
        }
    }

    fn draw_current_drawing_polygon(&mut self) {
        let len = self.current_drawing_polygon.len();
        if len < 2 {
            return;
        }

        let mut counter = 0;
        while counter < len - 1 {
            let (
                x0,
                y0,
                x1,
                y1,
            ) = pos_to_coords(
                &self.current_drawing_polygon[counter], 
                &self.current_drawing_polygon[counter + 1],
            );
            self.render.as_mut().unwrap().bresenham(self.current_outline_color, x0, y0, x1, y1);
            counter += 1;
        }
    }

    pub fn add_vertex_to_current_drawing_polygon(&mut self, x: f32, y: f32) {
        let new_vertex = Pos2::new(x, y);
        self.current_drawing_polygon.push(new_vertex);
        self.redraw();
    }

    pub fn add_polygon(&mut self) {
        let mut vertices = self.current_drawing_polygon.clone();
        vertices.push(self.current_drawing_polygon[0]);

        let mut polygon = Polygon::new(
            vertices,
            self.current_fill_color, 
            self.current_outline_color,
            self.outlined,
        );
        polygon.calculate_intersections();

        self.polygons.push(polygon);
        self.current_drawing_polygon.clear();
        self.redraw();
    }

    pub fn select_polygon(
        &mut self,
        x: f32,
        y: f32,
    ) {
        let len = self.polygons.len();
        if len == 0 {
            return;
        }

        self.selected_polygon = None;

        let mut index = len - 1;
        loop {
            if self.polygons[index].is_inside(x, y) {
                self.selected_polygon = Some(index);

                self.current_fill_color = self.polygons[index].fill_color;
                self.current_outline_color = self.polygons[index].outline_color;
                self.outlined = self.polygons[index].outlined;

                self.redraw();

                break;
            }

            if index == 0 {
                break;
            }

            index -= 1;
        }
    }

    pub fn delete_selected_polygon(&mut self) {
        if let Some(index) = self.selected_polygon {
            self.polygons.remove(index);
            self.selected_polygon = None;
            self.redraw();
        }
    }

    pub fn clear_current_drawing_polygon(&mut self) {
        self.current_drawing_polygon.clear();
        self.redraw();
    }

    pub fn clear_all(&mut self) {
        self.current_drawing_polygon.clear();
        self.polygons.clear();
        self.selected_polygon = None;
        self.redraw();
    }

    pub fn set_selected_outlined(&mut self, value: bool) {
        if let Some(index) = self.selected_polygon {
            self.polygons[index].outlined = value;
            self.redraw();
        }
    }
}

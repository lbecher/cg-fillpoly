use eframe::egui::{Color32, Pos2, Rect, Sense, Shape, Stroke, TextureId, TextureOptions, Ui, Vec2};
use eframe::egui::emath::RectTransform;
use crate::app::{App, Mode};
use crate::render::Render;

pub fn central_panel(app: &mut App, ui: &mut Ui) {

    // -------------------------------------
    // Aloca um Painter
    // -------------------------------------

    let x = ui.available_width();
    let y = ui.available_height();

    let desired_size = Vec2::new(x, y);
    let sense = Sense::click();

    let (
        response, 
        painter,
    ) = ui.allocate_painter(desired_size, sense);

    if app.render.is_none() {
        let x = x as usize;
        let y = y as usize;
        let render = Render::new(x, y);
        app.render = Some(render);
        app.redraw();
    }


    // -------------------------------------
    // Lida com os cliques do mouse
    // -------------------------------------

    if response.clicked() {
        match app.mode {
            Mode::Draw => {
                if let Some(position_on_window) = response.hover_pos() {
                    let painter_origin = painter.clip_rect().min;
                    let position_on_painter = position_on_window - painter_origin;
                    let x = position_on_painter.x;
                    let y = position_on_painter.y;
                    app.add_vertex_to_current_drawing_polygon(x, y);
                }
            }
            Mode::Select => {
                if let Some(position_on_window) = response.hover_pos() {
                    let painter_origin = painter.clip_rect().min;
                    let position_on_painter = position_on_window - painter_origin;
                    let x = position_on_painter.x.trunc();
                    let y = position_on_painter.y.trunc();
                    app.select_polygon(x, y);
                }
            }
        }
    }

    if response.secondary_clicked() && app.mode == Mode::Draw && app.current_drawing_polygon.len() > 2 {
        app.add_polygon();
    }


    // -------------------------------------
    // Renderiza a imagem
    // -------------------------------------

    if let Some(image) = &app.image {
        let name = "render";
        let options = TextureOptions::default();

        let texture = ui.ctx().load_texture(name, image.clone(), options);
        let texture_id = TextureId::from(&texture);
        let uv = Rect {
            min: Pos2::new(0.0, 0.0),
            max: Pos2::new(1.0, 1.0),
        };

        painter.image(texture_id, response.rect, uv, Color32::WHITE);
    }


    // -------------------------------------
    // Cria os pontos de controle para o desenho
    // -------------------------------------

    if app.mode == Mode::Draw {
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let mut redraw = false;
    
        let control_point_radius = 8.0;
        let control_point_shapes: Vec<Shape> = app.current_drawing_polygon
            .iter_mut()
            .enumerate()
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);
    
                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                if point_response.dragged() {
                    redraw = true;
                }

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);

                let control_points_stroke = Stroke::new(2.0, ui.ctx().style().visuals.text_color());
                Shape::circle_stroke(point_in_screen, control_point_radius, control_points_stroke)
            })
            .collect();
    
        if redraw {
            app.redraw();
        }
    
        painter.extend(control_point_shapes);
    } else {
        if let Some(index) = app.selected_polygon.clone() {
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            let control_point_radius = 8.0;
            let control_point_shapes: Vec<Shape> = app.polygons[index].vertices
                .iter_mut()
                .enumerate()
                .map(|(_, point)| {
                    let point_in_screen = to_screen.transform_pos(*point);
                    let control_points_stroke = Stroke::new(2.0, ui.ctx().style().visuals.text_color());
                    Shape::circle_stroke(point_in_screen, control_point_radius, control_points_stroke)
                })
                .collect();
        
            painter.extend(control_point_shapes);
        }
    }
}
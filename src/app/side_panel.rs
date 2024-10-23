use eframe::egui::{Grid, Ui, Vec2};
use crate::app::{App, Mode};

pub fn side_panel(app: &mut App, ui: &mut Ui) {
    ui.vertical(|ui| {
        if ui.radio_value(&mut app.mode, Mode::Draw, "Desenhar").clicked() {
            app.selected_polygon = None;
        };
        if ui.radio_value(&mut app.mode, Mode::Select, "Selecionar"). clicked() {
            app.delete_selected_polygon();
        };
    });
    
    Grid::new("Configurações")
        .num_columns(2)
        .spacing(Vec2::new(5.0, 5.0))
        .show(ui, |ui| {
            ui.label("Preenchimento:");
            if ui.color_edit_button_srgb(&mut app.current_fill_color).changed() {
                if let Some(index) = app.selected_polygon.clone() {
                    let fill_color = app.current_fill_color;
                    app.polygons[index].fill_color = fill_color;
                    app.redraw();
                }
            };
            ui.end_row();

            ui.label("Contorno:");
            if ui.color_edit_button_srgb(&mut app.current_outline_color).changed() {
                if let Some(index) = app.selected_polygon.clone() {
                    let outline_color = app.current_outline_color;
                    app.polygons[index].outline_color = outline_color;
                    app.redraw();
                } else if app.current_drawing_polygon.len() > 1 {
                    app.redraw();
                }
            };
            ui.end_row();

            ui.label("Pintar arestas?");
            if ui.checkbox(&mut app.outlined, "").changed() {
                app.set_selected_outlined(app.outlined);
            }
            ui.end_row();

            ui.label("Tempo:");
            ui.label(format!("{} ms", app.duration.as_millis()));
            ui.end_row();

            ui.label("Apagar:");
            ui.vertical(|ui| {
                if ui.button("Desenho").clicked() {
                    app.clear_current_drawing_polygon();
                }
                if ui.button("Polígono").clicked() {
                    app.delete_selected_polygon();
                }
                if ui.button("Tudo").clicked() {
                    app.clear_all();
                }
                
            });
        });
}
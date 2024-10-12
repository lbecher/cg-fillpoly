use eframe::egui::Pos2;

pub fn pos_to_coords(p0: &Pos2, p1: &Pos2) -> (usize, usize, usize, usize) {
    let x0 = p0.x.round() as usize;
    let y0 = p0.y.round() as usize;
    let x1 = p1.x.round() as usize;
    let y1 = p1.y.round() as usize;

    (x0, y0, x1, y1)
}
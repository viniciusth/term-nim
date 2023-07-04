use tui::layout::Rect;

pub fn get_center_of_rect_for_text(rect: &Rect, message: &str) -> Rect {
    let center_x = rect.width / 2;
    let center_y = rect.height / 2;
    let x = rect.x + center_x;
    let y = rect.y + center_y;
    let width = message.len() as u16;
    let height = 1;
    Rect::new(x - (width + 1) / 2, y, width, height)
}

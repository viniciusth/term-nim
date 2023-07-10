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

pub fn get_center_of_rect_for_list(rect: &Rect, list: &[String]) -> Rect {
    let center_x = rect.width / 2;
    let center_y = rect.height / 2;
    let x = rect.x + center_x;
    let y = rect.y + center_y;
    let width = list.iter().map(|s| s.len()).max().unwrap_or(0) as u16 + 5;
    let height = list.len() as u16;
    Rect::new(x - (width + 1) / 2 - 2, y - (height + 1) / 2, width, height)
}

pub fn get_center_of_rect_for_rect(
    rect: &Rect,
    inner_rect_width: u16,
    inner_rect_height: u16,
) -> Rect {
    let center_x = rect.width / 2;
    let center_y = rect.height / 2;
    let x = (rect.x + center_x) - (inner_rect_width + 1).min(2 * (rect.x + center_x)) / 2;
    let y = (rect.y + center_y) - (inner_rect_height + 1).min(2 * (rect.y + center_y)) / 2;

    Rect::new(
        x,
        y,
        inner_rect_width.min(rect.x + rect.width),
        inner_rect_height.min(rect.y + rect.height),
    )
}

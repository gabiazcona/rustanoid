const BLUE_SQUARE: &str = "./resources/element_blue_square.png";
const BLUE_RECTANGLE: &str = "./resources/element_blue_rectangle.png";
const GREEN_SQUARE: &str = "./resources/element_green_square.png";
const GREEN_RECTANGLE: &str = "./resources/element_green_rectangle.png";
const GREY_SQUARE: &str = "./resources/element_grey_square.png";
const GREY_RECTANGLE: &str = "./resources/element_grey_rectangle.png";
const PURPLE_SQUARE: &str = "./resources/element_purple_square.png";
const PURPLE_RECTANGLE: &str = "./resources/element_purple_rectangle.png";
const RED_SQUARE: &str = "./resources/element_red_square.png";
const RED_RECTANGLE: &str = "./resources/element_red_rectangle.png";
const YELLOW_SQUARE: &str = "./resources/element_yellow_square.png";
const YELLOW_RECTANGLE: &str = "./resources/element_yellow_rectangle.png";

pub const LEVEL1: [(&str, (f32, f32)); 12] = [
    (BLUE_RECTANGLE, (0.0,0.0)),
    (RED_SQUARE, (130.0, 0.0)),
    (BLUE_SQUARE, (230.0, 0.0)),
    (GREEN_RECTANGLE, (0.0, 230.0)),
    (GREEN_SQUARE, (330.0, 30.0)),
    (GREY_RECTANGLE, (430.0, 40.0)),
    (PURPLE_RECTANGLE, (530.0, 60.0)),
    (GREY_SQUARE, (330.0, 310.0)),
    (PURPLE_SQUARE, (15.0, 415.0)),
    (RED_RECTANGLE, (40.0, 260.0)),
    (YELLOW_RECTANGLE, (50.0, 160.0)),
    (YELLOW_SQUARE, (360.0, 360.0)),
    ];



// impl Blocks {
//     pub fn load(level: &[(&str, (f32, f32))]) -> Blocks {
//         let mut blocks = Vec::new();
//         for block in level.iter() {
//             blocks.push(Entity::new(Texture::nblock.0, Vec2::new(block[1][0], block[1][1])));
//         };
//         Blocks {
//             blocks
//         }
//     }
// }
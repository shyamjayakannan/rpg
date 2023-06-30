use crate::helpers::{Direction, Event};

pub trait Room {
    fn background_color(&self) -> &str;
    fn hero_position(&self) -> [u16; 2];
    fn upper_image(&self) -> &str;
    fn lower_image(&self) -> &str;
    fn walls(&self) -> &[[u16; 2]];
    fn npcs(&self) -> &[(&str, &str, f64, f64, &'static [(Event, Direction, u16)])];
    fn pizza_stones(&self) -> &[([u16; 2], &str, &[&[[&str; 2]]])];
    fn items(&self) -> &[([u16; 2], &str, &[&[[&str; 2]]])];
    fn npc_cutscenes(&self) -> &[(&str, &[(&[&str], &[&[[&str; 2]]])])];
    fn action_cutscenes(&self) -> &[([u16; 2], &[(&[&str], &[&[[&str; 2]]])])];
}

macro_rules! impl_for_all_rooms {
    ($(($x: ident, $y: ident)),*) => {
        $(
            mod $x;
            use self::$x::$y;

            impl Room for $y {
                fn background_color(&self) -> &str { Self::BACKGROUND_COLOR }
                fn hero_position(&self) -> [u16; 2] { Self::HERO_POSITION }
                fn upper_image(&self) -> &str { Self::UPPER_IMAGE }
                fn lower_image(&self) -> &str { Self::LOWER_IMAGE }
                fn walls(&self) -> &[[u16; 2]] { &Self::WALLS }
                fn npcs(&self) -> &[(&'static str, &'static str, f64, f64, &'static [(Event, Direction, u16)])] { &Self::NPCS }
                fn pizza_stones(&self) -> &[([u16; 2], &str, &[&[[&str; 2]]])] { &Self::PIZZA_STONES }
                fn items(&self) -> &[([u16; 2], &str, &[&[[&str; 2]]])] { &Self::ITEMS }
                fn npc_cutscenes(&self) -> &[(&str, &[(&[&str], &[&[[&str; 2]]])])] { &Self::NPC_CUTSCENES }
                fn action_cutscenes(&self) -> &[([u16; 2], &[(&[&str], &[&[[&str; 2]]])])] { &Self::ACTION_CUTSCENES }
            }
        )*
    };
}

impl_for_all_rooms!(
    (demo_room, DemoRoom),
    (dining_room, DiningRoom),
    (street, Street),
    (pizza_shop, PizzaShop),
    (kitchen, Kitchen),
    (street_north, StreetNorth)
);

pub fn get_room(name: &str) -> Box<dyn Room> {
    match name {
        "DiningRoom" => Box::new(DiningRoom),
        "Street" => Box::new(Street),
        "StreetNorth" => Box::new(StreetNorth),
        "Kitchen" => Box::new(Kitchen),
        "PizzaShop" => Box::new(PizzaShop),
        _ => Box::new(DemoRoom),
    }
}

use crate::helpers::{Event, Direction};

pub trait Room {
    fn hero_position(&self) -> [[f64; 2]; 2];
    fn upper_image(&self) -> &str;
    fn lower_image(&self) -> &str;
    fn walls(&self) -> &[[u8; 2]];
    fn npcs(&self) -> &[(&str, &str, f64, f64, &'static [(Event, Direction, u8)])];
    fn pizza_stones(&self) -> &[([u8; 2], &str, &[&[[&str; 2]]])];
    fn items(&self) -> &[([u8; 2], &str, &[&[[&str; 2]]])];
    fn npc_cutscenes(&self) -> &[(&str, &[(&[&str], &[&[[&str; 2]]])])];
    fn action_cutscenes(&self) -> &[([u8; 2], &[(&[&str], &[&[[&str; 2]]])])];
}

macro_rules! impl_for_all_rooms {
    ($(($x: ident, $y: ident)),*) => {
        $(
            mod $x;
            use self::$x::$y;

            impl Room for $y {
                fn hero_position(&self) -> [[f64; 2]; 2] { Self::HERO_POSITION }
                fn upper_image(&self) -> &str { Self::UPPER_IMAGE }
                fn lower_image(&self) -> &str { Self::LOWER_IMAGE }
                fn walls(&self) -> &[[u8; 2]] { &Self::WALLS }
                fn npcs(&self) -> &[(&'static str, &'static str, f64, f64, &'static [(Event, Direction, u8)])] { &Self::NPCS }
                fn pizza_stones(&self) -> &[([u8; 2], &str, &[&[[&str; 2]]])] { &Self::PIZZA_STONES }
                fn items(&self) -> &[([u8; 2], &str, &[&[[&str; 2]]])] { &Self::ITEMS }
                fn npc_cutscenes(&self) -> &[(&str, &[(&[&str], &[&[[&str; 2]]])])] { &Self::NPC_CUTSCENES }
                fn action_cutscenes(&self) -> &[([u8; 2], &[(&[&str], &[&[[&str; 2]]])])] { &Self::ACTION_CUTSCENES }
            }
        )*
    };
}

impl_for_all_rooms!((demo_room, DemoRoom));

pub fn get_room(name: &str) -> impl Room {
    match name {
        _ => DemoRoom,
    }
}
use crate::*;

pub struct InventoryPlugin;

#[derive(Resource)]
pub struct Inventory {
    pub current: usize,
    pub items: Vec<Block>,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            current: 0,
            items: vec![
                GRASS, DIRT, STONE, BRICKS, LOG, WOOD, LEAVES, GLASS, GLOWSTONE, WATER,
            ],
        }
    }
}

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
        app.add_systems(Update, input_inventory);
    }
}

fn input_inventory(mut inv: ResMut<Inventory>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Q) {
        inv.current += 1;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key1) {
        inv.current = 1;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key2) {
        inv.current = 2;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key3) || keys.just_pressed(KeyCode::R) {
        inv.current = 3;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key4) || keys.just_pressed(KeyCode::Z) {
        inv.current = 4;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key5) || keys.just_pressed(KeyCode::X) {
        inv.current = 5;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key6) || keys.just_pressed(KeyCode::C) {
        inv.current = 6;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key7) || keys.just_pressed(KeyCode::V) {
        inv.current = 7;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key8) || keys.just_pressed(KeyCode::G) {
        inv.current = 8;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key9) || keys.just_pressed(KeyCode::T) {
        inv.current = 9;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key0) || keys.just_pressed(KeyCode::F) {
        inv.current = 0;
        inv.current = inv.current % inv.items.len();
    }
}

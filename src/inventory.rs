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
                STONE,
                GRASS,
                DIRT,
                LIGHT_MAGIC,
                DARK_MAGIC,
                TRANSPERENT,
                WOOD_DARK_GREY,
                PINK_LEAVES,
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
    if keys.just_pressed(KeyCode::Key3) {
        inv.current = 3;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key4) {
        inv.current = 4;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key5) {
        inv.current = 5;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key6) {
        inv.current = 6;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key7) {
        inv.current = 7;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key8) {
        inv.current = 8;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key9) {
        inv.current = 9;
        inv.current = inv.current % inv.items.len();
    }
    if keys.just_pressed(KeyCode::Key0) {
        inv.current = 0;
        inv.current = inv.current % inv.items.len();
    }
}

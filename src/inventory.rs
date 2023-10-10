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
            items: vec![STONE, GRASS, DIRT, LIGHT_MAGIC, DARK_MAGIC, TRANSPERENT],
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
    if keys.just_pressed(KeyCode::N) {
        inv.current += 1;
        inv.current = inv.current % inv.items.len();
    }
}

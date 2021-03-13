use crate::item::Items;

pub fn goal(items: &Items) -> f32 {
    items.sum_value()
}
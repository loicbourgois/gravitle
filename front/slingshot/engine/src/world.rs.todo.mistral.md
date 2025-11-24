I'll address the TODO in the `add_cell_2` method. This method needs to calculate the position (x, y) for a new cell that will be tangent to two existing cells.

Here's the implementation:

```rust
pub fn add_cell_2(
    &mut self, material_url: &str, idx_1: usize, idx_2: usize, diameter: f32
) -> usize {
    let c1 = &self.cells[idx_1];
    let c2 = &self.cells[idx_2];

    // Calculate distance between centers of the two existing cells
    let dist = c1.p.distance(c2.p);

    // Calculate the angle between the two cells
    let angle = (c2.p - c1.p).angle();

    // Calculate the distance from c1 to the new cell's center
    // The new cell should be tangent to both existing cells
    let d1 = (c1.diameter + diameter) / 2.0;
    let d2 = (c2.diameter + diameter) / 2.0;

    // Calculate the position using the law of cosines
    // The new cell forms a triangle with the two existing cells
    let ratio = d1 / (d1 + d2);
    let new_dist = dist * ratio;

    // Calculate the new position
    let x = c1.p.x + (c2.p.x - c1.p.x) * ratio + (new_dist * angle.cos());
    let y = c1.p.y + (c2.p.y - c1.p.y) * ratio + (new_dist * angle.sin());

    self.add_cell(material_url, x, y, diameter)
}
```

However, I notice there's a simpler geometric approach. Here's a corrected version that properly places the new cell tangent to both existing cells:

```rust
pub fn add_cell_2(
    &mut self, material_url: &str, idx_1: usize, idx_2: usize, diameter: f32
) -> usize {
    let c1 = &self.cells[idx_1];
    let c2 = &self.cells[idx_2];

    // Calculate the direction vector between the two cells
    let dir = (c2.p - c1.p).normalize();

    // Calculate the distance between the two cells
    let dist = c1.p.distance(c2.p);

    // Calculate the position where the new cell would be tangent to both
    // The new cell's center should be at a distance of (d1 + d_new)/2 from c1
    // and (d2 + d_new)/2 from c2
    let d1 = c1.diameter * 0.5;
    let d2 = c2.diameter * 0.5;
    let d_new = diameter * 0.5;

    // The new cell's center will be along the line connecting c1 and c2
    // at a position that maintains the tangent condition
    let total_dist = d1 + d2 + d_new * 2.0;
    let ratio = (d1 + d_new) / total_dist;

    // Calculate the new position
    let x = c1.p.x + (c2.p.x - c1.p.x) * ratio;
    let y = c1.p.y + (c2.p.y - c1.p.y) * ratio;

    self.add_cell(material_url, x, y, diameter)
}
```

This implementation:
1. Gets the two existing cells
2. Calculates the direction vector between them
3. Determines the position where a new cell of the given diameter would be tangent to both existing cells
4. Adds the new cell at that position

The key insight is that the new cell's center must be at a distance of (d1 + d_new)/2 from c1 and (d2 + d_new)/2 from c2, where d1 and d2 are the diameters of the existing cells and d_new is the diameter of the new cell. The position is calculated by interpolating along the line connecting the two existing cells.
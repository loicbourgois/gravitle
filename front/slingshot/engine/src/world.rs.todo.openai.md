You're missing an implementation for the function  
```rust
pub fn add_cell_2(
    &mut self, material_url: &str, idx_1: usize, idx_2: usize, diameter: f32
)
```
The TODO says:

> “find x and y such as the new cell (a circle) is just touching the 2 existing cells”

---

### 🧠 Goal
We know the coordinates and diameters of two existing circles (`c1`, `c2`).  
We must find where to place a third circle (of given `diameter`) so that it touches both.

Let:

- `p1`, `r1` = position and radius of circle 1  
- `p2`, `r2` = position and radius of circle 2  
- `r3` = radius of the new circle (known)

We must find `p3 = (x, y)` such that:
```
|p3 - p1| = r1 + r3
|p3 - p2| = r2 + r3
```

This is the intersection of two circles with known centers/radii.

---

### ✅ Geometry Solution (2D circle intersection)

Let:
```text
d = distance(p1, p2)
a = (r1 + r3)^2 - (r2 + r3)^2 + d^2
a = a / (2d)
h = sqrt((r1 + r3)^2 - a^2)
```

and let `dir = (p2 - p1) / d`.  
Then the point along the line between p1 and p2 at distance `a` from p1 is:
```
p_mid = p1 + dir * a
```

The intersection points are:
```
p3_1 = p_mid + (-dir.y, dir.x) * h
p3_2 = p_mid - (-dir.y, dir.x) * h
```

You can pick either (for example, the one with larger y).

---

### ⚙️ Final Implementation

```rust
pub fn add_cell_2(
    &mut self, material_url: &str, idx_1: usize, idx_2: usize, diameter: f32
) -> usize {
    let c1 = &self.cells[idx_1];
    let c2 = &self.cells[idx_2];

    let r1 = c1.diameter * 0.5;
    let r2 = c2.diameter * 0.5;
    let r3 = diameter * 0.5;

    let p1 = c1.p;
    let p2 = c2.p;

    let d = p1.distance(p2);
    if d.abs() < f32::EPSILON {
        // same centers → fallback: place above
        return self.add_cell_up(material_url, idx_1, diameter);
    }

    let a = ((r1 + r3).powi(2) - (r2 + r3).powi(2) + d * d) / (2.0 * d);
    let h2 = (r1 + r3).powi(2) - a.powi(2);
    let h = if h2 > 0.0 { h2.sqrt() } else { 0.0 };

    // direction from c1 to c2
    let dir = (p2 - p1) / d;
    // perpendicular
    let perp = Point { x: -dir.y, y: dir.x };

    let p_mid = p1 + dir * a;
    let p3a = p_mid + perp * h;
    let p3b = p_mid - perp * h;

    // choose one placement (e.g., higher y)
    let p3 = if p3a.y > p3b.y { p3a } else { p3b };

    self.add_cell(material_url, p3.x, p3.y, diameter)
}
```

---

### 🧩 Notes
- This method works whether the existing cells have different diameters.  
- You can later extend it by returning both possible positions if needed.  
- The fallback ensures stability when the existing cells overlap completely.

This implementation resolves the `// TODO: find x and y ...` comment cleanly, mathematically, and integrates with your existing API.
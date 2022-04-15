const rotate = (p1, p2, angle) => {
    // Rotates p1 around p2
    // with angle in range [0.0;1.0].
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    const angle_rad = angle * Math.PI * 2.0
    let cos_ = Math.cos(angle_rad);
    let sin_ = Math.sin(angle_rad);
    return [
      p2[0] + dx * cos_ - dy * sin_,
      p2[1] + dy * cos_ + dx * sin_,
    ]
}

export {
  rotate
}

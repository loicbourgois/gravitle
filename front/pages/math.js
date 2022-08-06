const normalize = (p, d) => {
  if (!d) {
    d = distance(p)
  }
    return {
      x: p.x / d,
      y: p.y / d,
    }
}


const add = (a,b) => {
  return {
    x: b.x + a.x,
    y: b.y + a.y,
  }
}


const mod = (a,k) => {
  return {
    x: (a.x+1)%k,
    y: (a.y+1)%k,
  }
}


const del = (a,b) => {
  return {
    x: a.x - b.x,
    y: a.y - b.y,
  }
}


const mul = (a,k) => {
  return {
    x: a.x*k,
    y: a.y*k,
  }
}


const delta = (a,b) => {
  return {
    x: b.x - a.x,
    y: b.y - a.y,
  }
}


const distance_sqrd = (a,b) => {
  if (b === undefined) {
    b = {x:0,y:0}
  }
  const dp = delta(a,b)
  return dp.x*dp.x + dp.y*dp.y
}


const distance = (a,b) => {
  return Math.sqrt(distance_sqrd(a,b))
}


const dot = (a,b) => {
  return a.x*b.x + a.y * b.y
}


const collision_response = (p1, p2) => {
  // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
  const delta_velocity = {
    x: p1.dp.x - p2.dp.x,
    y: p1.dp.y - p2.dp.y,
  }
  const delta_position = {
    x: p1.np.x - p2.np.x,
    y: p1.np.y - p2.np.y,
  }
  let mass_1 = 1.0;
  let mass_2 = 1.0;
  let mass_factor = 2.0 * mass_2 / (mass_2 + mass_1);
  let dot_vp = dot(delta_velocity, delta_position);
  let distance_ = distance({x:0,y:0}, delta_position);
  let distance_squared = distance_ * distance_;
  let factor = mass_factor * dot_vp / distance_squared
  let acceleration = {
    x: delta_position.x * factor,
    y: delta_position.y * factor,
  };
  return acceleration
}


const wrap_around = (a,b) => {
  let o25 = 0.25;
  let o5 =  0.5;
  let m =   1.0;
  let m25 = o25+m;
  let m5 =  o5+m;
  let a2 =   {
    x: (a.x + m25)%m,
    y: (a.y + m25)%m
  }
  let b2 =   {
    x: (b.x + m25)%m,
    y: (b.y + m25)%m
  }
  let a3 =   {
    x: (a.x + m5)%m,
    y: (a.y + m5)%m
  }
  let b3 =   {
    x: (b.x + m5)%m,
    y: (b.y + m5)%m
  }
  const d1 = distance_sqrd(a,b)
  const d2 = distance_sqrd(a2,b2)
  const d3 = distance_sqrd(a3,b3)
  if (d1 < d2) {
    if (d1 < d3) {
      return {
        a: a,
        b: b,
        d_sqrd: d1,
      }
    } else {
      return {
        a: a3,
        b: b3,
        d_sqrd: d3,
      }
    }
  } else if (d2 < d3) {
    return {
      a: a2,
      b: b2,
      d_sqrd: d2,
    }
  } else {
    return {
      a: a3,
      b: b3,
      d_sqrd: d3,
    }
  }
}


const rotate = (p1, p2, angle) => {
  // Rotates p2 around p1
  angle = Math.PI * 2 * angle
  const dx = p2.x - p1.x;
  const dy = p2.y - p1.y;
  const cos_ = Math.cos(angle);
  const sin_ = Math.sin(angle);
  return {
    x: p1.x + dx*cos_ - dy*sin_,
    y: p1.y + dy*cos_ + dx*sin_
  }
}


const distance_wrap_arround = (a,b) => {
  return Math.sqrt(wrap_around(a,b).d_sqrd)
}


export {
  delta,
  distance_sqrd,
  distance,
  dot,
  mul,
  collision_response,
  wrap_around,
  normalize,
  rotate,
  add,
  del,
  mod,
}

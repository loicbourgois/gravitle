#version 450
// todo const local_size_x
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
#define max_collision_per_particle 64
#define max_collision_to_check 1024
#define max_particles_count 1024*64
#define max_grid_size 128
#define MAX_LINK_PER_PARTICLE 32
#define PADDER_COUNT 2
#define MAX_PARTICLE_DEFINITIONS 64
#define pat data.particles[pid_a][i_target]
#define pas data.particles[pid_a][i_source]
//#define pb_t data.particles[pid_b][i_target]
#define pbs data.particles[pid_b][i_source]
#define pas_xy vec2(pas.x, pas.y)
#define pbs_xy vec2(pbs.x, pbs.y)
#define consts push_constants
struct Particle {
  uint pdid;
  uint link_count;
  uint collision_pids [max_collision_per_particle];
  uint linked_pids [MAX_LINK_PER_PARTICLE];
  float velocity_x;
  float velocity_y;
  float momentum_x;
  float momentum_y;
  uint is_active;
  float d;
  float x;
  float y;
  float x_before;
  float y_before;
  float mass;
  float kinetic_energy;
  uint grid_x;
  uint grid_y;
  uint collisions_count;
  uint padder [PADDER_COUNT];
  //uint pdid;
};
struct CollisionCell {
  uint count;
  // collisions to check
  // they might not collide
  uint collision_pids [max_collision_to_check];
};
struct ParticleDefinition {
  float thrust;
};
layout(set = 0, binding = 0) buffer Data {
    Particle particles[max_particles_count][2];
    CollisionCell collision_grid[max_grid_size][max_grid_size];
    ParticleDefinition particle_definitions[MAX_PARTICLE_DEFINITIONS];
} data;
layout(push_constant) uniform pushConstants {
  vec2 gravity;
  uint i_source;
  uint i_target;
  float width;
  float height;
  uint grid_size;
  float delta_time_s;
  float collision_push_rate;
} push_constants;
bool particles_are_colliding(Particle pa, Particle pb) {
  float delta_x = pa.x - pb.x;
  float delta_y = pa.y - pb.y;
  float distance_squared_centers = delta_x * delta_x + delta_y * delta_y;
  float diameters = pa.d + pb.d;
  float radiuses_squared = diameters * diameters * 0.25;
  return distance_squared_centers < radiuses_squared;
}
vec2 delta_vector(vec2 va, vec2 vb) {
  return vec2(vb.x - va.x, vb.y - va.y);
}
void main() {
  uint pid_a = gl_GlobalInvocationID.x;
  uint i_source = push_constants.i_source;
  uint i_target = push_constants.i_target;
  if (pas.is_active == 0) {
    return;
  }
  uint grid_size = push_constants.grid_size;
  // Init target position
  pat.x = pas.x;
  pat.y = pas.y;
  pat.x_before = pas.x;
  pat.y_before = pas.y;
  // Collision detection
  vec2 force = vec2(0.0, 0.0);
  uint gi_min = pas.grid_x-1;
  if (pas.grid_x == 0) {
    gi_min = 0;
  }
  uint gi_max = min(grid_size-1, pas.grid_x+1);
  uint gj_min = pas.grid_y-1;
  if (pas.grid_y == 0) {
    gj_min = 0;
  }
  uint gj_max = min(grid_size-1, pas.grid_y+1);
  pas.collisions_count = 0;
  for (uint gi=gi_min ; gi <= gi_max ; gi++) {
    for (uint gj=gj_min ; gj <= gj_max ; gj++) {
      for(int i=0 ; i < data.collision_grid[gi][gj].count ; ++i) {
        uint pid_b = data.collision_grid[gi][gj].collision_pids[i];
        if (pid_a < pid_b && particles_are_colliding(pas, pbs)) {
          if (pas.collisions_count < max_collision_per_particle) {
            pas.collision_pids[pas.collisions_count] = pid_b;
          }
          pas.collisions_count += 1;
        }
      }
    }
  }
  // Compute link
  vec2 direction = vec2(0.0, 0.0);
  for (uint i=0 ; i < pas.link_count ; i+=1) {
    uint pid_b = pas.linked_pids[i];
    float l = (pas.d + pbs.d) * 0.5;
    float dl = l - distance(pas_xy, pbs_xy);
    vec2 ndv = normalize(delta_vector(pas_xy, pbs_xy));
    direction += ndv;
    vec2 dv = ndv * dl;
    float strength = 1.0;
    float damping = 0.5;
    pat.x -= dv.x * strength * consts.delta_time_s;
    pat.y -= dv.y * strength * consts.delta_time_s;
    pat.x_before -= dv.x * strength * damping * consts.delta_time_s;
    pat.y_before -= dv.y * strength * damping * consts.delta_time_s;
  }
  // Thrust
  vec2 thrust_force = vec2(0.0, 0.0);
  if (direction.x * direction.y != 0.0) {
    direction = normalize(direction);
    thrust_force = direction * data.particle_definitions[pas.pdid].thrust * consts.delta_time_s;
  }
  // Compute gravity force
  vec2 gravity_force = vec2(
    consts.gravity.x * pas.mass * consts.delta_time_s,
    consts.gravity.y * pas.mass * consts.delta_time_s
  );
  // Move target
  vec2 acceleration = (gravity_force+thrust_force) / pas.mass;
  pat.x += acceleration.x * consts.delta_time_s * consts.delta_time_s;
  pat.y += acceleration.y * consts.delta_time_s * consts.delta_time_s;
  float move_ratio = 1.0;
  pat.x += (pas.x - pas.x_before) * move_ratio;
  pat.y += (pas.y - pas.y_before) * move_ratio;
  // Ground response
  if (pat.y < 0.0) {
    float dy = pat.y - pat.y_before;
    pat.y = 0.0;
    pat.y_before = pat.y + dy;
  }
  if (pat.y > consts.height) {
    float dy = pat.y - pat.y_before;
    pat.y = consts.height;
    pat.y_before = pat.y + dy;
  }
  // Walls response
  if (pat.x < 0.0) {
    float dx = pat.x - pat.x_before;
    pat.x = 0.0;
    pat.x_before = pat.x + dx;
  }
  if (pat.x > consts.width) {
    float dx = pat.x - pat.x_before;
    pat.x = consts.width;
    pat.x_before = pat.x + dx;
  }
  // Update velocity
  pat.velocity_x = (pat.x - pat.x_before) / consts.delta_time_s;
  pat.velocity_y = (pat.y - pat.y_before) / consts.delta_time_s;
  // Update momentum
  pat.momentum_x = pat.mass * pat.velocity_x;
  pat.momentum_y = pat.mass * pat.velocity_y;
  // Update kinetic energy
  pat.kinetic_energy = 0.5 * pat.mass * (pat.velocity_x * pat.velocity_x + pat.velocity_y * pat.velocity_y);
  // Update grid
  pat.grid_x = min(max(uint(floor((pat.x * grid_size) / push_constants.width)), 0), consts.grid_size-1);
  pat.grid_y = min(max(uint(floor((pat.y * grid_size) / push_constants.height)), 0), consts.grid_size-1);
}
//
/*
float get_distance_squared(vec2 pa, vec2 pb) {
  float delta_x = pa.x - pb.x;
  float delta_y = pa.y - pb.y;
  return delta_x * delta_x + delta_y * delta_y;
}*/



//
// collision response
//
/*float distance_centers = distance(pas_xy, pbs_xy);
float radiuses = (pas.d * 0.5) + (pbs.d * 0.5);
float delta = radiuses - distance_centers;
vec2  v_ = delta_vector(pas_xy, pbs_xy);
vec2 delta_vector = normalize(v_) * delta;
pat.x -= delta_vector.x * consts.collision_push_rate;
pat.y -= delta_vector.y * consts.collision_push_rate;*/
//pat.x_before -= delta_vector.x * consts.collision_push_rate;
//pat.y_before -= delta_vector.y * consts.collision_push_rate;

//vec2 link_force = vec2(0.0, 0.0);
/*for (uint i=0 ; i < pas.link_count ; i+=1) {
  uint pid_b = pas.linked_pids[i];
  float l = (pas.d + pbs.d) * 0.5;
  float dl = l - distance(pas_xy, pbs_xy);
  float spring_strength = 200.0;
  vec2 dv = normalize(delta_vector(pas_xy, pbs_xy)) * dl;
  link_force.x -= dvn.x * spring_strength;
  link_force.y -= dvn.y * spring_strength;
}*/


// Damping links
//vec2 drag_force_link = -link_force / pas.mass * 0.1;
//vec2 drag_force_link = acceleration_link * 0.1 ;


//vec2 acceleration = (gravity_force + link_force) / pas.mass;


// Immobilize almost static particles
/*if ( abs(pat.x - pat.x_before) < 0.0001) {
  pat.x_before = pat.x;
}
if ( abs(pat.y - pat.y_before) < 0.0001) {
  pat.y_before = pat.y;
}*/

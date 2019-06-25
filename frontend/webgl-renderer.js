//
// Shaders
//
const vertex_shader_links_source = `#version 300 es
    in vec2 a_position;
    uniform vec2 u_resolution;
    void main() {
        vec2 position = a_position / u_resolution;
        gl_Position = vec4(position, 0, 1);
    }
`;

const fragment_shader_links_source = `#version 300 es
    precision mediump float;
    out vec4 outColor;
    void main() {
        outColor = vec4(1, 1, 1, 1);
    }
`;

const fragment_shader_thrusting_links_source = `#version 300 es
    precision mediump float;
    out vec4 outColor;
    void main() {
        outColor = vec4(0.5, 0.5, 1, 1);
    }
`;

const vertex_shader_particles_source = `#version 300 es
    in vec2 a_position;
    in vec2 a_center;
    in float a_radius;
    in float a_thrust;
    uniform vec2 u_resolution;
    out vec2 position;
    out vec2 center;
    out float radius;
    out float thrust;
    void main() {
        radius = a_radius;
        position = a_position;
        center = a_center;
        thrust = a_thrust;
        gl_Position = vec4(a_position / u_resolution, 0, 1);
    }
`;

const fragment_shader_particles_source = `#version 300 es
    precision mediump float;
    in vec2 position;
    in vec2 center;
    in float radius;
    in float thrust;
    out vec4 outColor;
    void main() {
        if (thrust > 0.0) {
            if (distance(center, position) < radius && distance(center, position) > radius * 0.5) {
                outColor = vec4(0.0, 0.5, 1.0, 1.0);
            } else if (distance(center, position) > radius) {
                float a = distance(center, position) - radius;
                float transparency = 1.0 - a * 0.1;
                outColor = vec4(0, 0.5, 1.0, transparency);
            } else {
                outColor = vec4(0, 0, 0, 0);
            }
        } else {
            if (distance(center, position) < radius && distance(center, position) > radius * 0.5) {
                outColor = vec4(1, 1, 1, 1);
            } else {
                outColor = vec4(0, 0, 0, 0);
            }
        }
    }
`;

const vertex_shader_gravitational_field_source = `#version 300 es
    in vec2 a_position;
    in float a_field_value;
    uniform vec2 u_resolution;
    out float field_value;
    void main() {
        field_value = a_field_value;
        vec2 position = a_position / u_resolution;
        gl_Position = vec4(position, 0, 1);
    }
`;

const fragment_shader_gravitational_field_source = `#version 300 es
    precision mediump float;
    in float field_value;
    out vec4 outColor;
    void main() {
        outColor = vec4(
            field_value * 0.9,
            field_value * 0.9,
            field_value * 0.9,
            1
        );
    }
`;

const trajectories_vertex_shader_source = `#version 300 es
    in vec2 a_position;
    in vec2 a_center;
    in float a_radius;
    uniform vec2 u_resolution;
    out vec2 position;
    out vec2 center;
    out float radius;
    void main() {
        radius = a_radius;
        position = a_position;
        center = a_center;
        gl_Position = vec4(a_position / u_resolution, 0, 1);
    }
`;

const trajectories_fragment_shader_source = `#version 300 es
    precision mediump float;
    in vec2 position;
    in vec2 center;
    in float radius;
    out vec4 outColor;
    void main() {
        if (distance(center, position) < radius) {
            outColor = vec4(.25, .25, .25, 1);
        } else {
            outColor = vec4(0, 0, 0, 0);
        }
    }
`;

const launchers_vertex_shader_source  = `#version 300 es
    in vec2 a_position;
    uniform vec2 u_resolution;
    void main() {
        gl_Position = vec4(a_position / u_resolution, 0, 1);
    }
`;

const launchers_fragment_shader_source = `#version 300 es
    precision mediump float;
    out vec4 color_out;
    void main() {
        float shade = 0.6;
        color_out = vec4(
            shade,
            shade,
            shade,
            1
        );
    }
`;

const current_launcher_vertex_shader_source  = `#version 300 es
    in vec2 a_position;
    uniform vec2 u_resolution;
    void main() {
        gl_Position = vec4(a_position / u_resolution, 0, 1);
    }
`;

const current_launcher_fragment_shader_source = `#version 300 es
    precision mediump float;
    out vec4 color_out;
    void main() {
        color_out = vec4(
            0.75,
            0.75,
            1,
            1
        );
    }
`;

const thrusting_links_trails_vertex_shader_source = `#version 300 es
    in float a_thrust_activated;
    in float a_age;
    in vec2 a_position;
    uniform vec2 u_resolution;
    out float thrust_activated;
    out float age;
    void main() {
        thrust_activated = a_thrust_activated;
        age = a_age;
        gl_Position = vec4(a_position / u_resolution, 0, 1);
    }
`;

const thrusting_links_trails_fragment_shader_source = `#version 300 es
    precision mediump float;
    in float thrust_activated;
    in float age;
    out vec4 outColor;
    void main() {
        if (thrust_activated > 0.5) {
            float transparency = (1.0 - age) * (1.0 - age) * (1.0 - age);
            transparency *= 0.5;
            outColor = vec4(0.0, 0.5, 1.0, transparency);
        } else {
            outColor = vec4(0.0, 0.0, 0.0, 0.0);
        }
    }
`;

//
// Returns a compiled shader.
//
const create_shader = (webgl_context, type, source) => {
    const shader = webgl_context.createShader(type);
    webgl_context.shaderSource(shader, source);
    webgl_context.compileShader(shader);
    const success = webgl_context.getShaderParameter(shader, webgl_context.COMPILE_STATUS);
    if (success) {
        return shader;
    } else {
        console.warn(webgl_context.getShaderInfoLog(shader));
        webgl_context.deleteShader(shader);
    }
}

//
// Returns a compiled program using compiled shaders.
//
const create_program = (gl, vertexShader, fragmentShader) => {
    var program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    var success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) {
        return program;
    }
    console.log(gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
}

//
// Returns a compiled program from shader sources
//
const create_program_from_sources = (
    webgl_context,
    vertex_shader_source,
    fragment_shader_source
) => {
    return create_program(
        webgl_context,
        create_shader(
            webgl_context,
            webgl_context.VERTEX_SHADER,
            vertex_shader_source
        ),
        create_shader(
            webgl_context,
            webgl_context.FRAGMENT_SHADER,
            fragment_shader_source
        )
    );
}

//
// Size of a data point for a particle
//
const particle_data_point_size = 4;

//
// WebGL renderer.
// Draws 
//
export default class WebGLRenderer {

    //
    // Constructor
    //
    constructor (
        webgl_context
    ) {
        this.webgl_context = webgl_context;
        this.thrusting_link_history = [];

        //
        // Setup blending
        //
        this.webgl_context.enable(this.webgl_context.BLEND);
        // At first, used additive blending to stack up drawings
        // Source : https://stackoverflow.com/a/35544537
        //this.webgl_context.blendFunc(
        //    this.webgl_context.ONE,
        //    this.webgl_context.ONE_MINUS_SRC_ALPHA
        //);
        // Now use another way, which looks better
        this.webgl_context.blendFunc(
            this.webgl_context.SRC_ALPHA,
            this.webgl_context.ONE
        );

        ////////////////////////////////////////////////////////////////////////
        // Create all shaders and programs and look up locations
        ////////////////////////////////////////////////////////////////////////

        //
        // Links init
        //
        this.links_program = create_program_from_sources(
            this.webgl_context,
            vertex_shader_links_source,
            fragment_shader_links_source
        );
        this.links_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.links_program,
            "u_resolution"
        );
        this.links_position_attribute_location = this.webgl_context.getAttribLocation(
            this.links_program,
            'a_position'
        );
        this.links_position_buffer = this.webgl_context.createBuffer();
        this.links_vao = this.webgl_context.createVertexArray();
        //
        // Thrusting links init
        //
        this.thrusting_links_program = create_program_from_sources(
            this.webgl_context,
            vertex_shader_links_source,
            fragment_shader_thrusting_links_source
        );
        this.thrusting_links_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.thrusting_links_program,
            "u_resolution");
        this.thrusting_links_position_attribute_location = this.webgl_context.getAttribLocation(
            this.thrusting_links_program,
            'a_position');
        this.thrusting_links_position_buffer = this.webgl_context.createBuffer();
        //
        // Init particles
        //
        this.particles_program = create_program_from_sources(
            this.webgl_context,
            vertex_shader_particles_source,
            fragment_shader_particles_source
        );
        this.particles_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.particles_program,
            'u_resolution'
        );
        this.particles_position_attribute_location = this.webgl_context.getAttribLocation(
            this.particles_program,
            'a_position'
        );
        this.particles_center_attribute_location = this.webgl_context.getAttribLocation(
            this.particles_program,
            'a_center'
        );
        this.particles_radius_attribute_location = this.webgl_context.getAttribLocation(
            this.particles_program,
            'a_radius'
        );
        this.particles_thrust_attribute_location = this.webgl_context.getAttribLocation(
            this.particles_program,
            'a_thrust'
        );
        this.particles_position_buffer = this.webgl_context.createBuffer();
        this.particles_center_buffer = this.webgl_context.createBuffer();
        this.particles_radius_buffer = this.webgl_context.createBuffer();
        this.particles_thrust_buffer = this.webgl_context.createBuffer();
        this.particles_vao = this.webgl_context.createVertexArray();
        //
        // Init gravitational grid
        //
        this.gravitational_grid_program = create_program_from_sources(
            this.webgl_context,
            vertex_shader_gravitational_field_source,
            fragment_shader_gravitational_field_source
        );
        this.gravitational_grid_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.gravitational_grid_program,
            'u_resolution'
        );
        this.gravitational_grid_position_attribute_location = this.webgl_context.getAttribLocation(
            this.gravitational_grid_program,
            'a_position'
        );
        this.gravitational_grid_field_value_attribute_location = this.webgl_context.getAttribLocation(
            this.gravitational_grid_program,
            'a_field_value'
        );
        this.gravitational_grid_position_buffer = this.webgl_context.createBuffer();
        this.gravitational_grid_field_value_buffer = this.webgl_context.createBuffer();
        this.gravitational_grid_vao = this.webgl_context.createVertexArray();
        //
        // Init trajectories
        //
        this.trajectories_program = create_program_from_sources(
            this.webgl_context,
            trajectories_vertex_shader_source,
            trajectories_fragment_shader_source
        );
        this.trajectories_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.trajectories_program,
            'u_resolution'
        );
        this.trajectories_position_attribute_location = this.webgl_context.getAttribLocation(
            this.trajectories_program,
            'a_position'
        );
        this.trajectories_center_attribute_location = this.webgl_context.getAttribLocation(
            this.trajectories_program,
            'a_center'
        );
        this.trajectories_radius_attribute_location = this.webgl_context.getAttribLocation(
            this.trajectories_program,
            'a_radius'
        );
        this.trajectories_position_buffer = this.webgl_context.createBuffer();
        this.trajectories_center_buffer = this.webgl_context.createBuffer();
        this.trajectories_radius_buffer = this.webgl_context.createBuffer();
        this.trajectories_vao = this.webgl_context.createVertexArray();
        //
        // Launchers init
        //
        this.launchers_program = create_program_from_sources(
            this.webgl_context,
            launchers_vertex_shader_source,
            launchers_fragment_shader_source
        );
        this.launchers_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.launchers_program,
            "u_resolution");
        this.launchers_position_attribute_location = this.webgl_context.getAttribLocation(
            this.launchers_program,
            'a_position');
        this.launchers_position_buffer = this.webgl_context.createBuffer();
        this.launchers_vao = this.webgl_context.createVertexArray();
        //
        // Current launcher init
        //
        this.current_launcher_program = create_program_from_sources(
            this.webgl_context,
            current_launcher_vertex_shader_source,
            current_launcher_fragment_shader_source
        );
        this.current_launcher_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.current_launcher_program,
            "u_resolution");
        this.current_launcher_position_attribute_location = this.webgl_context.getAttribLocation(
            this.current_launcher_program,
            'a_position');
        this.current_launcher_position_buffer = this.webgl_context.createBuffer();
        this.current_launcher_vao = this.webgl_context.createVertexArray();
        //
        // Thrusting links trails init
        //
        this.thrusting_links_trails_program = create_program_from_sources(
            this.webgl_context,
            thrusting_links_trails_vertex_shader_source,
            thrusting_links_trails_fragment_shader_source
        );
        this.thrusting_links_trails_resolution_uniform_location = this.webgl_context.getUniformLocation(
            this.thrusting_links_trails_program,
            "u_resolution");
        this.thrusting_links_trails_position_attribute_location = this.webgl_context.getAttribLocation(
            this.thrusting_links_trails_program,
            'a_position');
        this.thrusting_links_trails_thrust_activated_attribute_location = this.webgl_context.getAttribLocation(
            this.thrusting_links_trails_program,
            'a_thrust_activated');
        this.thrusting_links_trails_age_attribute_location = this.webgl_context.getAttribLocation(
            this.thrusting_links_trails_program,
            'a_age');
        this.thrusting_links_trails_position_buffer = this.webgl_context.createBuffer();
        this.thrusting_links_trails_thrust_activated_buffer = this.webgl_context.createBuffer();
        this.thrusting_links_trails_age_buffer = this.webgl_context.createBuffer();
        this.thrusting_links_trails_vao = this.webgl_context.createVertexArray();
    }

    //
    // Render function
    // Main function.
    // Calls specific functions to render each group of objects.
    //
    render (
        links_data,
        particles_data,
        gravitational_grid,
        gravitational_grid_resolution,
        universe_width,
        universe_height,
        SHOW_GRAVITATIONAL_FIELD,
        SHOW_TRAJECTORIES,
        trajectories_data,
        launchers_data,
        DRAW_LAUNCHERS,
        current_launcher_data,
        links_state_data
    ) {
        //this.resize();
        this.clear();
        if (SHOW_GRAVITATIONAL_FIELD) {
            this.draw_gravitational_grid(
                gravitational_grid,
                universe_width,
                universe_height,
                gravitational_grid_resolution
            );
        } else {
            // Do nothing
        }
        if (SHOW_TRAJECTORIES) {
            this.draw_trajectories(
                trajectories_data,
                universe_width,
                universe_height
            );
        } else {
            // Do nothing
        }
        if (DRAW_LAUNCHERS) {
            this.draw_launchers(
                launchers_data,
                universe_width,
                universe_height
            );
        } else {
            // Do nothing
        }
        this.draw_current_launcher(
            current_launcher_data,
            universe_width,
            universe_height
        );
        this.draw_links(
            links_data,
            universe_width,
            universe_height
        );
        this.draw_particles(
            particles_data,
            universe_width,
            universe_height
        );
        this.update_thrusting_link_trail_data(
            links_state_data
        );
        this.draw_thrusting_links_trails(
            this.thrusting_link_trails_positions_data,
            universe_width,
            universe_height,
            this.thrusting_link_trails_thrust_activated_data,
            this.thrusting_link_trails_age_data
        );
    }

    //
    // Draw gravitaional grid
    //
    draw_gravitational_grid(
        gravitational_grid,
        universe_width,
        universe_height,
        resolution
    ) {
        let data_field_value = [];
        let data_coordinates = [];
        for (let i = 0 ; i < resolution ; i += 1) {
            for (let j = 0 ; j < resolution ; j += 1) {
                const x_min = i * universe_width / resolution - universe_width / 2;
                const y_min = j * universe_height / resolution - universe_height / 2;
                const x_max = x_min + universe_width / resolution;
                const y_max = y_min + universe_height / resolution;
                data_coordinates.push(...[
                    x_min, y_min,
                    x_min, y_max,
                    x_max, y_min,
                    x_min, y_max,
                    x_max, y_max,
                    x_max, y_min
                ]);
                const field_value = gravitational_grid[i * resolution + j];
                data_field_value.push(...[
                    field_value,
                    field_value,
                    field_value,
                    field_value,
                    field_value,
                    field_value
                ]);
            }
        }
        const size = 2;
        const type = this.webgl_context.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;
        this.webgl_context.bindVertexArray(this.gravitational_grid_vao);
        // Coordinates
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.gravitational_grid_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_coordinates),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.gravitational_grid_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.gravitational_grid_position_attribute_location
        );
        // Field values
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.gravitational_grid_field_value_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_field_value),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.gravitational_grid_field_value_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.gravitational_grid_field_value_attribute_location
        );
        //
        this.webgl_context.useProgram(this.gravitational_grid_program);
        this.webgl_context.uniform2f(
            this.gravitational_grid_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        this.webgl_context.drawArrays(
            this.webgl_context.TRIANGLES,
            offset,
            data_coordinates.length / 2
        );
    }

    //
    // Draw particle trajectories
    //
    draw_trajectories(
        trajectories,
        universe_width,
        universe_height
    ) {
        let data_positions = [];
        let data_centers = [];
        let data_radiuses = [];
        for (let i = 0, l = trajectories.length, c = 2 ; i < l ; i += c) {
            const length = 0.5;
            data_positions.push(...[
                trajectories[i] - length, trajectories[i + 1] - length,
                trajectories[i] + length, trajectories[i + 1] - length,
                trajectories[i] - length, trajectories[i + 1] + length,
                trajectories[i] + length, trajectories[i + 1] + length,
                trajectories[i] + length, trajectories[i + 1] - length,
                trajectories[i] - length, trajectories[i + 1] + length
            ]);
        }
        for (let i = 0, l = trajectories.length, c = 2 ; i < l ; i += c) {
            const length = 0;
            data_centers.push(...[
                trajectories[i] - length, trajectories[i + 1] - length,
                trajectories[i] + length, trajectories[i + 1] - length,
                trajectories[i] - length, trajectories[i + 1] + length,
                trajectories[i] + length, trajectories[i + 1] + length,
                trajectories[i] + length, trajectories[i + 1] - length,
                trajectories[i] - length, trajectories[i + 1] + length
            ]);
        }
        for (let i = 0, l = trajectories.length, c = 2 ; i < l ; i += c) {
            const radius = 0.5;
            data_radiuses.push(...[
                radius,
                radius,
                radius,
                radius,
                radius,
                radius
            ]);
        }
        const data_count = data_positions.length / 2;
        const size = 2;          // 2 components per iteration
        const type = this.webgl_context.FLOAT;   // the data is 32bit floats
        const normalize = false; // don't normalize the data
        const stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
        const offset = 0;        // start at the beginning of the buffer
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.trajectories_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        // Positions
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.trajectories_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_positions),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.trajectories_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.trajectories_position_attribute_location
        );
        // Center
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.trajectories_center_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_centers),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.trajectories_center_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.trajectories_center_attribute_location
        );
        // Radii
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.trajectories_radius_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_radiuses),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.trajectories_radius_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.trajectories_radius_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.trajectories_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(
            this.trajectories_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(
            this.webgl_context.TRIANGLES,
            offset,
            data_count
        );
    }

    //
    // Draw launchers
    //
    draw_launchers(
        data,
        universe_width,
        universe_height
    ) {
        const size = 2;
        const type = this.webgl_context.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;
        const data_count = data.length / size;
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.launchers_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.launchers_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.launchers_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.launchers_position_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.launchers_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(
            this.launchers_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(this.webgl_context.LINES, offset, data_count);
    }

    //
    // Draw current launcher
    //
    draw_current_launcher(
        data,
        universe_width,
        universe_height
    ) {
        const size = 2;
        const type = this.webgl_context.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;
        const data_count = data.length / size;
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.current_launcher_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.current_launcher_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.current_launcher_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.current_launcher_position_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.current_launcher_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(
            this.current_launcher_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(this.webgl_context.LINES, offset, data_count);
    }

    //
    // Draw links
    //
    draw_links(data, universe_width, universe_height) {
        const links_data_count = data.length / 2;
        const size = 2;          // 2 components per iteration
        const type = this.webgl_context.FLOAT;   // the data is 32bit floats
        const normalize = false; // don't normalize the data
        const stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
        const offset = 0;        // start at the beginning of the buffer
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.links_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.links_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.links_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.links_position_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.links_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(this.links_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(this.webgl_context.LINES, offset, links_data_count);
    }

    //
    // Draw particles
    //
    // data is formated as follow for each particle
    //  position.x ;
    //  position.y ;
    //  radius ;
    //  thrust_force or 0.0
    //
    draw_particles(
        data,
        universe_width,
        universe_height
    ) {
        let data_positions = [];
        let data_centers = [];
        let data_radiuses = [];
        let data_thrust = [];
        for (let i = 0, l = data.length, c = particle_data_point_size ; i < l ; i += c) {
            // Length for the side of a rendering rectangle for particles
            // More than the actual radisu of the particle so thrusting effects
            // can be added.
            const side_half_length = data[i + 2] * 2.0;
            data_positions.push(...[
                data[i] - side_half_length, data[i + 1] - side_half_length,
                data[i] + side_half_length, data[i + 1] - side_half_length,
                data[i] - side_half_length, data[i + 1] + side_half_length,
                data[i] + side_half_length, data[i + 1] + side_half_length,
                data[i] + side_half_length, data[i + 1] - side_half_length,
                data[i] - side_half_length, data[i + 1] + side_half_length
            ]);
        }
        for (let i = 0, l = data.length, c = particle_data_point_size ; i < l ; i += c) {
            data_centers.push(...[
                data[i], data[i + 1],
                data[i], data[i + 1],
                data[i], data[i + 1],
                data[i], data[i + 1],
                data[i], data[i + 1],
                data[i], data[i + 1]
            ]);
        }
        for (let i = 0, l = data.length, c = particle_data_point_size ; i < l ; i += c) {
            const radius = data[i + 2];
            data_radiuses.push(...[
                radius,
                radius,
                radius,
                radius,
                radius,
                radius
            ]);
        }
        for (let i = 0, l = data.length, c = particle_data_point_size ; i < l ; i += c) {
            const thrust = data[i + 3];
            data_thrust.push(...[
                thrust,
                thrust,
                thrust,
                thrust,
                thrust,
                thrust
            ]);
        }
        const data_count = data_positions.length / 2;
        const size = 2;          // 2 components per iteration
        const type = this.webgl_context.FLOAT;   // the data is 32bit floats
        const normalize = false; // don't normalize the data
        const stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
        const offset = 0;        // start at the beginning of the buffer
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.particles_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        // Positions
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.particles_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_positions),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.particles_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.particles_position_attribute_location
        );
        // Centers
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.particles_center_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_centers),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.particles_center_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.particles_center_attribute_location
        );
        // Radiuses
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.particles_radius_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_radiuses),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.particles_radius_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.particles_radius_attribute_location
        );
        // Thrust
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.particles_thrust_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(data_thrust),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.particles_thrust_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.particles_thrust_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.particles_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(
            this.particles_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(
            this.webgl_context.TRIANGLES,
            offset,
            data_count
        );
    }

    //
    // Clear the canavs
    //
    clear() {
        this.webgl_context.viewport(
            0,
            0,
            this.webgl_context.canvas.width,
            this.webgl_context.canvas.height
        );
        this.webgl_context.clearColor(0, 0, 0, 0);
        this.webgl_context.clear(this.webgl_context.COLOR_BUFFER_BIT);
    }

    //
    // Resize the canvas
    //
    resize() {
        const canvas = this.webgl_context.canvas;
        var displayWidth  = canvas.clientWidth;
        var displayHeight = canvas.clientHeight;
        // Check if the canvas is not the same size.
        if (canvas.width !== displayWidth ||
          canvas.height !== displayHeight) {
            // Make the canvas the same size
            canvas.width  = displayWidth;
            canvas.height = displayHeight;
        }
    }

    //
    // Update data used to draw link trails 
    //
    update_thrusting_link_trail_data(
        links_state_data
    ) {
        this.thrusting_link_trails_positions_data = [];
        this.thrusting_link_trails_thrust_activated_data = [];
        this.thrusting_link_trails_age_data = [];
        for (let i = 0, l = links_state_data.length, c = 10 ; i < l ; i += c) {
            this.thrusting_link_trails_positions_data.push(
                // First triangle
                links_state_data[i+2], links_state_data[i+3],
                links_state_data[i], links_state_data[i+1],
                links_state_data[i+4], links_state_data[i+5],
                // Second triangle
                links_state_data[i+4], links_state_data[i+5],
                links_state_data[i+2], links_state_data[i+3],
                links_state_data[i+6], links_state_data[i+7]
            );
            this.thrusting_link_trails_thrust_activated_data.push(
                links_state_data[i+8],
                links_state_data[i+8],
                links_state_data[i+8],
                links_state_data[i+8],
                links_state_data[i+8],
                links_state_data[i+8]
            );
            this.thrusting_link_trails_age_data.push(
                links_state_data[i+9],
                links_state_data[i+9],
                links_state_data[i+9],
                links_state_data[i+9],
                links_state_data[i+9],
                links_state_data[i+9]
            );
        }
    }

    //
    // Draw thrusting links trails
    //
    draw_thrusting_links_trails(
        positions_data,
        universe_width,
        universe_height,
        thrust_activated_data,
        age_data
    ) {
        const size = 2;
        const type = this.webgl_context.FLOAT;   // the data is 32bit floats
        const normalize = false; // don't normalize the data
        const stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
        const offset = 0;        // start at the beginning of the buffer
        const data_count = positions_data.length / size;
        // bind the vertex array for that thing : call gl.bindVertexArray
        this.webgl_context.bindVertexArray(this.thrusting_links_trails_vao);
        // for each attribute call gl.bindBuffer, bufferData, gl.vertexAttribPointer, gl.enableVertexAttribArray
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.thrusting_links_trails_position_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(positions_data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.thrusting_links_trails_position_attribute_location,
            size,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.thrusting_links_trails_position_attribute_location
        );
        //
        // Links thrust activated
        //
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.thrusting_links_trails_thrust_activated_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(thrust_activated_data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.thrusting_links_trails_thrust_activated_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.thrusting_links_trails_thrust_activated_attribute_location
        );
        //
        // Links thrust age
        //
        this.webgl_context.bindBuffer(
            this.webgl_context.ARRAY_BUFFER,
            this.thrusting_links_trails_age_buffer
        );
        this.webgl_context.bufferData(
            this.webgl_context.ARRAY_BUFFER,
            new Float32Array(age_data),
            this.webgl_context.STATIC_DRAW
        );
        this.webgl_context.vertexAttribPointer(
            this.thrusting_links_trails_age_attribute_location,
            1,
            type,
            normalize,
            stride,
            offset
        );
        this.webgl_context.enableVertexAttribArray(
            this.thrusting_links_trails_age_attribute_location
        );
        // call gl.useProgram for the program needed to draw.
        this.webgl_context.useProgram(this.thrusting_links_trails_program);
        // setup uniforms for the thing you want to draw
        this.webgl_context.uniform2f(
            this.thrusting_links_trails_resolution_uniform_location,
            universe_width * 0.5,
            universe_height * 0.5
        );
        // call gl.drawArrays
        this.webgl_context.drawArrays(this.webgl_context.TRIANGLES, offset, data_count);
    }
}


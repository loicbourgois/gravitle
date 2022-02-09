import * as THREE from 'three';
import {
  last,
  len,
  assert,
} from "../util";
import {
  update_fps,
} from "../renderer_util";
const TM = 1.0
const LIMIT = 3000
function start(a) {
  console.log("starting")
  const camera = new THREE.PerspectiveCamera(a.fov, window.innerWidth / window.innerHeight, a.clipping.near, a.clipping.far);
  camera.position.x = 0.5;
  camera.position.y = 0.5;
  camera.position.z = 0.5;
  const renderer = new THREE.WebGLRenderer();
  renderer.setSize( window.innerWidth, window.innerHeight );
  document.body.appendChild( renderer.domElement );
  const scene = new THREE.Scene();
  const light = new THREE.PointLight( 0xffffff, 0.8, 100 );
  light.position.set(
    camera.position.x,
  camera.position.y,
    camera.position.z,
  );
  scene.add( light );
  const minimap = document.getElementById("minimap");
  minimap.width  = a.image_width;
  minimap.height = a.image_height;
  const ctx_minimap = minimap.getContext("2d");
  animate({
    scene: scene,
    renderer:renderer,
    camera:camera,
    minimap: minimap,
    ctx_minimap:ctx_minimap,
    pull: a.pull,
    fps_counter: [],
    fps_counter_length: 100,
    materials: {
      //'default': new THREE.MeshBasicMaterial( { color: 0xeeee00, wireframe: true } )
      //'default': new THREE.MeshDepthMaterial(),
      'default': new THREE.MeshLambertMaterial({ color: 0xeeee00 }),
      // MeshLambertMaterial
    },
    meshes: []
  });
}
function animate(a) {
  requestAnimationFrame( function () {
    animate(a)
  });
  const start = performance.now();
  // a.camera.rotation.z += 0.001;
  a.camera.rotation.y += 0.0003;
  const server_data = a.pull();
  let meshes = [];
  if (len(server_data)) {
    const d = JSON.parse(server_data)
    if (a.geometry === undefined && d.diameter) {
      a.geometry = new THREE.IcosahedronGeometry(d.diameter * 0.5, 4)
    }

    if (d.step) {
      document.getElementById("p_step").innerHTML = `Step: ${ d.step } `
    }
    document.getElementById("p_pids").innerHTML = `Particles: ${ d.pids.length } `
    a.ctx_minimap.clearRect(0, 0, a.minimap.width, a.minimap.height);
    let imesh = 0;
    for (let i in d.parts) {
      let p = d.parts[i];
      // a.ctx_minimap.beginPath();
      // a.ctx_minimap.arc(
      //   p.x * a.minimap.width,
      //   a.minimap.height - p.y * a.minimap.height,
      //   p.d * a.minimap.width * 0.5,
      //   0, 2 * Math.PI);
      // a.ctx_minimap.fill();
      if (imesh < LIMIT) {
        if (a.meshes.length == imesh) {
          a.meshes.push(new THREE.Mesh( a.geometry, a.materials['default'] ));
          a.scene.add( a.meshes[imesh] );
        }
        a.meshes[imesh].position.x = p.x * TM
        a.meshes[imesh].position.y = p.y * TM
        a.meshes[imesh].position.z = p.z * TM
        imesh++;
      }
    }
    for (let i = imesh ; i < a.meshes.length ; i++) {
      a.meshes[i].position.x = 0.0
      a.meshes[i].position.y = 0.0
      a.meshes[i].position.z = 0.0
    }
  }
  a.renderer.render( a.scene, a.camera );

  update_fps(a)
  const end = performance.now();
  a.fps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
}
export {
  start
}

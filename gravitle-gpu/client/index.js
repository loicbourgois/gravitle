'use strict';
{
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const vworld_urls = urlParams.get('urls')
  if (vworld_urls) {
    vworld_urls.split(";").forEach((vworld_server_url) => {
      conf.urls.unshift(vworld_server_url)
    })
  }
}
let parse_chunk_json = true;
const data = {
  socket_pairs: []
}
for (let i = 0 ; i < conf.urls.length ; i+=1 ) {
  var x = document.getElementById("chunk_select");
  var option = document.createElement("option");
  option.text = conf.urls[i];
  option.value = conf.urls[i];
  x.add(option);
}
const connect = () => {
  try {
    const chunk_select = document.getElementById("chunk_select");
    const url = chunk_select.options[chunk_select.selectedIndex].value;
    log(`Connecting to ${url}`)
    while (data.socket_pairs.length >= 1) {
      data.socket_pairs[0].reader.close()
      data.socket_pairs[0].writer.close()
      data.socket_pairs.shift()
    }
    let reader = new WebSocket(url);
    let writer = new WebSocket(url);
    let latency_checker = new WebSocket(url);
    data.socket_pairs.push({
      'reader': reader,
      'writer': writer,
      'latency_checker': latency_checker,
      'latency_ms': [],
    })
    setup_socket_pair(data.socket_pairs[data.socket_pairs.length - 1])
  } catch(error) {
    console.error(error)
  }
}
document.getElementById("chunk_select").addEventListener('change', (event) => {
  connect()
});
let logged_count = 0;
const log_x_time = (x, message) => {
  if (logged_count < x) {
    log(message);
  }
  logged_count += 1;
}
const log = (message) => {
  const textarea_logs = document.getElementById('logs')
  textarea_logs.value += message + '\n';
  textarea_logs.scrollTop = textarea_logs.scrollHeight;
}
const setup_socket_pair = (socket_pair) => {
  socket_pair.reader.addEventListener('open', function (event) {
      socket_pair.reader.send('server_to_client')
      log(`[reader] connected: ${socket_pair.reader.url}`)
      start_render_loop()
      window.onbeforeunload = function() {
          socket_pair.reader.onclose = function () {};
          socket_pair.reader.close();
      };
  });
  socket_pair.reader.addEventListener('close', function (event) {
      log(`[reader] connection closed: ${socket_pair.reader.url}`)
  });
  socket_pair.reader.addEventListener('error', function (event) {
      console.log('[reader] error')
  });
  socket_pair.reader.addEventListener('message', (event) => {
    if (parse_chunk_json == true) {
      chunk = JSON.parse(event.data)
      parse_chunk_json = false
      setTimeout(() => {
        parse_chunk_json = true
      }, 10)
    }
  });
  socket_pair.writer.addEventListener('open', function (event) {
      socket_pair.writer.send('writer')
      log(`[writer] connected: ${socket_pair.writer.url}`)
      window.onbeforeunload = function() {
          socket_pair.writer.onclose = function () {};
          socket_pair.writer.close();
      };
  });
  socket_pair.writer.addEventListener('close', function (event) {
      log(`[writer] connection closed: ${socket_pair.writer.url}`)
  });
  socket_pair.writer.addEventListener('error', function (event) {
      console.log('[writer] error')
  });
  socket_pair.writer.addEventListener('message', (event) => {
    // do nothing
  });
  socket_pair.latency_checker.addEventListener('open', function (event) {
      socket_pair.latency_checker.send('latency_checker')
      log(`[checker] connected: ${socket_pair.latency_checker.url}`)
      window.onbeforeunload = function() {
          socket_pair.latency_checker.onclose = function () {};
          socket_pair.latency_checker.close();
      };
      check_latency_start(socket_pair)
  });
  socket_pair.latency_checker.addEventListener('close', function (event) {
      log(`[checker] connection closed: ${socket_pair.latency_checker.url}`)
  });
  socket_pair.latency_checker.addEventListener('error', function (event) {
      console.log('[checker] error')
  });
  socket_pair.latency_checker.addEventListener('message', (event) => {
    check_latency_end(socket_pair)
  });
}
const check_latency_start = (socket_pair) => {
  socket_pair.check_start_ms = Date.now()
  socket_pair.latency_checker.send('check')
}
const check_latency_end = (socket_pair) => {
  socket_pair.check_end_ms = Date.now()
  socket_pair.latency_ms.push((socket_pair.check_end_ms - socket_pair.check_start_ms) / 2)
  while (socket_pair.latency_ms.length > 10) {
    socket_pair.latency_ms.shift()
  }
  const average_latency = socket_pair.latency_ms.reduce((a,b) => (a+b)) / socket_pair.latency_ms.length
  document.getElementById('latency').innerHTML = average_latency.toFixed(0) + " ms";
  setTimeout(() => {
    check_latency_start(socket_pair)
  }, 100)
}
const start_render_loop = () => {
  log(`starting rendering`)
  render_loop()
  //start_render_loop_gl();
}
let last_render_start_ms = Date.now();
let a = 0;
const render_loop = () => {
  last_render_start_ms = Date.now();
  // TODO
  // render_gl()
  //if (!a) {
    render()
  //} else {
    a = (a+1) % 2;
  //}
  requestAnimationFrame(render_loop)
  //setTimeout(render_loop, 0)
  //setTimeout(render_loop, 15 - (Date.now()-last_render_start_ms))
}
const tohhmmssms = (duration_second) => {
    var sec_num = parseInt(duration_second, 10);
    var hours   = Math.floor(sec_num / 3600);
    var minutes = Math.floor((sec_num - (hours * 3600)) / 60);
    var seconds = sec_num - (hours * 3600) - (minutes * 60);
    var ms = parseFloat(duration_second, 10) - sec_num;
    if (hours   < 10) {hours   = "0"+hours;}
    if (minutes < 10) {minutes = "0"+minutes;}
    if (seconds < 10) {seconds = "0"+seconds;}
    return hours+':'+minutes+':'+seconds+'.'+ms;
}
const tohhmmss = (duration_second) => {
    var sec_num = parseInt(duration_second, 10);
    var hours   = Math.floor(sec_num / 3600);
    var minutes = Math.floor((sec_num - (hours * 3600)) / 60);
    var seconds = sec_num - (hours * 3600) - (minutes * 60);
    if (hours   < 10) {hours   = "0"+hours;}
    if (minutes < 10) {minutes = "0"+minutes;}
    if (seconds < 10) {seconds = "0"+seconds;}
    return hours+':'+minutes+':'+seconds;
}
let last_render_time_ms = Date.now();
let now_ms = Date.now();
let fps_list = []
let chunk = {}
//
const canvas_1 = document.querySelector("#canvas_1")
canvas_1.width = window.innerWidth
canvas_1.height = window.innerHeight
const context_1 = canvas_1.getContext('2d')
//
const canvas_2 = document.querySelector("#canvas_2")
canvas_2.width = 300;
canvas_2.height = canvas_2.width
const context_2 = canvas_2.getContext('2d')
let center_x = 0.5;
let center_y = 0.5
let mousedown = false
canvas_2.onmousedown = function(e){
  mousedown = true
  const p = get_canvas_cursor_position(canvas_2, e)
  center_x = p.x / canvas_2.width
  center_y = p.y / canvas_2.height
}
canvas_2.onmousemove = function(e){
  if (mousedown) {
    const p = get_canvas_cursor_position(canvas_2, e)
    center_x = p.x / canvas_2.width
    center_y = p.y / canvas_2.height
  }
}
document.body.onmouseup = function(e){
  mousedown = false
}
/*const canvas_3 = document.querySelector("#canvas_3")
canvas_3.width = 250;
canvas_3.height = canvas_3.width
const context_3 = canvas_3.getContext('2d')
const canvas_4 = document.querySelector("#canvas_4")
canvas_4.width = 250;
canvas_4.height = canvas_4.width
const context_4 = canvas_4.getContext('2d')*/
document.getElementById('show_health').checked = true
document.querySelector('#use_distance_traveled_as_fitness_function').addEventListener('click', (event) => {
  data.socket_pairs[0].writer.send('use_distance_traveled_as_fitness_function')
});
document.querySelector('#use_distance_traveled_as_fitness_function_false').addEventListener('click', (event) => {
  data.socket_pairs[0].writer.send('use_distance_traveled_as_fitness_function_false')
});
const bindings = {
  // key: pid
  "a": [0],
  "z": [1],
}
const set_activation = (pid, value) => {
  let command_data = {
    "SetActivation": {
      "pid": pid,
      "value": value,
    }
  }
  let json_str = JSON.stringify(command_data);
  data.socket_pairs[0].writer.send(json_str);
}
const keyup = (e) => {
    if (bindings && bindings[e.key]) {
      for (const i in bindings[e.key]) {
        set_activation(bindings[e.key][i], 0.0);
      }
    } else {
        // Do nothing
    }
};
const keydown = (e) => {
    if (bindings && bindings[e.key]) {
      for (const i in bindings[e.key]) {
        set_activation(bindings[e.key][i], 1.0);
      }
    } else {
        // Do nothing
    }
};
document.addEventListener('keyup', keyup);
document.addEventListener('keydown', keydown);
connect()

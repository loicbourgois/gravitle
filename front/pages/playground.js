import {
  player_id
} from "../util"

function playground() {
  let socket = new WebSocket("ws://127.0.0.1:8000/ws");
  let data = {};
  socket.onopen = function(e) {
    console.log("[open] Connection established");
    console.log("Sending to server");
    socket.send(JSON.stringify({
      'request': 'create_sender',
      'uuid': player_id()
    }));
  };
  socket.onmessage = function(event) {
    data = event.data;
  };
  socket.onclose = function(event) {
    if (event.wasClean) {
      console.error(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      console.error('[close] Connection died');
    }
  };
  socket.onerror = function(error) {
    console.error(`[error] ${error.message}`);
  };
}
export {
  playground
}

console.log("plouf")


const socket = new WebSocket('ws://localhost:8080');

// Connection opened
socket.addEventListener('open', (event) => {
    socket.send('Hello Server!');
});

socket.binaryType = "arraybuffer";

// Listen for messages
socket.addEventListener('message', (event) => {
  if (event.data instanceof ArrayBuffer) {
    // binary frame
    const view = new DataView(event.data);
    const usize_length = view.getInt8(0);
    let step = null
    let elapsed = null
    if (usize_length == 8) {
      step = view.getBigInt64(1)
      elapsed = view.getInt32(9)
    }
    document.body.innerHTML = `
      <p>bytes: ${event.data.byteLength}</p>
      <p>usize_length: ${usize_length}</p>
      <p>step: ${step}</p>
      <p>elapsed: ${elapsed} μs</p>
    `
  } else {
    // text frame
    console.log(event.data);
  }
});

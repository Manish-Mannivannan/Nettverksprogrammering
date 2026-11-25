const net = require("net");
const crypto = require("crypto");

const connections = [];
const messages = [];

const wsServer = net.createServer((connection) => {
  console.log("Client connected");

  connection.on("data", (data) => {
    if (data.toString().split("\n")[0].includes("GET / HTTP")) {
      const info = data.toString().split("\n");
      var result = "";
      for (let i = 0; i < info.length; i++) {
        if (info[i].includes("Sec-WebSocket-Key")) {
          result = info[i];
        }
      }

      const clientKey = result.split(":")[1].trim();
      const magicString = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
      const serverKey = crypto
        .createHash("sha1")
        .update(clientKey + magicString)
        .digest("base64");

      const response =
        `HTTP/1.1 101 Switching Protocols\r\n` +
        `Upgrade: websocket\r\n` +
        `Connection: Upgrade\r\n` +
        `Sec-WebSocket-Accept: ${serverKey}\r\n\r\n`;

      connection.write(response);
      connections.push(connection);
      for (let i = 0; i < messages.length; i++) {
        connection.write(createWebSocketFrame(messages[i]));
      }
    } else {
      let length = data[1] & 127;
      let maskStart = 2;
      let dataStart = maskStart + 4;
      let message = "";
      for (let i = dataStart; i < dataStart + length; i++) {
        let byte = data[i] ^ data[maskStart + ((i - dataStart) % 4)];
        message += String.fromCharCode(byte).toUpperCase();
      }
      try {
        const sendMessage = JSON.parse(message).text;
        messages.push(sendMessage);
        const messageToClient = createWebSocketFrame(sendMessage);
        for (let index = 0; index < connections.length; index++) {
          connections[index].write(messageToClient);
        }
      } catch (error) {
        console.error(error);
      }
    }
  });

  connection.on("end", () => {
    console.log("Client disconnected");
  });
});

wsServer.on("error", (error) => {
  console.error("Error: ", error);
});

wsServer.listen(3001, () => {
  console.log("WebSocket server listening on port 3001");
});

function createWebSocketFrame(message) {
  const msg = Buffer.from(message);
  const msgSize = msg.length;

  const header = Buffer.alloc(2);
  header.writeUInt8(0x81, 0);
  header.writeUInt8(msgSize, 1);

  const frame = Buffer.concat([header, msg]);
  return frame;
}

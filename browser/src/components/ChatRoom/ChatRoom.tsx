import React, { useState, useEffect } from "react";
import { MessageInput } from "../MessageInput/MessageInput";
import { MessageView } from "../MessageView/MessageView";
import { RoomHeader } from "../RoomHeader/RoomHeader";
import type { Message } from "../../types/types";
import "./ChatRoom.css";

const generateRandomName = (): string => {
  return "defaultName";
};

const ChatRoom: React.FC<{
  initialRoomId: string;
  wsUrl: string;
}> = ({ initialRoomId, wsUrl }) => {
  const [name, setName] = useState(generateRandomName());
  const [roomId, setRoomId] = useState(initialRoomId);
  const [messages, setMessages] = useState<Message[]>([]);
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [isSocketReady, setIsSocketReady] = useState(false);

  useEffect(() => {
    console.log("Attempting connection to WebSocket");
    const newSocket = new WebSocket(wsUrl);
    newSocket.onopen = function (event) {
      console.log("Connected to WebSocket");
      setIsSocketReady(true);
    };
    newSocket.onmessage = function (event) {
      const newMessage: Message = JSON.parse(event.data);
      console.log("Received message:", newMessage);
      setMessages((prevMessages) => {
        return [...prevMessages, newMessage];
      });
    };
    newSocket.onclose = function (event) {
      console.log("Disconnected from WebSocket");
      setIsSocketReady(false);
    };
    setSocket(newSocket);
    return () => {
      newSocket.close();
    };
  }, [wsUrl]);

  useEffect(() => {
    if (socket === null || !isSocketReady) {
      console.log("Socket not ready");
      return;
    }
    console.log(`RoomId:${roomId}`);
    console.log(`Name:${name}`);
    socket.send(`UserUpdate:RoomId=${roomId}&Name=${name}`);
  }, [isSocketReady, roomId, name]);

  const handleSendMessage = (message: string) => {
    if (socket === null || !isSocketReady) {
      return;
    }
    socket.send(message);
  };

  return (
    <div className="room">
      <RoomHeader roomId={roomId} defaultName={name} onNameChange={setName} />
      <MessageView messages={messages} />
      <MessageInput isEnabled={isSocketReady} onSend={handleSendMessage} />
    </div>
  );
};

export default ChatRoom;

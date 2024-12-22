import React from "react";
import "./MessageView.css";
import type { Message } from "../../types/types";

export const MessageView: React.FC<{ messages: Message[] }> = ({
  messages,
}) => {
  return (
    <div className="chat-messages" id="chat-messages">
      {messages.map((msg, index) => (
        <div className="message-container" key={index}>
          <span className="message-author">{msg.author_name}</span>
          <span className="message-text">{msg.text}</span>
        </div>
      ))}
    </div>
  );
};

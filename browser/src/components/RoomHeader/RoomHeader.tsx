import React from "react";
import { CamouflageInputText } from "../CamouflageInputText/CamouflageInputText";
import "./RoomHeader.css";

export const RoomHeader: React.FC<{
  roomId: string;
  defaultName: string;
  onNameChange: (value: string) => void;
}> = ({ roomId, defaultName, onNameChange }) => {
  return (
    <div className="room-header">
      <h1>Room {roomId}</h1>
      Your name is&nbsp;
      <CamouflageInputText
        defaultValue={defaultName}
        onValueCommitted={onNameChange}
      />
    </div>
  );
};

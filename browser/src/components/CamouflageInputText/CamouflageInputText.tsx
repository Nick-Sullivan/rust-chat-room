import React, { useState } from "react";
import "./CamouflageInputText.css";

export const CamouflageInputText: React.FC<{
  defaultValue: string;
  onValueCommitted: (value: string) => void;
}> = ({ defaultValue, onValueCommitted }) => {
  const [value, setValue] = useState(defaultValue);
  const [isEditing, setIsEditing] = useState(false);

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsEditing(true);
  };
  const handleValueChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };
  const handleCommit = () => {
    onValueCommitted(value);
    setIsEditing(false);
  };
  const handleOnKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      (e.target as HTMLInputElement).blur();
    }
  };

  return (
    <input
      id="name-input"
      className={`editable ${isEditing ? "editing" : ""}`}
      value={value}
      onChange={handleValueChange}
      onFocus={handleFocus}
      onBlur={handleCommit}
      onKeyDown={handleOnKeyDown}
      spellCheck="false"
    />
  );
};

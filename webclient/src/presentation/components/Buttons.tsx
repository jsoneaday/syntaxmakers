import React, { ReactNode } from "react";
import "../theme/buttons.css";

export interface ButtonProps {
  children: ReactNode;
  containerStyle?: object;
  txtStyle?: object;
  onClick?: (e: React.MouseEvent<HTMLButtonElement>) => void;
  disabled?: boolean;
}

export function PrimaryButton({
  children,
  containerStyle = {},
  onClick,
  disabled = false,
}: ButtonProps) {
  return (
    <button
      onClick={onClick}
      className="primary-btn"
      style={{ ...containerStyle, opacity: !disabled ? 1 : 0.5 }}
      disabled={disabled}
    >
      {children}
    </button>
  );
}

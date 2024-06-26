import { ReactNode, MouseEvent } from "react";
import "../../theme/buttons.css";

export interface ButtonProps {
  children: ReactNode;
  containerStyle?: object;
  txtStyle?: object;
  onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
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

export interface SecondaryButtonProps {
  children: ReactNode;
  onClick: (e: MouseEvent<HTMLButtonElement>) => void;
  disabled?: boolean;
}
export function SecondaryButton({
  children,
  onClick,
  disabled = false,
}: SecondaryButtonProps) {
  return (
    <button className="secondary-btn" disabled={disabled} onClick={onClick}>
      {children}
    </button>
  );
}

import { ReactNode, MouseEvent, CSSProperties } from "react";
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
      style={{
        ...containerStyle,
        opacity: !disabled ? 1 : 0.5,
        cursor: disabled ? "not-allowed" : "pointer",
      }}
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
  style?: CSSProperties;
}
export function SecondaryButton({
  children,
  onClick,
  disabled = false,
  style = {},
}: SecondaryButtonProps) {
  return (
    <button
      className="secondary-btn"
      style={{
        ...style,
        opacity: !disabled ? 1 : 0.5,
        cursor: disabled ? "not-allowed" : "pointer",
      }}
      disabled={disabled}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

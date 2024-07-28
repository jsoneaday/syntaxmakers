import { ReactNode } from "react";
import Modal from "../Modal";

interface PopupProps {
  children: ReactNode;
  toggleOpen: () => void;
  isOpen: boolean;
}

export function Popup({ children, toggleOpen, isOpen }: PopupProps) {
  return (
    <Modal
      isOpen={isOpen}
      toggleOpen={toggleOpen}
      overlayClickClose={true}
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        height: "50px",
      }}
    >
      {children}
    </Modal>
  );
}

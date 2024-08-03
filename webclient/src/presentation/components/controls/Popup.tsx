import { ReactNode } from "react";
import Modal from "../Modal";

interface PopupProps {
  children: ReactNode;
  toggleOpen: () => void;
  isOpen: boolean;
}

export function Popup({ children, toggleOpen, isOpen }: PopupProps) {
  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen} overlayClickClose={true}>
      {children}
    </Modal>
  );
}

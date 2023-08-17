import { ReactNode } from "react";
import ReactModal from "react-modal";
import "../theme/modal.css";

interface ModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
  children: ReactNode;
}

export default function Modal({ isOpen, toggleOpen, children }: ModalProps) {
  return (
    <ReactModal
      className="modal-container"
      isOpen={isOpen}
      onRequestClose={toggleOpen}
    >
      {children}
    </ReactModal>
  );
}

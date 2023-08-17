import { ReactNode } from "react";
import ReactModal from "react-modal";
import "../theme/modal.css";

interface ModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
  children: ReactNode;
  overlayClickClose?: boolean;
}

export default function Modal({
  isOpen,
  toggleOpen,
  children,
  overlayClickClose = false,
}: ModalProps) {
  return (
    <ReactModal
      className="modal-container"
      isOpen={isOpen}
      onRequestClose={toggleOpen}
      shouldCloseOnOverlayClick={overlayClickClose}
    >
      {children}
    </ReactModal>
  );
}

import { ReactNode } from "react";
import * as ReactModal from "react-modal"; // needed to pass jest tests
import "../../theme/modal.css";

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

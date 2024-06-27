import { ReactNode } from "react";
import * as ReactModal from "react-modal"; // needed to pass jest tests
import "../theme/modal.css";

interface ModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
  children: ReactNode;
  overlayClickClose?: boolean;
  style?: ReactModal.Styles;
}

export default function Modal({
  isOpen,
  toggleOpen,
  children,
  overlayClickClose = false,
  style = {},
}: ModalProps) {
  return (
    <ReactModal
      style={style}
      className="modal-container"
      isOpen={isOpen}
      onRequestClose={toggleOpen}
      shouldCloseOnOverlayClick={overlayClickClose}
    >
      {children}
    </ReactModal>
  );
}

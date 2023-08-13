import { ReactNode } from "react";
import ReactModal from "react-modal";

interface ModalProps {
  isOpen: boolean;
  onRequestClose: () => void;
  children: ReactNode;
}

export default function Modal({
  isOpen,
  onRequestClose,
  children,
}: ModalProps) {
  return (
    <ReactModal
      isOpen={isOpen}
      onRequestClose={onRequestClose}
      shouldCloseOnOverlayClick={true}
    >
      {children}
    </ReactModal>
  );
}

import { ReactNode } from "react";
import ReactModal from "react-modal";

interface ModalProps {
  children: ReactNode;
}

export default function Modal({ children }: ModalProps) {
  return (
    <ReactModal isOpen={} onRequestClose={} shouldCloseOnOverlayClick={}>
      {children}
    </ReactModal>
  );
}

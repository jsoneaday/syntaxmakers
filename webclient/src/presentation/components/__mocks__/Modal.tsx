import { ReactNode } from "react";
import "../theme/modal.css";

interface ModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
  children: ReactNode;
  overlayClickClose?: boolean;
}

export default function MockModal({ children }: ModalProps) {
  return <div className="modal-container">{children}</div>;
}

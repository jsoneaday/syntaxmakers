import "../../theme/login.css";
import Modal from "../Modal";

interface LoginProps {
  isOpen: boolean;
  onRequestClose: () => void;
}

export default function Login({ isOpen, onRequestClose }: LoginProps) {
  return (
    <Modal isOpen={isOpen} onRequestClose={onRequestClose}>
      <form className="login-form">
        <div className="login-item">
          <span>Email</span>
          <input type="text" />
        </div>
        <div className="login-item">
          <span>Email</span>
          <input type="text" />
        </div>
      </form>
    </Modal>
  );
}

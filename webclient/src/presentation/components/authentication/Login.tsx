import "../../theme/login.css";
import Modal from "../Modal";

export default function Login() {
  return (
    <Modal>
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

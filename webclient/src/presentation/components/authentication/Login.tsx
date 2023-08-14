import { login } from "../../../domain/repository/AuthRepo";
import { getDeveloper } from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import "../../theme/login.css";
import { PrimaryButton } from "../Buttons";
import Modal from "../Modal";
import { convert as convertDev } from "../../models/DevProfile";

interface LoginProps {
  isDevOrEmployer: DevOrEmployer;
  isOpen: boolean;
  toggleOpen: () => void;
}

export default function Login({
  isDevOrEmployer,
  isOpen,
  toggleOpen,
}: LoginProps) {
  const [_profile, setProfile] = useProfile();

  const onClickLogin = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    login(isDevOrEmployer, "jon@jon.com", "test123")
      .then((id: BigInt) => {
        console.log("login success id:", id.toString());
        getDeveloper(id.toString())
          .then((dev) => {
            setProfile(dev ? convertDev(dev) : null);
          })
          .catch((error) => {
            console.log("Developer: failed to get developer", error);
          });
      })
      .catch((err) => {
        console.log("login", err);
      });
  };

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <form className="login-form">
        <div className="login-item">
          <span className="login-label">Email</span>
          <input className="login-input normal-font" type="text" />
        </div>
        <div className="login-item">
          <span className="login-label">Password</span>
          <input className="login-input normal-font" type="text" />
        </div>
        <div className="login-item">
          <PrimaryButton onClick={onClickLogin}>Login</PrimaryButton>
        </div>
      </form>
    </Modal>
  );
}

import { login } from "../../../domain/repository/AuthRepo";
import { getDeveloper } from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import "../../theme/login.css";
import { PrimaryButton } from "../controls/Buttons";
import Modal from "../Modal";
import { convert as convertDev } from "../../models/DevProfile";
import Checkbox from "../controls/Checkbox";
import { useState } from "react";

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
  const [email, setEmail] = useState("jon@jon.com");
  const [password, setPassword] = useState("test123");

  const onClickLogin = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    login(isDevOrEmployer, email, password)
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

  const onChangeEmail = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const onChangePassword = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <form className="login-form">
        <div className="login-item" style={{ marginBottom: "2em" }}>
          <span className="title-font">Welcome to SyntaxMakers</span>
          <span
            className="sub-title-font"
            style={{ color: "var(--primary-font-cl)" }}
          >
            Please login
          </span>
        </div>
        <div className="login-item">
          <span className="login-label">Email</span>
          <input
            className="login-input normal-font"
            type="text"
            value={email}
            onChange={onChangeEmail}
          />
        </div>
        <div className="login-item">
          <span className="login-label">Password</span>
          <input
            className="login-input normal-font"
            type="password"
            value={password}
            onChange={onChangePassword}
          />
        </div>
        <div className="login-item-row">
          <Checkbox>
            <span className="small-font">Remember me</span>
          </Checkbox>
          <span className="small-font">Forgot password</span>
        </div>
        <div className="login-item">
          <PrimaryButton onClick={onClickLogin}>Login</PrimaryButton>
        </div>
      </form>
    </Modal>
  );
}

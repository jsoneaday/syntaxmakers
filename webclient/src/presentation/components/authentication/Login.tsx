import { LoginResult, login } from "../../../domain/repository/AuthRepo";
import { getDeveloperByEmail } from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import "../../theme/login.css";
import { PrimaryButton } from "../controls/Buttons";
import Modal from "../Modal";
import { convert as convertDev } from "../../models/DevProfile";
import { convert as convertEmp } from "../../models/EmpProfile";
import Checkbox from "../controls/Checkbox";
import { useEffect, useState, useTransition } from "react";
import { useDevOrEmployer } from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import { getEmployerByEmail } from "../../../domain/repository/EmployerRepo";

interface LoginProps {
  isOpen: boolean;
  toggleOpen: () => void;
}

export default function Login({ isOpen, toggleOpen }: LoginProps) {
  const [_profile, setProfile] = useProfile();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("test123");
  const [_isPending, startTransition] = useTransition();
  const [errorMessage, setErrorMessage] = useState("");
  const [devOrEmp, _setDevOrEmp] = useDevOrEmployer();
  const [isRemote, setIsRemote] = useState(false);

  useEffect(() => {
    // todo: remove hard codings when ready
    if (devOrEmp === DevOrEmployer.Developer) {
      setEmail("jon@jon.com");
    } else {
      setEmail("jon@acmecorp.com");
    }
  }, [devOrEmp]);

  const onClickLogin = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    login(devOrEmp, email, password)
      .then(({ message, status }: LoginResult) => {
        console.log("token", message);
        if (status === 200) {
          if (devOrEmp === DevOrEmployer.Developer) {
            getDeveloperByEmail(email, message)
              .then((dev) => {
                if (dev) {
                  const profile = convertDev(dev, message);

                  startTransition(() => {
                    setProfile(profile);
                    toggleOpen();
                  });
                } else {
                  setProfile(null);
                  setErrorMessage(`Failed to find user with email ${email}`);
                }
              })
              .catch((error) => {
                console.log("Developer: failed to get developer", error);
              });
          } else {
            getEmployerByEmail(email, message)
              .then((emp) => {
                if (emp) {
                  const profile = convertEmp(emp, message);

                  setProfile(profile);
                  toggleOpen();
                } else {
                  setProfile(null);
                  setErrorMessage(`Failed to find user with email ${email}`);
                }
              })
              .catch((error) => {
                console.log("Developer: failed to get developer", error);
              });
          }
        } else {
          setErrorMessage(
            status === 401
              ? "Login failed. Invalid email or password"
              : "Login has failed. Please try again"
          );
        }
      })
      .catch((err) => {
        console.log("login", err);
        setErrorMessage(err);
      });
  };

  const onChangeEmail = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const onChangePassword = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const toggleIsRemote = () => {
    setIsRemote(!isRemote);
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
            className="input normal-font"
            type="text"
            value={email}
            onChange={onChangeEmail}
          />
        </div>
        <div className="login-item">
          <span className="login-label">Password</span>
          <input
            className="input normal-font"
            type="password"
            value={password}
            onChange={onChangePassword}
          />
        </div>
        <div className="login-item-row">
          <Checkbox isChecked={isRemote} toggleIsChecked={toggleIsRemote}>
            <span className="small-font">Remember me</span>
          </Checkbox>
          <span className="small-font">Forgot password</span>
        </div>
        <div className="login-item">
          <PrimaryButton onClick={onClickLogin}>Login</PrimaryButton>
        </div>
        <div className="login-item">
          <span>{errorMessage}</span>
        </div>
      </form>
    </Modal>
  );
}

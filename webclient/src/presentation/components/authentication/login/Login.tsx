import {
  DevOrEmployer,
  LoginResult,
  forgotPassword,
  login,
} from "../../../../domain/repository/AuthRepo";
import { getDeveloperByEmail } from "../../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import "../../../theme/login.css";
import { PrimaryButton } from "../../controls/Buttons";
import { convert as convertDev } from "../../../models/DevProfile";
import { convert as convertEmp } from "../../../models/EmpProfile";
import { ChangeEvent, MouseEvent, useState, useTransition } from "react";
import { useDevOrEmployer } from "../../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { UiDevOrEmployer } from "../../../models/DevOrEmployer";
import { getEmployerByEmail } from "../../../../domain/repository/EmployerRepo";
import { ValidationMsgView } from "../../controls/ValidationMsgView";

interface LoginProps {
  userType: UiDevOrEmployer;
  toggleOpen: () => void;
}

export default function Login({ userType, toggleOpen }: LoginProps) {
  const [isLogin, setIsLogin] = useState(true);

  const toggleIsLogin = () => {
    setIsLogin(!isLogin);
  };

  return (
    <form className="login-form">
      {isLogin ? (
        <LoginView toggleOpen={toggleOpen} toggleIsLogin={toggleIsLogin} />
      ) : (
        <ForgotPassword userType={userType} toggleIsLogin={toggleIsLogin} />
      )}
    </form>
  );
}

interface LoginViewProps {
  toggleOpen: () => void;
  toggleIsLogin: () => void;
}

function LoginView({ toggleOpen, toggleIsLogin }: LoginViewProps) {
  const [_profile, setProfile] = useProfile();
  const [devOrEmp, _setDevOrEmp] = useDevOrEmployer();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [_isPending, startTransition] = useTransition();
  const [errorMessage, setErrorMessage] = useState("");

  const onClickLogin = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    login(
      devOrEmp === UiDevOrEmployer.Developer
        ? DevOrEmployer.Developer
        : DevOrEmployer.Employer,
      email,
      password
    )
      .then(({ message, status }: LoginResult) => {
        if (status === 200) {
          // when successful message is access_token!
          if (devOrEmp === UiDevOrEmployer.Developer) {
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
            status === 401 ? "Login failed. Invalid email or password" : message
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

  const onClickForgotPassword = () => {
    toggleIsLogin();
  };

  return (
    <>
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
        <button className="small-font" onClick={onClickForgotPassword}>
          Forgot password
        </button>
      </div>
      <div className="login-item" style={{ alignItems: "flex-end" }}>
        <PrimaryButton onClick={onClickLogin}>Login</PrimaryButton>
      </div>
      <div className="login-item">
        <span>{errorMessage}</span>
      </div>
    </>
  );
}

interface ForgotPasswordProps {
  userType: UiDevOrEmployer;
  toggleIsLogin: () => void;
}

function ForgotPassword({ userType, toggleIsLogin }: ForgotPasswordProps) {
  const [forgotPasswordEmail, setForgotPasswordEmail] = useState("");
  const [successMsg, setSuccessMsg] = useState("");
  const [validationMsg, setValidationMsg] = useState("");

  const onChangeForgotPwdEmail = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setForgotPasswordEmail(e.target.value);
  };

  const onClickForgotPassword = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    try {
      await forgotPassword(
        forgotPasswordEmail,
        userType === UiDevOrEmployer.Developer
          ? DevOrEmployer.Developer
          : DevOrEmployer.Employer
      );
      setValidationMsg("");
      setSuccessMsg(
        "If the email account exists a reset password email will be sent"
      );
    } catch (e) {
      if (e instanceof Error) {
        setValidationMsg(e.message);
        setSuccessMsg("");
      }
    }
  };

  return (
    <>
      <span className="title-font">Reset Password</span>
      <span
        className="sub-title-font"
        style={{ color: "var(--primary-font-cl)", marginBottom: "1em" }}
      >
        Provide your email
      </span>
      <input
        type="text"
        className="input normal-font input-spacing"
        value={forgotPasswordEmail}
        onChange={onChangeForgotPwdEmail}
      />
      <div
        style={{
          width: "100%",
          textAlign: "right",
          marginBottom: "1em",
        }}
      >
        <button
          className="secondary-btn small-btn"
          style={{ marginRight: "1em" }}
          onClick={toggleIsLogin}
        >
          Cancel
        </button>
        <PrimaryButton onClick={onClickForgotPassword}>Reset</PrimaryButton>
      </div>
      <ValidationMsgView
        successMessage={successMsg}
        validationMessage={validationMsg}
      />
    </>
  );
}

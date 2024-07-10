import { ReactNode, useEffect, useState } from "react";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import { startViewTransition } from "../common/transitions/ViewTransition";
import { useLocation } from "react-router-dom";
import { useDevOrEmployer } from "../common/redux/devOrEmployer/DevOrEmployerHooks";
import { UiDevOrEmployer } from "../models/DevOrEmployer";
import { DEV_ROUTE_PREFIX } from "../../App";
import { AuthModal } from "./authentication/AuthModal";
import { useProfile } from "../common/redux/profile/ProfileHooks";
import { ConfirmEmailModal } from "./authentication/ConfirmEmailModal";

interface LayoutProps {
  children: ReactNode;
  userType?: UiDevOrEmployer;
  includeLogin?: boolean;
  includeEmailIsConfirmedDialog?: boolean;
}

export default function Layout({
  children,
  userType,
  includeLogin = true,
  includeEmailIsConfirmedDialog = false,
}: LayoutProps) {
  const [profile] = useProfile();
  const [loginOpen, setLoginOpen] = useLoginOpen();
  const [_devOrEmp, setDevOrEmp] = useDevOrEmployer();
  const location = useLocation();
  const [confirmEmailOpen, setConfirmEmailOpen] = useState(
    includeEmailIsConfirmedDialog
  );

  useEffect(() => {
    if (!profile && includeLogin) {
      setLoginOpen(includeLogin);
    }
  }, [profile]);

  useEffect(() => {
    setDevOrEmp(
      location.pathname.includes(DEV_ROUTE_PREFIX)
        ? UiDevOrEmployer.Developer
        : UiDevOrEmployer.Employer
    );
  }, [location]);

  const toggleLoginOpen = () => {
    startViewTransition(() => setLoginOpen(!loginOpen));
  };

  const toggleConfirmEmailOpen = () => {
    setConfirmEmailOpen(!confirmEmailOpen);
  };

  return (
    <div className="layout-container">
      {includeEmailIsConfirmedDialog ? (
        <ConfirmEmailModal
          isOpen={confirmEmailOpen}
          toggleOpen={toggleConfirmEmailOpen}
        />
      ) : null}

      {includeLogin ? (
        <AuthModal
          isOpen={loginOpen}
          toggleOpen={toggleLoginOpen}
          userType={userType!}
        />
      ) : null}

      {children}
    </div>
  );
}

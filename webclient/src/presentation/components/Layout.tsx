import { ReactNode, useEffect } from "react";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import { startViewTransition } from "../common/transitions/ViewTransition";
import { useLocation } from "react-router-dom";
import { useDevOrEmployer } from "../common/redux/devOrEmployer/DevOrEmployerHooks";
import { UiDevOrEmployer } from "../models/DevOrEmployer";
import { DEV_ROUTE_PREFIX } from "../../App";
import { AuthModal } from "./authentication/AuthModal";
import { useProfile } from "../common/redux/profile/ProfileHooks";

interface LayoutProps {
  children: ReactNode;
  userType?: UiDevOrEmployer;
  includeLogin?: boolean;
}

export default function Layout({
  children,
  userType,
  includeLogin = true,
}: LayoutProps) {
  const [profile] = useProfile();
  const [loginOpen, setLoginOpen] = useLoginOpen();
  const [_devOrEmp, setDevOrEmp] = useDevOrEmployer();
  const location = useLocation();

  useEffect(() => {
    console.log("Layout profile", profile);

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

  return (
    <div className="layout-container">
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

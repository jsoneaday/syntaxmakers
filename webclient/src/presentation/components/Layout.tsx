import { ReactNode, useEffect } from "react";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import { startViewTransition } from "../common/transitions/ViewTransition";
import { useLocation } from "react-router-dom";
import { useDevOrEmployer } from "../common/redux/devOrEmployer/DevOrEmployerHooks";
import { DevOrEmployer } from "../models/DevOrEmployer";
import { DEV_ROUTE_PREFIX } from "../../App";
import { AuthModal } from "./authentication/AuthModal";

interface LayoutProps {
  children: ReactNode;
  includeLogin?: boolean;
}

export default function Layout({ children, includeLogin = true }: LayoutProps) {
  const [loginOpen, setLoginOpen] = useLoginOpen();
  const [_devOrEmp, setDevOrEmp] = useDevOrEmployer();
  const location = useLocation();

  useEffect(() => {
    setDevOrEmp(
      location.pathname.includes(DEV_ROUTE_PREFIX)
        ? DevOrEmployer.Developer
        : DevOrEmployer.Employer
    );
  }, [location]);

  const toggleLoginOpen = () => {
    startViewTransition(() => setLoginOpen(!loginOpen));
  };

  return (
    <div className="layout-container">
      {includeLogin ? (
        <AuthModal isOpen={loginOpen} toggleOpen={toggleLoginOpen} />
      ) : null}

      {children}
    </div>
  );
}

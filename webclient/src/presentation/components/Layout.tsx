import { ReactNode } from "react";
import Login from "../components/authentication/Login";
import { DevOrEmployer } from "../models/DevOrEmployer";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import { startViewTransition } from "../common/transitions/ViewTransition";

interface LayoutProps {
  children: ReactNode;
  includeLogin?: boolean;
}

export default function Layout({ children, includeLogin = true }: LayoutProps) {
  const [loginOpen, setLoginOpen] = useLoginOpen();

  const toggleLoginOpen = () => {
    startViewTransition(() => setLoginOpen(!loginOpen));
  };

  return (
    <div className="layout-container">
      {includeLogin ? (
        <Login
          devOrEmployer={DevOrEmployer.Developer}
          isOpen={loginOpen}
          toggleOpen={toggleLoginOpen}
        />
      ) : null}

      {children}
    </div>
  );
}

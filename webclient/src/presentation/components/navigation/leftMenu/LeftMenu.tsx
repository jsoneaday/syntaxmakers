import "../../../theme/left_menu.css";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import { startViewTransition } from "../../../common/transitions/ViewTransition";
import { ReactNode } from "react";

interface LeftMenuProps {
  children: ReactNode;
}
export default function LeftMenu({ children }: LeftMenuProps) {
  const [profile, setProfile] = useProfile();

  const onClickLogout = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    startViewTransition(() => setProfile(null));
  };

  if (profile) {
    return (
      <nav className="leftmenu-container">
        {children}
        <button className="secondary-btn" onClick={onClickLogout}>
          logout
        </button>
      </nav>
    );
  } else {
    return null;
  }
}

import "../../../theme/left_menu.css";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import { startViewTransition } from "../../../common/transitions/ViewTransition";
import { ReactNode, useEffect } from "react";
import { useLoginOpen } from "../../../common/redux/loginOpen/LoginOpenHooks";
import { SecondaryButton } from "../../controls/Buttons";

interface LeftMenuProps {
  children: ReactNode;
}
export default function LeftMenu({ children }: LeftMenuProps) {
  const [profile, setProfile] = useProfile();
  const [_loginOpen, setLoginOpen] = useLoginOpen();

  useEffect(() => {
    if (!profile) {
      startViewTransition(() => setLoginOpen(true));
    }
  }, []);

  const onClickLogout = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    setProfile(null);
    setLoginOpen(true);
  };

  if (profile) {
    return (
      <nav className="leftmenu-container">
        {children}
        <SecondaryButton
          style={{ margin: "1em", marginLeft: "2em" }}
          onClick={onClickLogout}
        >
          logout
        </SecondaryButton>
      </nav>
    );
  } else {
    return null;
  }
}

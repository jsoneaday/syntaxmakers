import "../../presentation/theme/left_menu.css";
import safebox from "../../presentation/theme/assets/safe-box.png";
import notification from "../../presentation/theme/assets/notification.png";
import { useProfile } from "../common/redux/profile/ProfileHooks";
import { startViewTransition } from "../common/transitions/ViewTransition";

export default function LeftMenu() {
  const [profile, setProfile] = useProfile();

  const onClickLogout = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    startViewTransition(() => setProfile(null));
  };

  if (profile) {
    return (
      <nav className="dev-menu-container">
        <div className="dev-menu-body">
          <span className="title-font left-menu-header">{`@${profile?.userName}`}</span>
          <a className="sub-title-font dev-menu-item">
            <img className="dev-menu-icon" src={safebox} />
            <span>Saved jobs</span>
          </a>
          <a className="sub-title-font dev-menu-item">
            <img className="dev-menu-icon" src={notification} />
            <span>Job alerts</span>
          </a>
        </div>
        <button className="secondary-btn" onClick={onClickLogout}>
          logout
        </button>
      </nav>
    );
  } else {
    return null;
  }
}

import "../../presentation/theme/left_menu.css";
import safebox from "../../presentation/theme/assets/safe-box.png";
import notification from "../../presentation/theme/assets/notification.png";
import { useProfile } from "../common/redux/profile/ProfileHooks";

export default function LeftMenu() {
  const [profile, _setProfile] = useProfile();

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
        <button className="secondary-btn">logout</button>
      </nav>
    );
  } else {
    return null;
  }
}

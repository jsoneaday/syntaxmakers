import "../../../theme/left_menu.css";
import safebox from "../../../theme/assets/safe-box.png";
import notification from "../../../theme/assets/notification.png";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import LeftMenu from "./LeftMenu";

export default function LeftMenuDev() {
  const [profile, _setProfile] = useProfile();

  return (
    <LeftMenu>
      {profile ? (
        <div className="leftmenu-body">
          <span className="title-font leftmenu-header">{`@${profile?.userName}`}</span>
          <a className="sub-title-font leftmenu-item">
            <img className="leftmenu-icon" src={safebox} />
            <span>Saved jobs</span>
          </a>
          <a className="sub-title-font leftmenu-item">
            <img className="leftmenu-icon" src={notification} />
            <span>Job alerts</span>
          </a>
        </div>
      ) : null}
    </LeftMenu>
  );
}

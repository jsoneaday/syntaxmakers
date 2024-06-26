import "../../../theme/left_menu.css";
import safebox from "../../../theme/assets/safe-box.png";
import notification from "../../../theme/assets/notification.png";
import search from "../../../theme/assets/search.png";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import LeftMenu from "./LeftMenu";
import { RoutePaths } from "../../../../App";
import { Link } from "react-router-dom";

export default function LeftMenuDev() {
  const [profile, _setProfile] = useProfile();

  return (
    <LeftMenu>
      {profile ? (
        <>
          <header className="leftmenu-header">
            <strong>FreeAuth</strong>
          </header>
          <div className="leftmenu-body">
            <span className="title-font leftmenu-item">{`@${profile?.userName}`}</span>
            <Link
              to={RoutePaths.DevJobPosts}
              className="sub-title-font leftmenu-item"
            >
              <img className="leftmenu-icon" src={search} />
              <span>Search jobs</span>
            </Link>
            <Link
              to={RoutePaths.DevSavedJobs}
              className="sub-title-font leftmenu-item"
            >
              <img className="leftmenu-icon" src={safebox} />
              <span>Applied jobs</span>
            </Link>
            <Link
              to={RoutePaths.DevJobAlerts}
              className="sub-title-font leftmenu-item"
            >
              <img className="leftmenu-icon" src={notification} />
              <span>Job alerts</span>
            </Link>
          </div>
        </>
      ) : null}
    </LeftMenu>
  );
}

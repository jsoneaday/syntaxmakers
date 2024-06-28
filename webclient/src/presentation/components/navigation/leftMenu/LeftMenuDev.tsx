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
          <header className="header-container job-full-view-header">
            <Link to="/">
              <strong>FreeAuth</strong>
            </Link>
          </header>
          <div className="leftmenu-body">
            <span className="title-font leftmenu-item">{`@${profile?.userName}`}</span>
            <Link
              to={RoutePaths.DevJobSearch}
              className="sub-title-font leftmenu-item"
            >
              <img className="leftmenu-icon" src={search} />
              <span>Search Jobs</span>
            </Link>
            <Link
              to={RoutePaths.DevAppliedJobs}
              className="sub-title-font leftmenu-item"
            >
              <img className="leftmenu-icon" src={safebox} />
              <span>Applied Jobs</span>
            </Link>
          </div>
        </>
      ) : null}
    </LeftMenu>
  );
}

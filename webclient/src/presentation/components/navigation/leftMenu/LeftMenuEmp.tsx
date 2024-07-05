import "../../../theme/left_menu.css";
import safebox from "../../../theme/assets/safe-box.png";
import notification from "../../../theme/assets/notification.png";
import profileImg from "../../../theme/assets/profile.png";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import plus from "../../../theme/assets/plus.png";
import LeftMenu from "./LeftMenu";
import { Link } from "react-router-dom";
import { RoutePaths } from "../../../../App";

export default function LeftMenuEmp() {
  const [profile, _setProfile] = useProfile();

  return (
    <LeftMenu>
      {profile ? (
        <div className="leftmenu-body">
          <span className="title-font leftmenu-item">{`@${profile?.userName}`}</span>
          <Link
            to={RoutePaths.EmpPostNewJob}
            className="sub-title-font leftmenu-item"
          >
            <img className="leftmenu-icon" src={plus} />
            <span>Post new job</span>
          </Link>
          <Link
            to={RoutePaths.EmpJobPosts}
            className="sub-title-font leftmenu-item"
          >
            <img className="leftmenu-icon" src={safebox} />
            <span>Job posts</span>
          </Link>
          <Link
            to={RoutePaths.EmpJobApplicants}
            className="sub-title-font leftmenu-item"
          >
            <img className="leftmenu-icon" src={notification} />
            <span>Job applicants</span>
          </Link>
          <Link
            to={RoutePaths.EmpProfile}
            className="sub-title-font leftmenu-item"
          >
            <img
              className="leftmenu-icon"
              style={{
                width: "1.55em",
                marginLeft: "-.2em",
                marginRight: ".85em",
              }}
              src={profileImg}
            />
            <span>Profile</span>
          </Link>
        </div>
      ) : null}
    </LeftMenu>
  );
}

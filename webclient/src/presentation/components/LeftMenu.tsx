import "../../theme/left_menu.css";
import safebox from "../../theme/assets/safe-box.png";
import notification from "../../theme/assets/notification.png";

export default function LeftMenu() {
  return (
    <nav className="panel-col dev-menu-container">
      <span className="title-font left-menu-header">@jonny</span>
      <a className="sub-title-font dev-menu-item">
        <img className="dev-menu-icon" src={safebox} />
        <span>Saved jobs</span>
      </a>
      <a className="sub-title-font dev-menu-item">
        <img className="dev-menu-icon" src={notification} />
        <span>Job alerts</span>
      </a>
    </nav>
  );
}

import "../../theme/left_menu.css";

export default function LeftMenu() {
  return (
    <nav className="panel-col dev-menu-container">
      <span className="title-font left-menu-header">@jonny</span>
      <a className="sub-title-font dev-menu-item">
        <img className="dev-menu-icon" src="safe-box.png" />
        <span>Saved jobs</span>
      </a>
      <a className="sub-title-font dev-menu-item">
        <img className="dev-menu-icon" src="notification.png" />
        <span>Job alerts</span>
      </a>
    </nav>
  );
}

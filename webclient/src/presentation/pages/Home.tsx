import developer from "../../theme/assets/programmer.png";
import employer from "../../theme/assets/businessman.png";
import "../../theme/home.css";

export default function Home() {
  return (
    <div className="home">
      <div className="panel-col home-item-dev">
        <a href="dev" className="home-item-link">
          <img className="home-icon" src={developer} />
          <div className="title-font home-item-content">"I'm a Developer"</div>
        </a>
      </div>

      <div className="panel-col home-item-emp">
        <a href="employer" className="home-item-link">
          <img className="home-icon" src={employer} />
          <div className="title-font home-item-content">"I'm an Employer"</div>
        </a>
      </div>
    </div>
  );
}

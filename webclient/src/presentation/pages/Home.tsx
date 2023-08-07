import developer from "../../presentation/theme/assets/programmer.png";
import employer from "../../presentation/theme/assets/businessman.png";
import "../../presentation/theme/home.css";
import { Link } from "react-router-dom";

export default function Home() {
  return (
    <div className="home">
      <div className="panel-col home-item-dev">
        <Link to="developer" className="home-item-link">
          <img className="home-icon" src={developer} />
          <div className="title-font home-item-content">I'm a Developer</div>
        </Link>
      </div>

      <div className="panel-col home-item-emp">
        <Link to="employer" className="home-item-link">
          <img className="home-icon" src={employer} />
          <div className="title-font home-item-content">I'm an Employer</div>
        </Link>
      </div>
    </div>
  );
}

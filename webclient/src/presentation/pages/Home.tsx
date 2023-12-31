import developer from "../../presentation/theme/assets/programmer.png";
import employer from "../../presentation/theme/assets/businessman.png";
import "../../presentation/theme/home.css";
import { Link } from "react-router-dom";
import Layout from "../components/Layout";
import { RoutePaths } from "../../App";
import { useEffect } from "react";
import { useProfile } from "../common/redux/profile/ProfileHooks";

export default function Home() {
  const [_profile, setProfile] = useProfile();

  useEffect(() => {
    setProfile(null);
  }, []);

  return (
    <Layout includeLogin={false}>
      <div className="home" data-testid="home-page">
        <div className="panel-col home-item-dev">
          <Link
            to={RoutePaths.DevJobPosts}
            className="home-item-link"
            data-testid="dev-link"
          >
            <img className="home-icon" src={developer} />
            <div className="title-font home-item-content">I'm a Developer</div>
          </Link>
        </div>

        <div className="panel-col home-item-emp">
          <Link
            to={RoutePaths.EmpJobPosts}
            className="home-item-link"
            data-testid="emp-link"
          >
            <img className="home-icon" src={employer} />
            <div className="title-font home-item-content">I'm an Employer</div>
          </Link>
        </div>
      </div>
    </Layout>
  );
}

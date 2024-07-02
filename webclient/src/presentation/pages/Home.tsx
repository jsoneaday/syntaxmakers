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
    console.log("Home page");
    setProfile(null);
  }, []);

  return (
    <Layout includeLogin={false}>
      <div className="home" data-testid="home-page">
        <header className="home-header">
          <h1>SyntaxMakers Specializes in Unique Programming Languages</h1>
          <h2>Find a Developer that is an expert in the language you need</h2>
        </header>
        <div className="home-chooser">
          <div className="panel-col home-item-dev">
            <Link
              to={`${RoutePaths.DevJobSearch}`}
              className="home-item-link"
              data-testid="dev-link"
            >
              <img className="home-icon" src={developer} />
              <div className="title-font home-item-content">
                <strong>I'm a Developer</strong>
              </div>
            </Link>
          </div>

          <div className="panel-col home-item-emp">
            <Link
              to={RoutePaths.EmpJobPosts}
              className="home-item-link"
              data-testid="emp-link"
            >
              <img className="home-icon" src={employer} />
              <div className="title-font home-item-content">
                <strong>I'm an Employer</strong>
              </div>
            </Link>
          </div>
        </div>
      </div>
    </Layout>
  );
}

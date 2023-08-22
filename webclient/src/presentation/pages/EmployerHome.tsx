import "../../presentation/theme/emphome.css";
import "../../presentation/theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import Layout from "../components/Layout";
import LeftMenuEmp from "../components/navigation/LeftMenuEmp";

export default function DeveloperHome() {
  const [_loginOpen, _setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="userhome-container" data-testid="employer-page">
        <LeftMenuEmp />
        Employer
      </div>
    </Layout>
  );
}

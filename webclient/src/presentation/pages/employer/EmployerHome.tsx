import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { useLoginOpen } from "../../common/redux/loginOpen/LoginOpenHooks";
import Layout from "../../components/Layout";
import LeftMenuEmp from "../../components/navigation/leftMenu/LeftMenuEmp";
import LeftMenu from "../../components/navigation/leftMenu/LeftMenu";

export default function DeveloperHome() {
  const [_loginOpen, _setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="userhome-container" data-testid="employer-page">
        <LeftMenu>
          <LeftMenuEmp />
        </LeftMenu>
        Employer
      </div>
    </Layout>
  );
}

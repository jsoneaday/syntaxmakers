import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import JobPreviewList from "../../components/jobs/JobPreviewList";
import { useLoginOpen } from "../../common/redux/loginOpen/LoginOpenHooks";
import Layout from "../../components/Layout";
import LeftMenuDev from "../../components/navigation/leftMenu/LeftMenuDev";
import LeftMenu from "../../components/navigation/leftMenu/LeftMenu";

export default function DeveloperHome() {
  const [_loginOpen, setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="userhome-container" data-testid="developer-page">
        <LeftMenu>
          <LeftMenuDev />
        </LeftMenu>
        <JobPreviewList setLoginIsOpen={setLoginOpen} />
      </div>
    </Layout>
  );
}

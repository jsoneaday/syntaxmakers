import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { useLoginOpen } from "../../common/redux/loginOpen/LoginOpenHooks";
import Layout from "../../components/Layout";
import LeftMenuDev from "../../components/navigation/leftMenu/LeftMenuDev";
import LeftMenu from "../../components/navigation/leftMenu/LeftMenu";
import DevJobPreviewList from "../../components/developer/DevJobPreviewLists";

export default function DeveloperHome() {
  const [_loginOpen, setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="userhome-container" data-testid="developer-page">
        <LeftMenu>
          <LeftMenuDev />
        </LeftMenu>
        <DevJobPreviewList setLoginIsOpen={setLoginOpen} />
      </div>
    </Layout>
  );
}

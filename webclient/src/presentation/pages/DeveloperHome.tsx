import "../../presentation/theme/devhome.css";
import "../../presentation/theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import JobPreviewList from "../components/jobs/JobPreviewList";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";
import Layout from "../components/Layout";
import LeftMenuDev from "../components/navigation/LeftMenuDev";

export default function DeveloperHome() {
  const [_loginOpen, setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="userhome-container" data-testid="developer-page">
        <LeftMenuDev />
        <JobPreviewList setLoginIsOpen={setLoginOpen} />
      </div>
    </Layout>
  );
}

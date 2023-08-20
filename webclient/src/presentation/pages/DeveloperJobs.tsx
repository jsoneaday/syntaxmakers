import "../../presentation/theme/developer.css";
import LeftMenu from "../components/navigation/LeftMenu";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import JobPreviewList from "../components/jobs/JobPreviewList";
import Layout from "../components/Layout";
import { useLoginOpen } from "../common/redux/loginOpen/LoginOpenHooks";

export default function DeveloperJobs() {
  const [_loginOpen, setLoginOpen] = useLoginOpen();

  return (
    <Layout>
      <div className="dev-container" data-testid="developer-page">
        <LeftMenu />
        <JobPreviewList setLoginIsOpen={setLoginOpen} />
      </div>
    </Layout>
  );
}

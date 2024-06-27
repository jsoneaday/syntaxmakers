import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import DeveloperHome from "./DeveloperHome";
import { DevAppliedJobsList } from "../../components/developer/DevAppliedJobsList";

export default function DevAppliedJobs() {
  return (
    <DeveloperHome>
      <DevAppliedJobsList />
    </DeveloperHome>
  );
}

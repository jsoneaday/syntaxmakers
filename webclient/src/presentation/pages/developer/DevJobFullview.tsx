import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import DeveloperHome from "./DeveloperHome";
import JobFullview from "../../components/jobs/JobFullview";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

export default function DevJobFullview() {
  return (
    <DeveloperHome>
      <JobFullview userType={UiDevOrEmployer.Developer} />
    </DeveloperHome>
  );
}

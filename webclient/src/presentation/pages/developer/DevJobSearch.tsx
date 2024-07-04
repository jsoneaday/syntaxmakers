import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import JobSearchList from "../../components/jobs/JobSearchList";
import DeveloperHome from "./DeveloperHome";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

export default function DevJobSearch() {
  return (
    <DeveloperHome>
      <JobSearchList userType={UiDevOrEmployer.Developer} />
    </DeveloperHome>
  );
}

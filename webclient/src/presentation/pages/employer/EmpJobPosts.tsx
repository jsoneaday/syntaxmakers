import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import JobSearchList from "../../components/jobs/JobSearchList";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

export default function EmpJobPosts() {
  return (
    <EmployerHome>
      <JobSearchList userType={UiDevOrEmployer.Employer} />
    </EmployerHome>
  );
}

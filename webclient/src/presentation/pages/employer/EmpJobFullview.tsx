import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import JobFullview from "../../components/jobs/JobFullview";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

export default function EmpJobFullview() {
  return (
    <EmployerHome>
      <JobFullview userType={UiDevOrEmployer.Employer} />
    </EmployerHome>
  );
}

import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import { ApplicantList } from "../../components/developer/applicant/ApplicantList";

export default function EmpJobApplicants() {
  return (
    <EmployerHome>
      <ApplicantList />
    </EmployerHome>
  );
}

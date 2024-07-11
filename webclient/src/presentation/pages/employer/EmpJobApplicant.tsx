import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import { ApplicantFullview } from "../../components/developer/applicant/ApplicantFullview";

export default function EmpJobApplicant() {
  return (
    <EmployerHome>
      <ApplicantFullview />
    </EmployerHome>
  );
}

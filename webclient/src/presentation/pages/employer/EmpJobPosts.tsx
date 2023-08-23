import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import EmpJobPreviewList from "../../components/employer/EmpJobPreviewList";

export default function EmpJobPosts() {
  console.log("empjobposts");
  return (
    <EmployerHome>
      <EmpJobPreviewList />
    </EmployerHome>
  );
}

import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";

export default function EmpPostNewJob() {
  console.log("empjobposts");
  return <EmployerHome>Post new job</EmployerHome>;
}

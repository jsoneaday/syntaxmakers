import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import DevJobSearchList from "../../components/developer/DevJobSearchList";
import DeveloperHome from "./DeveloperHome";

export default function DevJobSearch() {
  return (
    <DeveloperHome>
      <DevJobSearchList />
    </DeveloperHome>
  );
}

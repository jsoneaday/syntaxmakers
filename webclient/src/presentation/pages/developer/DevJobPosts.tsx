import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import DevJobPreviewList from "../../components/developer/DevJobPreviewList";
import DeveloperHome from "./DeveloperHome";

export default function DevJobPosts() {
  return (
    <DeveloperHome>
      <DevJobPreviewList />
    </DeveloperHome>
  );
}

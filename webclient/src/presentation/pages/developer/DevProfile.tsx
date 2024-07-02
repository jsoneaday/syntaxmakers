import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import DeveloperHome from "./DeveloperHome";
import {
  ProfileForm,
  ProfileFormEditMode,
} from "../../components/authentication/ProfileForm";
import { DevOrEmployer } from "../../models/DevOrEmployer";

export default function DevProfile() {
  return (
    <DeveloperHome>
      <ProfileForm
        editMode={ProfileFormEditMode.Edit}
        userType={DevOrEmployer.Developer}
      />
    </DeveloperHome>
  );
}

import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import EmployerHome from "./EmployerHome";
import {
  ProfileForm,
  ProfileFormEditMode,
} from "../../components/authentication/Profile/ProfileForm";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

export default function DevProfile() {
  return (
    <EmployerHome>
      <ProfileForm
        isModalMode={false}
        editMode={ProfileFormEditMode.Edit}
        userType={UiDevOrEmployer.Employer}
      />
    </EmployerHome>
  );
}

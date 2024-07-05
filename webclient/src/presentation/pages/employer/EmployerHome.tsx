import "../../theme/emphome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import Layout from "../../components/Layout";
import LeftMenuEmp from "../../components/navigation/leftMenu/LeftMenuEmp";
import { ReactNode } from "react";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";
import { useProfile } from "../../common/redux/profile/ProfileHooks";

interface EmployerHomeProps {
  children: ReactNode;
}

export default function EmployerHome({ children }: EmployerHomeProps) {
  const [profile] = useProfile();

  return (
    <Layout userType={UiDevOrEmployer.Employer}>
      {profile ? (
        <div className="userhome-container" data-testid="employer-page">
          <LeftMenuEmp />
          {children}
        </div>
      ) : null}
    </Layout>
  );
}

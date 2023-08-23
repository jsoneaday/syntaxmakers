import "../../theme/devhome.css";
import "../../theme/userhome.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import Layout from "../../components/Layout";
import LeftMenuDev from "../../components/navigation/leftMenu/LeftMenuDev";
import { ReactNode } from "react";

interface DeveloperHomeProps {
  children: ReactNode;
}

export default function DeveloperHome({ children }: DeveloperHomeProps) {
  return (
    <Layout>
      <div className="userhome-container" data-testid="developer-page">
        <LeftMenuDev />
        {children}
      </div>
    </Layout>
  );
}

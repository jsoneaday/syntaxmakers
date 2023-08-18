import "../../presentation/theme/developer.css";
import LeftMenu from "../components/LeftMenu";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import Login from "../components/authentication/Login";
import { DevOrEmployer } from "../models/DevOrEmployer";
import { startViewTransition } from "../common/transitions/ViewTransition";
import { useState } from "react";
import JobPreviewList from "../components/jobs/JobPreviewList";

export default function DeveloperJobs() {
  const [loginIsOpen, setLoginIsOpen] = useState(false);

  const toggleOpen = () => {
    startViewTransition(() => setLoginIsOpen(!loginIsOpen));
  };

  return (
    <>
      <Login
        devOrEmployer={DevOrEmployer.Developer}
        isOpen={loginIsOpen}
        toggleOpen={toggleOpen}
      />
      <div className="dev-container" data-testid="developer-page">
        <LeftMenu />
        <JobPreviewList setLoginIsOpen={setLoginIsOpen} />
      </div>
    </>
  );
}

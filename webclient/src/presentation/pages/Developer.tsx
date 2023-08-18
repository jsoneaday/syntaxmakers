import "../../presentation/theme/developer.css";
import LeftMenu from "../components/LeftMenu";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import Login from "../components/authentication/Login";
import { DevOrEmployer } from "../models/DevOrEmployer";
import { startViewTransition } from "../common/transitions/ViewTransition";
import { useState } from "react";
import JobPreviewList from "../components/jobs/JobPreviewList";
import JobFullView from "../components/jobs/JobFullView";

export enum DeveloperViewType {
  JobPreviewList,
  JobFullView,
}

export default function Developer() {
  const [loginIsOpen, setLoginIsOpen] = useState(false);
  const [developerViewType, setDeveloperViewType] = useState<DeveloperViewType>(
    DeveloperViewType.JobPreviewList
  );

  const toggleOpen = () => {
    startViewTransition(() => setLoginIsOpen(!loginIsOpen));
  };

  const onClickSwitchView = (developerViewType: DeveloperViewType) => {
    setDeveloperViewType(developerViewType);
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
        {developerViewType === DeveloperViewType.JobPreviewList ? (
          <JobPreviewList
            setLoginIsOpen={setLoginIsOpen}
            onClickSwitchView={onClickSwitchView}
            currentDevViewType={developerViewType}
          />
        ) : (
          <JobFullView />
        )}
      </div>
    </>
  );
}

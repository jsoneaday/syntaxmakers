import { useState } from "react";
import PreviewApplicantList from "./PreviewApplicantList";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import { convert, JobApplicantModel } from "../../../models/JobApplicantModel";
import { PAGE_SIZE } from "../../../common/Paging";
import { getJobApplicants } from "../../../../domain/repository/JobRepo";
import { Paging } from "../../controls/Paging";

export function ApplicantList() {
  const [applicants, setApplicants] = useState<JobApplicantModel[]>([]);
  const [profile] = useProfile();

  const getApplicants = async (newOffset: number, setData: boolean) => {
    if (!profile) return [];
    const applicants = await getJobApplicants(profile.id, PAGE_SIZE, newOffset);
    setData && setApplicants(applicants.map((a) => convert(a)));
    return applicants;
  };

  return (
    <div className="userhome-main">
      <header className="header-container job-full-view-header">
        <strong>Job Applicants</strong>
      </header>
      <div style={{ padding: "2em", width: "100%" }}>
        <PreviewApplicantList applicants={applicants} />
        <Paging dataQuery={getApplicants} />
      </div>
    </div>
  );
}

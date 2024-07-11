import { GroupLister } from "../../Lister";
import { PreviewApplicant } from "./PreviewApplicant";
import { JobApplicantModel } from "../../../models/JobApplicantModel";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { SendEmailModal } from "./SendEmailModal";
import { useState } from "react";

interface PreviewProfileListProps {
  applicants: JobApplicantModel[];
}

export default function PreviewApplicantList({
  applicants,
}: PreviewProfileListProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedDevId, setSelectedDevId] = useState("");

  const toggleIsOpen = () => {
    setIsOpen(!isOpen);
  };

  const onSelectedDevId = (devId: string) => {
    setSelectedDevId(devId);
    toggleIsOpen();
  };

  return (
    <div className="dev-post-preview-container">
      <SendEmailModal
        isOpen={isOpen}
        toggleOpen={toggleIsOpen}
        receiverDevId={selectedDevId}
      />
      {applicants.length === 0 ? (
        <strong>No applicants found</strong>
      ) : (
        <GroupLister
          groupItems={applicants}
          elementCreator={(dataItem) => (
            <li
              key={`${dataItem.key}-${uuidv4()}`}
              className="dev-preview-item"
              style={{ width: "100%" }}
            >
              <PreviewApplicant
                applicant={dataItem}
                selectDevId={onSelectedDevId}
              />
            </li>
          )}
        />
      )}
    </div>
  );
}

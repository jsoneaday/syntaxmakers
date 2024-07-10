import { MouseEvent, useState } from "react";
import Modal from "../Modal";
import Login from "./Login";
import { Tabs } from "../controls/Tabs";
import { ProfileForm, ProfileFormEditMode } from "./Profile/ProfileForm";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

enum AuthModalMode {
  Login,
  Register,
}

interface AuthModalProps {
  isOpen: boolean;
  userType: UiDevOrEmployer;
  toggleOpen: () => void;
}

export function AuthModal({ isOpen, userType, toggleOpen }: AuthModalProps) {
  const [authModalMode, setAuthModalMode] = useState(AuthModalMode.Login);
  const [selectedTabId, setSelectedTabId] = useState(1);

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <div className="authmodal-container">
        <Tabs
          selectedId={selectedTabId}
          tabs={[
            {
              id: 1,
              label: "Login",
              onClick: (e: MouseEvent<HTMLButtonElement>) => {
                e.preventDefault();

                setSelectedTabId(1);
                setAuthModalMode(AuthModalMode.Login);
              },
            },
            {
              id: 2,
              label: "Register",
              onClick: (e: MouseEvent<HTMLButtonElement>) => {
                e.preventDefault();

                setSelectedTabId(2);
                setAuthModalMode(AuthModalMode.Register);
              },
            },
          ]}
        />
        {authModalMode === AuthModalMode.Login ? (
          <Login toggleOpen={toggleOpen} />
        ) : (
          <ProfileForm
            isModalMode={true}
            editMode={ProfileFormEditMode.Create}
            userType={userType}
          />
        )}
      </div>
    </Modal>
  );
}

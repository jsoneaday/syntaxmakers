import { ReactNode, useEffect, useState } from "react";
import Modal from "./Modal";
import useNotificationState from "../../common/redux/notification/NotificationStateHooks";
import "../../theme/notification_state.css";

export enum NotificationType {
  Info,
  Warning,
  Error,
}

const primaryColor = "var(--primary-cl)";

interface NotificationProps {
  title: string;
  notiType: NotificationType;
  children: ReactNode;
}

export default function Notification({
  title,
  notiType,
  children,
}: NotificationProps) {
  const [notificationState, setNotificationState] = useNotificationState();
  const [titleColor, setTitleColor] = useState(primaryColor);

  useEffect(() => {
    if (notiType === NotificationType.Error) {
      setTitleColor("var(--error-cl)");
    } else if (notiType === NotificationType.Warning) {
      setTitleColor("var(--secondary-cl)");
    } else {
      setTitleColor(primaryColor);
    }
  }, [notiType]);

  const toggleIsOpen = () => {
    const _notificationState = {
      ...notificationState,
      isOpen: !notificationState.isOpen,
    };

    setNotificationState(_notificationState);
  };

  return (
    <Modal isOpen={notificationState.isOpen} toggleOpen={toggleIsOpen}>
      <div className="noti-container">
        <span className="title-font" style={{ color: titleColor }}>
          {title}
        </span>
        {children}
      </div>
    </Modal>
  );
}

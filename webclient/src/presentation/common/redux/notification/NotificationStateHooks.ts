import { RootState } from "../Store";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { NotificationState } from "./NotificationStateSlice";
import { setNotificationState as setNotificationStateSlice } from "./NotificationStateSlice";

export default function useNotificationState(): [
  notificationState: NotificationState,
  setNotificationState: (notificationState: NotificationState) => void
] {
  const notificationState = useAppSelector(
    (state: RootState) => state.notificationState
  );
  const dispatch = useAppDispatch();
  const setNotificationState = (notificationState: NotificationState) => {
    dispatch(setNotificationStateSlice(notificationState));
  };
  return [notificationState, setNotificationState];
}

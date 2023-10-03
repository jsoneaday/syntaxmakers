import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface NotificationState {
  isOpen: boolean;
}

const initialState: NotificationState = {
  isOpen: false,
};

const notificationStateSlice = createSlice({
  name: "notification",
  initialState,
  reducers: {
    setNotificationState: (
      state: any,
      action: PayloadAction<NotificationState>
    ) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setNotificationState } = notificationStateSlice.actions;
export default notificationStateSlice.reducer;

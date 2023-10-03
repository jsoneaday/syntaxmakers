import { configureStore } from "@reduxjs/toolkit";
import ProfileReducer from "./profile/ProfileSlice";
import LoginOpenReducer from "./loginOpen/LoginOpenSlice";
import DevOrEmployerReducer from "./devOrEmployer/DevOrEmployerSlice";
import InTextEditModeReducer from "./inTextEditMode/InTextEditModeSlice";

const reducer = {
  profile: ProfileReducer,
  loginOpen: LoginOpenReducer,
  devOrEmployer: DevOrEmployerReducer,
  inTextEditMode: InTextEditModeReducer,
};

export const store = configureStore({
  reducer,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

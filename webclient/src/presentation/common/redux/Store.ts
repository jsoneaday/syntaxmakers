import { configureStore } from "@reduxjs/toolkit";
import ProfileReducer from "./profile/ProfileSlice";
import LoginOpenReducer from "./loginOpen/LoginOpenSlice";

const reducer = {
  profile: ProfileReducer,
  loginOpen: LoginOpenReducer,
};

export const store = configureStore({
  reducer,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

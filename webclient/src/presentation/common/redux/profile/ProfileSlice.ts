import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import DevProfile from "../../../models/DevProfile";
import EmpProfile from "../../../models/EmpProfile";

export type ProfileState = DevProfile | EmpProfile | null;
const initialState: ProfileState = null;

const profileSlice = createSlice({
  name: "profile",
  initialState,
  reducers: {
    setUserProfile: (state: any, action: PayloadAction<ProfileState>) => {
      state = action.payload;

      return state;
    },
  },
});

export const { setUserProfile } = profileSlice.actions;
export default profileSlice.reducer;

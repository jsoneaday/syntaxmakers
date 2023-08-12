import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import DevProfile from "../../../models/DevProfile";

const profileSlice = createSlice({
  name: "profile",
  initialState: null,
  reducers: {
    setUserProfile: (state: any, action: PayloadAction<DevProfile | null>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setUserProfile } = profileSlice.actions;
export default profileSlice.reducer;

import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { Developer } from "../../repository/DeveloperRepo";

const profileSlice = createSlice({
  name: "profile",
  initialState: null,
  reducers: {
    setUserProfile: (state: any, action: PayloadAction<Developer | null>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setUserProfile } = profileSlice.actions;
export default profileSlice.reducer;

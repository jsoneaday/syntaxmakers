import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { UiDevOrEmployer } from "../../../models/DevOrEmployer";

const initialState: UiDevOrEmployer = UiDevOrEmployer.Developer;

const devOrEmployerSlice = createSlice({
  name: "devOrEmployer",
  initialState,
  reducers: {
    setDevOrEmployer: (state: any, action: PayloadAction<UiDevOrEmployer>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setDevOrEmployer } = devOrEmployerSlice.actions;
export default devOrEmployerSlice.reducer;

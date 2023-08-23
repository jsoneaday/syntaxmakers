import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { DevOrEmployer } from "../../../models/DevOrEmployer";

const initialState: DevOrEmployer | null = null;

const devOrEmployerSlice = createSlice({
  name: "devOrEmployer",
  initialState,
  reducers: {
    setDevOrEmployer: (state: any, action: PayloadAction<DevOrEmployer>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setDevOrEmployer } = devOrEmployerSlice.actions;
export default devOrEmployerSlice.reducer;

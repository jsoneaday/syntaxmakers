import { PayloadAction, createSlice } from "@reduxjs/toolkit";

const initialState = false;

const inTextEditModeSlice = createSlice({
  name: "inTextEditMode",
  initialState,
  reducers: {
    setInTextEditMode: (state: any, action: PayloadAction<boolean>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setInTextEditMode } = inTextEditModeSlice.actions;
export default inTextEditModeSlice.reducer;

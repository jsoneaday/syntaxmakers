import { PayloadAction, createSlice } from "@reduxjs/toolkit";

const initialState = false;

const loginOpenSlice = createSlice({
  name: "loginOpen",
  initialState,
  reducers: {
    setLoginOpen: (state: any, action: PayloadAction<boolean>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setLoginOpen } = loginOpenSlice.actions;
export default loginOpenSlice.reducer;

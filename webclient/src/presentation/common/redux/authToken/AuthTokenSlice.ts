import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type AuthToken = {
  id: string; // cannot be bigint as it needs to be serializeable
  token: string;
};

const initialState: AuthToken | null = null;

const authTokenSlice = createSlice({
  name: "authToken",
  initialState,
  reducers: {
    setAuthToken: (state: any, action: PayloadAction<AuthToken | null>) => {
      state = action.payload;
      return state;
    },
  },
});

export const { setAuthToken } = authTokenSlice.actions;
export default authTokenSlice.reducer;

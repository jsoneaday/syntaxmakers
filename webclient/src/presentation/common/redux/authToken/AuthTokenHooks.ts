import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { AuthToken, setAuthToken as setAuthTokenSlice } from "./AuthTokenSlice";

export function useAuthToken(): [
  authToken: AuthToken,
  setAuthToken: (authToken: AuthToken | null) => void
] {
  const authToken = useAppSelector((state: any) => state.authToken);
  const dispatch = useAppDispatch();

  const setAuthToken = (authToken: AuthToken | null) => {
    dispatch(setAuthTokenSlice(authToken));
  };

  return [authToken, setAuthToken];
}

import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setLoginOpen as setLoginOpenSlice } from "./LoginOpenSlice";

export function useLoginOpen(): [
  loginOpen: boolean,
  setLoginOpen: (isOpen: boolean) => void
] {
  const loginOpen = useAppSelector((state: any) => state.loginOpen);
  const dispatch = useAppDispatch();

  const setLoginOpen = (isOpen: boolean) => {
    dispatch(setLoginOpenSlice(isOpen));
  };

  return [loginOpen, setLoginOpen];
}

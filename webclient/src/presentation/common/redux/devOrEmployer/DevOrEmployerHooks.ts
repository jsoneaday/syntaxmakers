import { DevOrEmployer } from "../../../models/DevOrEmployer";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setDevOrEmployer as setDevOrEmployerSlice } from "./DevOrEmployerSlice";

export function useDevOrEmployer(): [
  devOrEmployer: DevOrEmployer,
  setDevOrEmployer: (isOpen: DevOrEmployer) => void
] {
  const devOrEmployer = useAppSelector((state: any) => state.devOrEmployer);
  const dispatch = useAppDispatch();

  const setDevOrEmployer = (isOpen: DevOrEmployer) => {
    dispatch(setDevOrEmployerSlice(isOpen));
  };

  return [devOrEmployer, setDevOrEmployer];
}

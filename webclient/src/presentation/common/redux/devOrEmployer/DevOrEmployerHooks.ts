import { UiDevOrEmployer } from "../../../models/DevOrEmployer";
import { RootState } from "../Store";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setDevOrEmployer as setDevOrEmployerSlice } from "./DevOrEmployerSlice";

export function useDevOrEmployer(): [
  devOrEmployer: UiDevOrEmployer,
  setDevOrEmployer: (devOrEmp: UiDevOrEmployer) => void
] {
  const devOrEmployer = useAppSelector(
    (state: RootState) => state.devOrEmployer
  );
  const dispatch = useAppDispatch();

  const setDevOrEmployer = (devOrEmp: UiDevOrEmployer) => {
    dispatch(setDevOrEmployerSlice(devOrEmp));
  };

  return [devOrEmployer, setDevOrEmployer];
}

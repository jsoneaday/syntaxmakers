import { RootState } from "../Store";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setInTextEditMode as setInTextEditModeFromSlice } from "./InTextEditModeSlice";

export function useInTextEditMode(): [
  inTextEditMode: boolean,
  setInTextEditMode: (inTextEditMode: boolean) => void
] {
  const inTextEditMode = useAppSelector((state: RootState) => {
    return state.inTextEditMode;
  });

  const dispatch = useAppDispatch();

  const setInTextEditMode = (inTextEditMode: boolean) => {
    dispatch(setInTextEditModeFromSlice(inTextEditMode));
  };

  return [inTextEditMode, setInTextEditMode];
}

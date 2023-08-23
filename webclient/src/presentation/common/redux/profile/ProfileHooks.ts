import DevProfile from "../../../models/DevProfile";
import EmpProfile from "../../../models/EmpProfile";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setUserProfile } from "./ProfileSlice";

export function useProfile(): [
  profile: DevProfile | EmpProfile | null,
  setProfile: (profile: DevProfile | EmpProfile | null) => void
] {
  const profile = useAppSelector((state) => state.profile);

  const dispatch = useAppDispatch();

  const setProfile = (profile: DevProfile | EmpProfile | null) => {
    dispatch(setUserProfile(profile));
  };

  return [profile, setProfile];
}

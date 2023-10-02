import DevProfile from "../../../models/DevProfile";
import EmpProfile from "../../../models/EmpProfile";
import { RootState } from "../Store";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setUserProfile } from "./ProfileSlice";

export function useProfile(): [
  profile: DevProfile | EmpProfile | null,
  setProfile: (profile: DevProfile | EmpProfile | null) => void
] {
  const profile = useAppSelector((state: RootState) => state.profile);

  const dispatch = useAppDispatch();

  const setProfile = (profile: DevProfile | EmpProfile | null) => {
    const profileToDispatch = setUserProfile(profile);

    dispatch(profileToDispatch);
  };

  return [profile, setProfile];
}

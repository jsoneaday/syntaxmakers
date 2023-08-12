import DevProfile from "../../../models/DevProfile";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setUserProfile } from "./ProfileSlice";

export function useProfile(): [
  profile: DevProfile | null,
  setProfile: (profile: DevProfile | null) => void
] {
  const profile = useAppSelector((state) => state.profile);

  const dispatch = useAppDispatch();

  const setProfile = (profile: DevProfile | null) => {
    dispatch(setUserProfile(profile));
  };

  return [profile, setProfile];
}

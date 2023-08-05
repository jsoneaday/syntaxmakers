import { Developer } from "../../repository/DeveloperRepo";
import { useAppDispatch, useAppSelector } from "../StoreHooks";
import { setUserProfile } from "./ProfileSlice";

export function useProfile(): [
  profile: Developer | null,
  setProfile: (profile: Developer | null) => void
] {
  const profile = useAppSelector((state) => state.profile);

  const dispatch = useAppDispatch();

  const setProfile = (profile: Developer | null) => {
    dispatch(setUserProfile(profile));
  };

  return [profile, setProfile];
}

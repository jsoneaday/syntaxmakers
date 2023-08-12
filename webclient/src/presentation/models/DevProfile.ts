import { Key, useId } from "react";
import { KeyItem } from "../common/utils";
import { Developer } from "../../domain/repository/DeveloperRepo";

export default class DevProfile implements KeyItem {
  constructor(
    public key: Key,
    public id: string, // bigint is not serializable by Redux
    public updated_at: string,
    public user_name: string,
    public full_name: string,
    public email: string,
    public primary_lang_id: string,
    public secondary_lang_id?: string | null
  ) {}
}

export function convert(dev: Developer) {
  return new DevProfile(
    useId(),
    dev.id, // bigint is not serializable by Redux
    dev.updated_at.toISOString(),
    dev.user_name,
    dev.full_name,
    dev.email,
    dev.primary_lang_id,
    dev.secondary_lang_id
  );
}

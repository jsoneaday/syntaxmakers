import { Key } from "react";

export interface KeyItem {
  key: Key;
}

export interface GroupItem extends KeyItem {
  id: string;
  title: string;
}

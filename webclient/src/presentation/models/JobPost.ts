import { Key } from "react";
import { KeyItem } from "../common/utils";

export default class JobPost implements KeyItem {
  constructor(
    public key: Key,
    public id: bigint,
    public title: string,
    public company: string,
    public location: string,
    public salary: string,
    public timestamp: string,
    public icon_src: string
  ) {}
}

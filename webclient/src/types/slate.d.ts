// This example is for an Editor with `ReactEditor` and `HistoryEditor`
import { BaseEditor } from "slate";
import { ReactEditor } from "slate-react";
import { HistoryEditor } from "slate-history";
import { ElementTypes } from "../presentation/components/textEditor/ElementTypes";

export type CustomEditor = BaseEditor & ReactEditor & HistoryEditor;

export type ParagraphElement = {
  type: "paragraph";
  children: CustomText[];
};

export type Heading1Element = {
  type: ElementTypes.Heading1;
  level: number;
  children: CustomText[];
};

export type Heading2Element = {
  type: ElementTypes.Heading2;
  level: number;
  children: CustomText[];
};

export type Heading3Element = {
  type: ElementTypes.Heading3;
  level: number;
  children: CustomText[];
};

export type CustomElement =
  | ParagraphElement
  | Heading1Element
  | Heading2Element
  | Heading3Element;

export type FormattedText = { text: string; bold?: true; italic?: true };

export type CustomText = FormattedText;

declare module "slate" {
  interface CustomTypes {
    Editor: CustomEditor;
    Element: CustomElement;
    Text: CustomText;
  }
}

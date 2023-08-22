import { ReactNode } from "react";
import { Commands } from "./Commands";
import { MouseEvent } from "react";
import { CustomEditor } from "../../../types/slate";
import { ElementHeaderTypeLevels } from "./ElementTypes";

/// @ts-ignore
interface SlateProps {
  attributes: any;
  children: ReactNode;
}

export function Heading1({ attributes, children }: SlateProps) {
  return <h1 {...attributes}>{children}</h1>;
}

export function Heading2({ attributes, children }: SlateProps) {
  return <h2 {...attributes}>{children}</h2>;
}

export function Heading3({ attributes, children }: SlateProps) {
  return <h3 {...attributes}>{children}</h3>;
}

interface LeafProps {
  leaf: { bold?: boolean };
}

export function Leaf({ attributes, children, leaf }: SlateProps & LeafProps) {
  return (
    <span {...attributes} style={{ fontWeight: leaf.bold ? "bold" : "normal" }}>
      {children}
    </span>
  );
}

interface ToolbarProps {
  editor: CustomEditor;
}

export function Toolbar({ editor }: ToolbarProps) {
  const onClickHeader1 = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    Commands.toggleHeadingBlock(editor, ElementHeaderTypeLevels.Level1);
  };
  const onClickHeader2 = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    Commands.toggleHeadingBlock(editor, ElementHeaderTypeLevels.Level2);
  };
  const onClickHeader3 = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    Commands.toggleHeadingBlock(editor, ElementHeaderTypeLevels.Level3);
  };
  const onClickBold = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    Commands.toggleBoldMark(editor);
  };

  return (
    <div className="txtedit-toolbar">
      <button
        className="secondary-btn small-btn"
        style={{ width: "3em", marginRight: ".5em" }}
        onClick={onClickHeader1}
      >
        H1
      </button>
      <button
        className="secondary-btn small-btn"
        style={{ width: "3em", marginRight: ".5em" }}
        onClick={onClickHeader2}
      >
        H2
      </button>
      <button
        className="secondary-btn small-btn"
        style={{ width: "3em", marginRight: ".5em" }}
        onClick={onClickHeader3}
      >
        H3
      </button>
      <button
        className="secondary-btn small-btn"
        style={{ width: "4em", marginRight: ".5em" }}
        onClick={onClickBold}
      >
        Bold
      </button>
    </div>
  );
}

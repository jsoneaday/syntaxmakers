import { KeyboardEvent, useCallback, useState } from "react";
import { createEditor, Descendant, Editor, Element, Transforms } from "slate";
import { withHistory } from "slate-history";
import {
  Slate,
  Editable,
  withReact,
  RenderElementProps,
  RenderLeafProps,
} from "slate-react";
import { Heading1, Leaf } from "./ElementRenderers";

const initialValue: Descendant[] = [
  {
    type: "paragraph",
    children: [{ text: "A line of text in a paragraph." }],
  },
];

const renderElement = (props: RenderElementProps) => {
  switch (props.element.type) {
    case "heading":
      return <Heading1 {...props} />;
    case "paragraph":
      return <p {...props} />;
    default:
      return <span {...props} />;
  }
};

const renderLeaf = (props: RenderLeafProps) => {
  return <Leaf {...props} />;
};

export default function TextEditor() {
  const [editor] = useState(() => withReact(withHistory(createEditor())));

  const onKeyDown = useCallback(
    (e: KeyboardEvent<HTMLDivElement>) => {
      if (!e.ctrlKey) {
        return;
      }

      switch (e.key) {
        case "`":
          e.preventDefault();
          const [match] = Editor.nodes(editor, {
            match: (n: any) => n.type === "heading",
          });
          Transforms.setNodes(
            editor,
            {
              type: match ? "paragraph" : "heading",
            },
            {
              match: (n) => Element.isElement(n) && Editor.isBlock(editor, n),
            }
          );
          break;
        case "b":
          e.preventDefault();
          const marks = Editor.marks(editor);
          if (marks?.bold) {
            Editor.removeMark(editor, "bold");
          } else {
            Editor.addMark(editor, "bold", true);
          }
          break;
      }
    },
    [editor]
  );

  return (
    <Slate editor={editor} initialValue={initialValue}>
      <Editable
        renderLeaf={renderLeaf}
        renderElement={renderElement}
        onKeyDown={onKeyDown}
      />
    </Slate>
  );
}

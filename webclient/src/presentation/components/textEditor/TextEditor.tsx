import { KeyboardEvent, useCallback, useState } from "react";
import { createEditor, Descendant, Editor, Element, Transforms } from "slate";
import { withHistory } from "slate-history";
import { Slate, Editable, withReact, RenderElementProps } from "slate-react";
import { Heading1 } from "./ElementRenderers";

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

export default function TextEditor() {
  const [editor] = useState(() => withReact(withHistory(createEditor())));

  const onKeyDown = useCallback(
    (e: KeyboardEvent<HTMLDivElement>) => {
      e.preventDefault();

      const [match] = Editor.nodes(editor, {
        match: (n: any) => n.type === "heading",
      });

      if (e.key === "`" && e.ctrlKey) {
        Transforms.setNodes(
          editor,
          {
            type: match ? "paragraph" : "heading",
          },
          {
            match: (n) => Element.isElement(n) && Editor.isBlock(editor, n),
          }
        );
      }
    },
    [editor]
  );

  return (
    <Slate editor={editor} initialValue={initialValue}>
      <Editable renderElement={renderElement} onKeyDown={onKeyDown} />
    </Slate>
  );
}

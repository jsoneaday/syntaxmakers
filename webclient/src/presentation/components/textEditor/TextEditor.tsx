import { KeyboardEvent, useCallback, useState } from "react";
import { createEditor, Descendant } from "slate";
import { withHistory } from "slate-history";
import { Slate, Editable, withReact } from "slate-react";
import { renderElement, renderLeaf } from "./Renderers";
import { Commands } from "./Commands";
import { ElementHeaderTypeLevels } from "./ElementTypes";
import "../../theme/texteditor.css";
import { Toolbar } from "./TextEditorComponents";

interface TextEditorProps {
  readOnly: boolean;
  initialValue: Descendant[];
  currentValue: (text: string) => void;
}

export default function TextEditor({
  readOnly,
  initialValue,
  currentValue,
}: TextEditorProps) {
  const [_value, _setValue] = useState<string | null>();
  const [editor] = useState(() => withReact(withHistory(createEditor())));

  const onKeyDown = useCallback(
    (e: KeyboardEvent<HTMLDivElement>) => {
      if (!e.ctrlKey || readOnly) return;

      switch (e.key) {
        case "`":
          e.preventDefault();
          Commands.toggleHeadingBlock(editor, ElementHeaderTypeLevels.Level1);
          break;
        case "b":
          e.preventDefault();
          Commands.toggleBoldMark(editor);
          break;
      }
    },
    [editor]
  );

  const onChange = (value: Descendant[]) => {
    console.log("value", value);
    currentValue(JSON.stringify(value));
  };

  return (
    <Slate editor={editor} initialValue={initialValue} onChange={onChange}>
      {readOnly ? null : <Toolbar editor={editor} />}

      <Editable
        className="txtedit-container"
        renderLeaf={renderLeaf}
        renderElement={renderElement}
        onKeyDown={onKeyDown}
        readOnly={readOnly}
      />
    </Slate>
  );
}

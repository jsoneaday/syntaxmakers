import { useState } from "react";
import { Descendant, createEditor } from "slate";
import { withHistory } from "slate-history";
import { Slate, Editable, withReact } from "slate-react";

interface TextEditorProps {
  initialValue: Descendant[];
  readOnly: boolean;
}

const renderElement = (props: any) => <Element {...props} />;
const renderLeaf = (props: any) => <Leaf {...props} />;

export default function TextEditor({
  initialValue,
  readOnly,
}: TextEditorProps) {
  const [editor] = useState(() => withReact(withHistory(createEditor())));
  const [_value, setValue] = useState<Descendant[]>();

  const onChangeValue = (value: Descendant[]) => {
    setValue(value);
  };

  return (
    <Slate editor={editor} initialValue={initialValue} onChange={onChangeValue}>
      <Editable
        renderElement={renderElement}
        renderLeaf={renderLeaf}
        spellCheck
        readOnly={readOnly}
      />
    </Slate>
  );
}

const HOTKEYS = {
  "mod+b": "bold",
  "mod+i": "italic",
  "mod+u": "underline",
  //"mod+`": "code"
};

const LIST_TYPES = ["numbered-list", "bulleted-list"];

const Element = ({ attributes, children, element }: any) => {
  console.log(`Element ${attributes} ${children} ${element}`);
  switch (element.type) {
    case "heading-one":
      return `# ${children}`;
    case "heading-two":
      return `## ${children}`;
    default:
      return <p {...attributes}>{children}</p>;
  }
};

const Leaf = ({ attributes, children, leaf }: any) => {
  console.log(`Leaf ${attributes} ${children} ${leaf}`);

  if (leaf.bold) {
    children = <strong>{children}</strong>;
  }

  if (leaf.italic) {
    children = <em>{children}</em>;
  }

  if (leaf.underline) {
    children = <u>{children}</u>;
  }

  return <span {...attributes}>{children}</span>;
};

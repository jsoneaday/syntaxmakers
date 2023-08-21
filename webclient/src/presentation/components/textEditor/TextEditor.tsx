import { $getRoot, $getSelection, EditorState, LexicalEditor } from "lexical";
import { useEffect, useRef, useState } from "react";
import { OnChangePlugin } from "@lexical/react/LexicalOnChangePlugin";
import { LexicalComposer } from "@lexical/react/LexicalComposer";
import { PlainTextPlugin } from "@lexical/react/LexicalPlainTextPlugin";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import {
  DEFAULT_TRANSFORMERS,
  MarkdownShortcutPlugin,
} from "@lexical/react/LexicalMarkdownShortcutPlugin";
import {
  $convertToMarkdownString,
  $convertFromMarkdownString,
  TRANSFORMERS,
} from "@lexical/markdown";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { CodeNode } from "@lexical/code";
import { LinkNode } from "@lexical/link";
import { ListNode, ListItemNode } from "@lexical/list";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { HorizontalRuleNode } from "@lexical/react/LexicalHorizontalRuleNode";
import "../../theme/texteditor.css";

const theme = {};

function onError(error: Error, _editor: LexicalEditor): void {
  console.error(error);
}

interface TextEditorProps {
  initialValue: string;
}
const initialConfig = {
  namespace: "JobDescEditor",
  theme,
  onError,
  editable: true,
  nodes: [
    HorizontalRuleNode,
    CodeNode,
    HeadingNode,
    LinkNode,
    ListNode,
    ListItemNode,
    QuoteNode,
  ],
};
export default function TextEditor({ initialValue }: TextEditorProps) {
  const [_editorState, setEditorState] = useState<EditorState>();
  const editorStateRef = useRef<EditorState>();

  useEffect(() => {}, [initialValue]);

  const onChangeState = (
    editorState: EditorState,
    _editor: LexicalEditor,
    _tags: Set<string>
  ) => {
    editorStateRef.current = editorState;
    console.log("editorState", editorState.toJSON());
    setEditorState(editorState);
  };

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <RichTextPlugin
        contentEditable={<ContentEditable className="texteditor-container" />}
        placeholder={<div>Enter some text...</div>}
        ErrorBoundary={LexicalErrorBoundary}
      />
      <MarkdownShortcutPlugin transformers={TRANSFORMERS} />
      <HistoryPlugin />
      <OnChangePlugin onChange={onChangeState} />
    </LexicalComposer>
  );
}

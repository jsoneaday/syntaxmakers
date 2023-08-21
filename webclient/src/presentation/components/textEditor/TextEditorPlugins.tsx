import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { UpdateEditableStateType } from "lexical";
import { useEffect } from "react";

export function AutoFocusPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    // Focus the editor when the effect fires!
    editor.focus();
  }, [editor]);

  return null;
}

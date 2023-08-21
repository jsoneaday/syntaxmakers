import { EditorState, NodeKey } from "lexical";

declare module "lexical" {
  type IntentionallyMarkedAsDirtyElement = boolean;

  type UpdateEditableStateType = {
    dirtyElements: Map<NodeKey, IntentionallyMarkedAsDirtyElement>;
    dirtyLeaves: Set<NodeKey>;
    editorState: EditorState;
    normalizedNodes: Set<NodeKey>;
    prevEditorState: EditorState;
    tags: Set<string>;
  };
}

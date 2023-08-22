import { Editor, Element, Transforms } from "slate";
import { ElementHeaderTypeLevels, ElementTypesMap } from "./ElementTypes";
import { CustomEditor } from "../../../types/slate";

export const Commands = {
  isBoldMarked(editor: CustomEditor) {
    const marks = Editor.marks(editor);
    return marks ? marks.bold === true : false;
  },
  isHeaderBlock(editor: CustomEditor, level: ElementHeaderTypeLevels) {
    let headingType = ElementTypesMap.get(level);
    const [match] = Editor.nodes(editor, {
      match: (n: any) => n.type === headingType,
    });

    return !!match;
  },
  toggleBoldMark(editor: CustomEditor) {
    const isActive = Commands.isBoldMarked(editor);

    if (isActive) {
      Editor.removeMark(editor, "bold");
    } else {
      Editor.addMark(editor, "bold", true);
    }
  },
  toggleHeadingBlock(editor: CustomEditor, level: ElementHeaderTypeLevels) {
    const headingType = ElementTypesMap.get(level);
    const isActive = Commands.isHeaderBlock(editor, level);

    Transforms.setNodes(
      editor,
      { type: isActive ? undefined : headingType },
      { match: (n) => Element.isElement(n) && Editor.isBlock(editor, n) }
    );
  },
};

import { RenderElementProps, RenderLeafProps } from "slate-react";
import { Heading1, Heading2, Heading3, Leaf } from "./TextEditorComponents";
import { ElementTypes } from "./ElementTypes";

export const renderElement = (props: RenderElementProps) => {
  switch (props.element.type) {
    case ElementTypes.Heading1:
      return <Heading1 {...props} />;
    case ElementTypes.Heading2:
      return <Heading2 {...props} />;
    case ElementTypes.Heading3:
      return <Heading3 {...props} />;
    case "paragraph":
      return <p {...props} />;
    default:
      return <span {...props} />;
  }
};

export const renderLeaf = (props: RenderLeafProps) => {
  return <Leaf {...props} />;
};

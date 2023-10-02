import { RenderElementProps, RenderLeafProps } from "slate-react";
import { Heading1, Heading2, Heading3, Leaf } from "./TextEditorComponents";
import { ElementTypes } from "./ElementTypes";

export const renderElement = (props: RenderElementProps) => {
  console.log("element", props.element);
  switch (props.element.type) {
    case ElementTypes.Heading1:
      console.log("Heading1", props);
      return <Heading1 {...props} />;
    case ElementTypes.Heading2:
      console.log("Heading2", props);
      return <Heading2 {...props} />;
    case ElementTypes.Heading3:
      console.log("Heading3", props);
      return <Heading3 {...props} />;
    case "paragraph":
      console.log("paragraph", props);
      return <p {...props} />;
    default:
      console.log("unknown", props);
      return <span {...props} />;
  }
};

export const renderLeaf = (props: RenderLeafProps) => {
  return <Leaf {...props} />;
};

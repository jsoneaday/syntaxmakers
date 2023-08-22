import { ReactNode } from "react";

/// @ts-ignore
interface SlataeProps {
  attributes: any;
  children: ReactNode;
}

export function Heading1({ attributes, children }: SlataeProps) {
  console.log("heading attributes", attributes);
  return <h1 {...attributes}>{children}</h1>;
}

interface LeafProps {
  leaf: { bold?: boolean };
}

export function Leaf({ attributes, children, leaf }: SlataeProps & LeafProps) {
  return (
    <span {...attributes} style={{ fontWeight: leaf.bold ? "bold" : "normal" }}>
      {children}
    </span>
  );
}

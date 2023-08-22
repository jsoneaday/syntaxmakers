import { ReactNode } from "react";

/// @ts-ignore
interface SlataeProps {
  attributes: any;
  children: ReactNode;
}

export function Heading1({ attributes, children }: SlataeProps) {
  return <h1 {...attributes}>{children}</h1>;
}

export function Heading2({ attributes, children }: SlataeProps) {
  return <h2 {...attributes}>{children}</h2>;
}

export function Heading3({ attributes, children }: SlataeProps) {
  return <h3 {...attributes}>{children}</h3>;
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

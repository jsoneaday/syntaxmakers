import { ReactNode } from "react";

/// @ts-ignore
interface Heading1Props {
  attributes: any;
  children: ReactNode;
}

export function Heading1({ attributes, children }: Heading1Props) {
  console.log("heading attributes", attributes);
  return <h1 {...attributes}>{children}</h1>;
}

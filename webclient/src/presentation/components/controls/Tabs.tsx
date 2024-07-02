import { MouseEvent } from "react";

export type TabItem = {
  id: number;
  label: string;
  onClick: (e: MouseEvent<HTMLButtonElement>) => void;
};

interface TabsProps {
  selectedId: number;
  tabs: TabItem[];
}

export function Tabs({ selectedId, tabs }: TabsProps) {
  return (
    <header className="tab-container">
      {tabs.map((t) => (
        <button
          key={t.id}
          className="tab-item"
          style={
            selectedId === t.id
              ? { borderBottom: "solid 1px var(--header-border-cl)" }
              : {}
          }
          onClick={t.onClick}
        >
          {t.label}
        </button>
      ))}
    </header>
  );
}

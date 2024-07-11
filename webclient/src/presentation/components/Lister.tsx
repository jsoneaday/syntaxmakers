import { ReactNode, useEffect, useState } from "react";
import { GroupItem, KeyItem } from "../models/ModelUtils";
import "../theme/lister.css";

interface ListerProps<T extends KeyItem> {
  dataItems: T[];
  elementCreator: (item: T) => ReactNode;
}

export default function Lister<T extends KeyItem>({
  dataItems,
  elementCreator,
}: ListerProps<T>) {
  const [elements, setElements] = useState<ReactNode>();

  useEffect(() => {
    const elementList: ReactNode[] = [];

    for (let i = 0; i < dataItems.length; i++) {
      const item = dataItems[i];
      elementList.push(elementCreator(item));
    }

    setElements(elementList);
  }, [dataItems]);

  return <ul className="lister-container">{elements}</ul>;
}

interface GroupListerProps<T extends GroupItem> {
  groupItems: T[];
  elementCreator: (item: T) => ReactNode;
}

export function GroupLister<T extends GroupItem>({
  groupItems,
  elementCreator,
}: GroupListerProps<T>) {
  return (
    <>
      {groupItems.map((g) => (
        <section style={{ marginBottom: "2em", width: "100%" }}>
          <header>
            <strong>{g.title}</strong>
          </header>
          <Lister dataItems={groupItems} elementCreator={elementCreator} />
        </section>
      ))}
    </>
  );
}

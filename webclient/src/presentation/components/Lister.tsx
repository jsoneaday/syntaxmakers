import { ReactNode, useEffect, useState } from "react";
import { KeyItem } from "../models/ModelUtils";
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

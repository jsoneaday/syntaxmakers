import { ReactNode, useEffect, useState } from "react";
import { KeyItem } from "../common/utils";

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
  }, [elements]);

  return <ul>{elements}</ul>;
}

import { useEffect, useState } from "react";
import "../../theme/select.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

interface DropDownProps {
  optionItems: { name: string; value: any }[];
}

export default function DropDown({ optionItems }: DropDownProps) {
  const [options, setOptions] = useState<JSX.Element[]>();

  useEffect(() => {
    const _options = optionItems.map((item) => (
      <option key={uuidv4()} label={item.name}>
        {item.value}
      </option>
    ));

    setOptions(_options);
  }, [optionItems]);

  return <select>{options}</select>;
}

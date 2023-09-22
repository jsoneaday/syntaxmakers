import { useEffect, useState } from "react";
import "../../theme/select.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

export type OptionType = {
  name: string;
  value: any;
};

interface DropDownProps {
  keyName: string;
  label: string;
  optionItems: OptionType[];
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
  name?: string;
  value?: any;
}

export default function DropDown({
  keyName,
  label,
  optionItems,
  onChange,
  name,
  value,
}: DropDownProps) {
  const [options, setOptions] = useState<JSX.Element[]>();
  const [selectId, setSelectId] = useState("");

  useEffect(() => {
    setSelectId(uuidv4());
  }, []);

  useEffect(() => {
    const _options = optionItems.map((item) => (
      <option
        key={`${keyName}-opt-${item.name}-${item.value}`}
        label={item.name}
      >
        {item.value}
      </option>
    ));

    setOptions(_options);
  }, [optionItems]);

  return (
    <div>
      <label htmlFor={selectId}>{label}</label>
      <div className="select" style={{ marginTop: ".5em" }}>
        <select id={selectId} name={name} value={value} onChange={onChange}>
          {options}
        </select>
      </div>
    </div>
  );
}

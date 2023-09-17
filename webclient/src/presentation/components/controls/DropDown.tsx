import { useEffect, useState } from "react";
import "../../theme/select.css";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

export type OptionType = {
  name: string;
  value: any;
};

interface DropDownProps {
  label: string;
  optionItems: OptionType[];
  name?: string;
  value?: any;
}

export default function DropDown({
  label,
  optionItems,
  name,
  value,
}: DropDownProps) {
  const [options, setOptions] = useState<JSX.Element[]>();
  const [selectId, setSelectId] = useState("");
  const [localValue, setLocalValue] = useState<any>();

  useEffect(() => {
    setSelectId(uuidv4());
  }, []);

  useEffect(() => {
    const _options = optionItems.map((item) => (
      <option key={uuidv4()} label={item.name}>
        {item.value}
      </option>
    ));

    setOptions(_options);
  }, [optionItems]);

  useEffect(() => {
    setLocalValue(value);
  }, [value]);

  const onChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();
    setLocalValue(e.target.value);
  };

  return (
    <div style={{ marginRight: ".5em" }}>
      <label htmlFor={selectId}>{label}</label>
      <div className="select" style={{ marginTop: ".5em" }}>
        <select
          id={selectId}
          name={name}
          value={localValue}
          onChange={onChange}
        >
          {options}
        </select>
      </div>
    </div>
  );
}

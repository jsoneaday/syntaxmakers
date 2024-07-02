import { CSSProperties, useEffect, useState } from "react";
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
  selectStyle?: CSSProperties;
  value?: any;
  name?: string;
  isHorizontal?: boolean;
}

export default function DropDown({
  keyName,
  label,
  optionItems,
  onChange,
  selectStyle,
  value,
  name,
  isHorizontal = false,
}: DropDownProps) {
  const [options, setOptions] = useState<JSX.Element[]>();
  const [selectId, setSelectId] = useState("");

  useEffect(() => {
    console.log("select value", value);
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
    <div
      className={
        isHorizontal ? "dropdown-container-row" : "dropdown-container-col"
      }
    >
      <div style={{ marginBottom: ".5em" }}>
        <label htmlFor={selectId}>{label}</label>
      </div>
      <div className="select" style={selectStyle}>
        <select id={selectId} name={name} value={value} onChange={onChange}>
          {options}
        </select>
      </div>
    </div>
  );
}

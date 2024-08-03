import { ChangeEvent, ReactNode } from "react";
import "../../theme/checkbox.css";

interface CheckboxProps {
  isChecked: boolean;
  toggleIsChecked: () => void;
  children: ReactNode;
  name?: string;
}

export default function Checkbox({
  isChecked,
  toggleIsChecked,
  children,
  name,
}: CheckboxProps) {
  const onChange = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    toggleIsChecked();
  };

  return (
    <label className="form-control">
      <input
        type="checkbox"
        checked={isChecked}
        onChange={onChange}
        name={name}
        style={{ marginRight: ".6em" }}
      />
      {children}
    </label>
  );
}

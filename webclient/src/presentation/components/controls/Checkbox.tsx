import { ReactNode } from "react";
import "../../theme/checkbox.css";

interface CheckboxProps {
  isChecked: boolean;
  toggleIsChecked: () => void;
  children: ReactNode;
}

export default function Checkbox({
  isChecked,
  toggleIsChecked,
  children,
}: CheckboxProps) {
  const onChange = () => {
    toggleIsChecked();
  };

  return (
    <label className="form-control">
      <input
        type="checkbox"
        name="checkbox"
        checked={isChecked}
        onChange={onChange}
        style={{ marginRight: ".6em" }}
      />
      {children}
    </label>
  );
}

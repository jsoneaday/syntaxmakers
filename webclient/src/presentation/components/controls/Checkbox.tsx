import { ReactNode, useState } from "react";
import "../../theme/checkbox.css";

interface CheckboxProps {
  children: ReactNode;
}

export default function Checkbox({ children }: CheckboxProps) {
  const [isChecked, setIsChecked] = useState(false);

  const onChange = () => {
    setIsChecked(!isChecked);
  };

  return (
    <label className="form-control">
      <input
        type="checkbox"
        name="checkbox"
        checked={isChecked}
        onChange={onChange}
      />
      {children}
    </label>
  );
}

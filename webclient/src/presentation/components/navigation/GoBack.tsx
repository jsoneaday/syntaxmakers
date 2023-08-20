import { useNavigate } from "react-router-dom";
import downArrow from "../../theme/assets/down-arrow.png";
import "../../theme/goback.css";

interface GoBackProps {
  label?: string;
}

export default function GoBack({ label }: GoBackProps) {
  const navigate = useNavigate();

  const onClick = () => {
    navigate(-1);
  };

  return (
    <div onClick={onClick} className="goback-container">
      <img src={downArrow} title="Go back" className="goback-img" />
      {label ? <label>{label}</label> : null}
    </div>
  );
}

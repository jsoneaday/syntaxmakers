import "../theme/emphome.css";
import "../theme/userhome.css";
import Layout from "../components/Layout";

export default function ConfirmEmail() {
  return (
    <Layout includeLogin={false} includeEmailIsConfirmedDialog={true}>
      {null}
    </Layout>
  );
}

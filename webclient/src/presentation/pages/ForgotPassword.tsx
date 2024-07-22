import "../theme/emphome.css";
import "../theme/userhome.css";
import Layout from "../components/Layout";

export default function ForgotPassword() {
  return (
    <Layout
      includeLogin={false}
      includeEmailIsConfirmedDialog={false}
      includeForgotPassword={true}
    >
      {null}
    </Layout>
  );
}

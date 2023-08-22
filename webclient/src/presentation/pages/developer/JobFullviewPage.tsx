import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { useEffect, useState } from "react";
import "../../theme/job_full_view.css";
import { appendPlusLargeCurrency } from "../../common/CurrencyFormatter";
import Layout from "../../components/Layout";
import JobFullview from "../../components/jobs/JobFullview";

export default function JobFullviewPage() {
  const { state } = useLocation();
  const [_jobPost, setJobPost] = useState<JobPost>();
  const [_salary, setSalary] = useState("");

  useEffect(() => {
    const currentJobPost = state as JobPost;
    setJobPost(currentJobPost);
    setSalary(appendPlusLargeCurrency(currentJobPost?.salary || ""));
  }, [state]);

  return (
    <Layout>
      <JobFullview />
    </Layout>
  );
}

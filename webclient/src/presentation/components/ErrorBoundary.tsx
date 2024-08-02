import { useState, useEffect, ReactNode } from "react";

interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
}

function ErrorBoundary({
  children,
  fallback,
}: ErrorBoundaryProps): JSX.Element {
  const [hasError, setHasError] = useState(false);

  useEffect(() => {
    const errorHandler = (error: ErrorEvent): void => {
      console.log("Error caught by error boundary:", error);
      setHasError(true);
    };

    window.addEventListener("error", errorHandler);

    return () => {
      window.removeEventListener("error", errorHandler);
    };
  }, []);

  if (hasError) {
    return (fallback as JSX.Element) || <h1>Something went wrong.</h1>;
  }

  return children as JSX.Element;
}

export default ErrorBoundary;

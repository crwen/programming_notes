import { isRouteErrorResponse, useRouteError } from "react-router-dom";
import LinkButton from "./LinkButton";

function ErrorPage() {
  const error = useRouteError();
  let errorMessage: string;
  if (isRouteErrorResponse(error)) {
    errorMessage = error.data || error.statusText;
  } else if (error instanceof Error) {
    errorMessage = error.message;
  } else if (typeof error === "string") {
    errorMessage = error;
  } else {
    console.error(error);
    errorMessage = "Unknow error";
  }

  return (
    <div>
      <h1>Something went wrong 😢</h1>
      <p>{errorMessage}</p>
      <LinkButton to="-1">&larr; Go back</LinkButton>
    </div>
  );
}

export default ErrorPage;

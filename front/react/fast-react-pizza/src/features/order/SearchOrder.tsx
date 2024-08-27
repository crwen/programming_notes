import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";

export const SearchOrder = (props: {}) => {
  const [query, setQuery] = useState("");
  const navigate = useNavigate();
  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (!query) return;
    navigate(`/order/${query}`);
  };
  return (
    <form onSubmit={handleSubmit}>
      <input
        placeholder="Search order #"
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        className="w-28 rounded-full bg-yellow-200 px-4 py-2 text-sm transition-all duration-300 placeholder:text-stone-400 focus:outline-none focus:ring focus:ring-yellow-500 sm:w-64 sm:focus:w-72"
      />
    </form>
  );
};

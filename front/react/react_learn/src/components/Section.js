export default function Section({ children, title }) {
  return (
    <>
      <h2>{title}</h2>
      <article>{children}</article>
    </>
  );
}

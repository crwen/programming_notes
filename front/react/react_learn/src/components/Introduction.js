import Section from "./Section";

function Introduction() {
  return (
    <>
      <Section title={"Introduction"}>
        <p>
          React applications are entirely made out of components. Each component
          has its own data, logic, and appearance(how it works and looks). And
          it can be reused, nested inside each other, and pass data between
          them. Each component must return a block of <strong>JSX</strong>
        </p>
      </Section>

      <Section title={"JSX"}>
        <p>
          JSX is a <strong>declarative</strong> syntax to describe what
          components look like and how they work
        </p>
        <p>
          JSX is extension of JavaScript that allows us to embed JavaScript, CSS
          and React components into HTML (Each JSX element will be converted to
          a <em>React.createElement</em> function call).
        </p>
      </Section>

      <Section title={"State"}>
        <div className="flexContainer">
          <div>
            <p>
              Data that a component can hold over time, necessary for
              information that it needs to remember throughout the app's
              lifecycle.
            </p>
            <p>
              Component state: Single local component variable("Piece of state",
              "state variable")
            </p>
            <p>
              Updating component state will trigger React to re-render the
              component(call the component function again).
            </p>
            <p>
              <strong>only can be used on the top level of the function</strong>
            </p>
            <div className="codeStyle">
              <p>
                <em>const [user, setUser] = useState(null);</em>
              </p>
            </div>
          </div>
          <img src="react-arch.png" alt="React Architecture" width={"400px"} />
          <ul>
            <li>Render is triggered by updating state somewhere</li>
            <li>
              <strong>Render phase</strong>: React calls component functions and
              figures out how DOM should be updated
            </li>
            <li>
              <strong>Commit phase</strong>: React actually writes to the DOM,
              updating, inserting, and deleting elements
            </li>
            <li>Browser paint</li>
          </ul>
        </div>
      </Section>

      <Section title={"Component (Instance) Lifecycle"}>
        <p>
          <strong>
            We can define code to run at these specific points in time
          </strong>
        </p>
        <div className="flexContainer">
          <div>
            <p>These code will result in rendering over and over again. </p>
            <div className="codeStyle">
              <p>const [movies, setMovies] = useState([]);</p>
              <p>fetch(`http://www.omdbapi.com/?apikey=KEY&s=interstellar`)</p>
              <p>&emsp;.then((res) =&gt; res.json())</p>
              <p>&emsp;.then((res) =&gt; setMovies(data.Search))</p>
            </div>
            <p>
              Use <em>useEffect</em> hook, only be executed after certain
              renders
            </p>
            <div className="codeStyle">
              <p>useEffect(function () &#123; </p>
              <p>&emsp; code ...</p>

              <p>&#125;, []);</p>
            </div>
          </div>
          <ul>
            <li>
              <strong>Mount/Initial Render</strong>: :Component instance is
              rendered for the first time / Fresh state and props are created
            </li>
            <li>
              <strong>Re-Render</strong>: Happens when: State changes / Props
              changes / Parent re-render / Context changes /
            </li>
            <li>
              <strong>Unmount</strong>: Component instance is destroyed and
              removede State and props are destroyed
            </li>
          </ul>
        </div>
        <p>
          Components must be pure when it comes to <strong>render logic</strong>
          . But side effects are allowed in event handler functions!(
          <em>useEffect</em> or event handler functions)
        </p>
      </Section>
    </>
  );
}

export default Introduction;

import { createContext, useContext, useEffect, useState } from "react";

const CityContext = createContext();

// const BASE_URL = "http://localhost:9000";

const CityProvider = ({ children }) => {
  const [cities, setCities] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  useEffect(() => {
    async function fetchCities() {
      // try {
      //   setIsLoading(true);
      //   const res = await fetch(`${BASE_URL}/cities`);
      //   const data = await res.json();
      //   setCities(data);
      setCities([
        {
          cityName: "Lisbon",
          country: "Portugal",
          emoji: "🇵🇹",
          date: "2027-10-31T15:59:59.138Z",
          notes: "My favorite city so far!",
          position: {
            lat: 38.727881642324164,
            lng: -9.140900099907554,
          },
          id: 73930385,
        },
        {
          cityName: "Madrid",
          country: "Spain",
          emoji: "🇪🇸",
          date: "2027-07-15T08:22:53.976Z",
          notes: "",
          position: {
            lat: 40.46635901755316,
            lng: -3.7133789062500004,
          },
          id: 17806751,
        },
      ]);
      // } catch {
      // alert("There was an error loading data.");
      // } finally {
      setIsLoading(false);
      // }
    }
    fetchCities();
  }, []);
  return (
    <CityContext.Provider value={{ cities, isLoading }}>
      {children}
    </CityContext.Provider>
  );
};

function useCities() {
  const cities = useContext(CityContext);
  if (cities == undefined) throw new Error("Can not be used out of context!");
  return cities;
}

export { CityProvider, useCities };

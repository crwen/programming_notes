import { useCities } from "../../contexts/CityContext";
import CountryItem from "../CountryItem/CountryItem";
import Message from "../Message";
import Spanner from "../Spanner";
import styles from "./CountryList.module.css";

const CountryList = () => {
  // const CityList = (props) => {
  const { cities, isLoading } = useCities();
  if (isLoading) return <Spanner />;
  if (!cities.length)
    return <Message message="Add your first city by a city on the map" />;

  const countries = cities.reduce((arr, city) => {
    if (!arr.map((el) => el.countries).includes(city.country)) {
      return [...arr, { country: city.country, emoji: city.emoji }];
    } else {
      return arr;
    }
  }, []);
  return (
    <div className={styles.cityList}>
      <ul>
        {countries.map((country) => {
          return <CountryItem country={country} key={country.country} />;
        })}
      </ul>
    </div>
  );
};

export default CountryList;

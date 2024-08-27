import { useCities } from "../../contexts/CityContext";
import CityItem from "../CityItem";
import Spanner from "../Spanner";
// import PropTypes from "prop-types";

import styles from "./CityList.module.css";

// CityList.propTypes = {
//   cities: PropTypes.array,
//   isLoading: PropTypes.bool,
// };

function CityList() {
  const { cities, isLoading } = useCities();
  if (isLoading) return <Spanner />;
  if (!cities.length) return <p>There is no city.</p>;

  return (
    <div className={styles.cityList}>
      <ul>
        {cities.map((city) => {
          return <CityItem city={city} key={city.id} />;
        })}
      </ul>
    </div>
  );
}

export default CityList;

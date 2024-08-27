import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./index.css";
// import "./App.css";
import Product from "./pages/Product";
import Homepage from "./pages/Homepage";
import PageNotFound from "./pages/PageNotFound";
import Pricing from "./pages/Pricing";
import AppLayout from "./pages/AppLayout";
import Login from "./pages/Login";
import CityList from "./components/CityList";
import CountryList from "./components/CountryList";
import City from "./components/City/City";
import Form from "./components/Form/Form";
import { CityProvider } from "./contexts/CityContext";

function App() {
  // npm install eslint vite-plugin-eslint eslint-config-react-app --save-dev                                                                                                                                                                      15.3 s
  // npm install react-router-dom@6
  // npm i react-leaflet leaflet

  return (
    <CityProvider>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Homepage />} />
          <Route path="product" element={<Product />} />
          <Route path="pricing" element={<Pricing />} />
          <Route path="app" element={<AppLayout />}>
            <Route index element={<CityList />} />
            <Route path="cities" element={<CityList />} />
            <Route path="cities/:id" element={<City />} />
            <Route path="countries" element={<CountryList />} />
            <Route path="form" element={<Form />} />
          </Route>
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<PageNotFound />} />
        </Routes>
      </BrowserRouter>
    </CityProvider>
  );
}

export default App;

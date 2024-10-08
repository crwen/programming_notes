import { useLoaderData } from "react-router-dom";
import { getMenu } from "../../utils/apiRestaurant";
import MenuItem, { Pizza } from "./MenuItem";

function Menu() {
  const menu = useLoaderData() as Pizza[];
  return (
    <ul className="divide-y divide-stone-200 px-2">
      {menu.map((pizza: Pizza) => (
        <MenuItem pizza={pizza} key={pizza.id} />
      ))}
    </ul>
  );
}

export async function loader() {
  const menu = await getMenu();
  return menu;
}

export default Menu;

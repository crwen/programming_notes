import { useSelector } from "react-redux";
import { Link } from "react-router-dom";
import { RootState } from "../../store";
import { formatCurrency } from "../../utils/helper";
import { getTotalCartPrice, getTotalCartQuantity } from "./cartSlice";

function CartOverview() {
  const cart = useSelector((state: RootState) => state.cart.cart);
  const totalPrice = cart.reduce(
    (sum, item) => sum + item.quantity * item.unitPrice,
    0,
  );
  const numPizza = cart.reduce((sum, item) => sum + item.quantity, 0);
  // const totalPrice = useSelector(getTotalCartPrice);
  // const numPizza = useSelector(getTotalCartQuantity);
  return (
    <div className="flex items-center justify-between bg-stone-800 px-4 py-4 text-sm uppercase text-stone-200 sm:px-6 md:text-base">
      <p className="space-x-4 font-semibold text-stone-300 sm:space-x-6">
        <span>{numPizza} pizzas</span>
        <span>{formatCurrency(totalPrice)}</span>
      </p>
      <Link to="/cart">Open cart &rarr;</Link>
    </div>
  );
}

export default CartOverview;

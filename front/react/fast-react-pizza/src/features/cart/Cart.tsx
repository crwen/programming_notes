import { useSelector } from "react-redux";
import { RootState } from "../../store";
import Button from "../../ui/Button";
import LinkButton from "../../ui/LinkButton";
import CartItem from "./CartItem";

function Cart() {
  const cart = useSelector((state: RootState) => state.cart.cart);
  const username = useSelector((state: RootState) => state.user.username);

  return (
    <div className="px-4 py-3">
      <LinkButton to="/menu">&larr; Back to menu</LinkButton>

      <h2 className="mt-7 text-xl font-semibold">Your cart, {username}</h2>
      <ul className="mt-3 divide-y divide-stone-200 border-b">
        {cart.map((pizza) => (
          <CartItem item={pizza} key={pizza.id} />
        ))}
      </ul>

      <div className="mt-6 space-x-2">
        <Button to="/order/new">Order pizzas</Button>
        <Button type="secondary">Clear cart</Button>
      </div>
    </div>
  );
}

export default Cart;

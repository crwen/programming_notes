import { useDispatch } from "react-redux";
import Button from "../../ui/Button";
import { formatCurrency } from "../../utils/helper";
import { addOrderItem } from "../cart/cartSlice";
import { OrderItemDetail } from "../order/OrderItem";

export interface Pizza {
  id: number;
  name: string;
  unitPrice: number;
  ingredients: string[];
  soldOut: boolean;
  imageUrl: string;
}

interface PizzaProps {
  pizza: Pizza;
}

function MenuItem({ pizza }: PizzaProps) {
  const dispatch = useDispatch();
  const { id, name, unitPrice, ingredients, soldOut, imageUrl } = pizza;

  const onClick = () => {
    const orderItemDetail: OrderItemDetail = {
      id,
      name,
      quantity: 1,
      unitPrice,
    };
    dispatch(addOrderItem(orderItemDetail));
  };

  return (
    <li className="flex gap-4 py-2">
      <img
        src={imageUrl}
        alt={name}
        className={`h-24 ${soldOut ? "opacity-70 grayscale" : ""}`}
      />
      <div className="flex grow flex-col pt-0.5">
        <p className="font-medium">{name}</p>
        <p className="text-sm capitalize italic text-stone-500">
          {ingredients.join(", ")}
        </p>
        <div className="items-centerx mt-auto flex justify-between">
          {!soldOut ? (
            <p className="text-sm">{formatCurrency(unitPrice)}</p>
          ) : (
            <p className="text-sm font-medium uppercase text-stone-500">
              Sold out
            </p>
          )}
          {soldOut ? (
            ""
          ) : (
            <Button type="small" disabled={soldOut} onClick={onClick}>
              ADD TO CART
            </Button>
          )}
        </div>
      </div>
    </li>
  );
}

export default MenuItem;

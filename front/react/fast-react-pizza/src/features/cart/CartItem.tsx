import { useDispatch } from "react-redux";
import Button from "../../ui/Button";
import { formatCurrency } from "../../utils/helper";
import { OrderItemDetail } from "../order/OrderItem";
import { deleteItem } from "./cartSlice";

// interface PizzaItem {
//   pizzaId: number;
//   name: string;
//   quantity: number;
//   totalPrice: number;
// }

interface CartProps {
  item: OrderItemDetail;
}

function CartItem({ item }: CartProps) {
  const { id, name, quantity, unitPrice } = item;
  // const { name, quantity, totalPrice } = item;
  const totalPrice = quantity * unitPrice;
  const dispatch = useDispatch();

  const onClick = () => {
    const orderItemDetail: OrderItemDetail = {
      id,
      name,
      quantity: 1,
      unitPrice,
    };
    dispatch(deleteItem(orderItemDetail));
  };

  return (
    <li className="py-3 sm:flex sm:items-center sm:justify-between">
      <p className="mb-1 sm:mb-0">
        {quantity}&times; {name}
      </p>
      <div className="flex items-center justify-between sm:gap-6">
        <p className="text-sm font-bold">{formatCurrency(totalPrice)}</p>
        <Button type="small" onClick={onClick}>
          Delete
        </Button>
      </div>
    </li>
  );
}

export default CartItem;

import { useSelector } from "react-redux";
import { RootState } from "../../store";
import { formatCurrency } from "../../utils/helper";

interface OrderItemProps {
  item: OrderItemDetail;
  // isLoadingIngredients: boolean;
  // ingredients: string[];
}

export interface OrderItemDetail {
  id: number;
  quantity: number;
  name: string;
  unitPrice: number;
}

function OrderItem({ item }: OrderItemProps) {
  const cart = useSelector((state: RootState) => state.cart);
  console.log(cart);
  const { quantity, name, unitPrice } = item;

  return (
    <li className="py-3">
      <div className="flex items-center justify-between gap-4 text-sm">
        <p>
          <span className="font-bold">{quantity}&times;</span> {name}
        </p>
        <p className="font-bold">{formatCurrency(unitPrice)}</p>
      </div>
    </li>
  );
}

export default OrderItem;

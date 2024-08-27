import { createSlice } from "@reduxjs/toolkit";
import { OrderItemDetail } from "../order/OrderItem";

interface CartState {
  cart: OrderItemDetail[];
}

const initialState: CartState = {
  cart: [],
};

const cartSlice = createSlice({
  name: "cart",
  initialState,
  reducers: {
    addOrderItem(state, action) {
      const orderItem = action.payload as OrderItemDetail;
      const item = state.cart.find((item) => item.id === orderItem.id);
      if (item) {
        item.quantity += 1;
        item.unitPrice = orderItem.unitPrice;
      } else {
        state.cart = [...state.cart, orderItem];
      }
    },
    deleteItem(state, action) {
      const orderItem = action.payload as OrderItemDetail;
      state.cart = state.cart.filter((item) => item.id !== orderItem.id);
    },
    increaseItemQuantity(state, action) {
      const orderItem = action.payload as OrderItemDetail;
      const item = state.cart.find((item) => item.id === orderItem.id);
      if (item) {
        item.quantity += 1;
        item.unitPrice = orderItem.unitPrice;
      } else {
        state.cart = [...state.cart, orderItem];
      }
    },
    decreaseItemQuantity(state, action) {
      const orderItem = action.payload as OrderItemDetail;
      const item = state.cart.find((item) => item.id === orderItem.id);
      if (item) {
        item.quantity -= 1;
        item.unitPrice = orderItem.unitPrice;
        if (item.quantity === 0) {
          state.cart = state.cart.filter((item) => item.id !== orderItem.id);
        }
      }
    },
    clearCart(state) {
      state.cart = [];
    },
  },
});

const getTotalCartPrice = (state: CartState) =>
  state.cart.reduce((sum, item) => sum + item.quantity * item.unitPrice, 0);

const getTotalCartQuantity = (state: CartState) =>
  state.cart.reduce((sum, item) => sum + item.quantity, 0);

export const {
  addOrderItem,
  deleteItem,
  increaseItemQuantity,
  decreaseItemQuantity,
  clearCart,
} = cartSlice.actions;

export { getTotalCartQuantity, getTotalCartPrice };

export default cartSlice.reducer;

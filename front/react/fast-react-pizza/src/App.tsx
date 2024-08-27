import { RouterProvider, createBrowserRouter } from "react-router-dom";
import Home from "./ui/Home";
import Error from "./ui/Error";
import Menu, { loader as menuLoader } from "./features/menu/Menu";
import Cart from "./features/cart/Cart";
import CreateOrder, {
  action as createOrderAction,
} from "./features/order/CreateOrder";
import Order, { loader as orderLoader } from "./features/order/Order";
import { AppLayout } from "./ui/AppLayout";

const router = createBrowserRouter([
  {
    element: <AppLayout />,
    errorElement: <Error />,
    children: [
      { path: "/", element: <Home /> },
      {
        path: "menu",
        errorElement: <Error />,
        element: <Menu />,
        loader: menuLoader,
      },
      { path: "/cart", element: <Cart /> },
      {
        path: "order/new",
        element: <CreateOrder />,
        action: createOrderAction,
      },
      {
        path: "order/:orderId",
        element: <Order />,
        loader: orderLoader,
        errorElement: <Error />,
      },
    ],
  },
]);

function App() {
  // npm i -D tailwindcss@3 postcss autoprefixer
  // npx tailwindcss init -p
  // npm install -D prettier prettier-plugin-tailwindcss
  // https://github.com/tailwindlabs/prettier-plugin-tailwindcss
  // npm i @reduxjs/toolkit react-redux
  return <RouterProvider router={router}></RouterProvider>;
}

export default App;

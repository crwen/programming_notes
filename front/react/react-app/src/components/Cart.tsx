
interface Props {
  items: String[]
  onClear: () => void
}

const Cart = ({ items, onClear }: Props) => {
  return (
    <>
      <div>Cart</div>
      <ul>
        {items.map((item, index) => <li key={index}>{item}</li>)}
      </ul>

      <button onClick={onClear}>Clear</button>
    </>
  )
}

export default Cart

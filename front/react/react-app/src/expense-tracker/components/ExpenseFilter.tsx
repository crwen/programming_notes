interface Props {
  categories: readonly string[]
  onSelectCategory: (category: string) => void
}

const ExpenseFilter = ({ categories, onSelectCategory }: Props) => {
  return (
    <select className="form-select" onChange={(event) => onSelectCategory(event.target.value)}>
      {
        categories.map((category, index) =>
          <option key={index} value={category}>
            {category}
          </option>)
      }
      {/* <option value="All">All categories</option> */}
      {/* <option value="Groceries">Groceries</option> */}
      {/* <option value="Utilities">Utilities</option> */}
      {/* <option value="Entertainment">Entertainment</option> */}
    </select>
  )
}

export default ExpenseFilter

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"
import categories from "../categories"
// import ExpenseFilter from "./ExpenseFilter"

const schema = z.object({
  description: z.string().min(3, { message: 'Description should be at least 3 characters.' }),
  amount: z.number({ invalid_type_error: 'Amount is required.' }).max(100_000),
  category: z.string({ required_error: 'Category is required.' }).min(3, { message: 'Category should be at least 3 characters.' })
})


type FormData = z.infer<typeof schema>


interface Props {
  onAdd: (data: FormData) => void
}

const ExpenseForm = ({ onAdd }: Props) => {
  const { register, handleSubmit, reset, formState: { errors } } = useForm<FormData>({ resolver: zodResolver(schema) })

  const onSubmit = (data: FormData) => {
    console.log(data)
    // data.category = selectedCategory
    onAdd(data)
    reset()
  }
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className='mb-3'>
        <label htmlFor='description' className='form-label'>Description</label>
        <input
          {...register('description')}
          id='description' type='text' className='form-control' />
        {errors.description && <p className="text-danger">{errors.description.message}</p>}
      </div>
      <div className='mb-3'>
        <label htmlFor='amount' className='form-label'>Amount</label>
        <input
          {...register('amount', { valueAsNumber: true })}
          id='amount' type='number' className='form-control' />
        {errors.amount && <p className="text-danger">{errors.amount.message}</p>}
      </div>
      <div className='mb-3'>
        <label htmlFor='category' className='form-label'>Category</label>

        <select className="form-select" {...register('category')}>
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
        {/* <ExpenseFilter categories={categories} onSelectCategory={(category) => setSelectedCategory(category)}></ExpenseFilter> */}
        {/* <input */}
        {/*   {...register('category')} */}
        {/*   id='category' type='text' className='form-control' /> */}
        {/* {errors.category && <p className="text-danger">{errors.category.message}</p>} */}
      </div>

      <button type="submit" className="btn btn-primary">Submit</button>
    </form>

  )
}

export default ExpenseForm

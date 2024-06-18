import { FormEvent, useRef, useState } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { zodResolver } from '@hookform/resolvers/zod'

const schema = z.object({
  name: z.string().min(3, { message: 'The name field must be at least 3 characters.' }),
  age: z.number({ invalid_type_error: 'Age field is required.' }).min(18, { message: 'The age field must be at least 18.' })
})

type FormDateHook = z.infer<typeof schema>

// interface FormDateHook {
//   name: String
//   age: number
// }

const Form = () => {
  const nameRef = useRef<HTMLInputElement>(null)
  const ageRef = useRef<HTMLInputElement>(null)
  const person = { name: '', age: 0 }

  const [personData, setPersonDate] = useState({ name: '', age: 0 })

  const handleSubmitForm = (event: FormEvent) => {
    event.preventDefault()
    if (nameRef.current !== null) {
      person.name = nameRef.current.value
    }
    if (ageRef.current !== null) {
      person.age = parseInt(ageRef.current.value)
    }
    console.log(person);
  }


  // const { register, handleSubmit, formState: { errors } } = useForm();
  // const {
  //   register,
  //   handleSubmit,
  //   formState: { errors }
  // } = useForm<FormDateHook>();
  const {
    register,
    handleSubmit,
    formState: { errors, isValid }
  } = useForm<FormDateHook>({ resolver: zodResolver(schema) });

  return (
    <>
      <form onSubmit={handleSubmitForm}>
        <div className='mb-3'>
          <label htmlFor='name' className='form-label'>Name</label>
          <input ref={nameRef} id='name' type='text' className='form-control' />
        </div>
        <div className='mb-3'>
          <label htmlFor='age' className='form-label'>Age</label>
          <input ref={ageRef} id='age' type='number' className='form-control' />
        </div>
        <button className="btn btn-primary" type="submit">Submit</button>
      </form>

      <form onSubmit={(event) => {
        event.preventDefault()
        console.log(personData)
      }}>
        <div className='mb-3'>
          <label htmlFor='name' className='form-label'>Name</label>
          <input
            onChange={(event) => setPersonDate({ ...personData, name: event.target.value })}
            value={personData.name}
            id='name' type='text' className='form-control' />
        </div>
        <div className='mb-3'>
          <label htmlFor='age' className='form-label'>Age</label>
          <input
            onChange={(event) => setPersonDate({ ...personData, age: parseInt(event.target.value) })}
            value={personData.age}
            id='age' type='number' className='form-control' />
        </div>
        <button className="btn btn-primary" type="submit">Submit</button>
      </form>



      <form onSubmit={handleSubmit(data => console.log(data))} >
        <div className='mb-3'>
          <label htmlFor='name' className='form-label'>Name</label>

          <input
            {...register('name')}
            id='name' type='text' className='form-control' />
          {errors.name && <p className="text-danger">{errors.name.message}</p>}
          {/*
          <input
            {...register('name', { required: true, minLength: 3 })}
            id='name' type='text' className='form-control' />
          {errors.name?.type === 'required' && <p className="text-danger">The name field is required</p>}
          {errors.name?.type === 'minLength' && <p className="text-danger">The name field must be at least 3 characters</p>}
          */}
        </div>
        <div className='mb-3'>
          <label htmlFor='age' className='form-label'>Age</label>
          <input
            {...register('age', { valueAsNumber: true })}
            id='age' type='number' className='form-control' />
          {errors.age && <p className="text-danger">{errors.age.message}</p>}
        </div>
        <button disabled={!isValid} className="btn btn-primary" type="submit">Submit</button>
      </form>
    </>
  )
}

export default Form

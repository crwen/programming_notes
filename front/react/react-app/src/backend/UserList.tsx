// import axios, { AxiosError } from "axios"
// import axios, { CanceledError } from "axios"
import { CanceledError } from "../services/api-client"
import { useEffect, useState } from "react"
import userService, { User } from "../services/user-service"


const UserList = () => {
  const [users, setUsers] = useState<User[]>([])

  useEffect(() => {
    // const controller = new AbortController()
    const { request, cancel } = userService.getAll<User>()
    request.then(res => setUsers(res.data))
      .catch(err => {
        if (err instanceof CanceledError) {
          return
        }
        console.log(err)
      })
      .finally(() => {

      })
    // const fetchUsers = async () => {
    //   try {
    //     const res = await axios.get<User[]>('https://jsonplaceholder.typicode.com/users')
    //     setUsers(res.data)
    //   } catch (error) {
    //     console.log(error as AxiosError)
    //   }
    // }
    // fetchUsers()
    // return () => controller.abort()
    return () => cancel()
  }, [])
  return (
    <ul>
      {users.map(user => <li key={user.id}>{user.name}</li>)}
    </ul>
  )
}

export default UserList

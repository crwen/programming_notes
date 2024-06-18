// import apiClient from "./api-client"
import create from "./http-service";

export interface User {
  id: number;
  name: string;
  username: string;
  phone: string;
  email: string;
}

// export default new UserService()
export default create("/users");

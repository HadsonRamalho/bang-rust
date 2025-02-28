import axios from "axios";

const client = axios.create({
  baseURL: 'http://localhost:3069',
});

export { client };

import axios from "axios";

const client = axios.create({
  baseURL: 'https://g6v9psc0-3069.brs.devtunnels.ms/',
});

export { client };

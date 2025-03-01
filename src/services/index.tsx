import axios from "axios";

const client = axios.create({
  baseURL: 'https://kc9d45zp-3069.brs.devtunnels.ms/',
});

export { client };

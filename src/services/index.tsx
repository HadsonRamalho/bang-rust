import axios from "axios";

const client = axios.create({
  baseURL: 'https://j1p43lfm-3069.brs.devtunnels.ms/',
});

export { client };

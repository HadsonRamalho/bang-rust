import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import App from "./App";
import { Toaster } from "sonner";

const root = document.getElementById("root");

if (root){
    ReactDOM.createRoot(root).render(
        <BrowserRouter>
        <Toaster/>
          <Routes>
            <Route path="/" element={<App />} />
          </Routes>          
        </BrowserRouter>
      );
}

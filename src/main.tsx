import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { BrowserRouter, Route, Routes } from "react-router";
import React from "react";
import ReactDOM from "react-dom/client";
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


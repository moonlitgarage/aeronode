import React from "react";
import ReactDOM from "react-dom/client";
import "/app/globals.css";
import { RouterProvider } from "react-router-dom";
import { router } from "./router";
import { ThemeProvider } from "@/components/theme-provider"
import { Toaster } from "@/components/ui/toaster"

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeProvider>
      <RouterProvider router={router}/>
      <Toaster />
    </ThemeProvider>
  </React.StrictMode>
);

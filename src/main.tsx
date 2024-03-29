import { CssBaseline } from "@mui/material";
import { ThemeProvider } from "@mui/system";
import React from "react";
import ReactDOM from "react-dom";

import App from "./App";
import theme from "./theme";

ReactDOM.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <App />
    </ThemeProvider>
  </React.StrictMode>,
  document.getElementById("root") as HTMLElement
);

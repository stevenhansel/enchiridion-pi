import { Backdrop, CircularProgress, Snackbar } from "@mui/material";
import { useEffect, useState } from "react";
import { ApplicationError } from "./constants";
import { ApplicationContext } from "./context";

import { Authentication, Display } from "./pages";
import { DeviceInformation, getDeviceInformation } from "./tauri";

const App = () => {
  const [device, setDevice] = useState<DeviceInformation | null>(null);
  const [loading, setLoading] = useState(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [error, setError] = useState<ApplicationError | null>(null);

  useEffect(() => {
    setLoading(true);

    getDeviceInformation()
      .then((device) => {
        setIsInitialized(true);
        setLoading(false);
        setDevice(device);
      })
      .catch(() => {
        setIsInitialized(true);
        setLoading(false);
      });
  }, []);

  return (
    <ApplicationContext.Provider
      value={{ device, setDevice, loading, setLoading, error, setError }}
    >
      <div className="application-container">
        {isInitialized ? (
          <>
            <Backdrop
              sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 1 }}
              open={loading}
            >
              <CircularProgress color="inherit" />
            </Backdrop>

            {device === null ? <Authentication /> : <Display />}

            <Snackbar
              anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
              open={!!error}
              onClose={() => setError(null)}
              message={error?.message}
              key="application-error"
            />
          </>
        ) : (
          <CircularProgress
            sx={{
              position: "absolute",
              top: "50%",
              left: "50%",
              transform: "translate(-50%, -50%)",
            }}
            color="inherit"
          />
        )}
      </div>
    </ApplicationContext.Provider>
  );
};

export default App;

import {
  Backdrop,
  CircularProgress,
  Container,
  Snackbar,
  Typography,
} from "@mui/material";
import WifiOffIcon from "@mui/icons-material/WifiOff";
import { useEffect, useState } from "react";
import {
  ApplicationError,
  CAROUSEL_INTERVAL,
  NETWORK_CONNECTION_CHECK_MS,
} from "./constants";
import { ApplicationContext } from "./context";
import { useCarousel } from "./hooks";

import { Authentication, Display } from "./pages";
import { tauri, DeviceInformation } from "./tauri";

const App = () => {
  const [device, setDevice] = useState<DeviceInformation | null>(null);
  const [loading, setLoading] = useState(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [error, setError] = useState<ApplicationError | null>(null);
  const [isNetworkConnected, setIsNetworkConnected] = useState(true);

  const [carouselInterval, _setCarouselInterval] = useState(CAROUSEL_INTERVAL);

  const carousel = useCarousel(carouselInterval);
  const { pauseCarousel, continueCarousel } = carousel;

  useEffect(() => {
    setLoading(true);

    tauri
      .getDeviceInformation()
      .then((device) => {
        setIsInitialized(true);
        setLoading(false);
        setDevice(device);
      })
      .catch(() => {
        setIsInitialized(true);
        setLoading(false);
      });

    const unlistener = setInterval(async () => {
      const isNetworkConnected = await tauri.isNetworkConnected();

      if (isNetworkConnected) continueCarousel();
      else pauseCarousel();

      setIsNetworkConnected(isNetworkConnected);
    }, NETWORK_CONNECTION_CHECK_MS);

    return () => {
      clearInterval(unlistener);
    };
  }, []);

  return (
    <ApplicationContext.Provider
      value={{
        device,
        setDevice,
        loading,
        setLoading,
        error,
        setError,
        isNetworkConnected,
        setIsNetworkConnected,
        carousel,
      }}
    >
      <div className="application-container">
        {isInitialized ? (
          <>
            <Backdrop
              sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 1 }}
              open={loading || !isNetworkConnected}
            >
              {loading === true ? (
                <CircularProgress color="inherit" />
              ) : !isNetworkConnected ? (
                <Container
                  sx={{
                    display: "flex",
                    flexDirection: "column",
                    alignItems: "center",
                    justifyContent: "center",
                  }}
                >
                  <WifiOffIcon fontSize="large" sx={{ marginBottom: 2 }} />
                  <Typography variant="h6">No Internet</Typography>
                </Container>
              ) : null}
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

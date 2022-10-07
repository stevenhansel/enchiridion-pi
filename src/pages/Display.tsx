import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode, CAROUSEL_INTERVAL } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";

import { useCarousel } from "../hooks";
import {
  listenToMediaUpdateStart,
  listenToMediaUpdateEnd,
  tauri,
} from "../tauri";
import ApplicationSettings from "./ApplicationSettings";

const Display = () => {
  const { setLoading, setError } =
    useContext<ApplicationContextType>(ApplicationContext);
  const [images, setImages] = useState<string[]>([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL);

  const getAnnouncementMedias = async () => {
    try {
      setLoading(true);
      stopCarousel();

      const images = await tauri.getImages();
      if (images.length === 0) {
        setImages([]);
        setLoading(false);
        return;
      }

      setImages(images);
      startCarousel(images.length);
      setLoading(false);
    } catch (e) {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  };

  const initializeAnnouncementMedia = () => {
    getAnnouncementMedias().then(() => {
      listenToMediaUpdateStart(() => {
        setLoading(true);
      });

      listenToMediaUpdateEnd(async () => {
        await getAnnouncementMedias();
        setLoading(false);
      });
    });
  };

  const handleSettingsKeydownEvent = useCallback((event: KeyboardEvent) => {
    if (event.key === "Escape") {
      setIsSettingsOpen(true);
    }
  }, []);

  useEffect(() => {
    initializeAnnouncementMedia();

    document.addEventListener("keydown", handleSettingsKeydownEvent);
    return () => {
      document.removeEventListener("keydown", handleSettingsKeydownEvent);
    };
  }, []);

  return (
    <Box
      sx={{
        height: "100vh",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      {images.length > 0 ? (
        <>
          <img
            style={{
              position: "absolute",
              top: "50%",
              left: "50%",
              transform: "translate(-50%, -50%)",
              display: "block",
              width: "100vw",
              height: "auto",
              objectFit: "cover",
            }}
            src={images[index]}
          />
          <div style={{ position: "absolute", right: 25, bottom: 30 }}>
            <Typography>
              Created By Steven Hansel, Lukas Linardi, and Rudy Susanto
            </Typography>
          </div>
        </>
      ) : (
        <>
          <img
            style={{
              position: "absolute",
              top: "50%",
              left: "50%",
              transform: "translate(-50%, -50%)",
              display: "block",
              width: "100vw",
              height: "auto",
              objectFit: "cover",
            }}
            src="/binus.jpeg"
          />
          <div style={{ position: "absolute", right: 25, bottom: 30 }}>
            <Typography>
              Created By Steven Hansel, Lukas Linardi, and Rudy Susanto
            </Typography>
          </div>
        </>
      )}

      <ApplicationSettings
        open={isSettingsOpen}
        handleClose={() => setIsSettingsOpen(false)}
      />
    </Box>
  );
};

export default Display;

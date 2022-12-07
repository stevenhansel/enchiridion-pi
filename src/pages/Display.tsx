import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";

import {
  listenToMediaUpdateStart,
  listenToMediaUpdateEnd,
  spawnCamera,
  spawnAnnouncementConsumer,
  tauri,
} from "../tauri";
import ApplicationSettings from "./ApplicationSettings";

const Display = () => {
  const {
    setLoading,
    setError,
    isNetworkConnected,
    carousel: {
      index,
      startCarousel,
      stopCarousel,
      pauseCarousel,
      continueCarousel,
      updateMax,
    },
  } = useContext<ApplicationContextType>(ApplicationContext);
  const [images, setImages] = useState<string[]>([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const getAnnouncementMedias = async () => {
    try {
      const images = await tauri.getImages();
      if (images.length === 0) {
        setImages([]);
        return;
      }

      setImages(images);
      updateMax(images.length);
    } catch (e) {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  };

  const initializeAnnouncementMedia = () => {
    spawnAnnouncementConsumer()
	.then(() => spawnCamera())
	.then(() => getAnnouncementMedias())
	.then(() => {
	      startCarousel();

	      listenToMediaUpdateStart(() => {
		setLoading(true);
		pauseCarousel();
	      });

	      listenToMediaUpdateEnd(async () => {
		await getAnnouncementMedias();
		setLoading(false);
		continueCarousel();
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
      stopCarousel();
    };
  }, []);

  useEffect(() => {
    if (isSettingsOpen) {
      pauseCarousel();
    } else {
      continueCarousel();
    }
  }, [isSettingsOpen]);

  useEffect(() => {
    if (isNetworkConnected) {
      continueCarousel();
    } else {
      pauseCarousel();
    }
  }, [isNetworkConnected]);

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
            <Typography variant="h6">
              Created By Steven Hansel, Lukas Linardi, and Rudy Susanto
            </Typography>
            <Typography variant="h6">Computer Engineering 2022</Typography>
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
            <Typography variant="h6">
              Created By Steven Hansel, Lukas Linardi, and Rudy Susanto
            </Typography>
            <Typography variant="h6">Computer Engineering 2022</Typography>
          </div>
        </>
      )}

      <ApplicationSettings
        open={isSettingsOpen}
        handleClose={() => {
          setIsSettingsOpen(false);
        }}
      />
    </Box>
  );
};

export default Display;

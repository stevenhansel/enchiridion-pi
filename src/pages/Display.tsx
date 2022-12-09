import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { getMatches } from "@tauri-apps/api/cli";
import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";

import {
  listenToMediaUpdateStart,
  listenToMediaUpdateEnd,
  spawnCamera,
  spawnAnnouncementConsumer,
  tauri,
  Announcement,
  isCameraEnabled,
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
  const [announcements, setAnnouncements] = useState<Announcement[]>([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const getAnnouncementMedias = async () => {
    try {
      const appDataDirPath = await appDataDir();

      const rawAnnouncements = await tauri.getAnnouncements();
      const announcements = await Promise.all(
        rawAnnouncements.map(async (announcement) => {
          const image_path = await join(
            appDataDirPath,
            announcement.local_path
          );

          const local_path = convertFileSrc(image_path);

          return {
            id: announcement.id,
            announcement_id: announcement.announcement_id,
            local_path,
          };
        })
      );

      if (announcements.length === 0) {
        setAnnouncements([]);
        return;
      }

      setAnnouncements(announcements);
      updateMax(announcements.length);
    } catch (e) {
      console.error(e);
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  };

  const initializeAnnouncementMedia = async () => {
    try {
      const isCameraModuleEnabled = await isCameraEnabled();
      if (isCameraModuleEnabled) {
        await spawnCamera();
      }

      await spawnAnnouncementConsumer();
      await getAnnouncementMedias();

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
    } catch (err) {
      console.error(err);
    }
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
      {announcements.length > 0 ? (
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
            src={announcements[index].local_path}
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

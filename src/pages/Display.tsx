import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode, CAROUSEL_INTERVAL } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";
import { appDataDir, dataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import {
  listenToMediaUpdateStart,
  listenToMediaUpdateEnd,
  spawnStatusPoller,
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
      setDurations,
    },
  } = useContext<ApplicationContextType>(ApplicationContext);
  const [announcements, setAnnouncements] = useState<Announcement[]>([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const getAnnouncementMedias = async () => {
    try {
      const appDataDirPath = await appDataDir();

      const rawAnnouncements = await tauri.getAnnouncements();
      console.log('rawAnnouncements: ', rawAnnouncements);
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
            media_type: announcement.media_type,
            media_duration: announcement.media_duration,
            local_path,
          };
        })
      );

      console.log('announcements: ', announcements);

      const mediaDuration = announcements.map((a) => {
        if (a.media_duration !== null && a.media_type === "video") {
          return a.media_duration;
        } else {
          return CAROUSEL_INTERVAL;
        }
      });

      if (announcements.length === 0) {
        setAnnouncements([]);
        return;
      }
      setAnnouncements(announcements);
      updateMax(announcements.length);
      setDurations(mediaDuration);
    } catch (e) {
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

      await spawnStatusPoller();
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

  const media = () => {
    if (announcements[index].media_type === "image") {
      return (
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
      );
    } else if (announcements[index].media_type === "video") {
      return (
        <video
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
          controls
          autoPlay
          muted
        />
      );
    }
  };

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
          {media()}
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

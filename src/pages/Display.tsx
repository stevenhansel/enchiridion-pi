import React, { useCallback, useContext, useEffect, useState } from "react";
import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import { ApplicationErrorCode, CAROUSEL_INTERVAL } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";
import { MediaType } from '../hooks/useCarousel';

import {
  listenToMediaUpdateStart,
  listenToMediaUpdateEnd,
  spawnStatusPoller,
  spawnCamera,
  spawnAnnouncementConsumer,
  tauri,
  Announcement,
  isCameraEnabled,
  AnnouncementMedia,
} from "../tauri";

import ApplicationSettings from "./ApplicationSettings";

const VIDEO_OFFSET_MS = 500;

type ImageElementProps = {
  src: string;
};

const ImageElement = React.memo((props: ImageElementProps) => {
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
      src={props.src}
    />
  );
});

type VideoElementProps = {
  index: number;
  src: string;
  onEnded: () => void;
  shouldLoop: boolean;
};

const VideoElement = React.memo((props: VideoElementProps) => {
  return (
    <video
      id={`video-${props.index}`}
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
      src={props.src}
      onEnded={() => {
	props.onEnded();
      }}
      autoPlay
      muted
      loop={props.shouldLoop}
    />
  );
});

const Footer = () => {
  return (
    <div style={{ position: "absolute", right: 25, bottom: 30 }}>
      <Typography variant="h6">
        Created By Steven Hansel, Lukas Linardi, and Rudy Susanto
      </Typography>
      <Typography variant="h6">Computer Engineering 2022</Typography>
    </div>
  );
};

const Display = () => {
  const {
    setLoading,
    setError,
    isNetworkConnected,
    carousel: {
      index,
      setMediaTypes,
      startCarousel,
      stopCarousel,
      pauseCarousel,
      continueCarousel,
      updateMax,
      setDurations,
      onVideoEnd,
    },
  } = useContext<ApplicationContextType>(ApplicationContext);
  const [announcements, setAnnouncements] = useState<Announcement[]>([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);

  const getAnnouncementMedias = useCallback(async () => {
    try {
      const appDataDirPath = await appDataDir();

      const rawAnnouncements = await tauri.getAnnouncements();

      const announcements = await Promise.all(
        rawAnnouncements.map(async (announcement) => {
          let localPath = "";

          if (announcement.media_type === "image") {
            const image_path = await join(
              appDataDirPath,
              announcement.local_path
            );

            localPath = convertFileSrc(image_path);
          } else if (announcement.media_type === "video") {
            const response = await tauri.getAnnouncementMedia(announcement.announcement_id);
            localPath = (response as AnnouncementMedia).media;
          }
 
          return {
           id: announcement.id,
            announcement_id: announcement.announcement_id,
            media_type: announcement.media_type,
            media_duration: announcement.media_duration,
            local_path: localPath,
          };
        })
      );

      if (announcements.length === 0) {
        setAnnouncements([]);
	setMediaTypes([]);
	updateMax(0);

        return;
      }

      const newMediaTypes = announcements.map((announcement) => {
        if (announcement.media_type === "image") {
	  return MediaType.Image;
	} else {
	  return MediaType.Video;
	}
      });

      setAnnouncements(announcements);
      setMediaTypes(newMediaTypes);
      updateMax(announcements.length);
    } catch (e) {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  }, []);

  const initializeAnnouncementMedia = useCallback(async () => {
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
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  }, []);

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

  useEffect(() => {
    const announcement = announcements[index];

    if (announcement && announcement.media_type === "video") {
      const video = document.getElementById(
        `video-${index}`
      ) as HTMLVideoElement | null;
      if (video === null) return;

      video.pause();
      video.currentTime = 0;
      video.play();
    }
  }, [announcements, index]);

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
          {announcements.map((announcement, i) => {
            let child;
            if (announcement.media_type === "image") {
              child = <ImageElement src={announcement.local_path} />;
            } else {
              child = <VideoElement index={i} src={announcement.local_path} onEnded={onVideoEnd} shouldLoop={announcements.length === 1} />;
            }

            return (
              <Box
                key={announcement.id}
                style={{
                  display: index === i ? "block" : "none",
                }}
              >
                {child}
              </Box>
            );
          })}
          <Footer />
        </>
      ) : (
        <>
          <ImageElement src="/binus.jpeg" />
          <Footer />
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

import { Box } from "@mui/system";
import { Typography } from "@mui/material";
import { useCallback, useContext, useEffect, useState, useMemo } from "react";
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
    carouselIndex,
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
      const announcements = await Promise.all(
        rawAnnouncements.map(async (announcement) => {
	  let local_path: string;
	  if (announcement.media_type === "image") {
		  const image_path = await join(
		    appDataDirPath,
		    announcement.local_path
		  );

		  local_path = convertFileSrc(image_path);
	  } else if (announcement.media_type === "video") {
		 const response = await tauri.getAnnouncementMedia(announcement.id);
	      	 local_path = response.media; 
	  }

          return {
            id: announcement.id,
            announcement_id: announcement.announcement_id,
            media_type: announcement.media_type,
            media_duration: announcement.media_duration,
            local_path,
          };
        })
      );

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

  useEffect(() => {
    const announcement = announcements[index];
    if (announcement && announcement.media_type === "video") {
	const video = document.getElementById(`video-${index}`);

	video.currentTime = 0;
    }
  }, [announcements, index])

console.log('announcements: ', announcements);
console.log('carouselIndex.current', carouselIndex.current);

  const announcementElements = useMemo(() => {
	console.log('rerender triggered');
	if (announcements.length === 0) return;

	return announcements.map((announcement, i) => {
		return (
			<div key={i}>{announcement.media_type === "image" ? (
				<img
				  style={{
				    position: "absolute",
				    top: "50%",
				    left: "50%",
				    transform: "translate(-50%, -50%)",
				    display: carouselIndex.current === i ? "block" : "none",
				    width: "100vw",
				    height: "auto",
				    objectFit: "cover",
				  }}
				  src={announcement.local_path}
				/>
			) : (
				<video
				  id={`video-${i}`}
				  style={{
				    position: "absolute",
				    top: "50%",
				    left: "50%",
				    transform: "translate(-50%, -50%)",
				    display: carouselIndex.current === i ? "block" : "none",
				    width: "100vw",
				    height: "auto",
				    objectFit: "cover",
				  }}
				  src={announcement.local_path}
				  controls
				  autoPlay
				  muted
				/>

			)}</div>	
		);
	  })
  }, [announcements]);

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
	{announcementElements}

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

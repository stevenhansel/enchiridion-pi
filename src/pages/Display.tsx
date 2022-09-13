import { useCallback, useContext, useEffect, useState } from "react";
import { ApplicationErrorCode, CAROUSEL_INTERVAL } from "../constants";
import { ApplicationContext, ApplicationContextType } from "../context";

import { useCarousel } from "../hooks";
import { subscribeToAnnouncementUpdates, tauri } from "../tauri";

const Display = () => {
  const { setError } = useContext<ApplicationContextType>(ApplicationContext);
  const [images, setImages] = useState<string[]>([]);

  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL);

  const getAnnouncementMedias = async () => {
    try {
      stopCarousel();

      const images = await tauri.getImages();
      if (images.length === 0) {
        setImages([]);
        return;
      };

      setImages(images);
      startCarousel(images.length);
    } catch (e) {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  };

  const initialize = () => {
    const unlistener = getAnnouncementMedias()
      .then(() => {
        return subscribeToAnnouncementUpdates(() => {
          getAnnouncementMedias();
        });
      })
      .then((unlistener) => {
        return unlistener;
      });

    return unlistener;
  };

  useEffect(() => {
    initialize();
  }, []);

  return (
    <div>
      {images.length > 0 ? (
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
      ) : (
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
      )}
    </div>
  );
};

export default Display;

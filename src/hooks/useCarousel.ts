import React, { useCallback, useEffect, useState } from "react";
import { CAROUSEL_INTERVAL } from '../constants';

export enum MediaType {
  Image,
  Video
};

export type CarouselState = {
  index: number;

  mediaTypes: MediaType[],
  setMediaTypes: React.Dispatch<React.SetStateAction<MediaType[]>>

  isStart: boolean;
  isPaused: boolean;

  startCarousel: () => void;
  stopCarousel: () => void;
  pauseCarousel: () => void;
  continueCarousel: () => void;

  updateMax: (max: number) => void;
  onVideoEnd: () => void;
};


export const defaultCarouselState = (): CarouselState => ({
  index: 0,

  mediaTypes: [],
  setMediaTypes: () => {},

  isStart: false,
  isPaused: false,

  startCarousel: () => {},
  stopCarousel: () => {},
  pauseCarousel: () => {},
  continueCarousel: () => {},

  updateMax: (_max: number) => {},
  onVideoEnd: () => {},
});

const useCarousel = () => {
  const [index, setIndex] = useState(0);
  const [max, setMax] = useState<number | null>(null);
  const [isStart, setIsStart] = useState(false);
  const [isPaused, setIsPaused] = useState(false);

  const [mediaTypes, setMediaTypes] = useState<MediaType[]>([]);

  const startCarousel = useCallback(() => setIsStart(true), []);
  const stopCarousel = useCallback(() => setIsStart(false), []);
  const pauseCarousel = useCallback(() => setIsPaused(true), []);
  const continueCarousel = useCallback(() => setIsPaused(false), []);

  const updateMax = useCallback((max: number) => {
    setIndex(0);
    setMax(max);
  }, []);

  const onVideoEnd = useCallback(() => {
    setIndex((previousIndex) => {
      if (mediaTypes[previousIndex] === MediaType.Image) return previousIndex;
      const newIndex = previousIndex + 1 === max ? 0 : previousIndex + 1;

      return newIndex;
    });
  }, [max, mediaTypes]);

  useEffect(() => {
    if (isStart === false || isPaused === true) return;

    let timeout: number | undefined;

    const currentMediaType = mediaTypes[index];
    if (currentMediaType === MediaType.Image) {
      timeout = setTimeout(() => {
        setIndex((previousIndex) => {
          const updatedIndex = previousIndex + 1 === max ? 0 : previousIndex + 1;
          return updatedIndex;
        });
      }, CAROUSEL_INTERVAL);
    }

    return () => {
      if (timeout !== undefined) {
        clearTimeout(timeout);
      }
    };
  }, [index, mediaTypes, isStart, isPaused]);

  console.log(index);

  return {
    index,
    mediaTypes,
    setMediaTypes,
    isStart,
    isPaused,
    startCarousel,
    stopCarousel,
    pauseCarousel,
    continueCarousel,
    updateMax,
    onVideoEnd,
  };
};

export default useCarousel;

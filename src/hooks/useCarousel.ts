import { useCallback, useEffect, useState } from "react";

export type CarouselState = {
  index: number;
  durations: number[];
  isStart: boolean;
  isPaused: boolean;
  startCarousel: () => void;
  stopCarousel: () => void;
  pauseCarousel: () => void;
  continueCarousel: () => void;
  updateMax: (max: number) => void;
  setDurations: (durations: number[]) => void;
};

export const defaultCarouselState = (): CarouselState => ({
  index: 0,
  durations: [],
  isStart: false,
  isPaused: false,
  startCarousel: () => {},
  stopCarousel: () => {},
  pauseCarousel: () => {},
  continueCarousel: () => {},
  updateMax: (_max: number) => {},
  setDurations: (_durations: number[]) => {},
});

const useCarousel = () => {
  const [index, setIndex] = useState(0);
  const [durations, setDurations] = useState<number[]>([]);
  const [max, setMax] = useState<number | null>(null);
  const [isStart, setIsStart] = useState(false);
  const [isPaused, setIsPaused] = useState(false);

  const startCarousel = useCallback(() => setIsStart(true), []);

  const stopCarousel = useCallback(() => setIsStart(false), []);

  const pauseCarousel = useCallback(() => setIsPaused(true), []);

  const continueCarousel = useCallback(() => setIsPaused(false), []);

  const updateMax = useCallback((max: number) => setMax(max), []);

  useEffect(() => {
    if (isStart === false || isPaused === true) return;
    const timeout = setTimeout(() => {
      setIndex((previousIndex) =>
        previousIndex + 1 === max ? 0 : previousIndex + 1
      );
    }, durations[index]);

    return () => {
      clearTimeout(timeout);
    };
  }, [index, durations, isStart, isPaused]);

  return {
    index,
    durations,
    isStart,
    isPaused,
    startCarousel,
    stopCarousel,
    pauseCarousel,
    continueCarousel,
    updateMax,
    setDurations,
  };
};

export default useCarousel;

import { useCallback, useEffect, useRef, useState } from "react";

export type CarouselState = {
  index: number;
  isStart: boolean;
  isPaused: boolean;
  startCarousel: () => void;
  stopCarousel: () => void;
  pauseCarousel: () => void;
  continueCarousel: () => void;
  updateMax: (max: number) => void;
};

export const defaultCarouselState = (): CarouselState => ({
  index: 0,
  isStart: false,
  isPaused: false,
  startCarousel: () => {},
  stopCarousel: () => {},
  pauseCarousel: () => {},
  continueCarousel: () => {},
  updateMax: (_max: number) => {},
});

const useCarousel = (interval: number) => {
  const [index, setIndex] = useState(0);
  const [max, setMax] = useState<number | null>(null);
  const [isStart, setIsStart] = useState(false);
  const [isPaused, setIsPaused] = useState(false);

  const tick = useRef(0);

  const startCarousel = useCallback(() => setIsStart(true), []);

  const stopCarousel = useCallback(() => setIsStart(false), []);

  const pauseCarousel = useCallback(() => setIsPaused(true), []);

  const continueCarousel = useCallback(() => setIsPaused(false), []);

  const updateMax = useCallback((max: number) => setMax(max), []);

  useEffect(() => {
    if (!isStart) return;

    if (!tick.current && !isPaused) {
      tick.current = setInterval(() => {
        setIndex((previousIndex) =>
          previousIndex + 1 === max ? 0 : previousIndex + 1
        );
      }, interval);
    } else {
      clearInterval(tick.current);
      tick.current = 0;
    }

    return () => clearInterval(tick.current);
  }, [max, isStart, isPaused]);

  return {
    index,
    isStart,
    isPaused,
    startCarousel,
    stopCarousel,
    pauseCarousel,
    continueCarousel,
    updateMax,
  };
};

export default useCarousel;

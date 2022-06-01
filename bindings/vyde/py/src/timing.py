from .globals import Globals


class Duration:
    DURATIONS = []


    def __init__(self, frames) -> None:
        self.valid = True
        self.frames = frames
        self.start_frame = Globals.instance().frame
        self.elapsed_frames = 0

        Duration.DURATIONS.append(self)
    

    def progress(self) -> float:
        return float(self.frames - self.elapsed_frames) / float(self.frames)
    

    def tick(self, frame: int) -> None:
        assert self.valid

        self.elapsed_frames = frame - self.start_frame
        if self.elapsed_frames >= self.frames:
            self.delete()
        

    def finish(self) -> int:
        start_frames = self.elapsed_frames

        while self.valid:
            Globals.instance().tick()

        return self.elapsed_frames - start_frames


    def delete(self) -> None:
        assert self.valid

        self.valid = False
        Duration.DURATIONS.remove(self)
    

    def __iter__(self):
        return DurationIterator(origin = self, blocking = False)
    

    def blocking(self):
        return DurationIterator(origin = self, blocking = True)


    @staticmethod
    def seconds(seconds: float):
        return Duration(seconds * Globals.instance().frames_per_second)


    @staticmethod
    def update(frame: int) -> None:
        for duration in Duration.DURATIONS:
            duration.tick(frame)


class DurationIterator:
    def __init__(self, origin: Duration, blocking: bool = True) -> None:
        self.blocking = blocking
        self.origin = origin
    

    def __iter__(self):
        return self
    

    def __next__(self):
        if self.blocking:
            if self.origin.valid:
                Globals.instance().tick()
                return self.origin
            else:
                raise StopIteration()
        else:
            if self.origin.frames - self.origin.elapsed_frames >= 0:
                return self.origin
            else:
                raise StopIteration()


Globals.instance().duration_update = Duration.update

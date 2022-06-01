class Globals:
    INSTANCE = None
    

    def __init__(self) -> None:
        if Globals.INSTANCE:
            raise RuntimeError("Globals is a singleton and should not be instantiated twice.")
        
        self.frames_per_second = 60.0
        self.frame = 0
        self.duration_update = None

        Globals.INSTANCE = self
    

    def tick(self) -> int:
        self.frame += 1
        if self.duration_update: self.duration_update(self.frame)

        return self.frame
    

    @staticmethod
    def instance():
        return Globals.INSTANCE or Globals()
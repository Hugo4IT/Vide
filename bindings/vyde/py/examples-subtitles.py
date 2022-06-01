from src import *
import src.render as render


def subtitle(text: str):
    SUBTITLE_SIZE = 16.0

    position = SUBTITLE_SIZE
    for status in Duration.seconds(0.3):
        render.text(x = 0.0, y = BOTTOM + position, valign = BOTTOM, halign = CENTER, text = text)
        position = interpolate(start = SUBTITLE_SIZE, end = -16.0, by = status.progress(), using = INTERP_QUART, easing = EASE_OUT)


subtitle("Hello")
subtitle("This is played while the rest of the video keeps playing")
subtitle("But this subtitle will pause the video")
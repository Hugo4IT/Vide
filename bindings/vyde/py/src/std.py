from typing import Callable
from .timing import Duration
import math


BOTTOM = 2
CENTER = 1


def wait(duration: Duration):
    """
    Standard Library 1.0.0

    Skip frames for `duration`
    """

    while duration.tick():
        skip_frame()


"""
Standard Library 1.0.0

Await a Sequence and continue execution when done
"""
# def wait_for(sequence: Sequence, offset: Duration = Duration.seconds(0.0)):
#     wait(sequence.duration)


"""
Standard Library 1.0.0

Linear interpolation function (no smoothing)
"""
INTERP_LINEAR = lambda progress: progress


"""
Standard Library 1.0.0

Squared interpolation function (progress^2)
"""
INTERP_SQUARE = lambda progress: progress * progress


"""
Standard Library 1.0.0

Cubic interpolation function (progress^3)
"""
INTERP_CUBIC = lambda progress: progress * progress * progress


"""
Standard Library 1.0.0

Quad interpolation function (progress^4)
"""
INTERP_QUAD = lambda progress: progress * progress * progress * progress


"""
Standard Library 1.0.0

Quartic interpolation function (progress^5)
"""
INTERP_QUART = lambda progress: progress * progress * progress * progress * progress


"""
Standard Library 1.0.0

Quintic interpolation function (progress^6)
"""
INTERP_QUINT = lambda progress: progress * progress * progress * progress * progress * progress


"""
Standard Library 1.0.0

Exponential interpolation function (progress^10)
"""
INTERP_EXPO = lambda progress: progress * progress * progress * progress * progress * progress * progress * progress * progress * progress


"""
Standard Library 1.0.0

Sinus interpolation function (based on sine wave)
"""
INTERP_SINE = lambda progress: math.sin(progress * RADIANS * 0.5)


"""
Standard Library 1.0.0

Ease in
"""
EASE_IN = lambda interpolation_func, progress: 1.0 - (interpolation_func(1.0 - progress))


"""
Standard Library 1.0.0

Ease out
"""
EASE_OUT = lambda interpolation_func, progress: interpolation_func(progress)


def interpolate(
    start,
    end,
    by: float,
    using: Callable[[float], float] = INTERP_LINEAR,
    easing: Callable[[Callable[[float], float], float], float] = EASE_OUT
):
    """
    Standard Library 1.0.0

    Interpolate between two numbers using an easing function
    """

    return (end - start) * easing(using, by) + start
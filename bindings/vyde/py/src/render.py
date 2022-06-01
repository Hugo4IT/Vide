def text(
    x: float,
    y: float,
    valign: int,
    halign: int,
    text: str,
) -> (float, float):
    """
    Renders text on the screen

    Returns: size of rendered text in pixels
    """
    print(f"Render text: {text} at {x},{y} aligned by {valign} and {halign}")
    return (0.0, 0.0)
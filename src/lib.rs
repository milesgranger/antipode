//! Calculate the antipode of a geographic point.

/// Representation of a single geographic point.
pub type Coordinate = (f32, f32);

/// Calculate the antipode of a `Coordinate`
///
/// Example
/// -------
///
/// ```
/// use antipode::antipode;
///
/// let coord = (60.394306,  5.325919); // Bergen, Norway
/// let expected = (-60.394306, -174.674081);  // Somewhere off the coast of New Zealand
///
/// assert_eq!(expected, antipode(coord));
/// ```
pub fn antipode(coordinate: Coordinate) -> Coordinate {
    let lon = (coordinate.1).abs() - 180.0;
    (-coordinate.0, if coordinate.1 < 0.0 { -lon } else { lon })
}

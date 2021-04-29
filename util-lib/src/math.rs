//reminder: need to include documentation when adding new features
/// # Linearly interpolates between the starting value and ending value by the interpolation value between the two float.
///
/// Time is clamp to a range between 0 and 1, so the lerp function does not outstep either the from or to variable.
///
/// Since this function is dealing with floating point numbers there might be a rounding errors, but the approximation should be valid.
///
/// return a interpolation float result between the two float values.
/// <br/>
/// <br/>
/// ## Ex 1. Linear interpolation between the 0 and 1 with t as 0,5
///
/// ```
/// use util::math::lerp;
///
/// let half_point = lerp(0.0, 1.0, 0.5);
/// assert_eq!(half_point, 0.5);
/// ```
/// ## Ex 2. Clamped Time
///
/// ```
/// use util::math::lerp;
///
/// let clamped_max = lerp(-12.1258, 2.3124, 20.0);
/// assert!((clamped_max - 2.3124).abs() <= 0.000002);
///
/// let clamped_min = lerp(33.456, -34.011, -20.0);
/// assert!((clamped_min - 33.456).abs() <= 0.000002);
/// ```
pub fn lerp(from : f32, to : f32, time : f32) -> f32{
    from  + clamp(time, 0.,1.) * (to - from)
}


/// # Loop the value of time, so that is never larger than the length and never smaller than zero.
///
/// Since this function is dealing with floating point numbers there might be a rounding errors, but the approximation should be valid.
///
/// return the current value loop value of time.
/// <br/>
/// <br/>
/// ## Ex 1. Negative time
/// ```
/// use util::math::repeat;
///
/// // negative numbers works (it reset back to zero when -3.abs() reaches 2.)
/// // one is left if we make the time -4 then it will reset back to zero
/// let negative_t = repeat(-3., 2.);
/// assert_eq!(negative_t, 1.);
///
/// let negative_t1 = repeat(-4., 2.);
/// assert_eq!(negative_t1, 0.);
/// ```
/// ## Ex 2. Positive time
/// ```
/// use util::math::repeat;
///
/// let positive_t = repeat(3., 2.);
/// assert_eq!(positive_t, 1.);
///
/// let positive_t1 = repeat(4., 2.);
/// assert_eq!(positive_t1, 0.);
/// ```
pub fn repeat(time : f32, length : f32) -> f32{
    clamp(time - ((time / length).floor()) * length, 0., length)
}

/// #Clamps the given float within a minimum and maximum. That is no greater than max (inclusively) and no less than min (inclusively).
///
/// Since this function is dealing with floating point numbers there might be a rounding errors, but the approximation should be valid.
///
/// Return a float result between the minimum and maximum
/// <br/>
/// <br/>
/// ## Ex 1. Clamp between minimum and maximum
/// ```
/// use util::math::clamp;
///
///
/// let clamp_value1 = clamp(-13.0000021111,-13., -13.);
/// assert_eq!(clamp_value1, -13.);
///
/// let clamp_value2 = clamp(23.0123, -10.23, 0.);
/// assert_eq!(clamp_value2, 0.);
///
/// //should be equal to -5,032.84375....
/// let clamp_value3 = clamp(-5.5f32.powf(5.), -35.0123,10.3456);
/// assert_eq!(clamp_value3, -35.0123);
/// ```
pub fn clamp(target : f32, min : f32, max : f32) -> f32 {
    min.max(max.min(target))
}


/// # PingPongs the value, so it is never larger than length and never less than zero.
///
/// Since this function is dealing with floating point numbers there might be a rounding errors, but the approximation should be valid.
///
/// return the ping pong value in the range between the length (inclusively) and zero (inclusively).
///
/// ## Ex 1. Negative PingPong
/// ```
/// use util::math::ping_pong;
///
/// // negative number works (it goes to length once it reaches length it goes to zero)
/// let negative_ping_pong = ping_pong(-5.5, 2.);
/// assert_eq!(negative_ping_pong, 1.5);
///
/// let negative_ping_pong1 = ping_pong(-4.3, 2.3);
/// assert!((negative_ping_pong1 - 0.3).abs() < 0.000002);
/// ```
/// ## Ex 2. Positive PingPong
/// ```
/// use util::math::ping_pong;
///
/// let negative_ping_pong = ping_pong(360., 90.);
/// assert_eq!(negative_ping_pong, 0.);
///
/// let negative_ping_pong1 = ping_pong(4.3, 2.3);
/// assert!((negative_ping_pong1 - 0.3).abs() < 0.000002);
/// ```
pub fn ping_pong(time : f32, length : f32) -> f32{
    let time = repeat(time, length * 2 as f32);
     length - (time - length).abs()
}


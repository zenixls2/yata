use super::{IndicatorInstance, IndicatorResult};
use crate::core::{Error, OHLCV};

/// Each indicator has it's own **Configuration** with parameters
///
/// Each that config should implement `IndicatorConfig` trait
///
/// See example with [`Example Indicator`](crate::indicators::example)
// Config cannot be Copy because it might consist ov Vec-s. F.e. if indicator using Conv method with custom weights.
pub trait IndicatorConfig {
	/// Type of **State**
	type Instance: IndicatorInstance;

	/// Name of an indicator
	const NAME: &'static str;

	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Dynamically sets **Configuration** parameters
	fn set(&mut self, name: &str, value: String) -> Result<(), Error>;

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		Self::NAME
	}

	/// Returns an [`IndicatorResult`](crate::core::IndicatorResult) size processing by the indicator `(count of raw value, count of signals)`
	fn size(&self) -> (u8, u8);

	/// Initializes the **State** based on current **Configuration**
	fn init<T: OHLCV>(self, initial_value: &T) -> Result<Self::Instance, Error>
	where
		Self: Sized;

	/// Evaluates indicator config over sequence of OHLC and returns sequence of `IndicatorResult`s
	/// ```
	/// use yata::prelude::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let trix = Trix::default();
	/// let results = trix.over(&candles).unwrap();
	/// println!("{:?}", results);
	/// ```
	fn eval<T: OHLCV>(self, over_slice: &[T]) -> Result<Vec<IndicatorResult>, Error>
	where
		Self: Sized,
	{
		if over_slice.is_empty() {
			return Ok(Vec::new());
		}

		let first_element = &over_slice[0];
		let mut state = self.init(first_element)?;

		let result = over_slice.iter().map(|x| state.next(x)).collect();

		Ok(result)
	}
}

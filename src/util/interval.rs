pub struct ClosedIntervalIter {
	current: i32,
	end: i32,
}

#[derive(Debug)]
pub struct ClosedInterval<T = i32>
where
	T: PartialOrd + Copy,
{
	min: T,
	max: T,
}

pub type CInterval = ClosedInterval<i32>;

impl<T> ClosedInterval<T>
where
	T: PartialOrd + Copy + Default,
{
	pub fn new() -> ClosedInterval<T> {
		ClosedInterval {
			min: T::default(),
			max: T::default(),
		}
	}

	pub fn between(a: T, b: T) -> ClosedInterval<T> {
		if a < b {
			ClosedInterval { min: a, max: b }
		} else {
			ClosedInterval { min: b, max: a }
		}
	}

	pub fn range<I>(iter: I) -> ClosedInterval<T>
	where
		I: IntoIterator<Item = T>,
	{
		let mut iter = iter.into_iter();
		if let Some(first) = iter.next() {
			let (mut min_val, mut max_val) = (first, first);
			for v in iter {
				if v < min_val {
					min_val = v;
				}
				if v > max_val {
					max_val = v;
				}
			}
			ClosedInterval {
				min: min_val,
				max: max_val,
			}
		} else {
			ClosedInterval {
				min: T::default(),
				max: T::default(),
			}
		}
	}

	pub fn and(&self, i: &ClosedInterval<T>) -> ClosedInterval<T> {
		ClosedInterval {
			min: if self.min > i.min { self.min } else { i.min },
			max: if self.max < i.max { self.max } else { i.max },
		}
	}

	pub fn or(self, i: ClosedInterval<T>) -> ClosedInterval<T> {
		ClosedInterval {
			min: if self.min < i.min { self.min } else { i.min },
			max: if self.max > i.max { self.max } else { i.max },
		}
	}

	pub fn is_empty(self) -> bool {
		self.min > self.max
	}

	pub fn includes(&self, x:T) -> bool {
		self.min <= x && x <= self.max
	}
}

impl ClosedInterval<i32> {
	pub fn empty() -> ClosedInterval<i32> {
		ClosedInterval {
			min: 0,
			max: -1,
		}
	}
}
impl ClosedInterval<f64> {
	pub fn empty() -> ClosedInterval<f64> {
		ClosedInterval {
			min: 0.,
			max: -1.,
		}
	}
}

// Iteratorはi32専用
impl Iterator for ClosedIntervalIter {
	type Item = i32;
	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current;
		self.current = current + 1;
		if self.end > current {
			Some(current)
		} else {
			None
		}
	}
}

impl IntoIterator for ClosedInterval<i32> {
	type Item = i32;
	type IntoIter = ClosedIntervalIter;
	fn into_iter(self) -> Self::IntoIter {
		ClosedIntervalIter {
			current: self.min,
			end: self.max,
		}
	}
}

impl IntoIterator for &ClosedInterval<i32> {
	type Item = i32;
	type IntoIter = ClosedIntervalIter;
	fn into_iter(self) -> Self::IntoIter {
		ClosedIntervalIter {
			current: self.min,
			end: self.max,
		}
	}
}

use super::Rgb;
use crate::{ColorTuple, ColorTupleA, Hsl};

use std::ops::{Add, AddAssign, Sub, SubAssign};

fn calc(rgb1: &Rgb, rgb2: &Rgb, is_add: bool) -> (ColorTuple, Option<f32>) {
  let Rgb { r: r1, g: g1, b: b1, a: a1 } = rgb1;
  let Rgb { r: r2, g: g2, b: b2, a: a2 } = rgb2;

  let t = if is_add { (r1 + r2, g1 + g2, b1 + b2) } else { (r1 - r2, g1 - g2, b1 - b2) };

  let a = compute_add_alpha(*a1, *a2, is_add);
  (t, a)
}

fn compute_add_alpha(a1: Option<f32>, a2: Option<f32>, is_add: bool) -> Option<f32> {
  let has1 = a1.is_some();
  let has2 = a2.is_some();
  if !has1 && !has2 {
    return None;
  } else if has1 && has2 {
    let val1 = a1.unwrap();
    let val2 = a2.unwrap();
    let val = if is_add { val1 + val2 } else { val1 - val2 };
    return Some(val);
  }

  if has1 {
    a1
  } else {
    a2
  }
}

fn add(rgb1: &Rgb, rgb2: &Rgb) -> Rgb {
  let (t, a) = calc(rgb1, rgb2, true);
  let mut rgb = Rgb::from(t);
  rgb.a = a;
  rgb
}

fn sub(rgb1: &Rgb, rgb2: &Rgb) -> Rgb {
  let (t, a) = calc(rgb1, rgb2, false);
  let mut rgb = Rgb::from(t);
  rgb.a = a;
  rgb
}

impl<'a> Add for &'a Rgb {
  type Output = Rgb;
  fn add(self, rhs: &Rgb) -> Rgb {
    add(self, rhs)
  }
}

impl<'a> Add for &'a mut Rgb {
  type Output = Rgb;
  fn add(self, rhs: &'a mut Rgb) -> Rgb {
    add(self, rhs)
  }
}

impl Add for Rgb {
  type Output = Rgb;
  fn add(self, rhs: Self) -> Self {
    add(&self, &rhs)
  }
}

impl AddAssign for Rgb {
  fn add_assign(&mut self, rhs: Self) {
    *self = add(self, &rhs);
  }
}

impl Sub for Rgb {
  type Output = Rgb;
  fn sub(self, rhs: Self) -> Self {
    sub(&self, &rhs)
  }
}

impl<'a> Sub for &'a Rgb {
  type Output = Rgb;
  fn sub(self, rhs: Self) -> Rgb {
    sub(self, rhs)
  }
}

impl<'a> Sub for &'a mut Rgb {
  type Output = Rgb;
  fn sub(self, rhs: Self) -> Rgb {
    sub(self, rhs)
  }
}

impl SubAssign for Rgb {
  fn sub_assign(&mut self, rhs: Self) {
    *self = sub(self, &rhs);
  }
}

#[test]
fn rgb_add() {
  let rgb1 = Rgb::default();
  let rgb2 = Rgb::from_hex_str("ffcc00").unwrap();
  let rgb3: ColorTupleA = (rgb1 + rgb2).into();
  assert_eq!(Into::<ColorTupleA>::into(rgb3), (255.0, 204.0, 0.0, 1.0));

  let rgb1 = Rgb::new(200.0, 200.0, 200.0, Some(0.3));
  let rgb2 = Rgb::new(200.0, 200.0, 200.0, None);
  let rgb3: ColorTupleA = (rgb1 + rgb2).into();
  assert_eq!(rgb3, (255.0, 255.0, 255.0, 0.3));
}

#[test]
fn rgb_eq() {
  let rgb1 = Rgb::default();
  let mut rgb2 = Rgb::default();
  let rgb3 = Rgb::from((12.1, 123.21321, 12.002310123));
  let hsl: Hsl = rgb3.as_ref().into();
  let rgb4 = Rgb::from(hsl);
  rgb2 += rgb3.clone();
  rgb2 -= rgb3.clone();

  let r3 = rgb3.get_red();
  let r4 = rgb4.get_red();
  println!("{:?}", r3 - r4);

  assert_eq!(rgb1, rgb2);
  assert_eq!(rgb3, rgb4);
}

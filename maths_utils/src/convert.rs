// use std::collections::HashMap;
//
// use utility_macros::gen_variants;
//
// use crate::error::{Error, ErrorKind};
//
// #[gen_variants]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum BaseUnit {
//     Second,
//     Metre,
//     SquareMetre,
//     CubicMetre,
//     Kilogram,
//     Ampere,
//     Kelvin,
//     Mole,
//     Candela,
//     Radian,
//     Steradian,
//     Hertz,
//     Newton,
//     Pascal,
//     Joule,
//     Watt,
//     Coulomb,
//     Volt,
//     Farad,
//     Ohm,
//     Siemens,
//     Weber,
//     Tesla,
//     Henry,
//     Lumen,
//     Lux,
//     Becquerel,
//     Gray,
//     Sievert,
//     Katal,
// }
// impl BaseUnit {
//     fn symbol(&self) -> &'static str {
//         match self {
//             BaseUnit::Second => "s",
//             BaseUnit::Metre => "m",
//             BaseUnit::SquareMetre => "m^2",
//             BaseUnit::CubicMetre => "m^3",
//             BaseUnit::Kilogram => "kg",
//             BaseUnit::Ampere => "A",
//             BaseUnit::Kelvin => "K",
//             BaseUnit::Mole => "mol",
//             BaseUnit::Candela => "cd",
//             BaseUnit::Radian => "rad",
//             BaseUnit::Steradian => "sr",
//             BaseUnit::Hertz => "Hz",
//             BaseUnit::Newton => "N",
//             BaseUnit::Pascal => "P",
//             BaseUnit::Joule => "J",
//             BaseUnit::Watt => "W",
//             BaseUnit::Coulomb => "C",
//             BaseUnit::Volt => "V",
//             BaseUnit::Farad => "F",
//             BaseUnit::Ohm => "Ω",
//             BaseUnit::Siemens => "S",
//             BaseUnit::Weber => "Wb",
//             BaseUnit::Tesla => "T",
//             BaseUnit::Henry => "H",
//             BaseUnit::Lumen => "lm",
//             BaseUnit::Lux => "lx",
//             BaseUnit::Becquerel => "Bq",
//             BaseUnit::Gray => "G",
//             BaseUnit::Sievert => "Sv",
//             BaseUnit::Katal => "kat",
//         }
//     }
// }
// #[derive(Clone, Copy, PartialEq)]
// #[repr(packed)]
// pub struct LinearConversion(f64, f64);
// impl LinearConversion {
//     pub const IDENTITY: LinearConversion = LinearConversion(1.0, 0.0);
//     pub fn convert_to(&self, from: f64) -> f64 {
//         from * self.0 + self.1
//     }
//     pub fn convert_from(&self, to: f64) -> f64 {
//         (to - self.1) / self.0
//     }
// }
//
// #[derive(Clone)]
// pub struct Unit {
//     name: String,
//     base: BaseUnit,
//     conversion: LinearConversion,
// }
//
// impl From<BaseUnit> for Unit {
//     fn from(value: BaseUnit) -> Self {
//         Self {
//             name: format!("{:?}", &value),
//             conversion: LinearConversion::IDENTITY,
//             base: value,
//         }
//     }
// }
// impl Unit {
//     pub fn conversion_to(&self, other: &Unit) -> Result<LinearConversion, Error> {
//         if self.base != other.base {
//             return Err(Error::new_with_message(
//                 ErrorKind::ConversionError,
//                 format!(
//                     "Cannot convert between '{:?}' and '{:?}'",
//                     self.name, other.name
//                 ),
//             ));
//         }
//         let LinearConversion(mul_a, add_a) = self.conversion;
//         let LinearConversion(mul_b, add_b) = other.conversion;
//         Ok(LinearConversion(
//             mul_b / mul_a,
//             add_b - (add_a * mul_a / mul_b),
//         ))
//     }
//     pub fn name(&self) -> &String {
//         &self.name
//     }
// }
// pub struct Convertor {
//     units: HashMap<String, Unit>,
// }
// impl Convertor {
//     pub fn new() -> Self {
//         Convertor {
//             units: HashMap::new(),
//         }
//     }
//     pub fn initialise(&mut self) -> &mut Self {
//         self.units
//             .extend(BaseUnit::VARIANTS.map(|a| (a.symbol().to_string(), a.into())));
//         self.units.insert(
//             "ft".to_string(),
//             Unit {
//                 name: "Feet".to_string(),
//                 base: BaseUnit::Metre,
//                 conversion: LinearConversion(0.3048, 0.0),
//             },
//         );
//         self.units.insert(
//             "in".to_string(),
//             Unit {
//                 name: "Inch".to_string(),
//                 base: BaseUnit::Metre,
//                 conversion: LinearConversion(0.0254, 0.0),
//             },
//         );
//         self.units.insert(
//             "mm".to_string(),
//             Unit {
//                 name: "Millimetre".to_string(),
//                 base: BaseUnit::Metre,
//                 conversion: LinearConversion(0.001, 0.0),
//             },
//         );
//         self.units.insert(
//             "cm".to_string(),
//             Unit {
//                 name: "Centimetre".to_string(),
//                 base: BaseUnit::Metre,
//                 conversion: LinearConversion(0.01, 0.0),
//             },
//         );
//         self.units.insert(
//             "pt".to_string(),
//             Unit {
//                 name: "Point".to_string(),
//                 base: BaseUnit::Metre,
//                 conversion: LinearConversion(0.0254 / 72.27, 0.0),
//             },
//         );
//         self
//     }
//     pub fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, Error> {
//         match (self.units.get(from), self.units.get(to)) {
//             (Some(a), Some(b)) => Ok(a.conversion_to(b)?.convert_to(value)),
//             (None, None) => Err(Error::new_with_message(
//                 ErrorKind::ConversionError,
//                 format!("'{:?}' and '{:?}' are unknown units.", from, to),
//             )),
//             (None, _) => Err(Error::new_with_message(
//                 ErrorKind::ConversionError,
//                 format!("'{:?}' is an unknown unit.", from),
//             )),
//             (_, None) => Err(Error::new_with_message(
//                 ErrorKind::ConversionError,
//                 format!("'{:?}' is an unknown unit.", to),
//             )),
//         }
//     }
// }
//
// #[test]
// fn conversion() -> Result<(), Error> {
//     let mut c = Convertor::new();
//     c.initialise();
//
//     eprintln!("{:?}", c.convert(1.0, "in", "pt")?);
//     Ok(())
// }
//

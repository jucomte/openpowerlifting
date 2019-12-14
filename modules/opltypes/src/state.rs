//! Defines valid entries in the MeetState column.

use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;
use strum::ParseError;

use std::fmt;

use crate::Country;

/// The State column.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    InArgentina(ArgentinaState),
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InEngland(EnglandState),
    InGermany(GermanyState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
    InRomania(RomaniaState),
    InRussia(RussiaState),
    InSouthAfrica(SouthAfricaState),
    InUSA(USAState),
}

impl State {
    /// Constructs a State for a specific Country.
    ///
    /// This is how the checker interprets the State column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Country, State, USAState};
    /// let state = State::from_str_and_country("NY", Country::USA);
    /// assert_eq!(state.unwrap(), State::InUSA(USAState::NY));
    /// ```
    pub fn from_str_and_country(s: &str, country: Country) -> Result<State, ParseError> {
        match country {
            Country::Argentina => Ok(State::InArgentina(s.parse::<ArgentinaState>()?)),
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::England => Ok(State::InEngland(s.parse::<EnglandState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::Romania => Ok(State::InRomania(s.parse::<RomaniaState>()?)),
            Country::Russia => Ok(State::InRussia(s.parse::<RussiaState>()?)),
            Country::SouthAfrica => Ok(State::InSouthAfrica(s.parse::<SouthAfricaState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }

    /// Constructs a State given a full, unambiguous code like "USA-NY".
    ///
    /// This is how the server interprets the State column.
    /// Codes of this format are the result of serializing a State value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Country, State, USAState};
    /// let state = State::from_full_code("USA-NY");
    /// assert_eq!(state.unwrap(), State::InUSA(USAState::NY));
    /// ```
    pub fn from_full_code(s: &str) -> Result<State, ParseError> {
        // The codes are of the form "{Country}-{State}".
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseError::VariantNotFound);
        }

        let country: Country = parts[0].parse::<Country>()?;
        Self::from_str_and_country(parts[1], country)
    }

    /// Returns the Country for the given State.
    pub fn to_country(self) -> Country {
        match self {
            State::InArgentina(_) => Country::Argentina,
            State::InAustralia(_) => Country::Australia,
            State::InBrazil(_) => Country::Brazil,
            State::InCanada(_) => Country::Canada,
            State::InEngland(_) => Country::England,
            State::InGermany(_) => Country::Germany,
            State::InIndia(_) => Country::India,
            State::InMexico(_) => Country::Mexico,
            State::InNetherlands(_) => Country::Netherlands,
            State::InNewZealand(_) => Country::NewZealand,
            State::InRomania(_) => Country::Romania,
            State::InRussia(_) => Country::Russia,
            State::InSouthAfrica(_) => Country::SouthAfrica,
            State::InUSA(_) => Country::USA,
        }
    }
}

impl Serialize for State {
    /// Serialization for the server. The checker uses from_str_and_country().
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state: String = match self {
            State::InArgentina(s) => s.to_string(),
            State::InAustralia(s) => s.to_string(),
            State::InBrazil(s) => s.to_string(),
            State::InCanada(s) => s.to_string(),
            State::InEngland(s) => s.to_string(),
            State::InGermany(s) => s.to_string(),
            State::InIndia(s) => s.to_string(),
            State::InMexico(s) => s.to_string(),
            State::InNetherlands(s) => s.to_string(),
            State::InNewZealand(s) => s.to_string(),
            State::InRomania(s) => s.to_string(),
            State::InRussia(s) => s.to_string(),
            State::InSouthAfrica(s) => s.to_string(),
            State::InUSA(s) => s.to_string(),
        };

        let country = self.to_country().to_string();
        format!("{}-{}", country, state).serialize(serializer)
    }
}

/// Helper struct for State deserialization.
///
/// This is only used by the server, not by the checker.
/// The checker uses from_str_and_country().
struct StateVisitor;

impl<'de> Visitor<'de> for StateVisitor {
    type Value = State;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A Country-State code like USA-NY")
    }

    fn visit_str<E>(self, value: &str) -> Result<State, E>
    where
        E: de::Error,
    {
        State::from_full_code(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<State, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StateVisitor)
    }
}

/// A state in Argentina.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum ArgentinaState {
    /// Ciudad Autónoma de Buenos Aires.
    CA,
    /// Buenos Aires.
    BA,
    /// Catamarca.
    CT,
    /// Chaco.
    CC,
    /// Chubut.
    CH,
    /// Córdoba.
    CB,
    /// Corrientes.
    CN,
    /// Entre Ríos.
    ER,
    /// Formosa.
    FM,
    /// Jujuy.
    JY,
    /// La Pampa.
    LP,
    /// La Rioja.
    LR,
    /// Mendoza.
    MZ,
    /// Misiones.
    MN,
    /// Neuquén.
    NQ,
    /// Río Negro.
    RN,
    /// Salta.
    SA,
    /// San Juan.
    SJ,
    /// San Luis.
    SL,
    /// Santa Cruz.
    SC,
    /// Santa Fe.
    SF,
    /// Santiago del Estero.
    SE,
    /// Tierra del Fuego.
    TF,
    /// Tucumán.
    TM,
}

/// A state in Australia.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum AustraliaState {
    /// Australian Capital Territory.
    ACT,
    /// Jervis Bay Territory.
    JBT,
    /// New South Wales.
    NSW,
    /// Northern Territory.
    NT,
    /// Queensland.
    QLD,
    /// South Australia.
    SA,
    /// Tasmania.
    TAS,
    /// Victoria.
    VIC,
    /// Western Australia.
    WA,
}

/// A state in Brazil.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum BrazilState {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS, MG, PA,
    PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC, SP, SE, TO
}

/// A state in Canada.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A region in England, ill-defined and used only by BP.
///
/// This omits other divisions not in England: Scotland, N.Ireland, and Wales.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum EnglandState {
    /// East Midlands.
    EM,
    /// Greater London.
    GL,
    /// North Midlands.
    NM,
    /// North West.
    NW,
    /// South East.
    SE,
    /// South West.
    SW,
    /// South Midlands.
    SM,
    /// West Midlands.
    WM,
    /// Yorkshire North East.
    YNE,
}

/// A state in Germany.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum GermanyState {
    /// Baden-Württemberg.
    BW,
    /// Bavaria.
    BY,
    /// Berlin.
    BE,
    /// Brandenburg.
    BB,
    /// Bremen.
    HB,
    /// Hesse.
    HE,
    /// Hamburg.
    HH,
    /// Mecklenburg-Vorpommern.
    MV,
    /// Lower Saxony.
    NI,
    /// North Rhine-Westphalia.
    NRW,
    /// Rhineland-Palatinate.
    RP,
    /// Schleswig-Holstein.
    SH,
    /// Saarland.
    SL,
    /// Saxony.
    SN,
    /// Saxony-Anhalt.
    ST,
    /// Thuringia.
    TH,
}

/// A state in India.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum IndiaState {
    /// Andaman and Nicobar Islands.
    AN,
    /// Andhra Pradesh.
    AP,
    /// Arunachal Pradesh.
    AR,
    /// Assam.
    AS,
    /// Bihar.
    BR,
    /// Chhattisgarh.
    CG,
    /// Chandigarh.
    CH,
    /// Daman and Diu.
    DD,
    /// Dadra and Nagar Haveli.
    DH,
    /// Delhi.
    DL,
    /// Goa.
    GA,
    /// Gujarat.
    GJ,
    /// Haryana.
    HR,
    /// Himachal Pradesh.
    HP,
    /// Jammu and Kashmir.
    JK,
    /// Jharkhand.
    JH,
    /// Karnataka.
    KA,
    /// Kerala.
    KL,
    /// Lakshadweep.
    LD,
    /// Madhya Pradesh.
    MP,
    /// Maharashtra.
    MH,
    /// Manipur.
    MN,
    /// Meghalaya.
    ML,
    /// Mizoram.
    MZ,
    /// Nagaland.
    NL,
    /// Orissa.
    OR,
    /// Punjab.
    PB,
    /// Pondicherry / Puducherry.
    PY,
    /// Rajasthan.
    RJ,
    /// Sikkim.
    SK,
    /// Tamil Nadu.
    TN,
    /// Tripura.
    TR,
    /// Uttarakhand.
    UK,
    /// Uttar Pradesh.
    UP,
    /// West Bengal.
    WB,
}

/// A state in Mexico.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum MexicoState {
    /// Aguascalientes.
    AG,
    /// Baja California.
    BC,
    /// Baja California Sur.
    BS,
    /// Campeche.
    CM,
    /// Chiapas.
    CS,
    /// Chihuahua.
    CH,
    /// Coahuila.
    CO,
    /// Colima.
    CL,
    /// Mexico City.
    DF,
    /// Durango.
    DG,
    /// Guanajuato.
    GT,
    /// Guerrero.
    GR,
    /// Hidalgo.
    HG,
    /// Jalisco.
    JA,
    /// México.
    EM,
    /// Michoacán.
    MI,
    /// Morelos.
    MO,
    /// Nayarit.
    NA,
    /// Nuevo León.
    NL,
    /// Oaxaca.
    OA,
    /// Puebla.
    PU,
    /// Querétaro.
    QT,
    /// Quintana Roo.
    QR,
    /// San Luis Potosí.
    SL,
    /// Sinaloa.
    SI,
    /// Sonora.
    SO,
    /// Tabasco.
    TB,
    /// Tamaulipas.
    TM,
    /// Tlaxcala.
    TL,
    /// Veracruz.
    VE,
    /// Yucatán.
    YU,
    /// Zacatecas.
    ZA,
}

/// A state in the Netherlands.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum NetherlandsState {
    /// Drenthe.
    DR,
    /// Flevoland.
    FL,
    /// Friesland / Fryslân.
    FR,
    /// Gelderland.
    GE,
    /// Groningen.
    GR,
    /// Limburg.
    LI,
    /// North Brabant / Noord-Brabant.
    NB,
    /// North Holland / Noord-Holland.
    NH,
    /// Overijssel / Overissel.
    OV,
    /// Utrecht.
    UT,
    /// Zeeland.
    ZE,
    /// South Holland / Zuid-Holland.
    ZH,
}

/// A state in New Zealand.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum NewZealandState {
    NTL, AKL, WKO, BOP, GIS, HKB, TKI, MWT, WGN,
    TAS, NSN, MBH, WTC, CAN, OTA, STL
}

/// A county in Romania.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum RomaniaState {
    /// Alba.
    AB,
    /// Argeș.
    AG,
    /// Arad.
    AR,
    /// Bucharest.
    B,
    /// Bacău.
    BC,
    /// Bihor.
    BH,
    /// Bistrița-Năsăud.
    BN,
    /// Brăila.
    BR,
    /// Botoșani.
    BT,
    /// Brașov.
    BV,
    /// Buzău.
    BZ,
    /// Cluj.
    CJ,
    /// Călărași.
    CL,
    /// Caraș-Severin.
    CS,
    /// Constanța.
    CT,
    /// Covasna.
    CV,
    /// Dâmbovița.
    DB,
    /// Dolj.
    DJ,
    /// Gorj.
    GJ,
    /// Galați.
    GL,
    /// Giurgiu.
    GR,
    /// Hunedoara.
    HD,
    /// Harghita.
    HR,
    /// Ilfov.
    IF,
    /// Ialomița.
    IL,
    /// Iași.
    IS,
    /// Mehedinți.
    MH,
    /// Maramureș.
    MM,
    /// Mureș.
    MS,
    /// Neamț.
    NT,
    /// Olt.
    OT,
    /// Prahova.
    PH,
    /// Sibiu.
    SB,
    /// Sălaj.
    SJ,
    /// Satu Mare.
    SM,
    /// Suceava.
    SV,
    /// Tulcea.
    TL,
    /// Timiș.
    TM,
    /// Teleorman.
    TR,
    /// Vâlcea.
    VL,
    /// Vrancea.
    VN,
    /// Vaslui.
    VS,
}

/// An oblast in Russia.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum RussiaState {
    AD, AL, BA, BU, CE, CU, DA, IN, KB, KL, KC, KR, KK, KO, ME, MO, SA,
    SE, TA, TY, UD, ALT, KAM, KHA, KDA, KYA, PER, PRO, STA, ZAB, AMU, ARK,
    AST, BEL, BRY, CHE, IRK, IVA, KGD, KLU, KEM, KIR, KOS, KGN, KRS, LEN,
    LIP, MAG, MOS, MUR, NIZ, NGR, NVS, OMS, ORE, ORL, PNZ, PSK, ROS, RYA, 
    SAK, SAM, SAR, SMO, SVE, TAM, TOM, TUL, TVE, TYE, TYU, ULY, VLA, VGG,
    VLG, VOR, YAR, MOW, SPE, YEV, CHU, KHM, NEN, YAN
}

/// A province in South Africa, using conventional acronyms (non-ISO).
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum SouthAfricaState {
    /// Eastern Cape.
    EC,
    /// Free State.
    FS,
    /// Gauteng.
    GT,
    /// KwaZulu-Natal (ISO: NL).
    KZN,
    /// Limpopo.
    LP,
    /// Mpumalanga.
    MP,
    /// Northern Cape.
    NC,
    /// North-West.
    NW,
    /// Western Cape.
    WC,
}

/// A state in the USA.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}

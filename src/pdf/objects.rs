use core::str::utf8_char_width;
use std::{collections::HashMap, ops::Range, panic:: catch_unwind, str::FromStr, usize};


use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct PDFString(String);
#[derive(Debug)]
pub struct PDFName(String);
#[derive(Debug)]
pub struct PDFDictionary(HashMap<String, PDFObject>);
#[derive(Debug)]
pub struct StreamDictionary {
    length: u64,
    filter: Option<Vec<PDFName>>,
    decode_params: Vec<PDFObject>,
}
#[derive(Debug)]
pub enum PDFObject {
    Boolean(bool),
    Integral(i64),
    Real(f64),
    String(PDFString),
    Name(PDFName),
    Array(Vec<PDFObject>),
    Dictionary(PDFDictionary),
    Stream(StreamDictionary, Vec<u8>),
    Null,
    IndirectObject(i64, i64, Box<PDFObject>),
    IndirectReference(i64, i64),
}

impl FromStr for PDFObject {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('t' | 'f') if s == "true" || s == "false" => {
                if s == "true" {
                    return Ok(PDFObject::Boolean(true));
                } else {
                    return Ok(PDFObject::Boolean(false));
                }
            }
            Some('0'..='9' | '.') => {}
            Some('(') => {}
            Some('/') => {}
            Some('[') => {}
            Some('<') => {}
            Some('s') => {}
            Some('n') => {}
            _ => todo!("{:?}", s),
        };

        todo!();
    }
}
#[derive(Debug)]
enum Delimiter {
    BeginString,
    EndString,
    BeginHexStr,
    EndHexStr,
    BeginArray,
    EndArray,
    Lcurly,
    Rcurly,
    BeginName,
    PercentSign,
    BeginDict,
    EndDict,
}

#[derive(Debug,Clone,Copy)]
enum Keyword {
    KwObj,
    KwEndObj,
    InUseEntry,
    FreeEntry,
    Xref,
    Trailer,
    Startxref,
    ObjectReference,
    EndStream,
    Stream,


}

#[derive(Debug)]
enum PDFToken {
    Comment,
    Name(String),
    Integer(i64),
    Float(f64),
    Regular(Range<usize>),
    Stream(Range<usize>),
    Delimiter(Delimiter),
    Dictionary(Vec<(PDFToken,PDFToken)>),
    Keyword(Keyword),
    EndOfLine,
    WhiteSpace,
    PDFVersion(u8,u8),
    EndOfFile,
    String(Vec<u8>),
}

struct PDFObjectParserLexer<'a> {
    v: &'a [u8],
    current_location: usize,
    stack: Vec<PDFToken>,
}
impl<'a> PDFObjectParserLexer<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            v: data,
            current_location: 0,
            stack:Vec::new(),
        }
    }
    
    fn read_character_at(&self,offset:usize) -> (usize,Result<char,u8>)  {

        let s = utf8_char_width(self.v[offset]);
        if s == 0 {
            return (s,Err(self.v[offset]))
        }
        
        let v = match str::from_utf8(&self.v[offset..offset+s]) {
            Err(_) => return (1,Err(self.v[offset])),
            Ok(a) => a,
        };


        (s,Ok(
            match catch_unwind(|| v.chars().nth(0).unwrap()) {
                Ok(a) =>a,
                Err(e) => {
                    panic!("{:08b} s = {s:?}; v= {v:?} bytes = {:?}",&self.v[offset], &self.v[offset..offset+s]);
                }
            }))
    }
    


    fn _get_token_at(&self, offset:usize) -> Result<(PDFToken,usize), Error> {
        let (mut i,a ) = self.read_character_at(self.current_location+offset);
        let v = match a {
            Ok('%') => PDFToken::Delimiter(Delimiter::PercentSign),
            Ok('(') => PDFToken::Delimiter(Delimiter::BeginString),
            Ok(')') => PDFToken::Delimiter(Delimiter::EndString),
            Ok('[') => PDFToken::Delimiter(Delimiter::BeginArray),
            Ok(']') => PDFToken::Delimiter(Delimiter::EndArray),
            Ok('{') => PDFToken::Delimiter(Delimiter::Lcurly),
            Ok('}') => PDFToken::Delimiter(Delimiter::Rcurly),
            Ok('/') => PDFToken::Delimiter(Delimiter::BeginName),
            Ok('<') => if let (s,Ok('<')) = self.read_character_at(self.current_location+offset+i) {
                i += s+1;
                    PDFToken::Delimiter(Delimiter::BeginDict)
            } else {
                PDFToken::Delimiter(Delimiter::BeginHexStr)
            }
            Ok('>') => if let (s,Ok('>')) =  self.read_character_at(self.current_location+offset+i) {
                i += s;
                    PDFToken::Delimiter(Delimiter::EndDict)
            } else {
                PDFToken::Delimiter(Delimiter::EndHexStr)
            }
            Ok('\0' | '\x09'  | '\x0c' | ' ') => {
                while let (s,Ok( '\0' | '\x09' | '\x0c' | ' ' )) = self.read_character_at(self.current_location+offset+i) {
                    i+= s;
                }
                PDFToken::WhiteSpace
            }
            Ok('\r') => {if let (s,Ok('\n')) =self.read_character_at(self.current_location+i+offset) {
                i+= s;
            }
            PDFToken::EndOfLine
            }
            Ok('\n') => PDFToken::EndOfLine,
            _ => {
                
                while let (s,Ok('\u{1}'..='\u{8}' | '\u{b}' | '\u{e}'..='\u{1f}' | '!'..='$' | '&'..='\''
            | '*'..=';' | '=' | '?'..='Z' | '^'..='z' | '|' | '~' ..= '\u{10ffff}')) =self.read_character_at(self.current_location+i+offset) {
                
                i+= s ;
            } 
            PDFToken::Regular(self.current_location+offset..self.current_location+i+offset)
                
            }
            

            
        };
        Ok((v,i))
    }
    fn __get_token(&self) -> Result<(PDFToken,usize), Error> {
        return self._get_token_at(0);
    }
    fn match_number(&self, range:Range<usize>) -> Option<PDFToken> {
        let s = match str::from_utf8(&self.v[range.clone()]) {
            Ok(s) => s,
            Err(_) => return None,
        };
        if let Ok(v) = s.parse() {
            return Some(PDFToken::Integer(v));
        }
        if let Ok(v) = s.parse() {
            return Some(PDFToken::Float(v));
        }
        return None;


    }

    fn match_keyword(&self,range:Range<usize>) -> Option<Keyword> {
        let b = &[(Keyword::KwEndObj, "endobj"),(Keyword::KwObj, "obj"),
    (Keyword::InUseEntry,"n"),
    (Keyword::FreeEntry,"f"),
    (Keyword::Xref,"xref"),
    (Keyword::Trailer,"trailer"),
    (Keyword::Startxref,"startxref"),
    (Keyword::ObjectReference, "R"),
    (Keyword::Stream, "stream"),
    (Keyword::EndStream, "endstream"),
        ];
        let v = &self.v[range];
        for (r,a) in b.iter() {
            if v.starts_with(a.as_bytes()) {
                return Some(*r);
            }
        }
        None
    }
    #[inline(always)]
    fn _preprocessor(&mut self,token:PDFToken, size:usize) -> Result<PDFToken,Error> {
        match token {
            PDFToken::Delimiter(Delimiter::PercentSign) => return self.skip_comment(),
            PDFToken::Delimiter(Delimiter::BeginName) => { return self.read_name()},
            PDFToken::Delimiter(Delimiter::BeginHexStr) => { return self.read_hex_string()}
            PDFToken::Regular(range) => if let Some(e) = self.match_keyword(range.clone()) {
                     Ok(PDFToken::Keyword(e))
                    } else if let Some(t) = self.match_number(range.clone()){
                        Ok(t)
                    } else {
                        Err(Error::new_with_message(ErrorKind::PdfParseIllegalSymbol,
                            format!("{:?} {:?} {:?}",&range,&self.v[range.clone()], str::from_utf8(&self.v[range.clone()]))))
                    }
            a => Ok(a)
        }


    }

    fn get_token(&mut self) -> Option<Result<(), Error>> {
        let (token,size) =  match self.__get_token() {
            Ok(a) => a,
            Err(e) => return Some(Err(e)),
        };
        self.current_location += size;
        let token = match self._preprocessor(token,size) {
            Ok(a) => a,
            Err(e) => return  Some(Err(e))

        };
        let v = match token  {
            PDFToken::Comment => {return self.get_token()},
            PDFToken::Keyword(Keyword::Stream) => {self.current_location+=size; Some(self.read_stream())}
            PDFToken::EndOfLine => {
                        return self.get_token();
                    },
            PDFToken::WhiteSpace => {
                        return self.get_token();
                    },
            PDFToken::EndOfFile => return None,
            t => Some(Ok(t)),
        };
        match v {
            Some(Ok(t)) => {self.stack.push(t); Some(Ok(()))},
            Some(Err(e)) =>   Some(Err(e)),
            None=>  None
        }
    }
    pub fn parse() {


    }
    
    fn skip_comment(&mut self) -> Result<PDFToken,Error> {
        let mut i = 0;
        while let Ok((c,s)) = self._get_token_at(i) {
            i+=s;
            if let PDFToken::EndOfLine = c {
                break;
            }
        }

        eprintln!("{:?}",&self.v[self.current_location..(self.current_location+i)]); 
        let v = match str::from_utf8( &self.v[self.current_location..(self.current_location+i)]) {
            Ok(v) => v,
            Err(_) => {
                self.current_location += i;
                return Ok(PDFToken::WhiteSpace);
            },
        };
        if v.trim() == "%% EOF" {
            return Ok(PDFToken::EndOfFile);
        };
        
        if self.current_location == 0 {
        self.current_location += i;
            let regex = regex::Regex::new(r"^%PDF-(\d).(\d)").unwrap();
            let c = regex.captures(v);
            if let Some(result) = c {
                let (v1,v2) = (result.get(1).unwrap().as_str().parse()?, result.get(2).unwrap().as_str().parse()?);
                return Ok(PDFToken::PDFVersion(v1, v2));
            }
        }
        self.current_location += i;

        Ok(PDFToken::Comment)
    }
    
    fn read_name(&mut self) -> Result<PDFToken, Error> {
        if let Ok(( PDFToken::Regular(range), s)) = self.__get_token() {
            let s = str::from_utf8( &self.v[range.clone()])?;
            self.current_location = range.end;
            return Ok(PDFToken::Name(s.to_string()));
        } else {
            return Err(Error::new(ErrorKind::ParseError));
        }
    }
    
    fn read_hex_string(&mut self) -> Result<PDFToken, Error> {
        let mut v = Vec::new();
        let mut d = None;
        loop {
            let (s,data)= self.read_character_at(self.current_location);
            self.current_location += s;
            let v2: u8 = match data {
                Ok('0') => 0,
                Ok('1') => 1,
                Ok('2') => 2,
                Ok('3') => 3,
                Ok('4') => 4,
                Ok('5') => 5,
                Ok('6') => 6,
                Ok('7') => 7,
                Ok('8') => 8,
                Ok('9') => 9,
                Ok('A' | 'a') => 10,
                Ok('B' | 'b') => 11,
                Ok('C' | 'c') => 12,
                Ok('D' | 'd') => 13,
                Ok('E' | 'e') => 14,
                Ok('F' | 'f') => 15,
                Ok('>') => if d.is_none() {break} else {
                    return Err(Error::new_with_message(ErrorKind::ParseError, "Hexstring with uneaven amount of hex chars"));
                },
                Ok(a) if a.is_whitespace() => continue,
                Ok(a) => return Err(Error::new_with_message(ErrorKind::ParseError, format!("Illegal character {:?} in hexstring",a))),
                Err(a) => return Err(Error::new_with_message(ErrorKind::ParseError, format!("Illegal byte {:?} in hexstring",a))),
            };
            if let Some(v1) = d {
                v.push((v1<<4u8)|v2);
                d = None;
            } else {
                d = Some(v2);
            }
        };
        dbg!(&v);
        Ok(PDFToken::String(v))
    }
    
    fn read_stream(&mut self) -> Result<PDFToken, Error> {
        todo!()
    }
}

#[allow(clippy::never_loop)]
#[test]
fn test() {
    const DATA: &[&str] = &[
        "/Users/webstones/Downloads/1-s2.0-0164121279900189-main.pdf",
        "/Users/webstones/Downloads/It_Is_Well_With_My_Soul-It_Is_Well-Ville_Du_Havre.pdf",
        "/Users/webstones/Downloads/Slides_Week_08_Morphological_Image_Processing.pdf",
        "/Users/webstones/Downloads/book.pdf",
        "/Users/webstones/Downloads/BI-529_Determination_Citizenship.pdf",
        "/Users/webstones/Downloads/1-s2.0-0263786394900329-main.pdf",
        "/Users/webstones/Downloads/1__1___1_.pdf",
        "/Users/webstones/Downloads/Ons-bring-aanbidding-Chord-chart.pdf",
        "/Users/webstones/Downloads/document.pdf",
        "/Users/webstones/Downloads/CV - Plagiarism Declaration.pdf",
        "/Users/webstones/Downloads/(2010) Caroline Houston - Iris Recognition.pdf",
        "/Users/webstones/Downloads/l2_FP_2024.pdf",
        "/Users/webstones/Downloads/cuisine.pdf",
        "/Users/webstones/Downloads/SLA-05VDC-SL-C_Datasheet.pdf",
        "/Users/webstones/Downloads/palette.pdf",
        "/Users/webstones/Downloads/tex/tex.pdf",
        "/Users/webstones/Downloads/tex/glue.pdf",
        "/Users/webstones/Downloads/FP1_23Feb_21_30.pdf",
        "/Users/webstones/Downloads/Knuth_ Digital Typography.pdf",
        "/Users/webstones/Downloads/Stille Nag A.pdf",
        "/Users/webstones/Downloads/doc.pdf",
        "/Users/webstones/Downloads/Your_SU_Student_Contract.pdf",
        "/Users/webstones/Downloads/UCT PM M2 U1 Notes.pdf",
        "/Users/webstones/Downloads/UCT PM M1 U2 Notes.pdf",
        "/Users/webstones/Downloads/Prys die Heer - A - Oase Gemeente.pdf",
        "/Users/webstones/Downloads/Forever YHWH-E.pdf",
        "/Users/webstones/Downloads/mf/trapman.pdf",
        "/Users/webstones/Downloads/mf/mfbook.pdf",
        "/Users/webstones/Downloads/23607165.pdf",
        "/Users/webstones/Downloads/Archive/report/doc.pdf",
        "/Users/webstones/Downloads/walts_no2.pdf",
        "/Users/webstones/Downloads/Project Management Journal - 2015 - Eskerod - Stakeholder Inclusiveness Enriching Project Management with General.pdf",
        "/Users/webstones/Downloads/pdfreference1.0.pdf",
        "/Users/webstones/Downloads/UCT PM M1U2 Video Transcript.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch1.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch2.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch3.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch7.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch6.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch4.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch5.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch10.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch8.pdf",
        "/Users/webstones/Downloads/haskell-examples/programming-in-haskell/Slides/PDF/ch9.pdf",
        "/Users/webstones/Downloads/A. Required documents for postgraduate studies 2025.pdf",
        "/Users/webstones/Downloads/6446038.pdf",
        "/Users/webstones/Downloads/communicationprinciples.pdf",
        "/Users/webstones/Downloads/Infographics_SUNLearn-2025.pdf",
        "/Users/webstones/Downloads/BI-154_BirthCertificate.pdf",
        "/Users/webstones/Downloads/Agreement of lease with Andre Willemse_V1.1.pdf",
        "/Users/webstones/Downloads/Sunsynk_Hybrid_Inverter_3.6_5_UserManual_v41_English.pdf",
        "/Users/webstones/Downloads/EC 10507 28 katbos R 1 796.50.pdf",
        "/Users/webstones/Downloads/test (1).pdf",
        "/Users/webstones/Downloads/Nuwe Dinge (1).pdf",
        "/Users/webstones/Downloads/biblatex.pdf",
        "/Users/webstones/Downloads/porbanz_BusseOrbanzBuhmann_2007_1.pdf",
        "/Users/webstones/Downloads/jung-woo-2004-flexible-work-breakdown-structure-for-integrated-cost-and-schedule-control.pdf",
        "/Users/webstones/Downloads/Plan-1.pdf",
        "/Users/webstones/Downloads/ERS-2009-034-ORG.pdf",
        "/Users/webstones/Downloads/assignment6.pdf",
        "/Users/webstones/Downloads/week09.pdf",
        "/Users/webstones/Downloads/IJERA_www_ijera_com.pdf",
        "/Users/webstones/Downloads/UCT_Honour_Code_and_Course_Policies.pdf",
        "/Users/webstones/Downloads/week08.pdf",
        "/Users/webstones/Downloads/D1334480585.pdf",
        "/Users/webstones/Downloads/it-is-well-with-my-soul.pdf",
        "/Users/webstones/Downloads/l1_FP_2024.pdf",
        "/Users/webstones/Downloads/Liedboek-3.0.pdf",
        "/Users/webstones/Downloads/TW793_2024_Assignment_6.pdf",
        "/Users/webstones/Downloads/subcaption.pdf",
        "/Users/webstones/Downloads/Module 3 Downloads-20250227/UCT PM M3U2 Notes.pdf",
        "/Users/webstones/Downloads/Module 3 Downloads-20250227/UCT PM M3U1 Video Transcript.pdf",
        "/Users/webstones/Downloads/Module 3 Downloads-20250227/UCT PM M3U1 Notes.pdf",
        "/Users/webstones/Downloads/TW793_2024_Assignment_4.pdf",
        "/Users/webstones/Downloads/bi154(1).pdf",
        "/Users/webstones/Downloads/assignment5.pdf",
        "/Users/webstones/Downloads/GS397169.pdf",
        "/Users/webstones/Downloads/BI-1664_Retention_Of_Citizenship.pdf",
        "/Users/webstones/Downloads/Digital_Typography_by_Donald_E.pdf",
        "/Users/webstones/Downloads/mfbook.pdf",
        "/Users/webstones/Downloads/DaVinci-Resolve-17-Fusion-Visual-Effects.pdf",
        "/Users/webstones/Downloads/Lord-I-Lift-Your-Name-On-High-Chords-G.pdf",
        "/Users/webstones/Downloads/19726945/issta.cr.pdf",
        "/Users/webstones/Downloads/19726945/ISSTA/results/Defects4j_method/Chart/results.pdf",
        "/Users/webstones/Downloads/19726945/ISSTA/results/Defects4j_method/Lang/results.pdf",
        "/Users/webstones/Downloads/memman.pdf",
        "/Users/webstones/Downloads/Main campus map revised.pdf",
        "/Users/webstones/Downloads/Sunsynk_Hybrid_Inverter_3.6_5kW_Datasheet_v23_English.pdf",
        "/Users/webstones/Downloads/11.1_Camera_matrix.pdf",
        "/Users/webstones/Downloads/test.pdf",
        "/Users/webstones/Downloads/2025_Term_Dates.pdf",
        "/Users/webstones/Downloads/ArtsAndSocialSciences.pdf",
        "/Users/webstones/Downloads/DHA-154.pdf",
        "/Users/webstones/Downloads/UCT Project Management course handbook.pdf",
        "/Users/webstones/Downloads/download/Training Documents/Training Manual Sunsynk - Part 2 v1.2.pdf",
        "/Users/webstones/Downloads/download/Training Documents/Training Manual Sunsynk - Part 1 HQ.pdf",
        "/Users/webstones/Downloads/download/ENA Approvals/ENA_TTR_260422.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Single-Phase Hybrid Inverter/Sunsynk Ecco Hybrid Inverter/3.6kW-/ 5.5kW Ecco Hybrid Inverter/Sunsynk_Ecco_SG04LP1_Datasheet_v10_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Single-Phase Hybrid Inverter/3.6kW-/ 5kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_3.6_5_Datasheet_v24_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/L-Series/SUNSYNK-L3.0/SUNSYNK-L3.0_J1148_Datasheet_v2_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/L-Series/SUNSYNK-L5.3/SUNSYNK-L5.3_J1147_Datasheet_v1_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_Datasheet_v1_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/SUN-BATT-5.12/Rack Mounted/Sunsynk_SUNBAT_5.12R_RackMounted_Datasheet_v5_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/SUN-BATT-5.12/Wall Mounted/Sunsynk_SUNBAT_5.12_WallMounted_Datasheet_v8_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Batteries/G-Series/SUNSYNK-G5.3/SUNSYNK-G5.3_J1146_Datasheet_v2_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Three-Phase Hybrid Inverter/8kW-/ 10kW-/ 12kW Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_8_10_12K_Datasheet_v26_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/German/Three-Phase Hybrid Inverter/5kW - 25kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_25_50K_Datasheet_v2_German.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/8kW Ecco Hybrid Inverter/Sunsynk_Ecco_8kW_SG05LP1_Datasheet_v9_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/3.6kW-/ 5.5kW Ecco Hybrid Inverter/Sunsynk_Ecco_SG04LP1_Datasheet_v10_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/7kW Ecco Hybrid Inverter/Sunsynk_Ecco_7kW_SG05LP1_Datasheet_v4_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/7.6kW Rack-Mounted Inverter/Sunsynk_Rack-Mounted-Inverter_Datasheet_v13_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/10kW-/ 12kW Hybrid Inverter/Sunsynk_SinglePhase_10-12kW_Datasheet_v5_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/8kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_8kW_Datasheet_v21_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/16kW Hybrid Inverter (Sunsynk MAX)/Sunsynk_Max_Datasheet_v9_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Single-Phase Hybrid Inverters/3.6kW-/ 5kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_3.6_5kW_Datasheet_v23_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Energy Meter/Energy Meter Datasheet.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Micro Inverters/Sunsynk_MicroInverter_Datasheet_v3_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/String Inverters/Single-Phase/SUN-1/1.5/2/2.5/3/3.6/4K-G04P1/Sunsynk_1K~4K-G04P1_Datasheet_v4_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/String Inverters/Three-Phase/SUN-60/70/75/80K-G04P1/Sunsynk_60K~80K-G04P1_Datasheet_v2_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Busbar/Sunsynk_Busbar_Datasheet_v5_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/MECD/MECD Datasheet.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/High Voltage Batteries/SUNSYNK-G60/Sunsynk_High Voltage-Series_Datasheet_v9_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Battery Compatibility Document/Sunsynk_BatteryCompatibility_v19_English_j1196.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Battery Compatibility Document/Low Voltage/Sunsynk_Battery Compatibility List_English(8).pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Battery Compatibility Document/High Voltage/Sunsynk_Battery Compatibility List_High Voltage_English (4).pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L3.0/Sunsynk_L3.0_Datasheet_v8_English_1148.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.3/Sunsynk_L5.3_Datasheet_v7_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_Datasheet_v6_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/CATL SSLB1 Battery/Sunsynk_CATL_Battery_Datasheet_v12_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Rack Mounted/Sunsynk_SUNBAT_5.32R_RackMounted_Datasheet_v3_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Wall Mounted/Sunsynk_SUNBAT_5.32_WallMounted_Datasheet_v3_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/SUN-BATT-10.65/Sunsynk_SUN_BATT_10.65_WallMounted_Datasheet_v2_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/W-Series/SUNSYNK-W5.3/Sunsynk_W5.3_Datasheet_v2_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/W-Series/SUNSYNK-W10.6/Sunsynk_W10.6_Datasheet_v2_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/SUN-BATT-5.12/Rack Mounted/Sunsynk_SUNBAT_5.12R_RackMounted_Datasheet_v4_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/SUN-BATT-5.12/Wall Mounted/Sunsynk_SUNBAT_5.12_WallMounted_Datasheet_v4_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/Cable Sets/Medium Cable Set Specifications  - Datasheet.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/Cable Sets/Long Cable Set Specifications  - Datasheet.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Batteries/Low Voltage Batteries/G-Series/SUNSYNK-G5.3/Sunsynk_G5.3_Datasheet_v5_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Lifelynk Inverter/Sunsynk_Lifelynk Inverter_Datasheet_v4_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Sunsynk Mobile/Lifelynk XLS/Sunsynk Mobile_Datasheet_Lifelynk 6kW XLS(6)_AW.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Sunsynk Mobile/Lifelynk S, X & XL/Sunsynk Mobile_Datasheet_Lifelynk S X & XL(11)_AW.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Three-Phase Hybrid Inverters/29.9kW - 50kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_29.9-50K_Datasheet_v15_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Three-Phase Hybrid Inverters/8kW-/ 10kW-/ 12kW Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_8_12K_Datasheet_v26_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/English/Three-Phase Hybrid Inverters/5kW - 25kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_5_25K_Datasheet_v6_English.pdf",
        "/Users/webstones/Downloads/download/Datasheets/Polish/Batteries/L-Series/SUNSYNK-L3.0/Sunsynk_L3.0_Datasheet_v2_Polish.pdf",
        "/Users/webstones/Downloads/download/Datasheets/Polish/Batteries/L-Series/SUNSYNK-L5.3/Sunsynk_L5.3_Datasheet_v1_Polish.pdf",
        "/Users/webstones/Downloads/download/Datasheets/Polish/Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_Datasheet_v1_Polish.pdf",
        "/Users/webstones/Downloads/download/Datasheets/Polish/Batteries/G-Series/SUNSYNK-G5.3/Sunsynk_G5.3_Datasheet_v2_Polish.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Single-Phase Hybrid Inverter/Sunsynk Ecco Hybrid Inverter/8kW Ecco Hybrid Inverter/Sunsynk_Ecco_SG05LP1_UserManual_v9_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Single-Phase Hybrid Inverter/Sunsynk Ecco Hybrid Inverter/3.6kW-/ 5.5kW Ecco Hybrid Inverter/Sunsynk_Ecco_SG04LP1_UserManual_v17_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Single-Phase Hybrid Inverter/8kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_8kW_UserManual_v24_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Single-Phase Hybrid Inverter/3.6kW-/ 5kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_3.6_5_UserManual_v28_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L3.0/SUNSYNK-L3.0_J1148_UserManual_v3_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.3/SUNSYNK-L5.3_J1147_UserManual_v2_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_UserManual_v3_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Rack Mounted/Sunsynk_SUNBAT_5.32R_RackMounted_UserManual_v4_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Wall Mounted/Sunsynk_SUNBAT_5.32_WallMounted_UserManual_v4_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Batteries/Low Voltage Batteries/G-Series/SUNSYNK-G5.3/SUNSYNK-G5.3_J1146_UserManual_v3_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Three-Phase Hybrid Inverter/29.9kW - 50kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_25_50K_UserManual_v16_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/German/Three-Phase Hybrid Inverter/8kW-/ 10kW-/ 12kW Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_8_10_12K_UserManual_v29_German.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/8kW Ecco Hybrid Inverter/Sunsynk_Ecco_8kW_SG05LP1_UserManual_v18_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/3.6kW-/ 5.5kW Ecco Hybrid Inverter/Sunsynk_Ecco_SG04LP1_UserManual_v26_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/Sunsynk Ecco Hybrid Inverter/7kW Ecco Hybrid Inverter/Sunsynk_Ecco_7kW_SG05LP1_UserManual_v7_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/7.6kW Rack-Mounted Inverter/Sunsynk_Rack-Mounted-Inverter_UserManual_v20_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/Other/Installing a Generator with Hybrid Inverter/InDesign-Installing-Generator-Sunsynk-HI-V2-new-bat-list.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/10kW-/ 12kW Hybrid Inverter/Sunsynk_Hybrid_Inverter_10-12kW_UserManual_v5_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/8kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_8kW_UserManual_v34_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/16kW Hybrid Inverter (Sunsynk MAX)/Sunsynk_Max_UserManual_v17_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Single-Phase Hybrid Inverters/3.6kW-/ 5kW Standard Hybrid Inverter/Sunsynk_Hybrid_Inverter_3.6_5_UserManual_v41_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Micro Inverters/Sunsynk_MicroInverter_UserManual_v3_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/String Inverters/SinglePhase/SUN-1/1.5/2/2.5/3/3.6/4K-G04P1/Sunsynk_1K~4K-G04P1_UserManual_v3_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/String Inverters/ThreePhase/SUN-60/70/75/80K-G04P1/Sunsynk_60K~80K-G04P1_UserManual_v2_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Busbar/SUNSYNK-BB-300_UserManual_v9_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/MECD/MECD User Manual HQ.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/High Voltage Batteries/SUNSYNK-G60/Sunsynk_HIGH VOLTAGE SERIES_UserManual_v23_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Battery Compatibility Document/Sunsynk_BatteryCompatibility_v19_English_j1196.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L3.0/SUNSYNK-L3.0_J1148_UserManual_v6_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.3/SUNSYNK-L5.3_J1147_UserManual_v7_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_UserManual_v8_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/CATL Battery SSLB1/Sunsynk_CATL_Battery_UserManual_v13_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Rack Mounted/Sunsynk_SUNBAT_5.32R_RackMounted_UserManual_v5_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/SUN-BATT-5.32/Wall Mounted/Sunsynk_SUNBAT_5.32_WallMounted_UserManual_v4_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/W-Series/SUNSYNK-W5.3/SUNSYNK-W5.3_UserManual_v1_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/W-Series/SUNSYNK-W10.6/SUNSYNK-W10.6_UserManual_v1_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/SUN-BATT-5.12/Rack Mounted/Sunsynk_SUNBAT_5.12R_RackMounted_UserManual_v5_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/SUN-BATT-5.12/Wall Mounted/Sunsynk_SUNBAT_5.12_WallMounted_UserManual_v9_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Batteries/Low Voltage Batteries/G-Series/SUNSYNK-G5.3/SUNSYNK-G5.3_J1146_UserManual_v8_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/SolarMan Data Logger/Inverter APP User Manual v2.0.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Commissioning & Programming Document/Commissioning-And-Programming-V4.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Grid-Tied Inverters/SUN-(60-80K-G) User Manual v1.4.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Pocket Power Station 2/Pocket Power Station 2 Manual v3.0.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk XLS/SunsynkMobile_SM6.0kWLLXLS_Lifelynk 6kW XLS_v5.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk S/SunsynkMobile_SM2.5kWLL_Parallel_v26.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk XL/SunsynkMobile_SM5.5kWLL_Parallel_v11.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Solar Pool - Water Pump Inverter/Solar Pool - Water Heater Inverter - User Manual v1.1.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/PB1000/PB1000 Rev 4.0- 01-20-21.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Pocket Power Station 1/Pocket Power Station 1 Manual v3.0.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk X/SunsynkMobile_SM3.6kWLL_Parallel_v27.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk Mini/Lifelynk Mini - User Manual v3.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk Trolley/Trolley Manual v2.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Solar Pump/Solar Pump User Manual v2.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Lifelynk Cube/Lifelynk Cube - User Manual v3.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Mobile/Light Kit/Light Kit Manual v3.0.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/OSDA Solar Module/Installation Manual - OSDA.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Three-Phase Hybrid Inverters/29.9kW - 50kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_29.9-50K_UserManual_v27_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Three-Phase Hybrid Inverters/8kW-/ 10kW-/ 12kW Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_8_12K_UserManual_v38_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Three-Phase Hybrid Inverters/5kW - 25kW HV Three-Phase Hybrid Inverter/Sunsynk_ThreePhaseHI_5_25K_UserManual_v9_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/English/Sunsynk Data Logger/Sunsynk Connect_UserManual_v12_English.pdf",
        "/Users/webstones/Downloads/download/Manuals/Polish/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L3.0/SUNSYNK-L3.0_UserManual_v3_Polish.pdf",
        "/Users/webstones/Downloads/download/Manuals/Polish/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.3/SUNSYNK-L5.3_UserManual_v2_Polish.pdf",
        "/Users/webstones/Downloads/download/Manuals/Polish/Batteries/Low Voltage Batteries/L-Series/SUNSYNK-L5.1/Sunsynk_L5.1_UserManual_v3_Polish.pdf",
        "/Users/webstones/Downloads/download/Manuals/Polish/Batteries/Low Voltage Batteries/G-Series/SUNSYNK-G5.3/SUNSYNK-G5.3_UserManual_v3_Polish.pdf",
        "/Users/webstones/Downloads/D._Knuth-The_TeXbook.pdf",
        "/Users/webstones/Downloads/babel-code.pdf",
        "/Users/webstones/Downloads/Invoice_IV1510.pdf",
        "/Users/webstones/Downloads/texbook.pdf",
        "/Users/webstones/Downloads/the-countdown-problem.pdf",
        "/Users/webstones/Downloads/sangbundel_met_kitaardrukke.pdf",
        "/Users/webstones/Downloads/pythontex_quickstart.pdf",
        "/Users/webstones/Downloads/i-speak-jesus-A.pdf",
        "/Users/webstones/Downloads/l3_fp_2024.pdf",
        "/Users/webstones/Downloads/mpman.pdf",
        "/Users/webstones/Downloads/blue_danube-a4.pdf",
        "/Users/webstones/Downloads/FFT_tutorial_NI.pdf",
        "/Users/webstones/Downloads/RD-88_eng03_W.pdf",
        "/Users/webstones/Downloads/Approved-PIL-1.pdf",
        "/Users/webstones/Downloads/23607165-rw771-Data_Science_Project_1-main/report/report.pdf",
        "/Users/webstones/Downloads/Scan2024-12-30_110803.pdf",
        "/Users/webstones/Downloads/tb130knuth-tuneup21.pdf",
        "/Users/webstones/Downloads/memoir.pdf",
        "/Users/webstones/Downloads/standard-project-management.pdf",
        "/Users/webstones/Downloads/account_statement_1-Mar-2024_to_3-Mar-2025.pdf",
        "/Users/webstones/Downloads/user-guide.pdf",
        "/Users/webstones/Downloads/webman.pdf",
        "/Users/webstones/Downloads/bb.pdf",
        "/Users/webstones/Downloads/Birth_certificate.pdf",
        "/Users/webstones/Downloads/math2412-graphs-polar-equations.pdf",
        "/Users/webstones/Downloads/father forgets.pdf",
        "/Users/webstones/Downloads/bi154.pdf",
        "/Users/webstones/Downloads/preview-9780137494224_A42842110.pdf",
        "/Users/webstones/Downloads/babel.pdf",
        "/Users/webstones/Downloads/pxespec.pdf",
        "/Users/webstones/Downloads/rogers.pdf",
        "/Users/webstones/Downloads/cups-2.3.6/examples/document-letter.pdf",
        "/Users/webstones/Downloads/cups-2.3.6/examples/testfile.pdf",
        "/Users/webstones/Downloads/cups-2.3.6/examples/onepage-letter.pdf",
        "/Users/webstones/Downloads/cups-2.3.6/examples/onepage-a4.pdf",
        "/Users/webstones/Downloads/cups-2.3.6/examples/document-a4.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227 2/UCT PM M4 U1 Notes.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227 2/UCT PM M4U1 Video Transcript.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227 2/UCT PM M4 U2 Notes.pdf",
        "/Users/webstones/Downloads/UCT PM M1 U1 Notes.pdf",
        "/Users/webstones/Downloads/UCT PM M2 U2 Notes.pdf",
        "/Users/webstones/Downloads/Orientation Module Success Team Transcript.pdf",
        "/Users/webstones/Downloads/Birth-Certificate-application-form.pdf",
        "/Users/webstones/Downloads/xetex-reference.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227/UCT PM M4 U1 Notes.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227/UCT PM M4U1 Video Transcript.pdf",
        "/Users/webstones/Downloads/Module 4 Downloads-20250227/UCT PM M4 U2 Notes.pdf",
        "/Users/webstones/Downloads/haskell2010.pdf",
        "/Users/webstones/Downloads/Plan.pdf",
        "/Users/webstones/Downloads/Great Are You Lord-chords-E.pdf",
        "/Users/webstones/Downloads/riffmci.pdf",
        "/Users/webstones/Downloads/zen.pdf",
        "/Users/webstones/Downloads/Application_for_copy_of_birth_certificate.pdf",
        "/Users/webstones/Downloads/Donald E. Knuth - Digital Typography.pdf",
        "/Users/webstones/Downloads/C074461e.pdf",
        "/Users/webstones/Downloads/Science-2025.pdf",
        
    ];
    for data in DATA {
        let f = std::fs::read(data).unwrap();
        println!("{:?}",data);
        let mut parser = PDFObjectParserLexer::new(&f);
        while let Some(i) = parser.get_token() {
            // 
            if dbg!(i).is_err() {
                break;
            }

        }
        println!("{:?}",data);
        break;
        
    }
}

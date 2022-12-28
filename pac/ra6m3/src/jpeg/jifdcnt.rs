#[doc = "Register `JIFDCNT` reader"]
pub struct R(crate::R<JIFDCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDCNT` writer"]
pub struct W(crate::W<JIFDCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<JIFDCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTSWAP` reader - Byte/Word Swap Output image data in decompression is swapped."]
pub type DOUTSWAP_R = crate::FieldReader<u8, DOUTSWAP_A>;
#[doc = "Byte/Word Swap Output image data in decompression is swapped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOUTSWAP_A {
    #[doc = "0: (1) (2) (3) (4) (5) (6) (7) (8)"]
    _000 = 0,
    #[doc = "1: (2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    _001 = 1,
    #[doc = "2: (3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    _010 = 2,
    #[doc = "3: (4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    _011 = 3,
    #[doc = "4: (5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    _100 = 4,
    #[doc = "5: (6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    _101 = 5,
    #[doc = "6: (7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    _110 = 6,
    #[doc = "7: (8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    _111 = 7,
}
impl From<DOUTSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DOUTSWAP_A) -> Self {
        variant as _
    }
}
impl DOUTSWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUTSWAP_A {
        match self.bits {
            0 => DOUTSWAP_A::_000,
            1 => DOUTSWAP_A::_001,
            2 => DOUTSWAP_A::_010,
            3 => DOUTSWAP_A::_011,
            4 => DOUTSWAP_A::_100,
            5 => DOUTSWAP_A::_101,
            6 => DOUTSWAP_A::_110,
            7 => DOUTSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DOUTSWAP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DOUTSWAP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DOUTSWAP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DOUTSWAP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DOUTSWAP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DOUTSWAP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DOUTSWAP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DOUTSWAP_A::_111
    }
}
#[doc = "Field `DOUTSWAP` writer - Byte/Word Swap Output image data in decompression is swapped."]
pub type DOUTSWAP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFDCNT_SPEC, u8, DOUTSWAP_A, 3, O>;
impl<'a, const O: u8> DOUTSWAP_W<'a, O> {
    #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_000)
    }
    #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_001)
    }
    #[doc = "(3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_010)
    }
    #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_011)
    }
    #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_100)
    }
    #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_101)
    }
    #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_110)
    }
    #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DOUTSWAP_A::_111)
    }
}
#[doc = "Field `DOUTLC` reader - Count Mode for Stopping Output Image Data Lines"]
pub type DOUTLC_R = crate::BitReader<DOUTLC_A>;
#[doc = "Count Mode for Stopping Output Image Data Lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTLC_A {
    #[doc = "0: Count mode for stopping the output of image data lines is off."]
    _0 = 0,
    #[doc = "1: Count mode for stopping the output of image data lines is on"]
    _1 = 1,
}
impl From<DOUTLC_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTLC_A) -> Self {
        variant as u8 != 0
    }
}
impl DOUTLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUTLC_A {
        match self.bits {
            false => DOUTLC_A::_0,
            true => DOUTLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTLC_A::_1
    }
}
#[doc = "Field `DOUTLC` writer - Count Mode for Stopping Output Image Data Lines"]
pub type DOUTLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, DOUTLC_A, O>;
impl<'a, const O: u8> DOUTLC_W<'a, O> {
    #[doc = "Count mode for stopping the output of image data lines is off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUTLC_A::_0)
    }
    #[doc = "Count mode for stopping the output of image data lines is on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUTLC_A::_1)
    }
}
#[doc = "Field `DOUTRCMD` writer - Output Image Data Lines Resume Command This bit is valid only when the count mode for stopping the output of image data lines is on. Setting this bit to 1 resumes writing image data. This bit is always read as 0."]
pub type DOUTRCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, bool, O>;
#[doc = "Field `DOUTRINI` reader - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
pub type DOUTRINI_R = crate::BitReader<DOUTRINI_A>;
#[doc = "Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTRINI_A {
    #[doc = "0: The transfer address is not initialized when the output of lines of image data is restarted."]
    _0 = 0,
    #[doc = "1: The transfer address is initialized when the output of lines of image data is restarted"]
    _1 = 1,
}
impl From<DOUTRINI_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTRINI_A) -> Self {
        variant as u8 != 0
    }
}
impl DOUTRINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUTRINI_A {
        match self.bits {
            false => DOUTRINI_A::_0,
            true => DOUTRINI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTRINI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTRINI_A::_1
    }
}
#[doc = "Field `DOUTRINI` writer - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
pub type DOUTRINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, DOUTRINI_A, O>;
impl<'a, const O: u8> DOUTRINI_W<'a, O> {
    #[doc = "The transfer address is not initialized when the output of lines of image data is restarted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOUTRINI_A::_0)
    }
    #[doc = "The transfer address is initialized when the output of lines of image data is restarted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOUTRINI_A::_1)
    }
}
#[doc = "Field `JINSWAP` reader - Byte/Word/Longword Swap Input coded data in decompression is swapped."]
pub type JINSWAP_R = crate::FieldReader<u8, JINSWAP_A>;
#[doc = "Byte/Word/Longword Swap Input coded data in decompression is swapped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JINSWAP_A {
    #[doc = "0: (1) (2) (3) (4) (5) (6) (7) (8)"]
    _000 = 0,
    #[doc = "1: (2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    _001 = 1,
    #[doc = "2: (3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    _010 = 2,
    #[doc = "3: (4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    _011 = 3,
    #[doc = "4: (5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    _100 = 4,
    #[doc = "5: (6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    _101 = 5,
    #[doc = "6: (7) (8) (5) (6) (3) (4) (1) (2) \\[Word -Halfword swap\\]"]
    _110 = 6,
    #[doc = "7: (8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    _111 = 7,
}
impl From<JINSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: JINSWAP_A) -> Self {
        variant as _
    }
}
impl JINSWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JINSWAP_A {
        match self.bits {
            0 => JINSWAP_A::_000,
            1 => JINSWAP_A::_001,
            2 => JINSWAP_A::_010,
            3 => JINSWAP_A::_011,
            4 => JINSWAP_A::_100,
            5 => JINSWAP_A::_101,
            6 => JINSWAP_A::_110,
            7 => JINSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == JINSWAP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == JINSWAP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == JINSWAP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == JINSWAP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == JINSWAP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == JINSWAP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == JINSWAP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == JINSWAP_A::_111
    }
}
#[doc = "Field `JINSWAP` writer - Byte/Word/Longword Swap Input coded data in decompression is swapped."]
pub type JINSWAP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFDCNT_SPEC, u8, JINSWAP_A, 3, O>;
impl<'a, const O: u8> JINSWAP_W<'a, O> {
    #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(JINSWAP_A::_000)
    }
    #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(JINSWAP_A::_001)
    }
    #[doc = "(3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(JINSWAP_A::_010)
    }
    #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(JINSWAP_A::_011)
    }
    #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(JINSWAP_A::_100)
    }
    #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(JINSWAP_A::_101)
    }
    #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word -Halfword swap\\]"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(JINSWAP_A::_110)
    }
    #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(JINSWAP_A::_111)
    }
}
#[doc = "Field `JINC` reader - Count Mode Setting for Stopping Input Coded Data"]
pub type JINC_R = crate::BitReader<JINC_A>;
#[doc = "Count Mode Setting for Stopping Input Coded Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINC_A {
    #[doc = "0: Count mode for stopping the input of coded data is off."]
    _0 = 0,
    #[doc = "1: Count mode for stopping the input of coded data is on"]
    _1 = 1,
}
impl From<JINC_A> for bool {
    #[inline(always)]
    fn from(variant: JINC_A) -> Self {
        variant as u8 != 0
    }
}
impl JINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JINC_A {
        match self.bits {
            false => JINC_A::_0,
            true => JINC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINC_A::_1
    }
}
#[doc = "Field `JINC` writer - Count Mode Setting for Stopping Input Coded Data"]
pub type JINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, JINC_A, O>;
impl<'a, const O: u8> JINC_W<'a, O> {
    #[doc = "Count mode for stopping the input of coded data is off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JINC_A::_0)
    }
    #[doc = "Count mode for stopping the input of coded data is on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JINC_A::_1)
    }
}
#[doc = "Field `JINRCMD` writer - Input Coded Data Resume CommandThis bit is valid only when the count mode for stopping the input of coded data is on. Setting this bit to 1 resumes reading input coded data. This bit is always read as 0."]
pub type JINRCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, bool, O>;
#[doc = "Field `JINRINI` reader - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit."]
pub type JINRINI_R = crate::BitReader<JINRINI_A>;
#[doc = "Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINRINI_A {
    #[doc = "0: The transfer address is not initialized when the input of coded data is restarted."]
    _0 = 0,
    #[doc = "1: The transfer address is initialized when the input of coded data is restarted."]
    _1 = 1,
}
impl From<JINRINI_A> for bool {
    #[inline(always)]
    fn from(variant: JINRINI_A) -> Self {
        variant as u8 != 0
    }
}
impl JINRINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JINRINI_A {
        match self.bits {
            false => JINRINI_A::_0,
            true => JINRINI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINRINI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINRINI_A::_1
    }
}
#[doc = "Field `JINRINI` writer - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit."]
pub type JINRINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFDCNT_SPEC, JINRINI_A, O>;
impl<'a, const O: u8> JINRINI_W<'a, O> {
    #[doc = "The transfer address is not initialized when the input of coded data is restarted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JINRINI_A::_0)
    }
    #[doc = "The transfer address is initialized when the input of coded data is restarted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JINRINI_A::_1)
    }
}
#[doc = "Field `OPF` reader - Specifies output image data pixel format."]
pub type OPF_R = crate::FieldReader<u8, OPF_A>;
#[doc = "Specifies output image data pixel format.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPF_A {
    #[doc = "1: ARGB8888"]
    _01 = 1,
    #[doc = "2: RGB565"]
    _10 = 2,
}
impl From<OPF_A> for u8 {
    #[inline(always)]
    fn from(variant: OPF_A) -> Self {
        variant as _
    }
}
impl OPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPF_A> {
        match self.bits {
            1 => Some(OPF_A::_01),
            2 => Some(OPF_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OPF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OPF_A::_10
    }
}
#[doc = "Field `OPF` writer - Specifies output image data pixel format."]
pub type OPF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDCNT_SPEC, u8, OPF_A, 2, O>;
impl<'a, const O: u8> OPF_W<'a, O> {
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OPF_A::_01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OPF_A::_10)
    }
}
#[doc = "Field `HINTER` reader - Horizontal Subsampling Subsamples horizontal output image data."]
pub type HINTER_R = crate::FieldReader<u8, HINTER_A>;
#[doc = "Horizontal Subsampling Subsamples horizontal output image data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HINTER_A {
    #[doc = "0: No subsampling"]
    _00 = 0,
    #[doc = "1: Subsamples output data into 1/2."]
    _01 = 1,
    #[doc = "2: Subsamples output data into 1/4."]
    _10 = 2,
    #[doc = "3: Subsamples output data into 1/8."]
    _11 = 3,
}
impl From<HINTER_A> for u8 {
    #[inline(always)]
    fn from(variant: HINTER_A) -> Self {
        variant as _
    }
}
impl HINTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTER_A {
        match self.bits {
            0 => HINTER_A::_00,
            1 => HINTER_A::_01,
            2 => HINTER_A::_10,
            3 => HINTER_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HINTER_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HINTER_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HINTER_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HINTER_A::_11
    }
}
#[doc = "Field `HINTER` writer - Horizontal Subsampling Subsamples horizontal output image data."]
pub type HINTER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFDCNT_SPEC, u8, HINTER_A, 2, O>;
impl<'a, const O: u8> HINTER_W<'a, O> {
    #[doc = "No subsampling"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(HINTER_A::_00)
    }
    #[doc = "Subsamples output data into 1/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(HINTER_A::_01)
    }
    #[doc = "Subsamples output data into 1/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(HINTER_A::_10)
    }
    #[doc = "Subsamples output data into 1/8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(HINTER_A::_11)
    }
}
#[doc = "Field `VINTER` reader - Vertical SubsamplingSubsamples vertical output image data."]
pub type VINTER_R = crate::FieldReader<u8, VINTER_A>;
#[doc = "Vertical SubsamplingSubsamples vertical output image data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VINTER_A {
    #[doc = "0: No subsampling"]
    _00 = 0,
    #[doc = "1: Subsamples output data into 1/2."]
    _01 = 1,
    #[doc = "2: Subsamples output data into 1/4."]
    _10 = 2,
    #[doc = "3: Subsamples output data into 1/8."]
    _11 = 3,
}
impl From<VINTER_A> for u8 {
    #[inline(always)]
    fn from(variant: VINTER_A) -> Self {
        variant as _
    }
}
impl VINTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VINTER_A {
        match self.bits {
            0 => VINTER_A::_00,
            1 => VINTER_A::_01,
            2 => VINTER_A::_10,
            3 => VINTER_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == VINTER_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == VINTER_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == VINTER_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == VINTER_A::_11
    }
}
#[doc = "Field `VINTER` writer - Vertical SubsamplingSubsamples vertical output image data."]
pub type VINTER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFDCNT_SPEC, u8, VINTER_A, 2, O>;
impl<'a, const O: u8> VINTER_W<'a, O> {
    #[doc = "No subsampling"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(VINTER_A::_00)
    }
    #[doc = "Subsamples output data into 1/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(VINTER_A::_01)
    }
    #[doc = "Subsamples output data into 1/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(VINTER_A::_10)
    }
    #[doc = "Subsamples output data into 1/8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(VINTER_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:2 - Byte/Word Swap Output image data in decompression is swapped."]
    #[inline(always)]
    pub fn doutswap(&self) -> DOUTSWAP_R {
        DOUTSWAP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Count Mode for Stopping Output Image Data Lines"]
    #[inline(always)]
    pub fn doutlc(&self) -> DOUTLC_R {
        DOUTLC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    pub fn doutrini(&self) -> DOUTRINI_R {
        DOUTRINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Byte/Word/Longword Swap Input coded data in decompression is swapped."]
    #[inline(always)]
    pub fn jinswap(&self) -> JINSWAP_R {
        JINSWAP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Count Mode Setting for Stopping Input Coded Data"]
    #[inline(always)]
    pub fn jinc(&self) -> JINC_R {
        JINC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit."]
    #[inline(always)]
    pub fn jinrini(&self) -> JINRINI_R {
        JINRINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Specifies output image data pixel format."]
    #[inline(always)]
    pub fn opf(&self) -> OPF_R {
        OPF_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Horizontal Subsampling Subsamples horizontal output image data."]
    #[inline(always)]
    pub fn hinter(&self) -> HINTER_R {
        HINTER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Vertical SubsamplingSubsamples vertical output image data."]
    #[inline(always)]
    pub fn vinter(&self) -> VINTER_R {
        VINTER_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Byte/Word Swap Output image data in decompression is swapped."]
    #[inline(always)]
    #[must_use]
    pub fn doutswap(&mut self) -> DOUTSWAP_W<0> {
        DOUTSWAP_W::new(self)
    }
    #[doc = "Bit 4 - Count Mode for Stopping Output Image Data Lines"]
    #[inline(always)]
    #[must_use]
    pub fn doutlc(&mut self) -> DOUTLC_W<4> {
        DOUTLC_W::new(self)
    }
    #[doc = "Bit 5 - Output Image Data Lines Resume Command This bit is valid only when the count mode for stopping the output of image data lines is on. Setting this bit to 1 resumes writing image data. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn doutrcmd(&mut self) -> DOUTRCMD_W<5> {
        DOUTRCMD_W::new(self)
    }
    #[doc = "Bit 6 - Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    #[must_use]
    pub fn doutrini(&mut self) -> DOUTRINI_W<6> {
        DOUTRINI_W::new(self)
    }
    #[doc = "Bits 8:10 - Byte/Word/Longword Swap Input coded data in decompression is swapped."]
    #[inline(always)]
    #[must_use]
    pub fn jinswap(&mut self) -> JINSWAP_W<8> {
        JINSWAP_W::new(self)
    }
    #[doc = "Bit 12 - Count Mode Setting for Stopping Input Coded Data"]
    #[inline(always)]
    #[must_use]
    pub fn jinc(&mut self) -> JINC_W<12> {
        JINC_W::new(self)
    }
    #[doc = "Bit 13 - Input Coded Data Resume CommandThis bit is valid only when the count mode for stopping the input of coded data is on. Setting this bit to 1 resumes reading input coded data. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn jinrcmd(&mut self) -> JINRCMD_W<13> {
        JINRCMD_W::new(self)
    }
    #[doc = "Bit 14 - Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit."]
    #[inline(always)]
    #[must_use]
    pub fn jinrini(&mut self) -> JINRINI_W<14> {
        JINRINI_W::new(self)
    }
    #[doc = "Bits 24:25 - Specifies output image data pixel format."]
    #[inline(always)]
    #[must_use]
    pub fn opf(&mut self) -> OPF_W<24> {
        OPF_W::new(self)
    }
    #[doc = "Bits 26:27 - Horizontal Subsampling Subsamples horizontal output image data."]
    #[inline(always)]
    #[must_use]
    pub fn hinter(&mut self) -> HINTER_W<26> {
        HINTER_W::new(self)
    }
    #[doc = "Bits 28:29 - Vertical SubsamplingSubsamples vertical output image data."]
    #[inline(always)]
    #[must_use]
    pub fn vinter(&mut self) -> VINTER_W<28> {
        VINTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifdcnt](index.html) module"]
pub struct JIFDCNT_SPEC;
impl crate::RegisterSpec for JIFDCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifdcnt::R](R) reader structure"]
impl crate::Readable for JIFDCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifdcnt::W](W) writer structure"]
impl crate::Writable for JIFDCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDCNT to value 0x0100_0000"]
impl crate::Resettable for JIFDCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
